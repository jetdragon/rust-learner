// 数据类型演示

fn main() {
    println!("=== 数据类型演示 ===\n");

    // ========== 标量类型 ==========

    // 1. 整数类型
    println!("1. 整数类型");
    let decimal = 98_222;          // 十进制
    let hex = 0xff;                // 十六进制
    let octal = 0o77;              // 八进制
    let binary = 0b1111_0000;      // 二进制
    let byte = b'A';               // 字节 (仅限 u8)

    println!("十进制: {}", decimal);
    println!("十六进制 (0xff): {}", hex);
    println!("八进制 (0o77): {}", octal);
    println!("二进制 (0b1111_0000): {}", binary);
    println!("字节 (b'A'): {}", byte);
    println!();

    // 不同大小的整数
    println!("不同大小的整数:");
    let small: u8 = 255;
    let medium: i32 = 100000;
    let large: i64 = 10000000000;
    println!("u8: {}", small);
    println!("i32: {}", medium);
    println!("i64: {}", large);
    println!();

    // 2. 浮点类型
    println!("2. 浮点类型");
    let x = 2.0;        // f64 (默认)
    let y: f32 = 3.0;   // f32

    println!("f64: {}", x);
    println!("f32: {}", y);

    // 浮点运算
    let sum = 5.5 + 1.2;
    let difference = 10.0 - 3.5;
    let product = 4.0 * 2.5;
    let quotient = 10.0 / 2.0;
    println!("5.5 + 1.2 = {}", sum);
    println!("10.0 - 3.5 = {}", difference);
    println!("4.0 * 2.5 = {}", product);
    println!("10.0 / 2.0 = {}", quotient);
    println!();

    // 3. 布尔类型
    println!("3. 布尔类型");
    let t = true;
    let f: bool = false;

    println!("true: {}", t);
    println!("false: {}", f);

    // 布尔运算
    println!("true AND false = {}", t && f);
    println!("true OR false = {}", t || f);
    println!("NOT true = {}", !t);
    println!();

    // 4. 字符类型
    println!("4. 字符类型");
    let c = 'z';
    let z: char = 'ℤ';      // Unicode
    let heart_eyed_cat = '😻';

    println!("字符: {}", c);
    println!("Unicode: {}", z);
    println!("表情: {}", heart_eyed_cat);
    println!();

    // ========== 复合类型 ==========

    // 5. 元组
    println!("5. 元组类型");
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // 通过索引访问
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("元组: {:?}", tup);
    println!("tup.0 = {}", five_hundred);
    println!("tup.1 = {}", six_point_four);
    println!("tup.2 = {}", one);

    // 解构元组
    let (x, y, z) = tup;
    println!("解构后: x={}, y={}, z={}", x, y, z);
    println!();

    // 6. 数组
    println!("6. 数组类型");
    let a = [1, 2, 3, 4, 5];

    println!("数组: {:?}", a);
    println!("a[0] = {}", a[0]);
    println!("a[1] = {}", a[1]);
    println!("数组长度: {}", a.len());

    // 指定类型的数组
    let b: [i32; 5] = [1, 2, 3, 4, 5];      // [类型; 长度]
    let c = [3; 5];                          // [值; 长度] = [3, 3, 3, 3, 3]

    println!("相同值的数组: {:?}", c);
    println!();

    // 7. 字符串切片 (str slice)
    println!("7. 字符串切片");
    let hello = "你好，世界！";
    println!("字符串: {}", hello);

    // 字节字符串
    let bytes = b"ASCII";
    println!("字节字符串: {:?}", bytes);

    println!("\n=== 演示完成 ===");
}
