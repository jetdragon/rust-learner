// examples/btreemap.rs
// BTreeMap 和 BTreeSet 演示

fn main() {
    println!("=== BTreeMap 演示 ===\n");

    // 1. BTreeMap 基础
    println!("1. BTreeMap 基础:");
    use std::collections::BTreeMap;

    let mut scores = BTreeMap::new();
    scores.insert("Alice", 95);
    scores.insert("Bob", 87);
    scores.insert("Charlie", 92);

    println!("  成绩表:");
    for (name, score) in &scores {
        println!("    {}: {}", name, score);
    }

    // 2. BTreeMap 有序性
    println!("\n2. BTreeMap 自动排序:");
    let mut map = BTreeMap::new();
    map.insert(5, "five");
    map.insert(1, "one");
    map.insert(3, "three");
    map.insert(2, "two");
    map.insert(4, "four");

    println!("  插入顺序: 5, 1, 3, 2, 4");
    println!("  迭代顺序:");
    for (key, value) in &map {
        println!("    {} => {}", key, value); // 1, 2, 3, 4, 5
    }

    // 3. 范围查询
    println!("\n3. 范围查询:");
    let range = map.range(2..=4);
    println!("  范围 [2..=4]:");
    for (key, value) in range {
        println!("    {} => {}", key, value); // 2, 3, 4
    }

    // 4. BTreeSet 基础
    println!("\n4. BTreeSet 基础:");
    use std::collections::BTreeSet;

    let set1: BTreeSet<i32> = (1..=5).collect();
    let set2: BTreeSet<i32> = (3..=7).collect();

    println!("  set1: {:?}", set1); // {1, 2, 3, 4, 5}
    println!("  set2: {:?}", set2); // {3, 4, 5, 6, 7}

    // 5. 集合操作
    println!("\n5. 集合操作:");

    // 并集
    let union: BTreeSet<_> = set1.union(&set2).cloned().collect();
    println!("  并集: {:?}", union); // {1, 2, 3, 4, 5, 6, 7}

    // 交集
    let intersection: BTreeSet<_> = set1.intersection(&set2).cloned().collect();
    println!("  交集: {:?}", intersection); // {3, 4, 5}

    // 差集
    let difference: BTreeSet<_> = set1.difference(&set2).cloned().collect();
    println!("  set1 - set2: {:?}", difference); // {1, 2}

    // 对称差集
    let symmetric_difference: BTreeSet<_> = set1.symmetric_difference(&set2).cloned().collect();
    println!("  对称差集: {:?}", symmetric_difference); // {1, 2, 6, 7}

    // 6. BTreeSet 查询
    println!("\n6. BTreeSet 查询:");
    let numbers: BTreeSet<_> = (1..=10).collect();

    // contains
    println!("  包含 5: {}", numbers.contains(&5)); // true
    println!("  包含 15: {}", numbers.contains(&15)); // false

    // subset/superset
    let subset: BTreeSet<_> = (1..=5).collect();
    println!(
        "  subset 是 numbers 的子集: {}",
        numbers.is_superset(&subset)
    ); // true

    // first/last
    if let Some(&first) = numbers.first() {
        println!("  最小值: {}", first); // 1
    }
    if let Some(&last) = numbers.last() {
        println!("  最大值: {}", last); // 10
    }

    // 7. 实际应用：时间线
    println!("\n7. 实际应用：时间线:");
    use std::cmp::Ordering;

    #[derive(Debug, Clone)]
    struct Event {
        timestamp: i64,
        message: String,
    }

    impl Event {
        fn new(timestamp: i64, message: &str) -> Self {
            Event {
                timestamp,
                message: message.to_string(),
            }
        }
    }

    let mut timeline: BTreeMap<i64, Event> = BTreeMap::new();
    timeline.insert(3, Event::new(3, "任务完成"));
    timeline.insert(1, Event::new(1, "项目开始"));
    timeline.insert(4, Event::new(4, "代码审查"));
    timeline.insert(2, Event::new(2, "开始开发"));

    println!("  时间线（按时间排序）:");
    for (timestamp, event) in &timeline {
        println!("    t={}: {}", timestamp, event.message);
    }

    // 查询特定时间段的事件
    println!("\n  查询 t=2 到 t=3 的事件:");
    for (timestamp, event) in timeline.range(2..=3) {
        println!("    t={}: {}", timestamp, event.message);
    }

    // 8. 性能对比：HashMap vs BTreeMap
    println!("\n8. 性能对比（大样本）:");
    use std::collections::HashMap;
    use std::time::Instant;

    const N: usize = 100_000;

    // HashMap
    let mut hashmap: HashMap<i32, i32> = HashMap::new();
    let start = Instant::now();
    for i in 0..N {
        hashmap.insert(i as i32, i as i32);
    }
    let hashmap_insert = start.elapsed();
    println!("  HashMap 插入 {} 个元素: {:?}", N, hashmap_insert);

    let start = Instant::now();
    for i in 0..N {
        hashmap.get(&(i as i32));
    }
    let hashmap_lookup = start.elapsed();
    println!("  HashMap 查找 {} 次: {:?}", N, hashmap_lookup);

    // BTreeMap
    let mut btreemap: BTreeMap<i32, i32> = BTreeMap::new();
    let start = Instant::now();
    for i in 0..N {
        btreemap.insert(i as i32, i as i32);
    }
    let btreemap_insert = start.elapsed();
    println!("  BTreeMap 插入 {} 个元素: {:?}", N, btreemap_insert);

    let start = Instant::now();
    for i in 0..N {
        btreemap.get(&(i as i32));
    }
    let btreemap_lookup = start.elapsed();
    println!("  BTreeMap 查找 {} 次: {:?}", N, btreemap_lookup);

    println!("\n  结论:");
    println!("    HashMap 通常更快（O(1) vs O(log n)）");
    println!("    BTreeMap 提供有序性和范围查询");
}
