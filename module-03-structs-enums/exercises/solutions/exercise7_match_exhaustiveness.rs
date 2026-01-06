//! # 练习 7 解答: match 穷尽性
//!
//! **提示**：请先尝试自己实现，再参考此解答！
//!
//! # 实现思路
//!
//! 1. 穷尽性检查：Rust 编译器确保 match 处理所有可能
//! 2. 枚举匹配：必须列出所有枚举变体或使用通配符
//! 3. 模式：每个分支都是一个模式
//! 4. 守卫：使用 if 添加额外条件

/// 颜色枚举
pub enum Color {
    Red,
    Blue,
    Green,
    Yellow,
}

/// 返回颜色的中文名称
///
/// # 实现思路
/// 必须处理所有 Color 变体，否则编译失败
pub fn color_name(color: &Color) -> &str {
    match color {
        Color::Red => "红色",
        Color::Blue => "蓝色",
        Color::Green => "绿色",
        Color::Yellow => "黄色",
    }
}

/// 返回颜色的十六进制代码
pub fn color_hex(color: &Color) -> &str {
    match color {
        Color::Red => "#FF0000",
        Color::Blue => "#0000FF",
        Color::Green => "#00FF00",
        Color::Yellow => "#FFFF00",
    }
}

/// 判断是否为暖色调
///
/// # 实现思路
/// 红色和黄色是暖色调
pub fn is_warm_color(color: &Color) -> bool {
    matches!(color, Color::Red | Color::Yellow)
}

/// 混合两个颜色
///
/// # 实现思路
/// 使用匹配守卫和或模式
pub fn mix_colors(c1: &Color, c2: &Color) -> &'static str {
    match (c1, c2) {
        (Color::Red, Color::Yellow) | (Color::Yellow, Color::Red) => "橙色",
        (Color::Blue, Color::Yellow) | (Color::Yellow, Color::Blue) => "绿色",
        (Color::Red, Color::Blue) | (Color::Blue, Color::Red) => "紫色",
        (Color::Red, Color::Red) => "红色",
        (Color::Blue, Color::Blue) => "蓝色",
        (Color::Yellow, Color::Yellow) => "黄色",
        (Color::Green, _) | (_, Color::Green) => "未知", // 绿色不参与混合
    }
}

/// 使用 if let 简化单分支匹配
pub fn is_red(color: &Color) -> bool {
    matches!(color, Color::Red)
}

fn main() {
    let colors = vec![
        Color::Red,
        Color::Blue,
        Color::Green,
        Color::Yellow,
    ];

    println!("=== 颜色匹配练习 ===\n");

    for color in colors {
        println!("颜色: {}", color_name(&color));
        println!("十六进制: {}", color_hex(&color));
        println!("暖色调: {}", is_warm_color(&color));
        println!("是否为红色: {}", is_red(&color));
        println!();
    }

    // 颜色混合
    println!("--- 颜色混合 ---");
    println!("红 + 黄 = {}", mix_colors(&Color::Red, &Color::Yellow));
    println!("蓝 + 黄 = {}", mix_colors(&Color::Blue, &Color::Yellow));
    println!("红 + 蓝 = {}", mix_colors(&Color::Red, &Color::Blue));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_name() {
        assert_eq!(color_name(&Color::Red), "红色");
        assert_eq!(color_name(&Color::Blue), "蓝色");
        assert_eq!(color_name(&Color::Green), "绿色");
        assert_eq!(color_name(&Color::Yellow), "黄色");
    }

    #[test]
    fn test_color_hex() {
        assert_eq!(color_hex(&Color::Red), "#FF0000");
        assert_eq!(color_hex(&Color::Blue), "#0000FF");
        assert_eq!(color_hex(&Color::Green), "#00FF00");
        assert_eq!(color_hex(&Color::Yellow), "#FFFF00");
    }

    #[test]
    fn test_is_warm_color() {
        assert!(is_warm_color(&Color::Red));
        assert!(is_warm_color(&Color::Yellow));
        assert!(!is_warm_color(&Color::Blue));
        assert!(!is_warm_color(&Color::Green));
    }

    #[test]
    fn test_mix_colors() {
        assert_eq!(mix_colors(&Color::Red, &Color::Yellow), "橙色");
        assert_eq!(mix_colors(&Color::Blue, &Color::Yellow), "绿色");
        assert_eq!(mix_colors(&Color::Red, &Color::Blue), "紫色");
    }

    #[test]
    fn test_mix_same_colors() {
        assert_eq!(mix_colors(&Color::Red, &Color::Red), "红色");
        assert_eq!(mix_colors(&Color::Blue, &Color::Blue), "蓝色");
    }

    #[test]
    fn test_mix_colors_commutative() {
        // 测试交换律
        assert_eq!(
            mix_colors(&Color::Red, &Color::Yellow),
            mix_colors(&Color::Yellow, &Color::Red)
        );
    }

    #[test]
    fn test_is_red() {
        assert!(is_red(&Color::Red));
        assert!(!is_red(&Color::Blue));
        assert!(!is_red(&Color::Green));
        assert!(!is_red(&Color::Yellow));
    }

    #[test]
    fn test_match_all_variants() {
        // 这个测试确保所有变体都被正确处理
        let all_colors = vec![Color::Red, Color::Blue, Color::Green, Color::Yellow];

        for color in all_colors {
            // 如果有变体未被处理，这会在编译时就失败
            let _name = color_name(&color);
            let _hex = color_hex(&color);
            let _is_warm = is_warm_color(&color);
        }
    }

    #[test]
    fn test_wildcard_pattern() {
        // 使用通配符模式
        let color = Color::Green;

        let description = match color {
            Color::Red => "暖色",
            Color::Yellow => "暖色",
            _ => "其他",
        };

        assert_eq!(description, "其他");
    }

    #[test]
    fn test_or_pattern() {
        // 使用或模式匹配多个值
        let is_primary = |c: &Color| -> bool {
            matches!(c, Color::Red | Color::Blue | Color::Yellow)
        };

        assert!(is_primary(&Color::Red));
        assert!(!is_primary(&Color::Green));
    }
}
