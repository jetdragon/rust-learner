//! # 练习 9: 排序和分组解答
//!
//! **解答要点:**
//! 1. sort() 就地排序，需要 mut 引用
//! 2. partition() 分区满足/不满足条件的元素
//! 3. 使用解构接收分区结果

pub fn sort_vec(vec: &mut Vec<i32>) {
    vec.sort();
}

pub fn group_by_parity(vec: &Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let evens: Vec<i32> = vec.iter().filter(|x| *x % 2 == 0).copied().collect();
    let odds: Vec<i32> = vec.iter().filter(|x| *x % 2 != 0).copied().collect();
    (evens, odds)
}

// /// 使用 partition 的替代实现
// pub fn group_by_parity_partition(vec: &[i32]) -> (Vec<i32>, Vec<i32>) {
//     vec.iter()
//         .partition(|&&x| x % 2 == 0)
// }

fn main() {
    let mut numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
    sort_vec(&mut numbers);
    let (evens, odds) = group_by_parity(&numbers);
    println!("sorted: {:?}, evens: {:?}, odds: {:?}", numbers, evens, odds);
}
