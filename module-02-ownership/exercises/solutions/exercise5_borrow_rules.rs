//! # 练习 5 解答: 借用规则挑战
//!
//! **提示**：请先尝试自己实现，再参考此解答！
//!
//! # 实现思路
//!
//! 1. **借用规则**：
//!    - 规则 1：同一时间，要么只有一个可变引用，要么有多个不可变引用
//!    - 规则 2：引用必须始终有效
//!
//! 2. **解决冲突**：
//!    - 使用作用域 `{}` 限制引用的生命周期
//!    - 不可变引用使用完毕后才能创建可变引用
//!
//! 3. **编译器保护**：
//!    - Rust 编译器在编译时检查借用规则
//!    - 防止数据竞争和悬垂引用

/// 尝试同时使用多个引用
///
/// 这个函数展示不能在不可变引用存在时创建可变引用
pub fn multiple_refs_demo() -> String {
    let mut s = String::from("hello");

    // 创建不可变引用
    let r1 = &s;
    let r2 = &s;

    // 不能在这里创建可变引用！
    // let r3 = &mut s;  // 编译错误
    // 原因：r1 和 r2 仍然在使用中

    format!("r1={}, r2={}", r1, r2)
} // r1, r2 在这里离开作用域

/// 修复后的版本 - 正确使用多个引用
///
/// # 实现思路
/// 使用作用域块来限制不可变引用的生命周期
pub fn correct_multiple_refs() -> String {
    let mut s = String::from("hello");

    // 不可变引用作用域
    {
        let r1 = &s;
        let r2 = &s;
        println!("不可变引用: {} 和 {}", r1, r2);
    } // r1 和 r2 在这里离开作用域，不再被使用

    // 现在可以创建可变引用了
    let r3 = &mut s;
    r3.push_str(" world");
    r3.to_uppercase()
}

/// 计算字符串长度并修改它
///
/// # 实现思路
/// 分两步进行：先获取长度（不可变借用），然后修改（可变借用）
/// 关键是让不可变借用在使用完成后结束
pub fn len_then_modify(s: &mut String, suffix: &str) -> usize {
    // 第一步：获取长度（不可变借用）
    let len = s.len();
    // len 在这里就被计算出来，不再借用 s

    // 第二步：修改字符串（可变借用）
    s.push_str(suffix);
    len
}

/// 更复杂的例子：修改后返回新长度
pub fn modify_and_return_length(s: &mut String, to_append: &str) -> usize {
    // 可以多次借用，只要不是同时的
    let original_len = s.len(); // 不可变借用结束

    s.push_str(to_append); // 可变借用

    let new_len = s.len(); // 新的不可变借用
    new_len
}

fn main() {
    println!("=== 借用规则演示 ===\n");

    // 演示 1: 多个不可变引用
    let result1 = multiple_refs_demo();
    println!("演示 1: {}", result1);

    // 演示 2: 正确的引用使用
    let result2 = correct_multiple_refs();
    println!("演示 2: {}", result2);

    // 演示 3: 长度计算后修改
    let mut text = String::from("rust");
    let len = len_then_modify(&mut text, " is awesome");
    println!("演示 3: 长度={}, 结果={}", len, text);

    // 演示 4: 复杂场景
    println!("\n演示 4: 复杂修改");
    let mut s = String::from("hello");
    let len1 = s.len();

    s.push_str(" world");
    let len2 = s.len();

    println!("原始长度: {}, 新长度: {}, 内容: {}", len1, len2, s);

    // 演示 5: 作用域的重要性
    println!("\n演示 5: 作用域控制");
    let mut message = String::from("test");

    // 先读取
    {
        let r = &message;
        println!("读取: {}", r);
    } // r 离开作用域

    // 再修改
    {
        let r = &mut message;
        r.push_str("ing");
        println!("修改: {}", r);
    } // r 离开作用域

    // 再次读取
    let r = &message;
    println!("最终: {}", r);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiple_refs_demo() {
        let result = multiple_refs_demo();
        assert!(result.contains("hello"));
    }

    #[test]
    fn test_correct_multiple_refs() {
        let result = correct_multiple_refs();
        assert!(result.contains("HELLO WORLD"));
    }

    #[test]
    fn test_len_then_modify() {
        let mut s = String::from("test");
        let len = len_then_modify(&mut s, "ing");
        assert_eq!(s, "testing");
        assert_eq!(len, 4); // 原始长度
    }

    #[test]
    fn test_len_then_modify_empty() {
        let mut s = String::from("");
        let len = len_then_modify(&mut s, "hello");
        assert_eq!(s, "hello");
        assert_eq!(len, 0);
    }

    #[test]
    fn test_modify_and_return_length() {
        let mut s = String::from("rust");
        let len = modify_and_return_length(&mut s, "y");
        assert_eq!(s, "rusty");
        assert_eq!(len, 5);
    }

    #[test]
    fn test_borrow_checker_prevents_data_race() {
        // 这个测试展示借用检查器如何防止数据竞争
        let mut s = String::from("safe");

        // 多个不可变引用 - OK
        let r1 = &s;
        let r2 = &s;
        assert_eq!(r1, r2);

        // 在使用完 r1 和 r2 后才能创建可变引用
        let r3 = &mut s;
        r3.push_str("ty");

        assert_eq!(s, "safety");
    }

    #[test]
    #[should_panic] // 这个测试展示编译器会捕获的错误
    fn test_mutable_and_immutable_conflict() {
        let mut s = String::from("test");

        // 这个代码会编译失败，所以用 should_panic 标记
        // 实际上编译器会阻止这种代码运行

        let r1 = &s; // 不可变引用
        let r2 = &mut s; // 错误！不能同时有可变引用

        // 这行代码不会编译通过
        println!("{}, {}", r1, r2);
    }

    #[test]
    fn test_multiple_mutable_not_allowed() {
        let mut s = String::from("test");

        let r1 = &mut s;
        // let r2 = &mut s;  // 错误！不能有多个可变引用

        r1.push_str("ing");
        assert_eq!(r1, "testing");
    }

    #[test]
    fn test_scope_separation_allows_reborrow() {
        let mut s = String::from("hello");

        {
            let r1 = &mut s;
            r1.push_str(" there");
            assert_eq!(r1, "hello there");
        } // r1 离开作用域

        {
            let r2 = &mut s; // 现在可以创建新的可变引用
            r2.push_str("!");
            assert_eq!(r2, "hello there!");
        }

        assert_eq!(s, "hello there!");
    }

    #[test]
    fn test_practical_pattern_read_modify() {
        // 实际编程中的常见模式：读取数据，根据结果修改
        let mut data = String::from("start");

        // 读取阶段
        let should_append = data.len() < 10;

        // 修改阶段（在读取完成后）
        if should_append {
            data.push_str("_extended");
        }

        assert_eq!(data, "start_extended");
    }
}
