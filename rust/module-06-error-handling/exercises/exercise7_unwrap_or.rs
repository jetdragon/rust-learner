//! # 练习 7: 使用 unwrap_or
//!
//! **难度**: 入门
//! **预计时间**: 10 分钟

/// 使用 unwrap_or 返回第一个元素，如果没有则返回 0
pub fn get_first_or_default(vec: &Vec<i32>) -> i32 {
    todo!()
}

/// 获取最后一个元素或默认值
pub fn get_last_or_default(vec: &Vec<i32>) -> i32 {
    todo!()
}

fn main() {
    let vec = vec![1, 2, 3];
    println!("第一个: {}", get_first_or_default(&vec));
    println!("最后一个: {}", get_last_or_default(&vec));

    let empty: Vec<i32> = vec![];
    println!("空向量第一个: {}", get_first_or_default(&empty));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_first_or_default() {
        assert_eq!(get_first_or_default(&vec![1, 2, 3]), 1);
        assert_eq!(get_first_or_default(&vec![]), 0);
    }

    #[test]
    fn test_get_last_or_default() {
        assert_eq!(get_last_or_default(&vec![1, 2, 3]), 3);
        assert_eq!(get_last_or_default(&vec![]), 0);
    }
}
