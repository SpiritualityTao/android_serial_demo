use std::thread;

fn main() {
    println!("-- scoped thread 示例（thread::scope） --");

    let mut data = vec![10, 20, 30];

    // 在 scope 内可以借用栈上数据到线程里（线程必须在 scope 结束前结束）
    thread::scope(|s| {
        s.spawn(|| {
            // 这里借用了外部的 `data`（不可跨越 scope）
            println!("scoped thread read data[0] = {}", data[0]);
        });
    });

    println!("scope 结束后继续使用 data: {:?}", data);
}
