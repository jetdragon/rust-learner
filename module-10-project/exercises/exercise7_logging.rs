//! # 练习 7: 日志
//!
//! 难度: 简单
//! 时间: 10 分钟
//! 前置知识: 函数基础
//! 学习目标:
//!   - 使用 eprintln! 打印错误信息
//!   - 区分标准输出和错误输出

use std::env;

pub fn run(config: Config) -> Result<(), String> {
    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);

    // TODO: 读取文件并搜索
    let contents = "Hello World\nHello Rust\nProgramming in Rust";

    let results = if config.case_sensitive {
        search(&config.query, contents)
    } else {
        search_case_insensitive(&config.query, contents)
    };

    for result in results {
        println!("{}", result);
    }

    Ok(())
}

pub fn search(query: &str, text: &str) -> Vec<&str> {
    text.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive(query: &str, text: &str) -> Vec<&str> {
    let query = query.to_lowercase();
    text.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
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

    let config = Config::new(&args).unwrap_or_else(|err| {
        // TODO: 使用 eprintln! 打印错误信息
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    if let Err(e) = run(config) {
        // TODO: 使用 eprintln! 打印错误信息
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
