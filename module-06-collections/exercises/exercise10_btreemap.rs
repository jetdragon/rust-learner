//! # 练习 10: BTreeMap 有序映射
use std::collections::BTreeMap;
pub fn get_range(map: &BTreeMap<String, i32>, start: String, end: String) -> Vec<(&String, &i32)> {
    map.range::<String, _>(start..=end).collect()
}

fn main() {
    let mut map = BTreeMap::new();
    map.insert(String::from("a"), 1);
    map.insert(String::from("b"), 2);
    map.insert(String::from("c"), 3);
    map.insert(String::from("d"), 4);
    let range = get_range(&map, String::from("b"), String::from("c"));
    println!("range b-c: {:?}", range);
}
