//! # 练习 2: HashMap 基础解答
//!
//! **解答要点:**
//! 1. 使用 entry API 处理已存在/不存在的键
//! 2. or_insert() 在键不存在时插入默认值
//! 3. split_whitespace() 分割字符串

use std::collections::HashMap;

pub fn word_count(text: &str) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        *map.entry(word.to_string()).or_insert(0) += 1;
    }
    map
}

// /// 使用 fold 的函数式实现
// pub fn word_count_fold(text: &str) -> HashMap<String, usize> {
//     text.split_whitespace()
//         .fold(HashMap::new(), |mut map, word| {
//             *map.entry(word.to_string()).or_insert(0) += 1;
//             map
//         })
// }

fn main() {
    let result = word_count("hello world hello rust world");
    println!("word count: {:?}", result);
}
