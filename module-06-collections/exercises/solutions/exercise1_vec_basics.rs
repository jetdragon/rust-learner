//! # 练习 1: Vec 基础解答
//!
//! **解答要点:**
//! 1. 使用 iter() 创建迭代器
//! 2. 使用 filter() 过滤元素
//! 3. 使用 collect() 收集结果

pub fn vec_sum(vec: &Vec<i32>) -> i32 {
    vec.iter().sum()
}

// /// 使用 fold 的替代实现
// pub fn vec_sum_fold(vec: &Vec<i32>) -> i32 {
//     vec.iter().fold(0, |acc, &x| acc + x)
// }

pub fn vec_filter_positive(vec: &Vec<i32>) -> Vec<i32> {
    vec.iter().filter(|&&x| x > 0).copied().collect()
}

// /// 使用 for 循环的替代实现
// pub fn vec_filter_positive_loop(vec: &Vec<i32>) -> Vec<i32> {
//     let mut result = Vec::new();
//     for &x in vec {
//         if x > 0 {
//             result.push(x);
//         }
//     }
//     result
// }

fn main() {
    let numbers = vec![1, -2, 3, -4, 5];
    println!("Sum: {}", vec_sum(&numbers));
    println!("Positive: {:?}", vec_filter_positive(&numbers));
}
