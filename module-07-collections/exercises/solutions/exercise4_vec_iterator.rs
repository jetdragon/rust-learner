//! # 练习 4: Vec 迭代器解答
//!
//! **解答要点:**
//! 1. map() 转换每个元素
//! 2. filter() 过滤元素
//! 3. copied() 将 &i32 转为 i32

pub fn double_elements(vec: &Vec<i32>) -> Vec<i32> {
    vec.iter().map(|x| x * 2).collect()
}

pub fn filter_even(vec: &Vec<i32>) -> Vec<i32> {
    vec.iter().filter(|x| *x % 2 == 0).copied().collect()
}

// /// 链式调用示例
// pub fn double_and_filter(vec: &Vec<i32>) -> Vec<i32> {
//     vec.iter()
//         .map(|x| x * 2)
//         .filter(|&&x| x > 5)
//         .collect()
// }

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled = double_elements(&numbers);
    let evens = filter_even(&numbers);
    println!("doubled: {:?}, evens: {:?}", doubled, evens);
}
