use futures::future::join_all;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("-- stress client 示例：并发连接并发送消息 --");

    let clients = 50_usize; // 并发客户端数量，可调整
    let msgs_per = 30_usize; // 每个客户端发送消息数量

    let mut tasks = Vec::with_capacity(clients);
    for i in 0..clients {
        tasks.push(tokio::spawn(async move {
            if let Ok(mut s) = TcpStream::connect("127.0.0.1:6000").await {
                for j in 0..msgs_per {
                    let data = format!("client{}-msg{}\n", i, j);
                    if s.write_all(data.as_bytes()).await.is_err() {
                        break;
                    }
                    let mut buf = vec![0u8; 1024];
                    match s.read(&mut buf).await {
                        Ok(0) => break,
                        Ok(_n) => { /* got echo */ }
                        Err(_) => break,
                    }
                }
            }
        }));
    }

    join_all(tasks).await;
    println!("stress test 完成");
    Ok(())
}
