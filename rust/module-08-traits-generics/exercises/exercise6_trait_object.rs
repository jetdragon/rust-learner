//! # 练习 6: Trait 对象
//!
//! 难度: 中等
//! 时间: 15 分钟
//! 前置知识: trait, Box
//! 学习目标:
//!   - 使用 trait 对象实现动态分发
//!   - 理解 &dyn Trait 和 Box<dyn Trait>

pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "Drawing button: {} ({}x{})",
            self.label, self.width, self.height
        );
    }
}

pub struct TextField {
    pub width: u32,
    pub height: u32,
    pub placeholder: String,
}

// TODO: 为 TextField 实现 Draw trait

// TODO: 创建一个包含不同类型组件的函数
// 使用 &dyn Draw trait 对象

pub fn run_drawables(components: &[&dyn Draw]) {
    for component in components {
        component.draw();
    }
}

fn main() {
    let button = Button {
        width: 50,
        height: 10,
        label: String::from("Click"),
    };

    let text_field = TextField {
        width: 100,
        height: 20,
        placeholder: String::from("Enter text..."),
    };

    let components: Vec<&dyn Draw> = vec![&button, &text_field];
    run_drawables(&components);
}
