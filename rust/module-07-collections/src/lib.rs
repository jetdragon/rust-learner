//! # 集合类型 (Collections)
//!
//! 本模块提供 Rust 集合类型的实用函数示例。
//!
//! ## 模块内容
//!
//! - Vec 操作函数：求和、过滤、查找等
//! - HashMap 操作函数：合并、转换等
//! - HashSet 操作函数：集合运算等
//! - 迭代器使用示例
//!
//! ## 示例
//!
//! ```
//! use module_06_collections::vec_sum;
//! use std::collections::HashSet;
//!
//! assert_eq!(vec_sum(&vec![1, 2, 3]), 6);
//!
//! let unique: HashSet<i32> = [1, 2, 2, 3].iter().copied().collect();
//! assert_eq!(unique.len(), 3);
//! ```

use std::collections::{HashMap, HashSet};

// ========== Vec 操作函数 ==========

/// 计算 Vec 中所有元素的和
///
/// # 示例
///
/// ```
/// use module_06_collections::vec_sum;
///
/// assert_eq!(vec_sum(&vec![1, 2, 3, 4, 5]), 15);
/// ```
pub fn vec_sum(vec: &[i32]) -> i32 {
    vec.iter().sum()
}

/// 过滤出正数
pub fn vec_filter_positive(vec: &[i32]) -> Vec<i32> {
    vec.iter().filter(|&&x| x > 0).copied().collect()
}

/// 翻倍所有元素
pub fn double_elements(vec: &[i32]) -> Vec<i32> {
    vec.iter().map(|&x| x * 2).collect()
}

/// 过滤偶数
pub fn filter_even(vec: &[i32]) -> Vec<i32> {
    vec.iter().filter(|&&x| x % 2 == 0).copied().collect()
}

/// 查找最大值
pub fn find_largest(vec: &[i32]) -> Option<i32> {
    vec.iter().copied().max()
}

/// 查找元素索引
pub fn find_index(vec: &[i32], target: i32) -> Option<usize> {
    vec.iter().position(|&x| x == target)
}

/// 按奇偶分组
pub fn group_by_parity(vec: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let evens: Vec<i32> = vec.iter().filter(|&&x| x % 2 == 0).copied().collect();
    let odds: Vec<i32> = vec.iter().filter(|&&x| x % 2 != 0).copied().collect();
    (evens, odds)
}

/// 降序排序
pub fn sort_descending(mut vec: Vec<i32>) -> Vec<i32> {
    vec.sort_by(|a, b| b.cmp(a));
    vec
}

/// 返回前 n 个最大元素
pub fn top_n(vec: &[i32], n: usize) -> Vec<i32> {
    let mut sorted = vec.to_vec();
    sorted.sort_by(|a, b| b.cmp(a));
    sorted.into_iter().take(n).collect()
}

/// 计算元素频率
pub fn frequency_map(vec: &[i32]) -> HashMap<i32, usize> {
    let mut freq = HashMap::new();
    for &item in vec {
        *freq.entry(item).or_insert(0) += 1;
    }
    freq
}

// ========== HashMap 操作函数 ==========

/// 词频统计
pub fn word_count(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        let word = word.to_lowercase();
        *counts.entry(word).or_insert(0) += 1;
    }
    counts
}

/// 合并两个 HashMap
pub fn merge_maps(
    mut map1: HashMap<String, i32>,
    map2: HashMap<String, i32>,
) -> HashMap<String, i32> {
    for (key, value) in map2 {
        map1.entry(key).and_modify(|e| *e += value).or_insert(value);
    }
    map1
}

/// Vec 转换为 HashMap
pub fn vec_to_map(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    vec.into_iter().collect()
}

/// HashMap 转换为 Vec
pub fn map_to_vec(map: &HashMap<String, i32>) -> Vec<(String, i32)> {
    map.iter().map(|(k, v)| (k.clone(), *v)).collect()
}

// ========== HashSet 操作函数 ==========

/// 返回唯一元素
pub fn unique_elements(vec: &[i32]) -> Vec<i32> {
    let set: HashSet<i32> = vec.iter().copied().collect();
    set.into_iter().collect()
}

