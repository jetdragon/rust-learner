//! # 练习 8: if let 简化
//!
//! **难度**: 入门
//! **预计时间**: 15 分钟

/// 使用 if let 简化只关心一种情况的匹配
pub fn print_if_positive(value: Option<i32>) {
    if let Some(n) = value {
        if n > 0 {
            println!("正数: {}", n);
        } else {
            println!("不是正数");
        }
    } else {
        println!("没有值");
    }
}

/// TODO: 使用 let else 或 if let 只关心 None 的情况
pub fn print_if_none(value: Option<i32>) {
    if value.is_none() {
        println!("值为 None");
    } else {
        println!("值存在");
    }
}

/// TODO: 使用 if let 处理 Result
pub fn print_result(res: Result<i32, String>) {
    unimplemented!()
}

fn main() {
    println!("=== if let 示例 ===\n");

    print_if_positive(Some(5));
    print_if_positive(Some(-3));
    print_if_positive(None);

    print_if_none(Some(10));
    print_if_none(None);

    print_result(Ok(42));
    print_result(Err("错误".to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_if_none() {
        // 只测试不 panic
        print_if_none(Some(1));
        print_if_none(None);
    }
}
