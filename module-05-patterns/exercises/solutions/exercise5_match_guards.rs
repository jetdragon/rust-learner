//! # 练习 5 解答: 使用守卫
//!
//! **提示**：请先尝试自己实现，再参考此解答！
//!
//! # 实现思路
//!
//! 1. 匹配守卫：在匹配分支后添加额外的条件
//! 2. 语法：`pattern =>` 后面跟 `if condition`
//! 3. 优先级：守卫条件比模式本身更严格
//! 4. 灵活性：可以实现复杂的匹配逻辑

/// 描述数字特征
pub fn describe_number(n: i32) -> &'static str {
    match n {
        0 => "零",
        x if x > 0 && x % 2 == 0 => "正偶数",
        x if x > 0 => "正奇数",
        x if x < 0 && x % 2 == 0 => "负偶数",
        _ => "负奇数",
    }
}

/// 分类数字范围
pub fn categorize(n: i32) -> &'static str {
    match n {
        x if (1..=10).contains(&x) => "小",
        x if (11..=50).contains(&x) => "中等",
        x if x > 50 => "大",
        _ => "零或负",
    }
}

/// 检查是否在范围内
pub fn is_in_range(n: i32, min: i32, max: i32) -> bool {
    match n {
        x if min <= x && x <= max => true,
        _ => false,
    }
}

fn main() {
    let numbers = [4, 3, -4, -3, 0, 7, -8, 50];

    println!("=== 数字描述 ===\n");
    for n in numbers {
        println!("{}: {}", n, describe_number(n));
    }

    println!("\n=== 数字分类 ===");
    for n in [5, 25, 100, -5, 0] {
        println!("{}: {}", n, categorize(n));
    }

    // 演示范围检查
    println!("\n=== 范围检查 ===");
    println!("5 在 1-10: {}", is_in_range(5, 1, 10));
    println!("15 在 1-10: {}", is_in_range(15, 1, 10));

    // 演示复杂的守卫
    println!("\n=== 复杂守卫 ===");
    let point = (5, 5);
    match point {
        (x, y) if x == y => println!("在对角线上: ({}, {})", x, y),
        (x, y) if x > y => println!("x 大于 y: ({}, {})", x, y),
        (x, y) => println!("x 小于 y: ({}, {})", x, y),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe_number() {
        assert_eq!(describe_number(4), "正偶数");
        assert_eq!(describe_number(3), "正奇数");
        assert_eq!(describe_number(-4), "负偶数");
        assert_eq!(describe_number(-3), "负奇数");
        assert_eq!(describe_number(0), "零");
    }

    #[test]
    fn test_categorize() {
        assert_eq!(categorize(50), "中等");
        assert_eq!(categorize(10), "小");
        assert_eq!(categorize(100), "大");
        assert_eq!(categorize(0), "零或负");
        assert_eq!(categorize(-5), "零或负");
    }

    #[test]
    fn test_is_in_range() {
        assert!(is_in_range(5, 1, 10));
        assert!(!is_in_range(15, 1, 10));
        assert!(is_in_range(1, 1, 10));
        assert!(is_in_range(10, 1, 10));
    }

    #[test]
    fn test_guard_order() {
        // 守卫按顺序评估
        let result = match 5 {
            x if x > 0 => "positive",
            x if x > 3 => "greater than 3", // 永远不会匹配
            _ => "other",
        };
        assert_eq!(result, "positive");
    }

    #[test]
    fn test_multiple_conditions() {
        // 多个条件的守卫
        let result = match (10, 5) {
            (x, y) if x > 0 && y > 0 => "both positive",
            (x, y) if x > 0 || y > 0 => "one positive",
            _ => "none positive",
        };
        assert_eq!(result, "both positive");
    }
}
