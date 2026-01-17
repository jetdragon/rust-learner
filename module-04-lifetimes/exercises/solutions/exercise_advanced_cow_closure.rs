//! # 进阶练习：Cow 和闭包解答
//!
//! **解答要点:**
//! 1. Cow (Clone on Write) 允许延迟克隆
//! 2. 闭包可以捕获环境中的变量
//! 3. 闭包捕获变量的方式：move, 借用

use std::borrow::Cow;

/// 使用 Cow 实现条件克隆
pub fn to_uppercase<'a>(s: &'a str) -> Cow<'a, str> {
    if s.chars().any(|c| c.is_uppercase()) {
        // 已经有大写，直接返回借用的 str
        Cow::Borrowed(s)
    } else {
        // 需要转换，创建拥有的 String
        Cow::Owned(s.to_uppercase())
    }
}

/// Cow 示例：可能克隆，可能借用
pub fn maybe_clone<'a>(s: &'a str, should_clone: bool) -> Cow<'a, str> {
    if should_clone {
        Cow::Owned(s.to_string())
    } else {
        Cow::Borrowed(s)
    }
}

/// 闭包捕获变量
pub fn create_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

/// 闭包捕获引用
pub fn create_multiplier<'a>(factor: &'a i32) -> impl Fn(i32) -> i32 + 'a {
    move |x| x * factor
}

/// 高阶函数：使用闭包
pub fn apply_twice<F>(mut f: F, value: i32) -> i32
where
    F: FnMut(i32) -> i32,
{
    f(value);
    f(value)
}

/// 闭包计数器
pub fn create_counter() -> impl Fn() -> usize {
    let mut count = 0;
    move || {
        count += 1;
        count
    }
}

/// 惰性求值闭包
pub fn lazy_evaluator<T, F>(creator: F) -> impl Fn() -> T
where
    T: Clone,
    F: FnOnce() -> T,
{
    let mut value: Option<T> = None;
    move || {
        if let Some(v) = value {
            v
        } else {
            let v = creator();
            value = Some(v.clone());
            v
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_uppercase_borrowed() {
        let result = to_uppercase("HELLO");
        assert_eq!(result.as_ref(), "HELLO");
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    #[test]
    fn test_to_uppercase_owned() {
        let result = to_uppercase("hello");
        assert_eq!(result.as_ref(), "HELLO");
        assert!(matches!(result, Cow::Owned(_)));
    }

    #[test]
    fn test_maybe_clone_false() {
        let s = "hello";
        let result = maybe_clone(s, false);
        assert_eq!(result.as_ref(), "hello");
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    #[test]
    fn test_maybe_clone_true() {
        let result = maybe_clone("hello", true);
        assert_eq!(result.as_ref(), "hello");
        assert!(matches!(result, Cow::Owned(_)));
    }

    #[test]
    fn test_closure_adder() {
        let add_5 = create_adder(5);
        assert_eq!(add_5(10), 15);
        assert_eq!(add_5(20), 25);
    }

    #[test]
    fn test_closure_multiplier() {
        let factor = 3;
        let multiply = create_multiplier(&factor);
        assert_eq!(multiply(5), 15);
        assert_eq!(multiply(10), 30);
    }

    #[test]
    fn test_apply_twice() {
        let result = apply_twice(|x| x + 1, 5);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_counter() {
        let counter = create_counter();
        assert_eq!(counter(), 1);
        assert_eq!(counter(), 2);
        assert_eq!(counter(), 3);
    }

    #[test]
    fn test_lazy_evaluator() {
        let mut call_count = 0;
        let lazy = lazy_evaluator(|| {
            call_count += 1;
            42
        });

        assert_eq!(call_count, 0);
        assert_eq!(lazy(), 42);
        assert_eq!(call_count, 1);
        assert_eq!(lazy(), 42);
        assert_eq!(call_count, 2); // 使用缓存的值
    }
}
