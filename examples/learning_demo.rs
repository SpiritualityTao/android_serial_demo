// Rust 学习示例：演示常见语法与概念
// 运行：`cargo run --example learning_demo`

use std::collections::HashMap;

fn variables_and_shadowing() {
    println!("-- 变量与可变性 --");
    let x = 5; // 不可变
    println!("x = {}", x);
    let mut y = 10; // 可变
    println!("y = {}", y);
    y += 2;
    println!("y after += 2 = {}", y);

    // 影子（shadowing）可以重新绑定同名变量
    let x = x + 1;
    println!("x shadowed = {}", x);
}

fn ownership_and_borrowing() {
    println!("-- 所有权与借用 --");
    let s = String::from("hello"); // s 拥有字符串
    takes_ownership(s); // s 被移动，不能再使用
    // println!("s = {}", s); // 编译错误，已被移动

    let a = 42;
    makes_copy(a); // 基本类型实现 Copy，仍可使用
    println!("a after makes_copy = {}", a);

    let mut s2 = String::from("rust");
    borrow_mutably(&mut s2);
    println!("s2 after borrow_mutably = {}", s2);
}

fn takes_ownership(s: String) {
    println!("takes_ownership got: {}", s);
}

fn makes_copy(n: i32) {
    println!("makes_copy got: {}", n);
}

fn borrow_mutably(s: &mut String) {
    s.push_str("lang");
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn structs_and_methods() {
    println!("-- 结构体与方法 --");
    let p = Point { x: 3, y: 4 };
    println!("point = {:?}", p);

    let p2 = Point::origin();
    println!("origin = {:?}", p2);
}

impl Point {
    fn origin() -> Self {
        Point { x: 0, y: 0 }
    }

    fn distance_from_origin(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn enums_and_pattern_matching() {
    println!("-- 枚举与模式匹配 --");
    let msgs = vec![
        Message::Quit,
        Message::Move { x: 10, y: -2 },
        Message::Write(String::from("hi")),
        Message::ChangeColor(255, 0, 128),
    ];

    for m in msgs {
        match m {
            Message::Quit => println!("got Quit"),
            Message::Move { x, y } => println!("Move to {}, {}", x, y),
            Message::Write(s) => println!("Write( {} )", s),
            Message::ChangeColor(r, g, b) => println!("Color({}, {}, {})", r, g, b),
        }
    }
}

trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for Point {
    fn summarize(&self) -> String {
        format!("Point({}, {})", self.x, self.y)
    }
}

fn traits_and_generics() {
    println!("-- Trait 与泛型 --");
    let p = Point { x: 7, y: 8 };
    println!("summary: {}", p.summarize());

    // 泛型函数
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let nums = [3, 5, 1, 7, 2];
    println!("largest = {}", largest(&nums));
}

fn closures_and_iterators() {
    println!("-- 闭包与迭代器 --");
    let list = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = list.iter().map(|x| x * 2).collect();
    println!("doubled = {:?}", doubled);

    // 捕获环境值的闭包
    let factor = 3;
    let times = |v: i32| v * factor;
    println!("times(4) = {}", times(4));
}

fn hashmap_example() {
    println!("-- HashMap 示例 --");
    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 50);
    scores.insert(String::from("Bob"), 40);
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }
}

fn error_handling_examples() -> Result<(), Box<dyn std::error::Error>> {
    println!("-- 错误处理示例 --");

    // 使用 Option
    let maybe_number: Option<i32> = Some(10);
    if let Some(n) = maybe_number {
        println!("Option contains {}", n);
    }

    // 使用 Result
    fn parse_int(s: &str) -> Result<i32, std::num::ParseIntError> {
        s.parse::<i32>()
    }

    match parse_int("42")? {
        n => println!("parsed = {}", n),
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Rust 语法学习 Demo\n");

    variables_and_shadowing();
    ownership_and_borrowing();
    structs_and_methods();
    println!("Point distance = {}", Point { x: 3, y: 4 }.distance_from_origin());
    enums_and_pattern_matching();
    traits_and_generics();
    closures_and_iterators();
    hashmap_example();
    error_handling_examples()?;

    println!("\n示例执行完成。");
    Ok(())
}
