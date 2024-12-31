# httpping

httpping 是一个用 Rust 编写的 HTTP/HTTPS ping 命令行工具。它可以用来测量网络服务器的响应时间。

## 特性

- 支持 HTTP 和 HTTPS
- 可自定义 ping 的次数
- 可调整 ping 的间隔时间
- 如果未指定协议，自动添加 HTTPS 协议

## 使用方法

```
httpping [选项] <URL>

参数:
  <URL>  要 ping 的 URL

选项:
  -c, --count <COUNT>        ping 的次数 [默认: 4]
  -t, --interval <INTERVAL>  每次 ping 之间的间隔时间（秒） [默认: 1.0]
  -h, --help                 打印帮助信息
  -V, --version              打印版本信息
```

## 示例

```
httpping example.com -c 5 -t 2
```

这将对 `example.com` 进行 5 次 ping，每次 ping 之间间隔 2 秒。

## 安装

要安装 httpping，您需要在系统上安装 Rust 和 Cargo。然后，您可以使用以下命令构建和安装：

```
cargo install --path .
```

## 许可证

该项目采用 MIT 许可证 - 详情请参阅 [LICENSE](LICENSE) 文件。
