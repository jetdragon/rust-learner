//! # 练习 2: HashMap 基础
use std::collections::HashMap;

pub fn word_count(text: &str) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        *map.entry(word.to_string()).or_insert(0) += 1;
    }
    map
}

fn main() {
    let result = word_count("hello world hello");
    println!("word count: {:?}", result);
}
