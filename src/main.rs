use anyhow::{Result, anyhow};
use clap::Parser;
use std::time::{Duration, Instant};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;
use rustls::OwnedTrustAnchor;
use url::Url;
use webpki_roots;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// URL to ping
    url: String,

    /// Number of times to ping
    #[arg(short = 'c', long, default_value_t = 4)]
    count: u32,

    /// Interval between pings in seconds
    #[arg(short = 't', long, default_value_t = 1.0)]
    interval: f32,

    /// HTTP method
    #[arg(short = 'X', long = "request", default_value = "GET")]
    method: String,

    /// HTTP headers
    #[arg(short = 'H', long = "header", value_parser = parse_header)]
    headers: Vec<(String, String)>,

    /// HTTP request data
    #[arg(short = 'd', long = "data")]
    data: Option<String>,
}

fn parse_header(s: &str) -> Result<(String, String), String> {
    let parts: Vec<&str> = s.splitn(2, ':').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid header format: {}", s));
    }
    Ok((parts[0].trim().to_string(), parts[1].trim().to_string()))
}

async fn http_ping(input_url: &str, show_warning: bool, method: &str, headers: &[(String, String)], data: &Option<String>) -> Result<(Duration, String, u16)> {
    let start = Instant::now();

    // Handle URL without scheme
    let url_str = if !input_url.contains("://") {
        if show_warning {
            eprintln!("httpping: warning: URL scheme not specified, using https:// by default");
        }
        format!("https://{}", input_url)
    } else {
        input_url.to_string()
    };

    let url = Url::parse(&url_str)?;
    let host = url.host_str().ok_or_else(|| anyhow!("Invalid URL: missing host. Use -h for help."))?;
    let port = url.port_or_known_default().ok_or_else(|| anyhow!("Invalid port"))?;

    let addr = format!("{}:{}", host, port);
    let tcp_stream = TcpStream::connect(addr).await?;

    let mut request = format!(
        "{} {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n",
        method,
        url.path(),
        host
    );

    // Add custom headers
    for (key, value) in headers {
        request.push_str(&format!("{}: {}\r\n", key, value));
    }

    // Add Content-Length header if there's data
    if let Some(data) = data {
        request.push_str(&format!("Content-Length: {}\r\n", data.len()));
    }

    request.push_str("\r\n");

    // Add data if present
    if let Some(data) = data {
        request.push_str(data);
    }

    let mut buffer = [0; 4096];
    let mut response = String::new();
    let mut duration = Duration::new(0, 0);
    let mut first_byte_received = false;

    if url.scheme() == "https" {
        let mut root_store = rustls::RootCertStore::empty();
        root_store.add_trust_anchors(
            webpki_roots::TLS_SERVER_ROOTS
                .iter()
                .map(|ta| {
                    OwnedTrustAnchor::from_subject_spki_name_constraints(
                        ta.subject,
                        ta.spki,
                        ta.name_constraints,
                    )
                })
        );
        let config = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_store)
            .with_no_client_auth();
        let connector = TlsConnector::from(Arc::new(config));
        let domain = rustls::ServerName::try_from(host)
            .map_err(|_| anyhow!("Invalid DNS name"))?;
        let mut tls_stream = connector.connect(domain, tcp_stream).await?;
        tls_stream.write_all(request.as_bytes()).await?;
        
        loop {
            let bytes_read = tls_stream.read(&mut buffer).await?;
            if bytes_read == 0 {
                break;
            }
            if !first_byte_received {
                duration = start.elapsed();
                first_byte_received = true;
            }
            response.push_str(&String::from_utf8_lossy(&buffer[..bytes_read]));
            if response.contains("\r\n\r\n") {
                break;
            }
        }
    } else {
        let mut tcp_stream = tcp_stream;
        tcp_stream.write_all(request.as_bytes()).await?;
        
        loop {
            let bytes_read = tcp_stream.read(&mut buffer).await?;
            if bytes_read == 0 {
                break;
            }
            if !first_byte_received {
                duration = start.elapsed();
                first_byte_received = true;
            }
            response.push_str(&String::from_utf8_lossy(&buffer[..bytes_read]));
            if response.contains("\r\n\r\n") {
                break;
            }
        }
    }

    let status_code = response.lines().next()
        .and_then(|status_line| status_line.split_whitespace().nth(1))
        .and_then(|code| code.parse().ok())
        .unwrap_or(0);

    Ok((duration, url.to_string(), status_code))
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let mut total_duration = Duration::new(0, 0);
    let mut successful_pings = 0;
    let mut show_warning = true;

    for i in 1..=args.count {
        match http_ping(&args.url, show_warning, &args.method, &args.headers, &args.data).await {
            Ok((duration, full_url, status_code)) => {
                total_duration += duration;
                successful_pings += 1;
                println!(
                    "Ping {}: URL: {} - Method: {} - Status: {} - Time: {} ms",
                    i,
                    full_url,
                    args.method,
                    status_code,
                    duration.as_millis()
                );
                show_warning = false; // Disable warning after first successful ping
            }
            Err(e) => {
                println!("Ping {} failed: {}", i, e);
            }
        }

        if i < args.count {
            tokio::time::sleep(tokio::time::Duration::from_secs_f32(args.interval)).await;
        }
    }

    if successful_pings > 0 {
        let average_duration = total_duration / successful_pings;
        println!("\nAverage response time: {} ms", average_duration.as_millis());
    } else {
        println!("\nNo successful pings");
    }

    Ok(())
}
