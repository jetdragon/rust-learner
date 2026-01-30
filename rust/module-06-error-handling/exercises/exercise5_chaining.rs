//! # 练习 5: 链式操作
//!
//! **难度**: 中等
//! **预计时间**: 15 分钟

/// 如果 n 为正数，返回 2*n，否则返回错误
pub fn double_if_positive(n: i32) -> Result<i32, String> {
    todo!()
}

/// 将 Option 转为 Result，然后调用 double_if_positive
pub fn validate_and_double(v: Option<i32>) -> Result<i32, String> {
    todo!()
}

/// 链式调用示例
pub fn process_numbers(numbers: Vec<Option<i32>>) -> Result<Vec<i32>, String> {
    todo!()
}

fn main() {
    println!("=== 验证并翻倍 ===");
    println!("{:?}", validate_and_double(Some(5)));
    println!("{:?}", validate_and_double(Some(-1)));
    println!("{:?}", validate_and_double(None));

    println!("\n=== 处理数字列表 ===");
    let numbers = vec![Some(1), Some(2), None, Some(3)];
    println!("{:?}", process_numbers(numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_and_double() {
        assert_eq!(validate_and_double(Some(5)), Ok(10));
        assert!(validate_and_double(Some(-1)).is_err());
        assert!(validate_and_double(None).is_err());
    }

    #[test]
    fn test_process_numbers() {
        let numbers = vec![Some(1), Some(2), Some(3)];
        assert_eq!(process_numbers(numbers), Ok(vec![2, 4, 6]));

        let numbers = vec![Some(1), Some(-2), Some(3)];
        assert!(process_numbers(numbers).is_err());
    }
}
