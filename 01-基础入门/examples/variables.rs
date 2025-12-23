// 变量和可变性演示

fn main() {
    println!("=== 变量和可变性演示 ===\n");

    // 1. 不可变变量 (默认)
    println!("1. 不可变变量");
    let x = 5;
    println!("x = {}", x);
    // x = 6;  // 编译错误！不能给不可变变量二次赋值
    println!("不可变变量 x 的值不能被修改\n");

    // 2. 可变变量 (使用 mut 关键字)
    println!("2. 可变变量");
    let mut y = 5;
    println!("初始: y = {}", y);
    y = 6;
    println!("修改后: y = {}", y);
    println!("可变变量 y 可以被修改\n");

    // 3. 变量隐藏 (shadowing)
    println!("3. 变量隐藏 (Shadowing)");
    let x = 5;
    println!("第一次声明 x = {}", x);

    let x = x + 1;
    println!("隐藏后 x = {}", x);

    {
        let x = x * 2;
        println!("内部作用域 x = {}", x);
    }

    println!("外部作用域 x = {}", x);
    println!("变量隐藏允许我们重新声明同名变量\n");

    // 4. 隐藏与可变的区别
    println!("4. 隐藏 vs 可变");
    // 隐藏可以改变类型
    let spaces = "   ";
    println!("spaces 是字符串: '{}'", spaces);
    let spaces = spaces.len();
    println!("隐藏后 spaces 是数字: {}", spaces);

    // 如果使用 mut，不能改变类型
    // let mut spaces = "   ";
    // spaces = spaces.len();  // 编译错误！类型不匹配

    println!("\n=== 演示完成 ===");
}
