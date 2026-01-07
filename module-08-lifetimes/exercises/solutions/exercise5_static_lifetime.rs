//! # 练习 5: 'static 生命周期 - 解答
//!
//! 难度: 简单
//! 时间: 10 分钟
//! 前置知识: 生命周期基础
//! 学习目标:
//!   - 理解 'static 生命周期的含义
//!   - 区分字符串字面量和 String

// ============= 解决方案 =============

/// get_greeting - 返回字符串字面量
///
/// 返回类型: &'static str
///
/// 为什么是 'static:
/// - 字符串字面量被硬编码到二进制文件中
/// - 在整个程序运行期间都有效
/// - 不会因为作用域结束而失效
///
/// 字符串字面量的特点:
/// - 存储在程序的只读数据段
/// - 生命周期是 'static
/// - 类型是 &'static str
pub fn get_greeting() -> &'static str {
    "Hello, World!"
}

/// get_error_message - 返回静态错误消息
///
/// 应用场景:
/// - 错误消息
/// - 常量字符串
/// - 配置值
pub fn get_error_message() -> &'static str {
    "Something went wrong"
}

/// get_config_value - 返回可选的静态配置
///
/// 使用场景:
/// - 内置默认配置
/// - 静态查找表
pub fn get_config_value(key: &str) -> Option<&'static str> {
    match key {
        "host" => Some("localhost"),
        "port" => Some("8080"),
        "debug" => Some("true"),
        _ => None,
    }
}

// ============= 扩展示例 =============

/// 静态变量示例
///
/// 使用 static 声明的变量也有 'static 生命周期
static MAX_CONNECTIONS: i32 = 100;
static DEFAULT_NAME: &str = "Anonymous";

/// 使用静态常量
pub fn get_max_connections() -> i32 {
    MAX_CONNECTIONS
}

pub fn get_default_name() -> &'static str {
    DEFAULT_NAME
}

/// 字符串字面量 vs String
///
/// 展示两者的区别和使用场景
fn demonstrate_string_types() {
    // &'static str - 字符串字面量
    let static_str: &'static str = "Hello";

    // String - 堆分配的字符串
    let dynamic_string: String = String::from("Hello");

    // String 可以转换为 &str（但不一定是 'static）
    let s: &str = &dynamic_string;

    println!("Static: {}", static_str);
    println!("Dynamic: {}", dynamic_string);
    println!("Slice: {}", s);
}

/// 使用 'static 约束的泛型函数
///
/// 要求 T 必须包含 'static 生命周期的数据
pub fn print_static<T: 'static>(value: T) {
    println!("Static value: {:?}", value);
}

/// 只有 'static 数据才能被这样使用
pub fn spawn_thread_with_static<T: Send + 'static>(value: T) {
    // 线程可能比当前作用域活得更久
    // 所以需要 'static 约束
    std::thread::spawn(move || {
        println!("Thread got: {:?}", value);
    });
}

// ============= 测试代码 =============

fn main() {
    println!("=== 基础 'static 示例 ===");

    println!("Greeting: {}", get_greeting());
    println!("Error: {}", get_error_message());

    println!("\n=== 配置查找 ===");
    if let Some(host) = get_config_value("host") {
        println!("Host: {}", host);
    }

    if let Some(port) = get_config_value("port") {
        println!("Port: {}", port);
    }

    match get_config_value("unknown") {
        Some(v) => println!("Unknown config: {}", v),
        None => println!("Config not found"),
    }

    println!("\n=== String vs &'static str ===");

    // 对比: String vs &'static str
    let dynamic_string = String::from("Dynamic");
    let static_string = "Static";

    println!("Dynamic: {}, Static: {}", dynamic_string, static_string);

    // String 可以修改
    let mut mutable_string = String::from("Hello");
    mutable_string.push_str(", World!");
    println!("Mutable: {}", mutable_string);

    // &'static str 不能修改
    // static_string.push_str(", World!");  // 编译错误！

    println!("\n=== 静态常量 ===");
    println!("Max connections: {}", get_max_connections());
    println!("Default name: {}", get_default_name());

    println!("\n=== 'static 约束 ===");
    print_static("I'm static");
    print_static(42);

    println!("\n=== 线程中的 'static ===");
    spawn_thread_with_static("Hello from thread");
    spawn_thread_with_static(100);

    // 给线程一些时间完成
    std::thread::sleep(std::time::Duration::from_millis(100));

    println!("\n=== 常见误解 ===");
    common_misconceptions();
}

/// 常见误解说明
fn common_misconceptions() {
    // 误解 1: "所有 &str 都是 'static"
    println!("误解 1: 所有 &str 都是 'static");

    let s = String::from("Not static");
    let slice: &str = &s;
    // slice 不是 'static，它依赖 s

    println!("  纠正: slice 的生命周期与 s 相同，不是 'static");

    // 误解 2: "变量必须 'static 才能返回 &str"
    println!("\n误解 2: 返回 &'static str 必须用 static 变量");

    fn returns_static() -> &'static str {
        "This is static"  // 字符串字面量
    }

    println!("  纠正: 字符串字面量自动是 'static");

    // 误解 3: "Box::leak 是唯一创建 'static 的方法"
    println!("\n误解 3: 只能用 Box::leak 创建 'static");

    let leaked: &'static str = Box::leak(
        Box::new(String::from("Leaked string"))
    );
    println!("  纠正: 还有其他方法，如字符串字面量");
    println!("  Leaked: {}", leaked);
}

// ============= 'static 详解 =============

/*
'sstatic 生命周期的含义:

1. 定义:
   - 引用在整个程序运行期间都有效
   - 不会被释放（直到程序结束）

2. 来源:
   - 字符串字面量
   - static 声明的变量
   - Box::leak 创建的泄漏数据
   - 常量

3. 使用场景:
   - 常量配置
   - 错误消息
   - 全局状态
   - 线程间共享数据

重要区别:

&'static str:
- 字符串切片
- 不可变
- 固定在内存中
- 零成本

String:
- 可变
- 堆分配
- 可以增长/缩小
- 有所有权

什么时候使用 'static:

✅ 使用 'static:
1. 常量字符串
2. 全局配置
3. 错误消息
4. 线程间共享数据

❌ 不使用 'static:
1. 临时数据
2. 可变字符串
3. 依赖上下文的数据

类型转换关系:

String -> &str (隐式)
&str -> String (用 to_string() 或 .to_owned())
&'static str -> &str (隐式，'static 是任何生命周期的子类型)
&str -> &'static str (不安全！需要验证)

实际模式:

1. 函数返回 &'static str:
   fn error() -> &'static str {
       "Error message"
   }

2. 泛型 'static 约束:
   fn spawn<T: 'static>(value: T) {
       std::thread::spawn(move || {
           // 使用 value
       });
   }

3. Option<&'static str>:
   fn lookup(name: &str) -> Option<&'static str> {
       match name {
           "key" => Some("value"),
           _ => None,
       }
   }
*/
