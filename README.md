# android_serial_demo

Minimal demo showing serial communication on Android (USB-serial adapters and device node access). Includes example code to open/close ports, send/receive data, and basic UI for testing.

## 特性
- USB 串口（FTDI、CP210x、CH34x），通过 usb-serial-for-android
- 直接访问设备节点（/dev/ttyS*、/dev/ttyUSB*），适用于已 root 的设备或平台构建
- 可配置的波特率、数据位、校验位、停止位
- 简单的发送/接收界面和日志视图

## 先决条件
- Android Studio（推荐）或 Gradle
- Android SDK
- 可选：usb-serial-for-android 依赖（Gradle）或在使用设备节点时的本地 JNI 串口库

## 构建与运行
1. 在 Android Studio 中打开项目。
2. 让 Gradle 同步并下载依赖项。
3. 连接 Android 设备（启用 USB 调试）。
4. 在设备上运行应用。

## 使用方法
- 当提示时授予 USB 权限（用于 USB 串口适配器）。
- 在应用 UI 中选择设备或设备节点。
- 配置串口参数（波特率、校验、数据位、停止位）。
- 点击 连接，然后通过输入框发送/接收数据，并查看日志。

## 故障排除
- 确保主机上安装了正确的 USB 驱动（如果通过 ADB 转发使用适配器）。
- 对于设备节点，确保有适当的权限或在已 root 的设备/平台构建上运行以获得访问权限。
- 检查 logcat 以获取错误和堆栈跟踪。

## 贡献
欢迎 PR 和 issue。请保持修改小且有文档说明。

## 许可证
请指定项目的许可证（例如 MIT）。

## Rust 语法学习示例

本仓库包含一个轻量的学习示例，演示 Rust 常见语法与概念（变量、所有权、借用、结构体、枚举、Trait、泛型、闭包、迭代器、错误处理等）。

运行示例：

```bash
cargo run --example learning_demo
```

示例不会触碰串口硬件，仅用于学习语言语法。如果想恢复原来的串口演示，请运行主二进制：

```bash
cargo run --bin android_serial_demo
```

更多示例（位于 `examples/`）:

- `learning_demo` - 综合语法示例（已添加）。
- `file_io` - 文件读写与复制示例。
- `threading` - 线程与 `mpsc` 通道示例。
- `tcp_echo` - 简单 TCP Echo（本地监听与客户端示例）。

运行所有 examples 的快速方式：

```bash
cargo run --example learning_demo
cargo run --example file_io
cargo run --example threading
cargo run --example tcp_echo
```

说明：`tcp_echo` 绑定到本机端口 `127.0.0.1:4000`，仅用于本机学习测试。

进阶示例（位于 `examples/`）：

- `shared_state` - 使用 `Arc<Mutex<T>>` 在多个线程间共享并修改状态的示例。
- `scoped_threads` - 演示 `thread::scope`，展示如何安全地把栈上数据借用到线程中（线程在 scope 返回前结束）。
- `async_tcp` - 基于 `tokio` 的异步 TCP echo 示例（绑定 `127.0.0.1:5000`）。

运行进阶示例：

```bash
cargo run --example shared_state
cargo run --example scoped_threads
cargo run --example async_tcp
```