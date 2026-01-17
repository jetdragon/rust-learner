//! # 练习 4: 匹配枚举
//!
//! **难度**: 入门
//! **预计时间**: 10 分钟

/// 交通信号灯
#[derive(Debug, Clone, Copy)]
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

/// TODO: 返回对应动作
///
/// - Red: "停止"
/// - Yellow: "准备"
/// - Green: "通行"
pub fn action(light: TrafficLight) -> &'static str {
    match light {
        TrafficLight::Red => "停止",
        TrafficLight::Yellow => "准备",
        TrafficLight::Green => "通行",
    }
}

/// TODO: 返回信号灯持续时间（秒）
pub fn duration(light: TrafficLight) -> u32 {
    unimplemented!()
}

fn main() {
    let lights = [TrafficLight::Red, TrafficLight::Yellow, TrafficLight::Green];

    for light in lights {
        println!("{:?} -> {}", light, action(light));
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
}
