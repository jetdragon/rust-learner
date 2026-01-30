//! # 练习 7: match 穷尽性
//!
//! **难度**: 中等
//! **预计时间**: 15 分钟
//!
//! **前置知识**:
//! - 枚举定义
//! - match 表达式基础
//!
//! **学习目标**:
//! - 理解 match 必须穷尽所有可能
//! - 学会处理所有枚举变体
//! - 掌握默认匹配模式

/// 颜色枚举
pub enum Color {
    Red,
    Blue,
    Green,
    Yellow,
}

/// TODO: 返回颜色的中文名称
///
/// 必须处理所有 Color 变体，否则编译会失败
pub fn color_name(color: &Color) -> &str {
    match color {
        Color::Red => "红色",
        Color::Blue => "蓝色",
        Color::Green => "绿色",
        Color::Yellow => "黄色",
    }
}

/// TODO: 返回颜色的十六进制代码
pub fn color_hex(color: &Color) -> &str {
    unimplemented!()
}

/// TODO: 判断是否为暖色调
///
/// 红色和黄色是暖色调
pub fn is_warm_color(color: &Color) -> bool {
    unimplemented!()
}

/// TODO: 混合两个颜色
///
/// 简化的颜色混合规则：
/// - Red + Yellow = Orange (返回 "橙色")
/// - Blue + Yellow = Green (返回 "绿色")
/// - Red + Blue = Purple (返回 "紫色")
/// - 相同颜色 = 原颜色
/// - 其他 = "未知"
pub fn mix_colors(_c1: &Color, _c2: &Color) -> &'static str {
    unimplemented!()
}

fn main() {
    let colors = vec![Color::Red, Color::Blue, Color::Green, Color::Yellow];

    println!("=== 颜色匹配练习 ===\n");

    for color in colors {
        println!("颜色: {}", color_name(&color));
        println!("十六进制: {}", color_hex(&color));
        println!("暖色调: {}", is_warm_color(&color));
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
}
