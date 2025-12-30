// examples/generics.rs
// 泛型演示

use module_07_traits_generics::{largest, max, Pair};

fn main() {
    println!("=== 泛型演示 ===\n");

    // 1. 泛型结构体
    println!("1. 泛型结构体:");
    let pair = Pair::new(1, 2);
    println!("  Pair::new(1, 2)");
    println!("  first: {}", pair.first());
    println!("  second: {}", pair.second());

    // 2. 泛型方法
    println!("\n2. 泛型方法:");
    pair.print();
    pair.cmp_display();

    // 3. swap 方法
    println!("\n3. swap 方法:");
    let mut pair = Pair::new("hello", "world");
    println!("  交换前: ({}, {})", pair.first(), pair.second());
    pair.swap();
    println!("  交换后: ({}, {})", pair.first(), pair.second());

    // 4. 不同类型的 Pair
    println!("\n4. 不同类型的 Pair:");
    let int_pair = Pair::new(10, 20);
    let float_pair = Pair::new(3.14, 2.71);
    let string_pair = Pair::new("foo", "bar");

    int_pair.print();
    float_pair.print();
    string_pair.print();

    // 5. largest 函数
    println!("\n5. largest 函数:");
    let numbers = vec![1, 2, 3, 4, 5];
    match largest(&numbers) {
        Some(&n) => println!("  最大值: {}", n),
        None => println!("  空列表"),
    }

    let chars = vec!['a', 'b', 'c', 'd'];
    match largest(&chars) {
        Some(&c) => println!("  最大字符: {}", c),
        None => println!("  空列表"),
    }

    let strings = vec!["apple", "banana", "cherry"];
    match largest(&strings) {
        Some(s) => println!("  最大字符串: {}", s),
        None => println!("  空列表"),
    }

    // 6. max 函数
    println!("\n6. max 函数:");
    println!("  max(10, 20) = {}", max(10, 20));
    println!("  max(3.14, 2.71) = {}", max(3.14, 2.71));
    println!("  max('a', 'z') = {}", max('a', 'z'));

    // 7. 空列表处理
    println!("\n7. 空列表处理:");
    let empty: Vec<i32> = vec![];
    match largest(&empty) {
        Some(&n) => println!("  最大值: {}", n),
        None => println!("  空列表"),
    }
}
