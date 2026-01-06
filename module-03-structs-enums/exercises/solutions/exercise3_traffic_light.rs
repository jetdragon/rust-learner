//! # 练习 3 解答: 定义枚举
//!
//! **提示**：请先尝试自己实现，再参考此解答！
//!
//! # 实现思路
//!
//! 1. 枚举定义：使用 `enum` 定义一组可能的值
//! 2. 无数据枚举：简单的枚举变体不携带数据
//! 3. match 表达式：必须穷尽所有可能
//! 4. 模式匹配：每个枚举变体都是一个模式

/// 交通信号灯枚举
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    /// 返回信号灯的持续时间（秒）
    ///
    /// # 实现思路
    /// 使用 match 表达式处理所有变体
    pub fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 3,
            TrafficLight::Green => 25,
        }
    }

    /// 返回信号灯的中文描述
    ///
    /// # 实现思路
    /// 匹配每个变体并返回对应描述
    pub fn description(&self) -> &str {
        match self {
            TrafficLight::Red => "红灯",
            TrafficLight::Yellow => "黄灯",
            TrafficLight::Green => "绿灯",
        }
    }

    /// 返回下一个信号灯
    ///
    /// # 实现思路
    /// 转换顺序：Red -> Green -> Yellow -> Red
    pub fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Green => TrafficLight::Yellow,
            TrafficLight::Yellow => TrafficLight::Red,
        }
    }

    /// 判断是否可以通行
    pub fn can_go(&self) -> bool {
        matches!(self, TrafficLight::Green)
    }

    /// 判断是否应该减速
    pub fn should_slow(&self) -> bool {
        matches!(self, TrafficLight::Yellow)
    }
}

fn main() {
    let lights = vec![
        TrafficLight::Red,
        TrafficLight::Yellow,
        TrafficLight::Green,
    ];

    println!("=== 交通信号灯 ===\n");

    for light in lights {
        println!("信号灯: {}", light.description());
        println!("持续时间: {} 秒", light.duration());
        println!("下一个: {}", light.next().description());
        println!("可以通行: {}", light.can_go());
        println!("应该减速: {}", light.should_slow());
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_red_duration() {
        let red = TrafficLight::Red;
        assert_eq!(red.duration(), 30);
    }

    #[test]
    fn test_yellow_duration() {
        let yellow = TrafficLight::Yellow;
        assert_eq!(yellow.duration(), 3);
    }

    #[test]
    fn test_green_duration() {
        let green = TrafficLight::Green;
        assert_eq!(green.duration(), 25);
    }

    #[test]
    fn test_descriptions() {
        assert_eq!(TrafficLight::Red.description(), "红灯");
        assert_eq!(TrafficLight::Yellow.description(), "黄灯");
        assert_eq!(TrafficLight::Green.description(), "绿灯");
    }

    #[test]
    fn test_next_from_red() {
        let next = TrafficLight::Red.next();
        assert_eq!(next.description(), "绿灯");
    }

    #[test]
    fn test_next_from_green() {
        let next = TrafficLight::Green.next();
        assert_eq!(next.description(), "黄灯");
    }

    #[test]
    fn test_next_from_yellow() {
        let next = TrafficLight::Yellow.next();
        assert_eq!(next.description(), "红灯");
    }

    #[test]
    fn test_full_cycle() {
        let red = TrafficLight::Red;
        let green = red.next();
        let yellow = green.next();
        let back_to_red = yellow.next();

        assert_eq!(back_to_red.description(), "红灯");
    }

    #[test]
    fn test_can_go() {
        assert!(!TrafficLight::Red.can_go());
        assert!(!TrafficLight::Yellow.can_go());
        assert!(TrafficLight::Green.can_go());
    }

    #[test]
    fn test_should_slow() {
        assert!(!TrafficLight::Red.should_slow());
        assert!(TrafficLight::Yellow.should_slow());
        assert!(!TrafficLight::Green.should_slow());
    }

    #[test]
    fn test_enum_size() {
        // 枚举的大小取决于其最大变体
        assert_eq!(std::mem::size_of::<TrafficLight>(), 1); // 只有3个变体，1字节足够
    }

    #[test]
    fn test_copy_trait() {
        // 简单枚举实现了 Copy trait
        let light = TrafficLight::Red;
        let _copy = light; // 不会移动 light
        let _another = light; // 仍然可以使用
    }
}
