// examples/hashset.rs
// HashSet 演示

use module_06_collections::{difference, intersection, union, unique_elements};

use std::collections::HashSet;

fn main() {
    println!("=== HashSet 演示 ===\n");

    // 1. 创建 HashSet
    println!("1. 创建 HashSet:");
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(2); // 重复被忽略
    println!("  set = {:?}", set);

    // 2. 包含检查
    println!("\n2. 包含检查:");
    println!("  包含 2: {}", set.contains(&2));
    println!("  包含 5: {}", set.contains(&5));

    // 3. 去重
    println!("\n3. 去重:");
    let vec = vec![1, 2, 2, 3, 3, 3];
    println!("  原始: {:?}", vec);
    println!("  唯一: {:?}", unique_elements(&vec));

    // 4. 集合运算
    println!("\n4. 集合运算:");
    let set1: HashSet<i32> = [1, 2, 3].iter().copied().collect();
    let set2: HashSet<i32> = [3, 4, 5].iter().copied().collect();
    println!("  set1 = {:?}", set1);
    println!("  set2 = {:?}", set2);

    println!("  并集: {:?}", union(&set1, &set2));
    println!("  交集: {:?}", intersection(&set1, &set2));
    println!("  差集: {:?}", difference(&set1, &set2));

    // 5. 遍历
    println!("\n5. 遍历:");
    for item in &set {
        println!("  {}", item);
    }

    // 6. 删除
    println!("\n6. 删除:");
    let mut set = HashSet::from([1, 2, 3]);
    set.remove(&2);
    println!("  删除2后: {:?}", set);

    // 7. 长度
    println!("\n7. 长度:");
    println!("  len = {}", set.len());
    println!("  is_empty = {}", set.is_empty());
}
