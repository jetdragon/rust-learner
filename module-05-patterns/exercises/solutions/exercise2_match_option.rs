//! # 练习 2 解答: 匹配 Option
//!
//! **提示**：请先尝试自己实现，再参考此解答！
//!
//! # 实现思路
//!
//! 1. Option 类型：表示可能存在或不存在
//! 2. Some(T)：值存在
//! 3. None：值不存在
//! 4. 模式匹配：处理 Option 的标准方式

/// 安全地获取向量中的第二个元素
pub fn second<T>(vec: &Vec<T>) -> Option<&T> {
    if vec.len() > 1 {
        Some(&vec[1])
    } else {
        None
    }
}

/// 获取第 n 个元素
pub fn nth<T>(vec: &Vec<T>, n: usize) -> Option<&T> {
    if n < vec.len() {
        Some(&vec[n])
    } else {
        None
    }
}

/// 获取第一个元素
pub fn first<T>(vec: &Vec<T>) -> Option<&T> {
    vec.first()
}

/// 获取最后一个元素
pub fn last<T>(vec: &Vec<T>) -> Option<&T> {
    vec.last()
}

/// 使用 map 转换 Option
pub fn second_squared(vec: &Vec<i32>) -> Option<i32> {
    second(vec).map(|x| x * x)
}

/// 使用 and_then 链接操作
pub fn second_then_add(vec: &Vec<i32>, add: i32) -> Option<i32> {
    second(vec).map(|x| x + add)
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // 第二个元素
    match second(&v) {
        Some(n) => println!("第二个元素: {}", n),
        None => println!("没有第二个元素"),
    }

    // 第 n 个元素
    match nth(&v, 3) {
        Some(n) => println!("第4个元素: {}", n),
        None => println!("超出范围"),
    }

    // 第一个元素
    match first(&v) {
        Some(n) => println!("第一个元素: {}", n),
        None => println!("向量为空"),
    }

    // 最后一个元素
    match last(&v) {
        Some(n) => println!("最后一个元素: {}", n),
        None => println!("向量为空"),
    }

    // 使用 map
    println!("\n第二个元素的平方: {:?}", second_squared(&v));

    // 使用 and_then
    println!("第二个元素加10: {:?}", second_then_add(&v, 10));

    // 空向量示例
    let empty: Vec<i32> = vec![];
    println!("\n空向量的第二个元素: {:?}", second(&empty));
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
    fn test_first() {
        assert_eq!(first(&vec![1, 2, 3]), Some(&1));
        assert_eq!(first(&Vec::<i32>::new()), None);
    }

    #[test]
    fn test_last() {
        assert_eq!(last(&vec![1, 2, 3]), Some(&3));
        assert_eq!(last(&vec![1]), Some(&1));
        assert_eq!(last(&Vec::<i32>::new()), None);
    }

    #[test]
    fn test_second_squared() {
        assert_eq!(second_squared(&vec![1, 2, 3]), Some(4));
        assert_eq!(second_squared(&vec![1]), None);
    }

    #[test]
    fn test_second_then_add() {
        assert_eq!(second_then_add(&vec![1, 2, 3], 10), Some(12));
        assert_eq!(second_then_add(&vec![1], 10), None);
    }

    #[test]
    fn test_empty_vec() {
        assert_eq!(second(&vec![]), None);
        assert_eq!(nth(&vec![], 0), None);
        assert_eq!(first(&vec![]), None);
        assert_eq!(last(&vec![]), None);
    }

    #[test]
    fn test_option_combinators() {
        // 演示 Option 的组合子
        let vec = vec![1, 2, 3];

        // map
        assert_eq!(second(&vec).map(|x| x * 2), Some(4));

        // filter
        assert_eq!(second(&vec).filter(|x| **x > 1), Some(&2));
        assert_eq!(second(&vec).filter(|x| **x > 10), None);

        // unwrap_or
        assert_eq!(second(&vec).unwrap_or(&0), &2);
        assert_eq!(second(&vec![1]).unwrap_or(&0), &0);
    }

    #[test]
    fn test_option_with_different_types() {
        let v = vec!["hello", "world"];
        assert_eq!(second(&v), Some(&"world"));

        let v = vec![Some(1), Some(2), None];
        assert_eq!(second(&v), Some(&Some(2)));
    }
}
