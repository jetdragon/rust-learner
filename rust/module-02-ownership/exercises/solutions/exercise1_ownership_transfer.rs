//! # 练习 1 解答: 所有权转移
//!
//! **提示**：请先尝试自己实现，再参考此解答！
//!
//! # 实现思路
//!
//! 1. **所有权转移**：在 Rust 中，`String` 类型的赋值会转移所有权
//! 2. **移动语义**：`let s2 = s1` 后，s1 不再有效
//! 3. **避免重复释放**：这防止了内存的二次释放（double free）
//!
//! # 关键概念
//!
//! - **Move**：String、Vec 等类型在赋值时发生移动
//! - **Copy**：整数、布尔、浮点等类型在赋值时自动复制
//! - **有效性**：被移动后的变量不能再使用

fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // 所有权从 s1 转移到 s1

    // println!("{}, world!", s1);  // 错误！s1 已经被移动
    println!("{}, world!", s2); // 正确

    // 对比：实现 Copy 特征的类型
    let x = 5;
    let y = x; // i32 实现了 Copy，所以 x 仍然有效

    println!("x = {}, y = {}", x, y);

    // 克隆：如果需要保留 s1，可以使用 clone
    let s3 = String::from("rust");
    let s4 = s3.clone(); // 深拷贝，s3 仍然有效

    println!("s3 = {}, s4 = {}", s3, s4);
}

/// 判断第一个字符是否大写
pub fn first_char(s: String) -> char {
    s.chars().next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ownership_transfer() {
        let s1 = String::from("hello");
        let s2 = s1;
        assert_eq!(s2, "hello");
        // assert_eq!(s1, "hello");  // 这行会编译失败
    }

    #[test]
    fn test_copy_types() {
        let x = 42;
        let y = x;
        assert_eq!(x, 42); // x 仍然有效
        assert_eq!(y, 42);
    }

    #[test]
    fn test_clone() {
        let s1 = String::from("test");
        let s2 = s1.clone();
        assert_eq!(s1, "test"); // s1 仍然有效
        assert_eq!(s2, "test");
    }

    #[test]
    fn test_first_char_takes_ownership() {
        let s = String::from("Hello");
        let c = first_char(s);
        assert_eq!(c, 'H');
        // println!("{}", s);  // 错误！s 的所有权已被转移
    }

    #[test]
    fn test_function_transfer() {
        let s = String::from("rust");
        let len = string_length(s);
        assert_eq!(len, 4);
        // assert_eq!(s, "rust");  // 错误！s 已被移动
    }

    #[test]
    fn test_vec_ownership() {
        let v1 = vec![1, 2, 3];
        let v2 = v1;
        assert_eq!(v2, vec![1, 2, 3]);
        // assert_eq!(v1, vec![1, 2, 3]);  // 错误
    }
}

/// 返回字符串长度（获取所有权）
pub fn string_length(s: String) -> usize {
    s.len()
}

// Stack-Only 的 Copy 类型示例：
// - 所有整数类型 (i32, u8, usize, 等)
// - 布尔类型 bool
// - 浮点类型 f64, f32
// - 字符类型 char
// - 元组（如果所有元素都是 Copy 的）
