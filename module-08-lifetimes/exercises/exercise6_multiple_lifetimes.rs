//! # 练习 6: 多个生命周期参数
//!
//! 难度: 中等
//! 时间: 15 分钟
//! 前置知识: 生命周期标注
//! 学习目标:
//!   - 使用多个生命周期参数
//!   - 理解不同生命周期之间的关系

// TODO: 添加生命周期参数
pub fn format_with_title<'a, 'b>(title: &'a str, content: &'b str) -> String
where
    'a: 'b, // 'a 必须至少和 'b 一样长
{
    format!("=== {} ===\n{}", title, content)
}

pub struct Document<'a, 'b> {
    pub title: &'a str,
    pub content: &'b str,
}

impl<'a, 'b> Document<'a, 'b> {
    pub fn new(title: &'a str, content: &'b str) -> Self {
        Document { title, content }
    }

    // TODO: 这个方法的返回值应该使用哪个生命周期？
    pub fn get_summary(&self) -> String
    where
        'a: 'b,
    {
        format!("{}: {}", self.title, self.content)
    }
}

fn main() {
    let title = String::from("My Document");
    let content = String::from("This is the content.");

    {
        let local_content = String::from("Short content");
        let doc = Document::new(&title, &local_content);
        println!("Summary: {}", doc.get_summary());
    }
}
