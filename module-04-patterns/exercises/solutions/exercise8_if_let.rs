//! # 练习 8 解答: if let 简化
//!
//! **提示**：请先尝试自己实现，再参考此解答！
//!
//! # 实现思路
//!
//! 1. if let：简化只关心一种情况的 match
//! 2. 代码更简洁：减少嵌套和样板代码
//! 3. 适用场景：单分支匹配
//! 4. else 分支：可选，处理其他情况

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

/// 使用 if let 只关心 None 的情况
pub fn print_if_none(value: Option<i32>) {
    if value.is_none() {
        println!("值为 None");
    } else {
        println!("值存在");
    }
}

/// 使用 if let 处理 Result
pub fn print_result(res: Result<i32, String>) {
    if let Ok(v) = res {
        println!("成功: {}", v);
    } else if let Err(e) = res {
        println!("错误: {}", e);
    }
}

/// 简化的 if let（仅处理成功情况）
pub fn handle_option(value: Option<i32>) -> String {
    if let Some(v) = value {
        format!("值是 {}", v)
    } else {
        "没有值".to_string()
    }
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

    // 对比 match 和 if let
    println!("\n=== match vs if let ===");

    let value = Some(42);

    // 使用 match（更冗长）
    let _match_result = match value {
        Some(v) => println!("match: 值为 {}", v),
        None => println!("match: 没有值"),
    };

    // 使用 if let（更简洁）
    if let Some(v) = value {
        println!("if let: 值为 {}", v);
    }

    // let else 模式的思想
    println!("\n=== let else 模式思想 ===");
    let result = parse_or_default("42");
    println!("解析结果: {}", result);

    let result2 = parse_or_default("abc");
    println!("解析失败结果: {}", result2);
}

/// 模拟 let else 模式（Rust 1.65+）
fn parse_or_default(s: &str) -> i32 {
    // 传统方式：使用 match
    /*
    match s.parse::<i32>() {
        Ok(n) => n,
        Err(_) => 0,
    }
    */

    // 使用 ok().unwrap_or() 更简洁
    s.parse().ok().unwrap_or(0)
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

    #[test]
    fn test_handle_option() {
        assert_eq!(handle_option(Some(42)), "值是 42");
        assert_eq!(handle_option(None), "没有值");
    }

    #[test]
    fn test_parse_or_default() {
        assert_eq!(parse_or_default("42"), 42);
        assert_eq!(parse_or_default("abc"), 0);
    }

    #[test]
    fn test_if_let_vs_match() {
        // if let 和 match 的对比
        let value = Some(5);

        // match 版本
        let match_result = match value {
            Some(v) => v * 2,
            None => 0,
        };

        // if let 版本（需要 else）
        let if_let_result = if let Some(v) = value {
            v * 2
        } else {
            0
        };

        assert_eq!(match_result, if_let_result);
    }

    #[test]
    fn test_combinators_vs_if_let() {
        // 组合子 vs if let 的对比
        let value = Some(5);

        // 使用组合子
        let combinator_result = value.map(|v| v * 2).unwrap_or(0);

        // 使用 if let
        let if_let_result = if let Some(v) = value {
            v * 2
        } else {
            0
        };

        assert_eq!(combinator_result, if_let_result);
    }

    #[test]
    fn test_while_let() {
        // while let 的使用
        let mut optional = Some(0);
        let mut count = 0;

        while let Some(i) = optional {
            if i < 5 {
                optional = Some(i + 1);
                count += 1;
            } else {
                optional = None;
            }
        }

        assert_eq!(count, 6);
    }
}
