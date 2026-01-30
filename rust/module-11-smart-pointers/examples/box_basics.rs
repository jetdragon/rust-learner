// examples/box_basics.rs
// Box 智能指针基础演示

fn main() {
    println!("=== Box 智能指针演示 ===\n");

    // 1. Box 基础使用
    println!("1. Box 基础使用:");
    let x = 5;
    let y = Box::new(5);

    println!("  x 在栈上: {}", x);
    println!("  y 在堆上: {}", y);
    println!("  解引用 y: {}", *y);

    // 2. Box 的内存布局
    println!("\n2. Box 的内存布局:");
    println!("  Box 包含一个堆指针");
    println!("  数据存储在堆上，Box 存储在栈上");

    // 3. 大数据使用 Box
    println!("\n3. 大数据使用 Box:");
    let large_data: Box<[u8; 1000]> = Box::new([0; 1000]);
    println!(
        "  大数据数组大小: {} bytes",
        std::mem::size_of_val(&*large_data)
    );
    println!(
        "  Box 本身大小: {} bytes",
        std::mem::size_of_val(&large_data)
    );

    // 4. 自动解引用
    println!("\n4. 自动解引用:");
    let boxed = Box::new(42);
    println!("  boxed = {}", boxed); // 自动解引用
    let doubled = *boxed * 2;
    println!("  *boxed * 2 = {}", doubled);

    // 5. Box 与函数
    println!("\n5. Box 作为函数参数:");
    fn process_box(boxed: Box<i32>) -> i32 {
        *boxed * 2
    }

    let result = process_box(Box::new(21));
    println!("  process_box(Box::new(21)) = {}", result);

    // 6. 深拷贝 vs Box
    println!("\n6. 深拷贝 vs Box:");
    #[derive(Debug, Clone)]
    struct Data {
        values: Vec<i32>,
    }

    impl Data {
        fn new(values: Vec<i32>) -> Self {
            Data { values }
        }
    }

    // Clone 会进行深拷贝
    let data1 = Data::new(vec![1, 2, 3]);
    let data2 = data1.clone();
    data1.values.push(4);
    println!("  data1 (clone后修改）: {:?}", data1.values);
    println!("  data2: {:?}", data2.values);

    // Box 共享（不复制数据）
    let boxed_data = Box::new(Data::new(vec![1, 2, 3]));
    let shared = &*boxed_data; // 借用 Box 中的数据
    println!("  shared: {:?}", shared.values);
}
