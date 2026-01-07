//! # 练习 8: 文档注释
//!
//! 难度: 简单
//! 时间: 10 分钟
//! 前置知识: 函数基础
//! 学习目标:
//!   - 编写文档注释
//!   - 使用文档测试

/// 在文本中搜索匹配的行
///
/// # Arguments
///
/// * `query` - 要搜索的字符串
/// * `text` - 要搜索的文本
///
/// # Returns
///
/// 包含查询字符串的所有行
///
/// # Examples
///
/// ```
/// let text = "Hello World\nHello Rust";
/// let results = search("Hello", text);
/// assert_eq!(results.len(), 2);
/// ```
pub fn search<'a>(query: &str, text: &'a str) -> Vec<&'a str> {
    text.lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// 配置结构体
///
/// # Fields
///
/// * `query` - 搜索查询字符串
/// * `filename` - 要搜索的文件名
/// * `case_sensitive` - 是否区分大小写
///
/// # Examples
///
/// ```
/// let config = Config {
///     query: String::from("test"),
///     filename: String::from("test.txt"),
///     case_sensitive: true,
/// };
/// ```
pub struct Config {
    /// 搜索查询字符串
    pub query: String,
    /// 要搜索的文件名
    pub filename: String,
    /// 是否区分大小写
    pub case_sensitive: bool,
}

impl Config {
    /// 从命令行参数创建 Config
    ///
    /// # Arguments
    ///
    /// * `args` - 命令行参数（不包括程序名）
    ///
    /// # Errors
    ///
    /// 如果参数不足，返回错误字符串
    ///
    /// # Examples
    ///
    /// ```
    /// let args = vec![
    ///     String::from("program"),
    ///     String::from("test"),
    ///     String::from("file.txt"),
    /// ];
    /// let config = Config::new(&args).unwrap();
    /// assert_eq!(config.query, "test");
    /// ```
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = true;

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

fn main() {
    let args = vec![
        String::from("program"),
        String::from("test"),
        String::from("file.txt"),
    ];

    let config = Config::new(&args).unwrap();
    println!("Config: {:?}", config);
}
