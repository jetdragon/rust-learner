// examples/slices.rs
// 切片类型演示

fn main() {
    println!("=== 切片类型演示 ===\n");

    // 字符串切片
    println!("1. 字符串切片 (&str):");
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("  原字符串: \"{}\"", s);
    println!("  [0..5]: \"{}\"", hello);
    println!("  [6..11]: \"{}\"", world);

    // 切片语法的简化
    let slice = &s[0..5];
    let slice = &s[..5]; // 等价，从 0 开始可省略
    let slice = &s[6..];
    let slice = &s[..]; // 整个字符串
    println!("  简化语法:");
    println!("    [0..5] 等价于 [..5]: \"{}\"", &s[..5]);
    println!("    [6..11] 等价于 [6..]: \"{}\"", &s[6..]);
    println!("    整个字符串 [..]: \"{}\"\n", &s[..]);

    // 字符串字面量是切片
    println!("2. 字符串字面量就是切片:");
    let s: &str = "Hello, world!";
    println!("  字符串字面量类型: &str");
    println!("  s = \"{}\"", s);
    println!("  字符串字面量存储在程序二进制文件中\n");

    // 数组切片
    println!("3. 数组切片:");
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("  原数组: {:?}", a);
    println!("  [1..3] 切片: {:?}", slice);
    println!("  切片类型: &[i32]\n");

    // 多种切片语法
    println!("4. 多种切片语法:");
    let arr = [10, 20, 30, 40, 50];
    println!("  原数组: {:?}", arr);
    println!("  [0..3]: {:?}", &arr[0..3]); // [10, 20, 30]
    println!("  [..3]: {:?}", &arr[..3]); // [10, 20, 30]
    println!("  [2..]: {:?}", &arr[2..]); // [30, 40, 50]
    println!("  [..]: {:?}", &arr[..]); // [10, 20, 30, 40, 50]
    println!("  [3..4]: {:?}", &arr[3..4]); // [40]\n");

    // 切片是不可变的
    println!("5. 切片的不可变性:");
    let s = String::from("hello");
    let slice = &s[0..2];
    println!("  s = \"{}\", slice = \"{}\"", s, slice);
    println!("  切片是对原始数据的引用");
    println!("  不拥有数据，只是借用\n");

    // 可变切片
    println!("6. 可变切片:");
    let mut arr = [1, 2, 3, 4, 5];
    let slice = &mut arr[1..3];
    slice[0] = 10;
    slice[1] = 20;
    println!("  原数组修改后: {:?}", arr);
    println!("  通过可变切片修改元素\n");

    // 切片的长度
    println!("7. 切片的长度:");
    let s = String::from("hello world");
    let hello = &s[0..5];
    println!("  原字符串长度: {}", s.len());
    println!("  切片长度: {}", hello.len());

    // 切片边界
    println!("8. 切片边界:");
    let s = String::from("hello");
    // let slice = &s[0..10];  // 运行时 panic！超出边界
    println!("  切片必须在有效范围内");
    println!("  否则会导致运行时 panic\n");

    // 切片作为参数
    println!("9. 切片作为函数参数:");
    let s = String::from("hello world");
    let word = first_word(&s); // 传入 &String
    println!("  first_word(&String): \"{}\"", word);

    let s_literal = "hello world";
    let word = first_word(s_literal); // 传入 &str
    println!("  first_word(&str): \"{}\"\n", word);

    // 多维数组切片
    println!("10. 多维数组切片:");
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let row = &matrix[1];
    println!("  二维数组: {:?}", matrix);
    println!("  第二行切片: {:?}", row);
    println!("  行类型: &[i32; 3]\n");

    // 切片是动态大小的
    println!("11. 切片是动态大小类型 (DST):");
    let arr = [1, 2, 3, 4, 5];
    let slice1 = &arr[0..2];
    let slice2 = &arr[0..4];
    println!(
        "  slice1 长度: {}, slice2 长度: {}",
        slice1.len(),
        slice2.len()
    );
    println!("  切片长度在运行时确定\n");

    // 字节数组切片
    println!("12. 字节数组切片:");
    let s = String::from("hello");
    let bytes = s.as_bytes();
    let byte_slice = &bytes[0..2];
    println!("  字节数组: {:?}", bytes);
    println!("  [0..2] 字节: {:?}", byte_slice);
    println!("  类型: &[u8]\n");
}

// 获取第一个单词（接受 &str 参数）
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
