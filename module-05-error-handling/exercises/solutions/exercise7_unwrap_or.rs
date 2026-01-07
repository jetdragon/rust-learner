//! # 练习 7: 使用 unwrap_or 解答
//!
//! **解答要点:**
//! 1. 使用 unwrap_or() 提供默认值
//! 2. 使用 unwrap_or_else() 延迟计算默认值
//! 3. 使用 first()/last() 获取元素

/// 使用 unwrap_or 返回第一个元素，如果没有则返回 0
pub fn get_first_or_default(vec: &Vec<i32>) -> i32 {
    vec.first().copied().unwrap_or(0)
}

// /// 使用 unwrap_or_else 的替代实现
// pub fn get_first_or_default_else(vec: &Vec<i32>) -> i32 {
//     vec.first()
//         .copied()
//         .unwrap_or_else(|| {
//             println!("使用默认值 0");
//             0
//         })
// }

/// 获取最后一个元素或默认值
pub fn get_last_or_default(vec: &Vec<i32>) -> i32 {
    vec.last().copied().unwrap_or(0)
}

// /// 使用 match 的替代实现
// pub fn get_first_or_default_match(vec: &Vec<i32>) -> i32 {
//     match vec.first() {
//         Some(&n) => n,
//         None => 0,
//     }
// }

fn main() {
    let vec = vec![1, 2, 3];
    println!("第一个: {}", get_first_or_default(&vec));
    println!("最后一个: {}", get_last_or_default(&vec));

    let empty: Vec<i32> = vec![];
    println!("空向量第一个: {}", get_first_or_default(&empty));
    println!("空向量最后一个: {}", get_last_or_default(&empty));

    // unwrap_or 与 unwrap_or_else 的区别
    println!("\n=== unwrap_or vs unwrap_or_else ===");

    // unwrap_or 的参数总是会求值
    let default = expensive_computation();
    let _ = get_first_or_default(&empty);

    // unwrap_or_else 只在需要时求值
    let _ = vec.first().copied().unwrap_or_else(|| expensive_computation());
}

// 演示延迟计算
fn expensive_computation() -> i32 {
    println!("执行昂贵计算...");
    0
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
