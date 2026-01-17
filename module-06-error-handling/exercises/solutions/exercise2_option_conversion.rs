//! # 练习 2: Option 转换解答
//!
//! **解答要点:**
//! 1. 使用 Option 的 ok() 和 ok_or() 方法进行转换
//! 2. 使用 Result 的 ok() 方法转换为 Option

/// 将 Option 转换为 Result
///
/// - Some(v) -> Ok(v)
/// - None -> Err(err)
///
/// **方法1:** 使用 ok_or()
pub fn option_to_result(opt: Option<i32>, err: String) -> Result<i32, String> {
    opt.ok_or(err)
}

/// **方法2:** 使用 match 表达式
// pub fn option_to_result_match(opt: Option<i32>, err: String) -> Result<i32, String> {
//     match opt {
//         Some(v) => Ok(v),
//         None => Err(err),
//     }
// }

/// 将 Result 转换为 Option
///
/// - Ok(v) -> Some(v)
/// - Err(_) -> None
///
/// **方法:** 使用 ok() 方法
pub fn result_to_option(res: Result<i32, String>) -> Option<i32> {
    res.ok()
}

/// **方法2:** 使用 match 表达式
// pub fn result_to_option_match(res: Result<i32, String>) -> Option<i32> {
//     match res {
//         Ok(v) => Some(v),
//         Err(_) => None,
//     }
// }

fn main() {
    println!("=== Option 转换 ===");

    let some_val: Option<i32> = Some(5);
    println!("{:?}", option_to_result(some_val, "错误".to_string()));

    let none_val: Option<i32> = None;
    println!("{:?}", option_to_result(none_val, "没有值".to_string()));

    println!("\n=== Result 转换 ===");
    println!("{:?}", result_to_option(Ok(42)));
    println!("{:?}", result_to_option(Err("错误".to_string())));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_to_result() {
        assert_eq!(option_to_result(Some(5), "错误".to_string()), Ok(5));
        assert_eq!(
            option_to_result(None, "错误".to_string()),
            Err("错误".to_string())
        );
    }

    #[test]
    fn test_result_to_option() {
        assert_eq!(result_to_option(Ok(42)), Some(42));
        assert_eq!(result_to_option(Err("error".to_string())), None);
    }
}
