//! # 练习 9: 排序和分组
pub fn sort_vec(vec: &mut Vec<i32>) {
    vec.sort();
}
pub fn group_by_parity(vec: &Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let evens: Vec<i32> = vec.iter().filter(|x| *x % 2 == 0).copied().collect();
    let odds: Vec<i32> = vec.iter().filter(|x| *x % 2 != 0).copied().collect();
    (evens, odds)
}

fn main() {
    let mut numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
    sort_vec(&mut numbers);
    let (evens, odds) = group_by_parity(&numbers);
    println!("sorted: {:?}, evens: {:?}, odds: {:?}", numbers, evens, odds);
}
