// 06-集合类型 模块集成测试

use module_06_collections::*;

#[test]
fn test_vec_operations() {
    let numbers = vec![1, 2, 3, 4, 5];
    assert_eq!(vec_sum(&numbers), 15);
    assert_eq!(double_elements(&numbers), vec![2, 4, 6, 8, 10]);
    assert_eq!(find_largest(&numbers), Some(5));
    assert_eq!(find_index(&numbers, 3), Some(2));
}

#[test]
fn test_hashmap_operations() {
    let counts = word_count("hello world hello");
    assert_eq!(counts.get("hello"), Some(&2));
}

#[test]
fn test_hashset_operations() {
    let mut result = unique_elements(&[1, 2, 2, 3]);
    result.sort();
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn test_statistics() {
    assert_eq!(average(&[1, 2, 3, 4, 5]), Some(3.0));
    assert_eq!(median(&[1, 2, 3]), Some(2.0));
}

#[test]
fn test_comprehensive() {
    // Vec test
    let vec = vec![1, 2, 2, 3, 3, 3];
    let freq = frequency_map(&vec);
    assert_eq!(freq.get(&1), Some(&1));
    assert_eq!(freq.get(&3), Some(&3));

    // HashMap test
    let text = "a b c a b a";
    let counts = word_count(text);
    assert_eq!(counts.get("a"), Some(&3));

    // HashSet test
    let unique = unique_elements(&vec);
    assert_eq!(unique.len(), 3);
}
