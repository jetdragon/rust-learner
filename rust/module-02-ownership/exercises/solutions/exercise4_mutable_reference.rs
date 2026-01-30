//! # 练习 4 解答: 可变引用
//!
//! **提示**：请先尝试自己实现，再参考此解答！
//!
//! # 实现思路
//!
//! 1. **可变引用语法**：`&mut T` 创建类型 T 的可变引用
//! 2. **可变变量**：创建可变引用前，原变量必须声明为 `mut`
//! 3. **唯一性**：同一时间只能有一个可变引用
//! 4. **修改数据**：通过可变引用可以修改原始数据
//!
//! # 关键概念
//!
//! - **可变借用**：`&mut T` 允许修改借用的值
//! - **独占访问**：可变引用保证独占访问，防止数据竞争
//! - **作用域限制**：可变引用的作用域结束后才能创建新的引用

/// 向字符串追加 ", world"
///
/// # 实现思路
/// 1. 使用 `&mut String` 接收可变引用
/// 2. 使用 `push_str` 方法追加字符串
pub fn append_world(s: &mut String) {
    s.push_str(", world");
}

/// 将数字翻倍
///
/// # 实现思路
/// 解引用后修改值
fn double_value(n: &mut i32) {
    *n *= 2;
}

/// 给数字增加指定值
pub fn add_value(n: &mut i32, value: i32) {
    *n += value;
}

/// 清空字符串
pub fn clear_string(s: &mut String) {
    s.clear();
}

/// 修剪字符串到指定长度
pub fn truncate_string(s: &mut String, new_len: usize) {
    s.truncate(new_len);
}

fn main() {
    println!("=== 可变引用演示 ===\n");

    // 演示 1: 基本可变引用
    println!("演示 1: 基本可变引用");
    let mut s = String::from("hello");
    append_world(&mut s);
    println!("结果: {}", s);

    // 演示 2: 修改数字
    println!("\n演示 2: 修改数字");
    let mut num = 5;
    double_value(&mut num);
    println!("翻倍: {}", num);

    // 演示 3: 链式修改
    println!("\n演示 3: 链式修改");
    let mut value = 10;
    add_value(&mut value, 5);
    add_value(&mut value, 3);
    println!("最终值: {}", value);

    // 演示 4: 字符串操作
    println!("\n演示 4: 字符串操作");
    let mut text = String::from("Hello World!");
    clear_string(&mut text);
    println!("清空后: '{}'", text);

    text.push_str("New content");
    truncate_string(&mut text, 3);
    println!("截断后: '{}'", text);

    // 演示 5: 可变引用作用域
    println!("\n演示 5: 可变引用作用域");
    let mut s = String::from("test");
    {
        let r1 = &mut s;
        r1.push_str("ing");
        println!("内部: {}", r1);
    } // r1 离开作用域

    let r2 = &mut s; // 现在可以创建新的可变引用
    r2.push_str(" completed");
    println!("外部: {}", r2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_world() {
        let mut s = String::from("hello");
        append_world(&mut s);
        assert_eq!(s, "hello, world");
    }

    #[test]
    fn test_append_world_custom() {
        let mut s = String::from("rust");
        append_world(&mut s);
        assert_eq!(s, "rust, world");
    }

    #[test]
    fn test_double_value() {
        let mut n = 5;
        double_value(&mut n);
        assert_eq!(n, 10);
    }

    #[test]
    fn test_double_value_zero() {
        let mut n = 0;
        double_value(&mut n);
        assert_eq!(n, 0);
    }

    #[test]
    fn test_double_value_negative() {
        let mut n = -3;
        double_value(&mut n);
        assert_eq!(n, -6);
    }

    #[test]
    fn test_add_value() {
        let mut n = 10;
        add_value(&mut n, 5);
        assert_eq!(n, 15);
    }

    #[test]
    fn test_add_value_negative() {
        let mut n = 10;
        add_value(&mut n, -3);
        assert_eq!(n, 7);
    }

    #[test]
    fn test_clear_string() {
        let mut s = String::from("hello");
        clear_string(&mut s);
        assert!(s.is_empty());
        assert_eq!(s.len(), 0);
    }

    #[test]
    fn test_truncate_string() {
        let mut s = String::from("hello world");
        truncate_string(&mut s, 5);
        assert_eq!(s, "hello");
    }

    #[test]
    fn test_truncate_string_zero() {
        let mut s = String::from("test");
        truncate_string(&mut s, 0);
        assert!(s.is_empty());
    }

    #[test]
    fn test_mutable_reference_uniqueness() {
        let mut s = String::from("test");
        let r1 = &mut s;

        // let r2 = &mut s;  // 错误！不能同时有多个可变引用
        // println!("{}, {}", r1, r2);

        r1.push_str("ing");
        assert_eq!(r1, "testing");
    } // r1 离开作用域后，r2 才能创建

    #[test]
    fn test_mutable_after_immutable_scope() {
        let mut s = String::from("hello");

        {
            let r1 = &s;
            let r2 = &s;
            assert_eq!(r1, r2);
        } // 不可变引用离开作用域

        let r3 = &mut s; // 现在可以创建可变引用
        r3.push_str(" world");
        assert_eq!(r3, "hello world");
    }
}