/// 集合并集
pub fn union(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32> {
    set1.union(set2).copied().collect()
}

/// 集合交集
pub fn intersection(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32> {
    set1.intersection(set2).copied().collect()
}

/// 集合差集
pub fn difference(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32> {
    set1.difference(set2).copied().collect()
}

// ========== 统计函数 ==========

/// 计算平均值
pub fn average(vec: &[i32]) -> Option<f64> {
    if vec.is_empty() {
        return None;
    }
    Some(vec_sum(vec) as f64 / vec.len() as f64)
}

/// 计算中位数
pub fn median(vec: &[i32]) -> Option<f64> {
    if vec.is_empty() {
        return None;
    }
    let mut sorted = vec.to_vec();
    sorted.sort();
    let len = sorted.len();
    if len.is_multiple_of(2) {
        Some((sorted[len / 2 - 1] + sorted[len / 2]) as f64 / 2.0)
    } else {
        Some(sorted[len / 2] as f64)
    }
}

// ========== 测试 ==========

#[cfg(test)]
mod tests {
    use super::*;

    // Vec 测试
    #[test]
    fn test_vec_sum() {
        assert_eq!(vec_sum(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(vec_sum(&[]), 0);
        assert_eq!(vec_sum(&[-1, -2, -3]), -6);
    }

    #[test]
    fn test_vec_filter_positive() {
        assert_eq!(vec_filter_positive(&[1, -2, 3, -4]), vec![1, 3]);
        assert_eq!(vec_filter_positive(&[-1, -2]), vec![]);
    }

    #[test]
    fn test_double_elements() {
        assert_eq!(double_elements(&[1, 2, 3]), vec![2, 4, 6]);
    }

    #[test]
    fn test_filter_even() {
        assert_eq!(filter_even(&[1, 2, 3, 4]), vec![2, 4]);
    }

    #[test]
    fn test_find_largest() {
        assert_eq!(find_largest(&[1, 5, 3]), Some(5));
        assert_eq!(find_largest(&[]), None);
    }

    #[test]
    fn test_find_index() {
        assert_eq!(find_index(&[1, 2, 3, 4], 3), Some(2));
        assert_eq!(find_index(&[1, 2, 3], 5), None);
    }

    #[test]
    fn test_group_by_parity() {
        assert_eq!(group_by_parity(&[1, 2, 3, 4]), (vec![2, 4], vec![1, 3]));
    }

    #[test]
    fn test_sort_descending() {
        assert_eq!(sort_descending(vec![3, 1, 2]), vec![3, 2, 1]);
    }

    #[test]
    fn test_top_n() {
        assert_eq!(top_n(&[5, 1, 3, 2, 4], 3), vec![5, 4, 3]);
    }

    #[test]
    fn test_frequency_map() {
        let freq = frequency_map(&[1, 2, 2, 3, 3, 3]);
        assert_eq!(freq.get(&1), Some(&1));
        assert_eq!(freq.get(&2), Some(&2));
        assert_eq!(freq.get(&3), Some(&3));
    }

    // HashMap 测试
    #[test]
    fn test_word_count() {
        let counts = word_count("hello world hello");
        assert_eq!(counts.get("hello"), Some(&2));
        assert_eq!(counts.get("world"), Some(&1));
    }

    #[test]
    fn test_merge_maps() {
        let mut map1 = HashMap::new();
        map1.insert("a".to_string(), 1);

        let mut map2 = HashMap::new();
        map2.insert("a".to_string(), 2);
        map2.insert("b".to_string(), 3);

        let merged = merge_maps(map1, map2);
        assert_eq!(merged.get("a"), Some(&3));
        assert_eq!(merged.get("b"), Some(&3));
    }

    #[test]
    fn test_vec_to_map() {
        let vec = vec![("a".to_string(), 1), ("b".to_string(), 2)];
        let map = vec_to_map(vec);
        assert_eq!(map.get("a"), Some(&1));
    }

    #[test]
    fn test_map_to_vec() {
        let mut map = HashMap::new();
        map.insert("a".to_string(), 1);
        let vec = map_to_vec(&map);
        assert_eq!(vec.len(), 1);
    }

    // HashSet 测试
    #[test]
    fn test_unique_elements() {
        let mut result = unique_elements(&[1, 2, 2, 3, 3, 3]);
        result.sort();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_union() {
        let set1: HashSet<i32> = [1, 2].iter().copied().collect();
        let set2: HashSet<i32> = [2, 3].iter().copied().collect();
        let result = union(&set1, &set2);
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_intersection() {
        let set1: HashSet<i32> = [1, 2, 3].iter().copied().collect();
        let set2: HashSet<i32> = [2, 3, 4].iter().copied().collect();
        let result = intersection(&set1, &set2);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_difference() {
        let set1: HashSet<i32> = [1, 2, 3].iter().copied().collect();
        let set2: HashSet<i32> = [2, 3, 4].iter().copied().collect();
        let result = difference(&set1, &set2);
        assert_eq!(result.len(), 1);
    }

    // 统计测试
    #[test]
    fn test_average() {
        assert_eq!(average(&[1, 2, 3, 4, 5]), Some(3.0));
        assert_eq!(average(&[]), None);
    }

    #[test]
    fn test_median() {
        assert_eq!(median(&[1, 2, 3]), Some(2.0));
        assert_eq!(median(&[1, 2, 3, 4]), Some(2.5));
        assert_eq!(median(&[]), None);
    }

    // 综合测试
    #[test]
    fn test_comprehensive_collections() {
        // Vec 操作
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(vec_sum(&numbers), 15);
        assert_eq!(double_elements(&numbers), vec![2, 4, 6, 8, 10]);

        // HashMap 操作
        let text = "hello world hello rust";
        let counts = word_count(text);
        assert_eq!(counts.get("hello"), Some(&2));

        // HashSet 操作
        let unique = unique_elements(&numbers);
        assert_eq!(unique.len(), 5);

        // 统计
        assert_eq!(average(&numbers), Some(3.0));
    }
}
