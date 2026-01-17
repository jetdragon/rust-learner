//! # 练习 5: HashMap 合并解答
//!
//! **解答要点:**
//! 1. into_iter() 消费 HashMap
//! 2. chain() 连接两个迭代器
//! 3. collect() 生成新 HashMap

use std::collections::HashMap;

pub fn merge_maps(
    map1: HashMap<String, i32>,
    map2: HashMap<String, i32>,
) -> HashMap<String, i32> {
    map1.into_iter().chain(map2).collect()
}

// /// 处理重复键的合并
// pub fn merge_maps_with_override(
//     mut map1: HashMap<String, i32>,
//     map2: HashMap<String, i32>,
// ) -> HashMap<String, i32> {
//     map2.into_iter().for_each(|(k, v)| {
//         map1.insert(k, v);
//     });
//     map1
// }

fn main() {
    let mut map1 = HashMap::new();
    map1.insert(String::from("a"), 1);
    map1.insert(String::from("b"), 2);

    let mut map2 = HashMap::new();
    map2.insert(String::from("b"), 3);
    map2.insert(String::from("c"), 4);

    let result = merge_maps(map1, map2);
    println!("merged map: {:?}", result);
}
