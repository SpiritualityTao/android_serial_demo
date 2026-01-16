// 异步 TCP Echo 示例，使用 tokio 运行时
// 运行：cargo run --example async_tcp

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("-- tokio 异步 TCP Echo 示例 --");

    let listener = TcpListener::bind("127.0.0.1:5000").await?;
    println!("listening on 127.0.0.1:5000");

    // 接受一个连接并回显（示例简化，只处理一个客户端）
    tokio::spawn(async move {
        if let Ok((mut socket, addr)) = listener.accept().await {
            println!("accepted {}", addr);
            let mut buf = vec![0u8; 1024];
            match socket.read(&mut buf).await {
                Ok(n) if n > 0 => {
                    println!("server got {} bytes", n);
                    let _ = socket.write_all(&buf[..n]).await;
                }
                _ => println!("no data or read error"),
            }
        }
    });

    // 客户端示例
    let mut client = TcpStream::connect("127.0.0.1:5000").await?;
    client.write_all(b"hello async world").await?;
    let mut r = Vec::new();
    client.read_to_end(&mut r).await.ok();
    println!("client got: {}", String::from_utf8_lossy(&r));

    Ok(())
}
