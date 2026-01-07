//! # 练习 8: 集合运算
use std::collections::HashSet;

pub fn union(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32> {
    set1.union(set2).copied().collect()
}

pub fn intersection(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32> {
    set1.intersection(set2).copied().collect()
}

pub fn difference(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32> {
    set1.difference(set2).copied().collect()
}

fn main() {
    let set1: HashSet<_> = vec![1, 2, 3].into_iter().collect();
    let set2: HashSet<_> = vec![3, 4, 5].into_iter().collect();

    println!("并集: {:?}", union(&set1, &set2));
    println!("交集: {:?}", intersection(&set1, &set2));
    println!("差集: {:?}", difference(&set1, &set2));
}
