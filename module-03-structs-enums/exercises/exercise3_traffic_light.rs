//! # 练习 3: 定义枚举
//!
//! **难度**: 入门
//! **预计时间**: 10 分钟
//!
//! **前置知识**:
//! - 基本的 Rust 语法
//! - match 表达式基础
//!
//! **学习目标**:
//! - 学习如何定义枚举
//! - 理解枚举变体
//! - 掌握使用 match 处理枚举

/// 交通信号灯枚举
///
/// # TODO
///
/// 定义 TrafficLight 枚举，包含三个变体：
/// - Red
/// - Yellow
/// - Green
pub enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    /// TODO: 返回信号灯的持续时间（秒）
    ///
    /// - Red: 30 秒
    /// - Yellow: 3 秒
    /// - Green: 25 秒
    pub fn duration(&self) -> u32 {
        unimplemented!()
    }

    /// TODO: 返回信号灯的中文描述
    ///
    pub fn description(&self) -> &str {
        unimplemented!()
    }

    /// TODO: 返回下一个信号灯
    ///
    /// 转换顺序：Red -> Green -> Yellow -> Red
    pub fn next(&self) -> TrafficLight {
        unimplemented!()
    }
}

fn main() {
    let lights = vec![TrafficLight::Red, TrafficLight::Yellow, TrafficLight::Green];

    println!("=== 交通信号灯 ===\n");

    for light in lights {
        println!("信号灯: {}", light.description());
        println!("持续时间: {} 秒", light.duration());
        println!("下一个: {}", light.next().description());
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
}
