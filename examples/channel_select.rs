// 演示 crossbeam-channel 的 select 与超时用法
// 运行：cargo run --example channel_select

use crossbeam_channel::{bounded, select, tick, Receiver};
use std::thread;
use std::time::Duration;

fn spawn_producer(id: &str, rx: Receiver<()>, sender: crossbeam_channel::Sender<String>, delay_ms: u64) {
    let id = id.to_string();
    thread::spawn(move || {
        let mut i = 1;
        while rx.try_recv().is_err() {
            let msg = format!("{}-msg-{}", id, i);
            sender.send(msg).unwrap();
            i += 1;
            thread::sleep(Duration::from_millis(delay_ms));
        }
    });
}

fn main() {
    println!("-- Channel select & timeout 示例 --");

    // 主通道：两个生产者发消息到 main_rx
    let (stop_s, stop_r) = bounded::<()>(0);
    let (s1, r1) = bounded::<String>(100);
    let (s2, r2) = bounded::<String>(100);

    // 启动两个生产者，分别速度不同
    spawn_producer("fast", stop_r.clone(), s1.clone(), 150);
    spawn_producer("slow", stop_r.clone(), s2.clone(), 400);

    // 使用一个 tick 定时器作为超时信号
    let ticker = tick(Duration::from_millis(500));

    // 主循环：select 多路接收，并在超时时打印提示
    for _ in 0..10 {
        select! {
            recv(r1) -> msg => match msg {
                Ok(m) => println!("received from fast: {}", m),
                Err(_) => println!("fast channel closed"),
            },
            recv(r2) -> msg => match msg {
                Ok(m) => println!("received from slow: {}", m),
                Err(_) => println!("slow channel closed"),
            },
            recv(ticker) -> _ => {
                println!("timeout: no messages within 500ms");
            }
        }
    }

    // 请求生产者停止（发送 stop 信号）
    let _ = stop_s.send(());
    println!("示例结束");
}
