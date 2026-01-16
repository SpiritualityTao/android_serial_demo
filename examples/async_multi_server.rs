use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("-- async multi-client TCP server 示例 --");
    let listener = TcpListener::bind("127.0.0.1:6000").await?;
    let active = Arc::new(AtomicUsize::new(0));
    println!("listening on 127.0.0.1:6000");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        let a = active.clone();
        a.fetch_add(1, Ordering::SeqCst);

        tokio::spawn(async move {
            println!("accepted {}", addr);
            let mut buf = [0u8; 1024];
            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => break, // connection closed
                    Ok(n) => {
                        // echo back
                        if let Err(e) = socket.write_all(&buf[..n]).await {
                            eprintln!("write error: {}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("read error: {}", e);
                        break;
                    }
                }
            }
            a.fetch_sub(1, Ordering::SeqCst);
            println!("closed {}", addr);
        });
    }
}
