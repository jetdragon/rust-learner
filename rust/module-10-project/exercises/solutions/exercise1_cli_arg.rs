//! # 练习 1: CLI 参数解析 - 解答
//!
//! 难度: 简单
//! 时间: 15 分钟
//! 学习目标: 解析命令行参数

use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next(); // 跳过程序名

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <query> <filename>", args[0]);
        eprintln!("Example: {} hello.txt world", args[0]);
        std::process::exit(1);
    }

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    println!(
        "Searching for '{}' in file '{}', case sensitive: {}",
        config.query, config.filename, config.case_sensitive
    );
}
