//! # 练习 3: 平方根计算解答
//!
//! **解答要点:**
//! 1. 使用 if 检查前置条件
//! 2. 对合法输入执行计算
//! 3. 返回适当的 Ok 或 Err

/// 带错误处理的平方根
///
/// 如果 n >= 0，返回 Ok(sqrt(n))
/// 否则返回 Err("不能计算负数的平方根")
pub fn safe_sqrt(n: f64) -> Result<f64, String> {
    if n >= 0.0 {
        Ok(n.sqrt())
    } else {
        Err("不能计算负数的平方根".to_string())
    }
}

/// 计算立方根（总是成功）
///
/// 立方根可以是负数，所以不需要错误处理
/// cbrt(x) = sign(x) * |x|^(1/3)
pub fn cbrt(n: f64) -> f64 {
    n.signum() * n.abs().powf(1.0 / 3.0)
}

/// **替代方法：使用 match**
// pub fn safe_sqrt_match(n: f64) -> Result<f64, String> {
//     match n >= 0.0 {
//         true => Ok(n.sqrt()),
//         false => Err("不能计算负数的平方根".to_string()),
//     }
// }

fn main() {
    println!("=== 平方根 ===");
    match safe_sqrt(9.0) {
        Ok(result) => println!("√9 = {}", result),
        Err(e) => println!("错误: {}", e),
    }

    match safe_sqrt(-1.0) {
        Ok(result) => println!("结果: {}", result),
        Err(e) => println!("错误: {}", e),
    }

    println!("\n=== 立方根 ===");
    println!("∛8 = {}", cbrt(8.0));
    println!("∛-8 = {}", cbrt(-8.0));

    println!("\n=== 比较 ===");
    println!("√(-2)² = {:?}", safe_sqrt(-2.0));
    println!("∛(-2)³ = {}", cbrt(-8.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_sqrt() {
        assert_eq!(safe_sqrt(9.0), Ok(3.0));
        assert!(safe_sqrt(-1.0).is_err());
    }

    #[test]
    fn test_safe_sqrt_zero() {
        assert_eq!(safe_sqrt(0.0), Ok(0.0));
    }

    #[test]
    fn test_cbrt() {
        assert!((cbrt(8.0) - 2.0).abs() < 0.001);
        assert!((cbrt(-8.0) - (-2.0)).abs() < 0.001);
    }
}
