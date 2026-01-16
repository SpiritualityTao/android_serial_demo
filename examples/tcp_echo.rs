use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0u8; 512];
    match stream.read(&mut buf) {
        Ok(n) if n > 0 => {
            println!("server received: {} bytes", n);
            let _ = stream.write_all(&buf[0..n]);
        }
        _ => println!("no data or read error"),
    }
}

fn main() -> std::io::Result<()> {
    println!("-- 简单 TCP Echo 示例 --");
    let listener = TcpListener::bind("127.0.0.1:4000")?;
    println!("Listening on 127.0.0.1:4000");

    // 在后台线程接受客户端
    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(s) => {
                    println!("新连接来自 {}", s.peer_addr().unwrap());
                    thread::spawn(|| handle_client(s));
                }
                Err(e) => eprintln!("accept error: {}", e),
            }
        }
    });

    // 客户端示例：连接并发送数据
    thread::sleep(std::time::Duration::from_millis(200));
    if let Ok(mut client) = TcpStream::connect("127.0.0.1:4000") {
        client.write_all(b"hello tcp server")?;
        let mut r = vec![];
        client.read_to_end(&mut r).ok();
        println!("client got echo: {}", String::from_utf8_lossy(&r));
    }

    // 保持主线程短暂等待观察输出
    thread::sleep(std::time::Duration::from_millis(500));
    Ok(())
}
