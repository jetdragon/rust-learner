//! # 练习 6: Vec 查找
pub fn find_largest(vec: &Vec<i32>) -> Option<i32> {
    vec.iter().max().copied()
}
pub fn find_index(vec: &Vec<i32>, target: i32) -> Option<usize> {
    vec.iter().position(|&x| x == target)
}

fn main() {
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let largest = find_largest(&numbers);
    let index = find_index(&numbers, 5);
    println!("largest: {:?}, index of 5: {:?}", largest, index);
}
