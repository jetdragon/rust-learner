//! # 练习 1 解答: 基本 match 表达式
//!
//! **提示**：请先尝试自己实现，再参考此解答！
//!
//! # 实现思路
//!
//! 1. match 表达式：Rust 的模式匹配控制流
//! 2. 范围模式：使用 `..=` 表示包含范围
//! 3. 穷尽性：必须处理所有可能的值
//! 4. 通配符：`_` 匹配所有未明确匹配的值

/// 根据分数返回评级
pub fn grade(score: u32) -> &'static str {
    match score {
        90..=100 => "优秀",
        80..=89 => "良好",
        60..=79 => "及格",
        0..=59 => "不及格",
        _ => "无效分数",
    }
}

/// 使用 match 表达式作为表达式返回值
pub fn grade_emoji(score: u32) -> &'static str {
    match grade(score) {
        "优秀" => "🏆",
        "良好" => "👍",
        "及格" => "✅",
        "不及格" => "❌",
        _ => "❓",
    }
}

/// 使用 match 处理枚举
pub fn describe_score(score: u32) -> String {
    format!(
        "{}分: {} {}",
        score,
        grade(score),
        grade_emoji(score)
    )
}

fn main() {
    let scores = [95, 85, 70, 50, 105, 60];

    println!("=== 分数评级 ===\n");
    for score in scores {
        println!("{}", describe_score(score));
    }

    // 演示 match 作为表达式
    println!("\n=== match 作为表达式 ===");
    let score = 85;
    let level = match score {
        s if s >= 90 => "A",
        s if s >= 80 => "B",
        s if s >= 70 => "C",
        s if s >= 60 => "D",
        _ => "F",
    };
    println!("分数 {} 的等级: {}", score, level);

    // 演示范围匹配
    println!("\n=== 范围匹配示例 ===");
    for i in 0..=10 {
        let description = match i {
            0 => "零",
            1..=3 => "小",
            4..=7 => "中",
            _ => "大",
        };
        println!("{}: {}", i, description);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grade() {
        assert_eq!(grade(95), "优秀");
        assert_eq!(grade(85), "良好");
        assert_eq!(grade(70), "及格");
        assert_eq!(grade(50), "不及格");
    }

    #[test]
    fn test_boundary() {
        assert_eq!(grade(90), "优秀");
        assert_eq!(grade(89), "良好");
        assert_eq!(grade(60), "及格");
        assert_eq!(grade(59), "不及格");
    }

    #[test]
    fn test_invalid() {
        assert_eq!(grade(101), "无效分数");
        assert_eq!(grade(200), "无效分数");
    }

    #[test]
    fn test_grade_emoji() {
        assert_eq!(grade_emoji(95), "🏆");
        assert_eq!(grade_emoji(85), "👍");
        assert_eq!(grade_emoji(70), "✅");
        assert_eq!(grade_emoji(50), "❌");
    }

    #[test]
    fn test_all_ranges_covered() {
        // 确保所有范围都被覆盖
        for i in 0..=100 {
            let g = grade(i);
            assert!(["优秀", "良好", "及格", "不及格"].contains(&g));
        }
    }

    #[test]
    fn test_match_as_expression() {
        // match 可以作为表达式使用
        let result = match 5 {
            1..=5 => "small",
            6..=10 => "large",
            _ => "unknown",
        };
        assert_eq!(result, "small");
    }
}
