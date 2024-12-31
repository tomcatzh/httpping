# httpping

httpping is a command-line tool for HTTP/HTTPS ping, written in Rust. It allows you to measure the response time of web servers.

## Features

- Support for both HTTP and HTTPS
- Customizable number of pings
- Adjustable interval between pings
- Automatic HTTPS scheme addition if not specified

## Usage

```
httpping [OPTIONS] <URL>

Arguments:
  <URL>  URL to ping

Options:
  -c, --count <COUNT>        Number of times to ping [default: 4]
  -t, --interval <INTERVAL>  Interval between pings in seconds [default: 1.0]
  -h, --help                 Print help
  -V, --version              Print version
```

## Example

```
httpping example.com -c 5 -t 2
```

This will ping `example.com` 5 times with a 2-second interval between each ping.

## Installation

To install httpping, you need to have Rust and Cargo installed on your system. Then, you can build and install it using:

```
cargo install --path .
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
