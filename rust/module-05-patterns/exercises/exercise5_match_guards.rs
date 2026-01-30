//! # 练习 5: 使用守卫
//!
//! **难度**: 中等
//! **预计时间**: 15 分钟

/// TODO: 描述数字特征
///
/// - 正偶数: "正偶数"
/// - 正奇数: "正奇数"
/// - 负偶数: "负偶数"
/// - 负奇数: "负奇数"
/// - 0: "零"
pub fn describe_number(n: i32) -> &'static str {
    match n {
        0 => "零",
        x if x > 0 && x % 2 == 0 => "正偶数",
        x if x > 0 => "正奇数",
        x if x < 0 && x % 2 == 0 => "负偶数",
        _ => "负奇数",
    }
}

/// TODO: 分类数字范围
pub fn categorize(n: i32) -> &'static str {
    unimplemented!()
}

fn main() {
    let numbers = [4, 3, -4, -3, 0, 7, -8];

    println!("=== 数字描述 ===\n");
    for n in numbers {
        println!("{}: {}", n, describe_number(n));
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
    }
}
