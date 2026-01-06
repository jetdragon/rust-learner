//! # 练习 4 解答: 匹配枚举
//!
//! **提示**：请先尝试自己实现，再参考此解答！
//!
//! # 实现思路
//!
//! 1. 枚举匹配：每个枚举变体都是一个模式
//! 2. 穷尽性：必须处理所有变体
//! 3. 简洁语法：使用路径引用枚举变体
//! 4. 无数据枚举：最简单的匹配形式

/// 交通信号灯
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

/// 返回对应动作
pub fn action(light: TrafficLight) -> &'static str {
    match light {
        TrafficLight::Red => "停止",
        TrafficLight::Yellow => "准备",
        TrafficLight::Green => "通行",
    }
}

/// 返回信号灯持续时间（秒）
pub fn duration(light: TrafficLight) -> u32 {
    match light {
        TrafficLight::Red => 30,
        TrafficLight::Yellow => 3,
        TrafficLight::Green => 25,
    }
}

/// 判断是否应该停止
pub fn should_stop(light: TrafficLight) -> bool {
    matches!(light, TrafficLight::Red | TrafficLight::Yellow)
}

/// 信号灯循环中的下一个
pub fn next_light(light: TrafficLight) -> TrafficLight {
    match light {
        TrafficLight::Red => TrafficLight::Green,
        TrafficLight::Yellow => TrafficLight::Red,
        TrafficLight::Green => TrafficLight::Yellow,
    }
}

fn main() {
    let lights = [TrafficLight::Red, TrafficLight::Yellow, TrafficLight::Green];

    for light in lights {
        println!("{:?} -> {}", light, action(light));
        println!("持续时间: {}秒", duration(light));
        println!("下一个: {:?}", next_light(light));
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action() {
        assert_eq!(action(TrafficLight::Red), "停止");
        assert_eq!(action(TrafficLight::Yellow), "准备");
        assert_eq!(action(TrafficLight::Green), "通行");
    }

    #[test]
    fn test_duration() {
        assert_eq!(duration(TrafficLight::Red), 30);
        assert_eq!(duration(TrafficLight::Yellow), 3);
        assert_eq!(duration(TrafficLight::Green), 25);
    }

    #[test]
    fn test_should_stop() {
        assert!(should_stop(TrafficLight::Red));
        assert!(should_stop(TrafficLight::Yellow));
        assert!(!should_stop(TrafficLight::Green));
    }

    #[test]
    fn test_next_light() {
        assert_eq!(next_light(TrafficLight::Red), TrafficLight::Green);
        assert_eq!(next_light(TrafficLight::Green), TrafficLight::Yellow);
        assert_eq!(next_light(TrafficLight::Yellow), TrafficLight::Red);
    }

    #[test]
    fn test_full_cycle() {
        let start = TrafficLight::Red;
        let cycle = vec![
            next_light(start),
            next_light(next_light(start)),
            next_light(next_light(next_light(start))),
        ];
        assert_eq!(cycle, vec![TrafficLight::Green, TrafficLight::Yellow, TrafficLight::Red]);
    }
}
