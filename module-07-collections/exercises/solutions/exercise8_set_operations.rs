//! # 练习 8: Set 操作解答
//!
//! **解答要点:**
//! 1. 使用集合运算: intersection, union, difference
//! 2. 使用 retain() 就地修改集合
//! 3. 理解借用规则

use std::collections::HashSet;

pub fn set_union(a: &HashSet<i32>, b: &HashSet<i32>) -> HashSet<i32> {
    a.union(b).copied().collect()
}

pub fn set_intersection(a: &HashSet<i32>, b: &HashSet<i32>) -> HashSet<i32> {
    a.intersection(b).copied().collect()
}

pub fn set_difference(a: &HashSet<i32>, b: &HashSet<i32>) -> HashSet<i32> {
    a.difference(b).copied().collect()
}

// /// 就地修改集合
// pub fn retain_gt(mut set: HashSet<i32>, threshold: i32) -> HashSet<i32> {
//     set.retain(|&x| x > threshold);
//     set
// }

fn main() {
    let a: HashSet<_> = [1, 2, 3].iter().cloned().collect();
    let b: HashSet<_> = [3, 4, 5].iter().cloned().collect();
    println!("union: {:?}", set_union(&a, &b));
    println!("intersection: {:?}", set_intersection(&a, &b));
    println!("difference: {:?}", set_difference(&a, &b));
}
