//! # 练习 5: HashMap 合并
use std::collections::HashMap;
pub fn merge_maps(map1: HashMap<String, i32>, map2: HashMap<String, i32>) -> HashMap<String, i32> {
    map1.into_iter().chain(map2).collect()
}

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
