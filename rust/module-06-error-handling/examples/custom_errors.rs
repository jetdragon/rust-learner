// examples/custom_errors.rs
// 自定义错误类型演示

use module_05_error_handling::{MathError, ParseError};
use std::error::Error;
use std::fmt;

fn main() {
    println!("=== 自定义错误类型演示 ===\n");

    // 1. 定义简单的错误枚举
    println!("1. 使用 MathError:");
    let err = MathError::DivisionByZero;
    println!("  错误: {}", err);
    println!("  调试: {:?}", err);

    // 2. 错误可以用在 Result 中
    println!("\n2. 在 Result 中使用:");
    let result: Result<i32, MathError> = Err(MathError::NegativeSquareRoot);
    match result {
        Ok(_) => println!("  成功"),
        Err(e) => println!("  失败: {}", e),
    }

    // 3. 自定义结构体错误
    println!("\n3. 结构体错误:");
    let parse_err = ParseError::new("意外的字符".to_string(), 42);
    println!("  {}", parse_err);

    // 4. 错误链
    println!("\n4. 使用 std::error::Error trait:");
    fn describe_error(err: &dyn Error) {
        println!("  错误: {}", err);
        if let Some(source) = err.source() {
            println!("  源错误: {}", source);
        }
    }

    describe_error(&MathError::DivisionByZero);

    // 5. 从其他错误转换
    println!("\n5. 错误转换:");
    fn parse_number(s: &str) -> Result<i32, MathError> {
        s.parse::<i32>()
            .map_err(|_| MathError::Other("解析失败".to_string()))
    }

    match parse_number("42") {
        Ok(n) => println!("  解析成功: {}", n),
        Err(e) => println!("  解析失败: {}", e),
    }

    match parse_number("abc") {
        Ok(n) => println!("  解析成功: {}", n),
        Err(e) => println!("  解析失败: {}", e),
    }

    // 6. 使用带有数据的错误
    println!("\n6. 带有数据的错误:");
    let err = MathError::Other("详细错误信息".to_string());
    println!("  {}", err);

    // 7. 模式匹配错误
    println!("\n7. 模式匹配错误:");
    fn handle_error(err: &MathError) {
        match err {
            MathError::DivisionByZero => println!("  除零错误"),
            MathError::NegativeSquareRoot => println!("  负数平方根错误"),
            MathError::OutOfRange => println!("  超出范围错误"),
            MathError::Other(msg) => println!("  其他错误: {}", msg),
        }
    }

    handle_error(&MathError::DivisionByZero);
    handle_error(&MathError::Other("自定义错误".to_string()));

    // 8. 比较错误
    println!("\n8. 比较错误:");
    assert_eq!(MathError::DivisionByZero, MathError::DivisionByZero);
    assert_ne!(MathError::DivisionByZero, MathError::NegativeSquareRoot);
    println!("  DivisionByZero == DivisionByZero: true");
    println!("  DivisionByZero != NegativeSquareRoot: true");

    // 9. 克隆错误
    println!("\n9. 克隆错误:");
    let err1 = MathError::DivisionByZero;
    let err2 = err1.clone();
    println!("  err1: {:?}", err1);
    println!("  err2: {:?}", err2);

    // 10. 创建带有上下文的错误
    println!("\n10. 创建上下文错误:");
    fn read_config(key: &str) -> Result<String, MathError> {
        Err(MathError::Other(format!("缺少配置键: {}", key)))
    }

    match read_config("database_url") {
        Ok(_) => println!("  配置读取成功"),
        Err(e) => println!("  错误: {}", e),
    }

    // 11. 使用 Box<dyn Error>
    println!("\n11. 使用 Box<dyn Error>:");
    fn returns_dyn_error() -> Result<(), Box<dyn Error>> {
        Err(MathError::DivisionByZero.into())
    }

    match returns_dyn_error() {
        Ok(()) => println!("  成功"),
        Err(e) => println!("  错误: {}", e),
    }

    // 12. 组合多个错误类型
    println!("\n12. 组合多个错误类型:");
    #[derive(Debug)]
    enum AppError {
        Math(MathError),
        Parse(ParseError),
        Io(String),
    }

    impl fmt::Display for AppError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                AppError::Math(e) => write!(f, "数学错误: {}", e),
                AppError::Parse(e) => write!(f, "解析错误: {}", e),
                AppError::Io(msg) => write!(f, "IO错误: {}", msg),
            }
        }
    }

    impl Error for AppError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            match self {
                AppError::Math(e) => Some(e),
                AppError::Parse(e) => Some(e),
                AppError::Io(_) => None,
            }
        }
    }

    let app_err = AppError::Math(MathError::DivisionByZero);
    println!("  {}", app_err);
}
