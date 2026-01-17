//! # 练习 11: BTreeMap 有序操作解答
//!
//! **解答要点:**
//! 1. BTreeMap 会按键自动排序
//! 2. range() 方法用于范围查询
//! 3. range().collect() 收集范围内的键值对

use std::collections::BTreeMap;

/// 获取排序后前 k 个键值对
pub fn get_first_k_sorted(map: &BTreeMap<i32, String>, k: usize) -> Vec<(i32, String)> {
    map.iter().take(k).map(|(&k, &v)| (k, v.clone())).collect()
}

/// 范围查询
pub fn range_query(map: &BTreeMap<i32, String>, start: i32, end: i32) -> Vec<(i32, String)> {
    map.range(start..=end)
        .map(|(&k, &v)| (k, v.clone()))
        .collect()
}

/// 获取排序后的所有键
pub fn sorted_keys(map: &BTreeMap<i32, String>) -> Vec<i32> {
    map.keys().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_first_k_sorted() {
        let mut map = BTreeMap::new();
        map.insert(3, "c".to_string());
        map.insert(1, "a".to_string());
        map.insert(2, "b".to_string());
        map.insert(5, "e".to_string());
        map.insert(4, "d".to_string());

        let result = get_first_k_sorted(&map, 3);
        assert_eq!(
            result,
            vec![
                (1, "a".to_string()),
                (2, "b".to_string()),
                (3, "c".to_string())
            ]
        );
    }

    #[test]
    fn test_range_query() {
        let mut map = BTreeMap::new();
        for i in 1..=10 {
            map.insert(i, format!("value_{}", i));
        }

        let result = range_query(&map, 3, 6);
        assert_eq!(result.len(), 4);
        assert_eq!(result[0].0, 3);
        assert_eq!(result[3].0, 6);
    }

    #[test]
    fn test_sorted_keys() {
        let mut map = BTreeMap::new();
        map.insert(3, "c".to_string());
        map.insert(1, "a".to_string());
        map.insert(2, "b".to_string());

        let keys = sorted_keys(&map);
        assert_eq!(keys, vec![1, 2, 3]);
    }
}
