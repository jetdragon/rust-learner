// examples/hashmap.rs
// HashMap 演示

use module_06_collections::{map_to_vec, merge_maps, vec_to_map, word_count};

use std::collections::HashMap;

fn main() {
    println!("=== HashMap 演示 ===\n");

    // 1. 创建 HashMap
    println!("1. 创建 HashMap:");
    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 10);
    scores.insert(String::from("Bob"), 20);
    println!("  scores = {:?}", scores);

    // 2. 访问值
    println!("\n2. 访问值:");
    println!("  Alice: {:?}", scores.get("Alice"));
    println!("  Eve: {:?}", scores.get("Eve"));

    // 3. 更新值
    println!("\n3. 更新值:");
    scores.insert(String::from("Alice"), 15);
    println!("  更新后: {:?}", scores.get("Alice"));

    // 4. 词频统计
    println!("\n4. 词频统计:");
    let text = "hello world hello rust";
    let counts = word_count(text);
    println!("  {:?}", counts);

    // 5. 遍历
    println!("\n5. 遍历:");
    for (key, value) in &counts {
        println!("  {} -> {}", key, value);
    }

    // 6. Entry API
    println!("\n6. Entry API:");
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.entry("count").or_insert(0);
    println!("  {:?}", map);

    // 7. Vec 转 HashMap
    println!("\n7. Vec 转 HashMap:");
    let vec = vec![("a".to_string(), 1), ("b".to_string(), 2)];
    let map = vec_to_map(vec);
    println!("  {:?}", map);

    // 8. HashMap 转 Vec
    println!("\n8. HashMap 转 Vec:");
    let vec = map_to_vec(&map);
    println!("  {:?}", vec);

    // 9. 合并 HashMap
    println!("\n9. 合并 HashMap:");
    let mut map1 = HashMap::new();
    map1.insert("a".to_string(), 1);
    let mut map2 = HashMap::new();
    map2.insert("a".to_string(), 2);
    map2.insert("b".to_string(), 3);
    let merged = merge_maps(map1, map2);
    println!("  {:?}", merged);
}
