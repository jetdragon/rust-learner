//! # 练习 7: HashMap 转换
use std::collections::HashMap;

pub fn vec_to_map(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    vec.into_iter().collect()
}

pub fn map_to_vec(map: &HashMap<String, i32>) -> Vec<(String, i32)> {
    map.iter().map(|(k, v)| (k.clone(), *v)).collect()
}

fn main() {
    let vec = vec![("a".to_string(), 1), ("b".to_string(), 2)];
    let map = vec_to_map(vec);
    println!("Map: {:?}", map);

    let vec2 = map_to_vec(&map);
    println!("Vec: {:?}", vec2);
}
