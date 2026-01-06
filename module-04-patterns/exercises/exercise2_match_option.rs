//! # 练习 2: 匹配 Option
//!
//! **难度**: 入门
//! **预计时间**: 10 分钟

/// 安全地获取向量中的第二个元素
///
/// # TODO
///
/// 返回向量的第二个元素（索引 1）
/// 如果向量长度不足 2，返回 None
pub fn second<T>(vec: &Vec<T>) -> Option<&T> {
    if vec.len() > 1 {
        Some(&vec[1])
    } else {
        None
    }
}

/// 获取第 n 个元素
pub fn nth<T>(vec: &Vec<T>, n: usize) -> Option<&T> {
    unimplemented!()
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    match second(&v) {
        Some(n) => println!("第二个元素: {}", n),
        None => println!("没有第二个元素"),
    }

    match nth(&v, 3) {
        Some(n) => println!("第三个元素: {}", n),
        None => println!("超出范围"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_second() {
        assert_eq!(second(&vec![1, 2, 3]), Some(&2));
        assert_eq!(second(&vec![1]), None);
        assert_eq!(second(&Vec::<i32>::new()), None);
    }

    #[test]
    fn test_nth() {
        let v = vec![10, 20, 30, 40];
        assert_eq!(nth(&v, 0), Some(&10));
        assert_eq!(nth(&v, 2), Some(&30));
        assert_eq!(nth(&v, 4), None);
        assert_eq!(nth(&v, 10), None);
    }

    #[test]
    fn test_empty_vec() {
        assert_eq!(second(&vec![]), None);
        assert_eq!(nth(&vec![], 0), None);
    }
}
