use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("-- 共享可变状态（Arc<Mutex<T>）示例 --");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&counter);
        let h = thread::spawn(move || {
            for _ in 0..100 {
                let mut num = c.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(h);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("最终计数 = {}", *counter.lock().unwrap());
}
