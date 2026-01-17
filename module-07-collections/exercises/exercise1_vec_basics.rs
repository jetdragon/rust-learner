//! # 练习 1: Vec 基础操作
pub fn vec_sum(vec: &Vec<i32>) -> i32 {
    vec.iter().sum()
}

pub fn vec_filter_positive(vec: &Vec<i32>) -> Vec<i32> {
    vec.iter().filter(|&&x| x > 0).copied().collect()
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    println!("和: {}", vec_sum(&v));
    println!("正数: {:?}", vec_filter_positive(&v));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vec_sum() {
        assert_eq!(vec_sum(&vec![1, 2, 3, 4, 5]), 15);
    }
    #[test]
    fn test_vec_filter_positive() {
        assert_eq!(vec_filter_positive(&vec![-1, 2, -3, 4]), vec![2, 4]);
    }
}
