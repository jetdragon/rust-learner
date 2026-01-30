//! # 练习 5: 模块组织 - 解答
//!
//! 难度: 中等
//! 时间: 15 分钟
//! 学习目标: 组织代码到模块

mod config {
    pub struct Config {
        pub query: String,
        pub filename: String,
    }

    impl Config {
        pub fn new(args: &[String]) -> Result<Config, &'static str> {
            if args.len() < 3 {
                return Err("not enough arguments");
            }

            let query = args[1].clone();
            let filename = args[2].clone();

            Ok(Config { query, filename })
        }
    }
}

mod search {
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
}

mod file_io {
    use std::fs;
    use std::error::Error;

    pub fn read_file(filename: &str) -> Result<String, Box<dyn Error>> {
        fs::read_to_string(filename)?
            .parse()
            .map_err(|e| e.into())
    }
}

use config::Config;
use search::{search, search_case_insensitive};

fn main() {
    let args = vec![
        String::from("program"),
        String::from("test"),
        String::from("file.txt"),
    ];

    match Config::new(&args) {
        Ok(config) => {
            println!("Searching for '{}' in '{}'", config.query, config.filename);

            let text = "Hello\nWorld\nTest\nTesting";
            let results = search(&config.query, text);

            for result in results {
                println!("Found: {}", result);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
