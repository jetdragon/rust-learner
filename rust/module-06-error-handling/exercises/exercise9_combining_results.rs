//! # 练习 9: 组合多个 Result
//!
//! **难度**: 中等
//! **预计时间**: 15 分钟

/// 如果两个都是 Ok，返回和；否则返回第一个错误
pub fn sum_results(a: Result<i32, String>, b: Result<i32, String>) -> Result<i32, String> {
    todo!()
}

/// 组合三个 Result
pub fn sum_three_results(
    a: Result<i32, String>,
    b: Result<i32, String>,
    c: Result<i32, String>,
) -> Result<i32, String> {
    todo!()
}

fn main() {
    println!("=== 组合 Result ===");
    println!("{:?}", sum_results(Ok(5), Ok(3)));
    println!("{:?}", sum_results(Ok(5), Err("错误".to_string())));

    println!("\n=== 组合三个 Result ===");
    println!("{:?}", sum_three_results(Ok(1), Ok(2), Ok(3)));
    println!(
        "{:?}",
        sum_three_results(Ok(1), Err("e1".to_string()), Ok(3))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_results() {
        assert_eq!(sum_results(Ok(5), Ok(3)), Ok(8));
        assert_eq!(
            sum_results(Ok(5), Err("错误".to_string())),
            Err("错误".to_string())
        );
        assert_eq!(
            sum_results(Err("e1".to_string()), Ok(5)),
            Err("e1".to_string())
        );
    }

    #[test]
    fn test_sum_three_results() {
        assert_eq!(sum_three_results(Ok(1), Ok(2), Ok(3)), Ok(6));
        assert!(sum_three_results(Ok(1), Err("e1".to_string()), Ok(3)).is_err());
    }
}
