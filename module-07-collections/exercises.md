# 练习题

## 集合类型练习

完成以下练习以巩固集合类型的知识。

### 练习 1: Vec 基础操作

```rust
pub fn vec_sum(vec: &Vec<i32>) -> i32 {
    todo!()
}

pub fn vec_filter_positive(vec: &Vec<i32>) -> Vec<i32> {
    todo!()
}

#[test]
fn test_vec_sum() {
    assert_eq!(vec_sum(&vec![1, 2, 3, 4, 5]), 15);
}

#[test]
fn test_vec_filter_positive() {
    assert_eq!(vec_filter_positive(&vec![-1, 2, -3, 4]), vec![2, 4]);
}
```

### 练习 2: HashMap 基础

```rust
use std::collections::HashMap;

pub fn word_count(text: &str) -> HashMap<String, usize> {
    todo!()
}

#[test]
fn test_word_count() {
    let result = word_count("hello world hello");
    assert_eq!(result.get("hello"), Some(&2));
    assert_eq!(result.get("world"), Some(&1));
}
```

### 练习 3: HashSet 去重

```rust
use std::collections::HashSet;

pub fn unique_elements(vec: &Vec<i32>) -> Vec<i32> {
    todo!()
}

#[test]
fn test_unique_elements() {
    assert_eq!(unique_elements(&vec![1, 2, 2, 3, 3, 3]), vec![1, 2, 3]);
}
```

### 练习 4: Vec 迭代器

```rust
pub fn double_elements(vec: &Vec<i32>) -> Vec<i32> {
    todo!()
}

pub fn filter_even(vec: &Vec<i32>) -> Vec<i32> {
    todo!()
}

#[test]
fn test_double_elements() {
    assert_eq!(double_elements(&vec![1, 2, 3]), vec![2, 4, 6]);
}

#[test]
fn test_filter_even() {
    assert_eq!(filter_even(&vec![1, 2, 3, 4, 5]), vec![2, 4]);
}
```

### 练习 5: HashMap 合并

```rust
pub fn merge_maps(map1: HashMap<String, i32>, map2: HashMap<String, i32>) -> HashMap<String, i32> {
    todo!()
}

#[test]
fn test_merge_maps() {
    let mut map1 = HashMap::new();
    map1.insert("a".to_string(), 1);

    let mut map2 = HashMap::new();
    map2.insert("b".to_string(), 2);

    let merged = merge_maps(map1, map2);
    assert_eq!(merged.len(), 2);
}
```

### 练习 6: Vec 查找

```rust
pub fn find_largest(vec: &Vec<i32>) -> Option<i32> {
    todo!()
}

pub fn find_index(vec: &Vec<i32>, target: i32) -> Option<usize> {
    todo!()
}

#[test]
fn test_find_largest() {
    assert_eq!(find_largest(&vec![1, 5, 3]), Some(5));
    assert_eq!(find_largest(&vec![]), None);
}
```

### 练习 7: HashMap 转换

```rust
pub fn vec_to_map(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    todo!()
}

pub fn map_to_vec(map: &HashMap<String, i32>) -> Vec<(String, i32)> {
    todo!()
}
```

### 练习 8: 集合运算

```rust
pub fn union(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32> {
    todo!()
}

pub fn intersection(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32> {
    todo!()
}

pub fn difference(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32> {
    todo!()
}
```

### 练习 9: Vec 分组和排序

```rust
pub fn group_by_parity(vec: &Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    // 返回 (偶数, 奇数)
    todo!()
}

pub fn sort_descending(mut vec: Vec<i32>) -> Vec<i32> {
    todo!()
}
```

### 练习 10: 复杂操作

```rust
pub fn top_n(vec: &Vec<i32>, n: usize) -> Vec<i32> {
    // 返回前 n 个最大的元素
    todo!()
}

pub fn frequency_map(vec: &Vec<i32>) -> HashMap<i32, usize> {
    // 返回每个元素出现的次数
    todo!()
}
```

## 提示

1. 使用 iter() 创建迭代器
2. 使用 map() 转换元素
3. 使用 filter() 过滤元素
4. 使用 collect() 收集结果
5. HashMap 使用 insert() 和 get()
6. HashSet 使用 insert() 和 contains()

## 答案

查看 [src/lib.rs](src/lib.rs) 中的实现和测试。
