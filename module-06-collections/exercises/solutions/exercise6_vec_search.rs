//! # 练习 6: Vec 查找解答
//!
//! **解答要点:**
//! 1. max() 返回最大值的引用
//! 2. position() 查找元素位置
//! 3. 使用 copied() 转换引用类型

pub fn find_largest(vec: &Vec<i32>) -> Option<i32> {
    vec.iter().max().copied()
}

pub fn find_index(vec: &Vec<i32>, target: i32) -> Option<usize> {
    vec.iter().position(|&x| x == target)
}

// /// 使用 rposition 从后查找
// pub fn find_last_index(vec: &Vec<i32>, target: i32) -> Option<usize> {
//     vec.iter().rposition(|&x| x == target)
// }

fn main() {
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let largest = find_largest(&numbers);
    let index = find_index(&numbers, 5);
    println!("largest: {:?}, index of 5: {:?}", largest, index);
}
