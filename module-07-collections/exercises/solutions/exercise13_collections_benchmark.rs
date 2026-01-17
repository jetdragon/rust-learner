//! # 练习 13: 集合性能对比解答
//!
//! **解答要点:**
//! 1. HashMap: O(1) 平均查找，无序
//! 2. BTreeMap: O(log n) 查找，按键排序
//! 3. 性能取决于数据量和使用场景

use std::collections::{BTreeMap, HashMap};
use std::time::Instant;

/// Benchmark Vec 查找（线性时间）
pub fn benchmark_vec_lookup(n: usize) -> u128 {
    let vec: Vec<i32> = (0..n).collect();
    let target = n - 1;

    let start = Instant::now();
    for _ in 0..1000 {
        vec.iter().find(|&&x| x == target);
    }
    start.elapsed().as_nanos()
}

/// Benchmark HashMap 查找（O(1) 平均）
pub fn benchmark_hashmap_lookup(n: usize) -> u128 {
    let mut map: HashMap<i32, String> = HashMap::new();
    for i in 0..n {
        map.insert(i, format!("value_{}", i));
    }
    let target = n - 1;

    let start = Instant::now();
    for _ in 0..1000 {
        map.get(&target);
    }
    start.elapsed().as_nanos()
}

/// Benchmark BTreeMap 查找（O(log n)）
pub fn benchmark_btreemap_lookup(n: usize) -> u128 {
    let mut map: BTreeMap<i32, String> = BTreeMap::new();
    for i in 0..n {
        map.insert(i, format!("value_{}", i));
    }
    let target = n - 1;

    let start = Instant::now();
    for _ in 0..1000 {
        map.get(&target);
    }
    start.elapsed().as_nanos()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmarks() {
        let n = 10000;
        let vec_time = benchmark_vec_lookup(n);
        let hashmap_time = benchmark_hashmap_lookup(n);
        let btreemap_time = benchmark_btreemap_lookup(n);

        // HashMap 应该比 Vec 快
        assert!(hashmap_time < vec_time / 10);

        // BTreeMap 查找时间应该介于 Vec 和 HashMap 之间
        assert!(vec_time > btreemap_time);
        assert!(hashmap_time < btreemap_time);
    }

    #[test]
    fn test_correctness() {
        let mut map = HashMap::new();
        map.insert(1, "one");
        assert_eq!(map.get(&1), Some(&"one".to_string()));
    }

    #[test]
    fn test_btreemap_sorted() {
        let mut map = BTreeMap::new();
        map.insert(3, "c");
        map.insert(1, "a");
        map.insert(2, "b");

        let keys: Vec<_> = map.keys().cloned().collect();
        assert_eq!(keys, vec![1, 2, 3]);
    }
}
