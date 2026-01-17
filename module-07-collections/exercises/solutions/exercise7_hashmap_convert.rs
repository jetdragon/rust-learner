//! # 练习 7: HashMap 转换解答
//!
//! **解答要点:**
//! 1. 使用 into_iter() + map() 转换键值对
//! 2. 使用 drain() 保留原 HashMap
//! 3. 灵活处理键和值的转换

use std::collections::HashMap;

pub fn invert_map(map: HashMap<String, i32>) -> HashMap<i32, String> {
    map.into_iter().map(|(k, v)| (v, k)).collect()
}

pub fn swap_keys_and_values(map: HashMap<&str, i32>) -> HashMap<i32, &str> {
    map.into_iter().map(|(k, v)| (v, k)).collect()
}

// /// 使用 drain 的非消费版本
// pub fn invert_map_drain(mut map: HashMap<String, i32>) -> HashMap<i32, String> {
//     map.drain().map(|(k, v)| (v, k)).collect()
// }

fn main() {
    let mut map = HashMap::new();
    map.insert(String::from("a"), 1);
    map.insert(String::from("b"), 2);
    let inverted = invert_map(map);
    println!("inverted: {:?}", inverted);
}
