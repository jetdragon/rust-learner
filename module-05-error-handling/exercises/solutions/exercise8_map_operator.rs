//! # 练习 8: 使用 map 转换值解答
//!
//! **解答要点:**
//! 1. 使用 map() 对 Option/Result 中的值进行转换
//! 2. map() 只在 Some/Ok 时执行，None/Err 直接传递
//! 3. 可以链式调用多个 map()

/// 对正数进行平方
pub fn square_if_positive(n: i32) -> Option<i32> {
    if n > 0 {
        Some(n * n)
    } else {
        None
    }
}

// /// 使用 filter() 的替代实现
// pub fn square_if_positive_filter(n: i32) -> Option<i32> {
//     Some(n)
//         .filter(|&x| x > 0)
//         .map(|x| x * x)
// }

/// 对 Option 中的值加 1
pub fn increment_if_some(opt: Option<i32>) -> Option<i32> {
    opt.map(|n| n + 1)
}

// /// 链式调用示例
// pub fn square_and_increment(opt: Option<i32>) -> Option<i32> {
//     opt.map(|n| n * 2)
//         .map(|n| n + 1)
// }

/// 对 Result 中的值进行平方
pub fn square_result(res: Result<i32, String>) -> Result<i32, String> {
    res.map(|n| n * n)
}

fn main() {
    println!("=== 平方 ===");
    println!("{:?}", square_if_positive(Some(5)));
    println!("{:?}", square_if_positive(Some(-1)));
    println!("{:?}", square_if_positive(None));

    println!("\n=== 加 1 ===");
    println!("{:?}", increment_if_some(Some(5)));
    println!("{:?}", increment_if_some(None));

    println!("\n=== 链式调用 ===");
    let result = Some(5)
        .map(|n| n * 2)  // Some(10)
        .map(|n| n + 1); // Some(11)
    println!("{:?}", result);

    println!("\n=== Result map ===");
    println!("{:?}", square_result(Ok(5)));
    println!("{:?}", square_result(Err("错误".to_string())));

    // map 与 map_err 的区别
    println!("\n=== map vs map_err ===");
    let ok_result: Result<i32, String> = Ok(5);
    let err_result: Result<i32, String> = Err("错误".to_string());

    // map 只转换 Ok 中的值
    println!("map Ok: {:?}", ok_result.map(|n| n * 2));   // Ok(10)
    println!("map Err: {:?}", err_result.map(|n| n * 2));  // Err("错误")

    // map_err 只转换 Err 中的值
    println!("map_err Ok: {:?}", ok_result.map_err(|e| format!("Error: {}", e))); // Ok(5)
    println!(
        "map_err Err: {:?}",
        err_result.map_err(|e| format!("Error: {}", e))
    ); // Err("Error: 错误")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_if_positive() {
        assert_eq!(square_if_positive(Some(5)), Some(25));
        assert_eq!(square_if_positive(None), None);
    }

    #[test]
    fn test_increment_if_some() {
        assert_eq!(increment_if_some(Some(5)), Some(6));
        assert_eq!(increment_if_some(None), None);
    }

    #[test]
    fn test_square_result() {
        assert_eq!(square_result(Ok(5)), Ok(25));
        assert_eq!(
            square_result(Err("错误".to_string())),
            Err("错误".to_string())
        );
    }
}
