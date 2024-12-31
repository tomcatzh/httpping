# httpping

English | [中文](README.zh.md)

httpping is a command-line tool for HTTP/HTTPS ping, written in Rust. It allows you to measure the response time of web servers and supports various HTTP methods and custom headers.

## Features

- Support for both HTTP and HTTPS
- Customizable number of pings
- Adjustable interval between pings
- Automatic HTTPS scheme addition if not specified
- Support for different HTTP methods (GET, POST, etc.)
- Custom headers
- Request data support

## Usage

```
httpping [OPTIONS] <URL>

Arguments:
  <URL>  URL to ping

Options:
  -c, --count <COUNT>        Number of times to ping [default: 4]
  -t, --interval <INTERVAL>  Interval between pings in seconds [default: 1.0]
  -X, --request <METHOD>     HTTP method to use [default: GET]
  -H, --header <HEADER>      Custom header(s) to include (can be used multiple times)
  -d, --data <DATA>          Data to include in the request body
  -h, --help                 Print help
  -V, --version              Print version
```

## Examples

1. Simple GET request:
```
httpping example.com -c 5 -t 2
```
This will ping `example.com` 5 times with a 2-second interval between each ping. The output will now include the HTTP status code:

```
Ping 1: URL: https://example.com - Method: GET - Status: 200 - Time: 123 ms
Ping 2: URL: https://example.com - Method: GET - Status: 200 - Time: 118 ms
Ping 3: URL: https://example.com - Method: GET - Status: 200 - Time: 120 ms
Ping 4: URL: https://example.com - Method: GET - Status: 200 - Time: 119 ms
Ping 5: URL: https://example.com - Method: GET - Status: 200 - Time: 121 ms

Average response time: 120 ms
```

2. POST request with custom headers and data:
```
httpping http://localhost:8866/admin/api-key/apply -X POST -H "Content-Type: application/json" -H "Authorization: Bearer br_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx" -d '{"name": "adminuser","group_id": 1,"role": "admin","email": "", "month_quota":"20"}'
```
This will send a POST request to the specified URL with custom headers and JSON data in the request body.

## Installation

To install httpping, you need to have Rust and Cargo installed on your system. Then, you can build and install it using:

```
cargo install --path .
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
