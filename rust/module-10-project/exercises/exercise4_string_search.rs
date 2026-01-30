//! # 练习 4: 字符串搜索
//!
//! 难度: 中等
//! 时间: 20 分钟
//! 前置知识: 迭代器, 闭包
//! 学习目标:
//!   - 实现大小写敏感搜索
//!   - 实现大小写不敏感搜索

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub fn search_with_context<'a>(
    query: &str,
    contents: &'a str,
    context_lines: usize,
) -> Vec<ContextLine<'a>> {
    let lines: Vec<&str> = contents.lines().collect();
    let mut results = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        if line.contains(query) {
            let start = if i > context_lines {
                i - context_lines
            } else {
                0
            };

            let end = std::cmp::min(i + context_lines + 1, lines.len());

            for j in start..end {
                let context_type = if j == i {
                    ContextType::Match
                } else if j < i {
                    ContextType::Before
                } else {
                    ContextType::After
                };

                results.push(ContextLine {
                    line: lines[j],
                    line_number: j + 1,
                    context_type,
                });
            }

            results.push(ContextLine {
                line: "---",
                line_number: 0,
                context_type: ContextType::Separator,
            });
        }
    }

    results
}

#[derive(Debug)]
pub struct ContextLine<'a> {
    pub line: &'a str,
    pub line_number: usize,
    pub context_type: ContextType,
}

#[derive(Debug)]
pub enum ContextType {
    Before,
    Match,
    After,
    Separator,
}

fn main() {
    let contents = "Hello World\nhello rust\nProgramming in Rust\nRust is awesome";

    println!("Case sensitive search for 'hello':");
    for line in search("hello", contents) {
        println!("  - {}", line);
    }

    println!("\nCase insensitive search for 'hello':");
    for line in search_case_insensitive("hello", contents) {
        println!("  - {}", line);
    }

    println!("\nSearch with context:");
    for ctx in search_with_context("Rust", contents, 1) {
        match ctx.context_type {
            ContextType::Match => println!("{}: > {}", ctx.line_number, ctx.line),
            ContextType::Before => println!("{}:   {}", ctx.line_number, ctx.line),
            ContextType::After => println!("{}:   {}", ctx.line_number, ctx.line),
            ContextType::Separator => println!("{}", ctx.line),
        }
    }
}
