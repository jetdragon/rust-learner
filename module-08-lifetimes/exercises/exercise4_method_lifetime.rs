//! # 练习 4: 方法的生命周期
//!
//! 难度: 中等
//! 时间: 15 分钟
//! 前置知识: 结构体生命周期
//! 学习目标:
//!   - 理解 impl 块中的生命周期声明
//!   - 区分 self 的生命周期和其他参数的生命周期

pub struct Context<'a> {
    data: &'a str,
}

impl<'a> Context<'a> {
    pub fn new(data: &'a str) -> Self {
        Context { data }
    }

    // TODO: 这个方法的返回值生命周期是什么？
    // 提示: 省略规则适用于此方法
    pub fn get_data(&self) -> &str {
        self.data
    }

    // TODO: 比较这个方法与上面有什么不同
    pub fn compare_with_input<'b>(&self, input: &'b str) -> bool {
        self.data == input
    }
}

fn main() {
    let text = String::from("Hello, World!");
    let ctx = Context::new(&text);

    println!("Data: {}", ctx.get_data());
    println!("Match: {}", ctx.compare_with_input("Hello, World!"));
}
