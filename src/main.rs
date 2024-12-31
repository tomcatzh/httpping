use anyhow::{Result, anyhow};
use clap::Parser;
use native_tls::TlsConnector;
use std::time::{Duration, Instant};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use url::Url;

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
}

async fn http_ping(input_url: &str, show_warning: bool) -> Result<(Duration, String)> {
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
    let mut tcp_stream = TcpStream::connect(addr).await?;

    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        url.path(),
        host
    );

    if url.scheme() == "https" {
        let tls_connector = TlsConnector::new()?;
        let tls_connector = tokio_native_tls::TlsConnector::from(tls_connector);
        let mut tls_stream = tls_connector.connect(host, tcp_stream).await?;
        tls_stream.write_all(request.as_bytes()).await?;

        let mut buffer = [0; 1024];
        tls_stream.read(&mut buffer).await?;
    } else {
        tcp_stream.write_all(request.as_bytes()).await?;

        let mut buffer = [0; 1024];
        tcp_stream.read(&mut buffer).await?;
    }

    let duration = start.elapsed();
    Ok((duration, url.to_string()))
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let mut total_duration = Duration::new(0, 0);
    let mut successful_pings = 0;
    let mut show_warning = true;

    for i in 1..=args.count {
        match http_ping(&args.url, show_warning).await {
            Ok((duration, full_url)) => {
                total_duration += duration;
                successful_pings += 1;
                println!(
                    "Ping {}: URL: {} - Time: {} ms",
                    i,
                    full_url,
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
