//! # 练习 2: 文件 I/O - 解答
//!
//! 难度: 简单
//! 时间: 15 分钟
//! 学习目标: 读取文件内容

use std::fs;
use std::error::Error;

pub fn read_file(filename: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    Ok(contents)
}

pub fn count_lines(contents: &str) -> usize {
    contents.lines().count()
}

pub fn count_words(contents: &str) -> usize {
    contents.split_whitespace().count()
}

fn main() {
    let filename = "test.txt";

    fs::write(filename, "Hello world\nThis is a test\nRust is awesome").unwrap();

    match read_file(filename) {
        Ok(contents) => {
            println!("File contents:");
            println!("{}", contents);
            println!("Lines: {}", count_lines(&contents));
            println!("Words: {}", count_words(&contents));
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }

    let _ = fs::remove_file(filename);
}
