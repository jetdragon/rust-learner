//! # 练习 10: BTreeMap 有序映射解答
//!
//! **解答要点:**
//! 1. BTreeMap 保持键的排序
//! 2. range() 查询范围
//! 3. 使用 String 类型作为范围参数

use std::collections::BTreeMap;

pub fn get_range(
    map: &BTreeMap<String, i32>,
    start: String,
    end: String,
) -> Vec<(&String, &i32)> {
    map.range::<String, _>(start..=end).collect()
}

// /// 使用 &str 的替代版本
// pub fn get_range_str<'a>(
//     map: &BTreeMap<String, i32>,
//     start: &'a str,
//     end: &'a str,
// ) -> Vec<(&String, &i32)> {
//     map.range::<&str, _>(start..=end).collect()
// }

fn main() {
    let mut map = BTreeMap::new();
    map.insert(String::from("a"), 1);
    map.insert(String::from("b"), 2);
    map.insert(String::from("c"), 3);
    map.insert(String::from("d"), 4);
    let range = get_range(&map, String::from("b"), String::from("c"));
    println!("range b-c: {:?}", range);
    println!("all keys in order: {:?}", map.keys().collect::<Vec<_>>());
}
