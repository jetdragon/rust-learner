// examples/standard_traits.rs
// 标准库 Trait 演示

use module_07_traits_generics::{
    make_all_animals_speak, Animal, Cat, Celsius, Config, Counter, Dog, Fahrenheit, Point,
};

fn main() {
    println!("=== 标准库 Trait 演示 ===\n");

    // 1. Debug - 调试输出
    println!("1. Debug Trait:");
    let point = Point::new(1, 2);
    println!("  Debug: {:?}", point);

    // 2. Clone 和 Copy
    println!("\n2. Clone 和 Copy:");
    let p1 = Point::new(1, 2);
    let p2 = p1; // Copy - p1 仍然有效
    println!("  p1 = {:?}, p2 = {:?}", p1, p2);

    // 3. PartialEq 和 Eq
    println!("\n3. PartialEq 和 Eq:");
    let v1 = Point::new(1, 2);
    let v2 = Point::new(1, 2);
    let v3 = Point::new(3, 4);

    println!("  Point(1, 2) == Point(1, 2): {}", v1 == v2);
    println!("  Point(1, 2) == Point(3, 4): {}", v1 == v3);

    // 4. PartialOrd 和 Ord
    println!("\n4. PartialOrd 和 Ord:");
    println!("  1 < 2: {}", 1 < 2);
    println!("  \"abc\" < \"xyz\": {}", "abc" < "xyz");

    // 5. From 和 Into
    println!("\n5. From 和 Into:");
    let celsius = Celsius(0.0);
    let fahrenheit: Fahrenheit = celsius.into();
    println!("  0°C = {:.1}°F", fahrenheit.0);

    let fahrenheit = Fahrenheit(100.0);
    let celsius: Celsius = fahrenheit.into();
    println!("  100°F = {:.1}°C", celsius.0);

    // 6. Default
    println!("\n6. Default:");
    let config = Config::new();
    println!("  默认配置: {:?}", config);

    let config = Config::new()
        .with_timeout(60)
        .with_max_retries(5)
        .with_debug(true);
    println!("  自定义配置: {:?}", config);

    // 7. Iterator
    println!("\n7. Iterator:");
    let counter = Counter::new();
    let sum: u32 = counter.sum();
    println!("  Counter 0-4 的和: {}", sum);

    let counter = Counter::new();
    let doubled: Vec<u32> = counter.map(|x| x * 2).collect();
    println!("  Counter 0-4 翻倍: {:?}", doubled);

    // 8. Display (通过 Debug 格式化)
    println!("\n8. Display (使用 Debug):");
    let point = Point::new(10, 20);
    println!("  Point: {:?}", point);

    // 9. 运算符重载
    println!("\n9. 运算符重载:");
    let p1 = Point::new(1, 2);
    let p2 = Point::new(3, 4);
    let sum = p1 + p2;
    println!("  {:?} + {:?} = {:?}", p1, p2, sum);

    let p3 = Point::new(5, 10);
    let sum2 = p3 + 5;
    println!("  {:?} + 5 = {:?}", p3, sum2);

    // 10. Trait 对象
    println!("\n10. Trait 对象:");
    let animals: Vec<Box<dyn std::fmt::Debug>> = vec![
        Box::new(Dog::new(String::from("Buddy"))),
        Box::new(Cat::new(String::from("Whiskers"))),
    ];

    for animal in animals {
        println!("  {:?}", animal);
    }

    // 11. 多重 Trait Bound
    println!("\n11. 多重 Trait Bound:");
    print_debug_and_display(&Point::new(1, 2));
    print_debug_and_display(&42);
    print_debug_and_display(&"hello");

    // 12. Animal trait
    println!("\n12. Animal Trait:");
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog::new(String::from("Buddy"))),
        Box::new(Cat::new(String::from("Whiskers"))),
    ];
    make_all_animals_speak(&animals[..]);
}

// 多重 Trait Bound 示例
fn print_debug_and_display<T: std::fmt::Debug + std::fmt::Display>(item: &T) {
    println!("  Debug: {:?}", item);
    println!("  Display: {}", item);
}
