use std::fs::{self, File};
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    println!("-- 文件 I/O 示例 --");

    // 1. 写入文件
    let mut f = File::create("example_output.txt")?;
    f.write_all(b"Hello from Rust file I/O!\n")?;
    println!("写入 example_output.txt");

    // 2. 读取文件
    let mut s = String::new();
    File::open("example_output.txt")?.read_to_string(&mut s)?;
    println!("读取内容:\n{}", s);

    // 3. 简单复制
    fs::copy("example_output.txt", "example_output.copy.txt")?;
    println!("已复制到 example_output.copy.txt");

    // 4. 清理（可选）
    // fs::remove_file("example_output.txt")?;
    // fs::remove_file("example_output.copy.txt")?;

    Ok(())
}
