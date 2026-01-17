//! # 练习 4: Vec 迭代器
pub fn double_elements(vec: &Vec<i32>) -> Vec<i32> {
    vec.iter().map(|x| x * 2).collect()
}
pub fn filter_even(vec: &Vec<i32>) -> Vec<i32> {
    vec.iter().filter(|x| *x % 2 == 0).copied().collect()
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled = double_elements(&numbers);
    let evens = filter_even(&numbers);
    println!("doubled: {:?}, evens: {:?}", doubled, evens);
}
