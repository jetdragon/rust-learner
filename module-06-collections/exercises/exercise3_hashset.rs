//! # 练习 3: HashSet 去重
use std::collections::HashSet;
pub fn unique_elements(vec: &Vec<i32>) -> Vec<i32> {
    let set: HashSet<_> = vec.iter().copied().collect();
    set.into_iter().collect()
}

fn main() {
    let input = vec![1, 2, 2, 3, 4, 4, 5];
    let result = unique_elements(&input);
    println!("unique elements: {:?}", result);
}
