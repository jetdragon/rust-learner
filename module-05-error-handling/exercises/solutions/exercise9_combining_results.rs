//! # 练习 9: 组合多个 Result 解答
//!
//! **解答要点:**
//! 1. 使用 and_then() 链接依赖的 Result 操作
//! 2. 使用 zip() 或组合器模式组合独立的 Result
//! 3. 理解提前返回机制

/// 如果两个都是 Ok，返回和；否则返回第一个错误
pub fn sum_results(
    a: Result<i32, String>,
    b: Result<i32, String>,
) -> Result<i32, String> {
    // 方法 1: 使用 and_then
    a.and_then(|val_a| b.map(|val_b| val_a + val_b))
}

// /// 方法 2: 使用 match
// pub fn sum_results_match(
//     a: Result<i32, String>,
//     b: Result<i32, String>,
// ) -> Result<i32, String> {
//     match a {
//         Ok(val_a) => match b {
//             Ok(val_b) => Ok(val_a + val_b),
//             Err(e) => Err(e),
//         },
//         Err(e) => Err(e),
//     }
// }

/// 组合三个 Result
pub fn sum_three_results(
    a: Result<i32, String>,
    b: Result<i32, String>,
    c: Result<i32, String>,
) -> Result<i32, String> {
    // 链式调用
    a.and_then(|val_a| {
        b.and_then(|val_b| c.map(|val_c| val_a + val_b + val_c))
    })
}

// /// 使用迭代器的替代方法
// pub fn sum_three_results_iter(
//     a: Result<i32, String>,
//     b: Result<i32, String>,
//     c: Result<i32, String>,
// ) -> Result<i32, String> {
//     [a, b, c]
//         .into_iter()
//         .try_fold(0, |acc, res| res.map(|v| acc + v))
// }

/// 组合任意数量的 Result
pub fn sum_many_results(results: Vec<Result<i32, String>>) -> Result<i32, String> {
    results.into_iter().try_fold(0, |acc, res| res.map(|v| acc + v))
}

fn main() {
    println!("=== 组合 Result ===");
    println!("{:?}", sum_results(Ok(5), Ok(3)));
    println!("{:?}", sum_results(Ok(5), Err("错误".to_string())));
    println!("{:?}", sum_results(Err("e1".to_string()), Ok(5)));

    println!("\n=== 组合三个 Result ===");
    println!("{:?}", sum_three_results(Ok(1), Ok(2), Ok(3)));
    println!(
        "{:?}",
        sum_three_results(Ok(1), Err("e1".to_string()), Ok(3))
    );

    println!("\n=== 组合多个 Result ===");
    let results = vec![Ok(1), Ok(2), Ok(3), Ok(4)];
    println!("{:?}", sum_many_results(results));

    let results = vec![Ok(1), Err("错误".to_string()), Ok(3)];
    println!("{:?}", sum_many_results(results));
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

    #[test]
    fn test_sum_many_results() {
        assert_eq!(
            sum_many_results(vec![Ok(1), Ok(2), Ok(3)]),
            Ok(6)
        );
        assert!(sum_many_results(vec![Ok(1), Err("e".to_string())]).is_err());
    }
}
