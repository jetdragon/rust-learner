//! # 练习 8: 使用 map 转换值
//!
//! **难度**: 入门
//! **预计时间**: 10 分钟

/// 使用 map 对 Some 值进行平方
pub fn square_if_positive(n: i32) -> Option<i32> {
    todo!()
}

/// 对 Option 中的值加 1
pub fn increment_if_some(opt: Option<i32>) -> Option<i32> {
    todo!()
}

fn main() {
    println!("=== 平方 ===");
    println!("{:?}", square_if_positive(Some(5)));
    println!("{:?}", square_if_positive(None));

    println!("\n=== 加 1 ===");
    println!("{:?}", increment_if_some(Some(5)));
    println!("{:?}", increment_if_some(None));
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
}
