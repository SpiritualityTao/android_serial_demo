use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("-- 线程与通道示例 --");

    let (tx, rx) = mpsc::channel();

    // 生产者线程
    let producer = thread::spawn(move || {
        for i in 1..=5 {
            println!("producer: sending {}", i);
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    // 消费者在主线程
    for received in rx {
        println!("consumer: got {}", received);
    }

    producer.join().unwrap();
    println!("线程示例完成");
}
