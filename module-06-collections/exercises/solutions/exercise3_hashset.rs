//! # 练习 3: HashSet 去重解答
//!
//! **解答要点:**
//! 1. HashSet 自动去重
//! 2. 使用 copied() 转换 &i32 为 i32
//! 3. 收集时保持唯一性

use std::collections::HashSet;

pub fn unique_elements(vec: &Vec<i32>) -> Vec<i32> {
    let set: HashSet<_> = vec.iter().copied().collect();
    set.into_iter().collect()
}

// /// 保持顺序的去重
// pub fn unique_ordered(vec: &Vec<i32>) -> Vec<i32> {
//     let mut seen = HashSet::new();
//     vec.iter()
//         .filter(|&x| seen.insert(x))
//         .copied()
//         .collect()
// }

fn main() {
    let input = vec![1, 2, 2, 3, 4, 4, 5];
    let result = unique_elements(&input);
    println!("unique elements: {:?}", result);
}
