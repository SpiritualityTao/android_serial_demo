use anyhow::Result;
use serialport::SerialPort;
use std::time::Duration;

///1 串口配置与读写核心函数
fn serial_communication() -> Result<()> {
    // 1. 配置串口参数（根据你的硬件设备调整）
    let serial_port_path = "/dev/ttyUS0"; // Android设备上的串口设备路径（关键！）
                                           // 常见串口路径：/dev/ttyUSB0（USB转串口）、/dev/ttyS0（内置串口）、/dev/ttyACM0（ACM设备）
    let baud_rate = 115200; // 波特率（必须与外设一致）
    let data_bits = serialport::DataBits::Eight;
    let stop_bits = serialport::StopBits::One;
    let parity = serialport::Parity::None;
    let flow_control = serialport::FlowControl::None;

    // 2. 打开串口
    println!("正在打开串口：{}，波特率：{}", serial_port_path, baud_rate);
    let mut port = serialport::new(serial_port_path, baud_rate)
        .data_bits(data_bits)
        .stop_bits(stop_bits)
        .parity(parity)
        .flow_control(flow_control)
        .timeout(Duration::from_secs(5)) // 读写超时时间：5秒
        .open()?;
    println!("串口打开成功！");

    // 3. 向串口写入数据（示例：发送字符串）
    let send_data = b"Hello, Serial Port!"; // 要发送的字节数据
    port.write(send_data)?;
    println!("已向串口写入数据：{}", String::from_utf8_lossy(send_data));
    println!("等待从串口读取数据...");

    // 4. 从串口读取数据（示例：读取最多1024字节）
    let mut recv_buffer = [0u8; 1024]; // 接收数据缓冲区
    let recv_size = port.read(&mut recv_buffer)?;
    let recv_data = &recv_buffer[0..recv_size];

    if recv_size > 0 {
        println!("从串口读取到 {} 字节数据：", recv_size);
        println!("十六进制：{:x?}", recv_data);
        println!("字符串格式：{}", String::from_utf8_lossy(recv_data));
    } else {
        println!("超时未读取到串口数据");
    }

    // 5. 关闭串口（Rust会自动释放资源，此处可省略手动关闭）
    Ok(())
}

fn main() {
    // 调用串口通信函数，处理错误
    match serial_communication() {
        Ok(_) => println!("串口通信流程执行完成！"),
        Err(e) => eprintln!("串口通信出错：{}", e),
    }
}
