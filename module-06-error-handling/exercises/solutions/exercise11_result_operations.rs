//! # 练习 11: Result 基础操作解答
//!
//! **解答要点:**
//! 1. 使用 map, or, unwrap_or 等 Result 方法
//! 2. 理解如何组合多个 Result
//! 3. 提供默认值处理错误情况

pub fn map_result_option(opt: Option<i32>) -> Result<i32, String> {
    opt.map_or(Err("没有值".to_string()), |v| Ok(v * 2))
}

pub fn combine_results(
    a: Result<i32, String>,
    b: Result<i32, String>,
) -> Result<(i32, i32), String> {
    match (a, b) {
        (Ok(x), Ok(y)) => Ok((x, y)),
        (Err(e), _) | (_, Err(e)) => Err(e),
    }
}

pub fn unwrap_or_default(result: Result<i32, String>) -> i32 {
    result.unwrap_or(0)
}

pub fn and_then_option(r: Result<i32, String>) -> Option<i32> {
    r.ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_result_option() {
        assert_eq!(map_result_option(Some(5)), Ok(10));
        assert_eq!(map_result_option(None), Err("没有值".to_string()));
    }

    #[test]
    fn test_combine_results() {
        assert_eq!(combine_results(Ok(5), Ok(3)), Ok((5, 3)));
        assert!(combine_results(Ok(5), Err("错误".to_string())).is_err());
    }

    #[test]
    fn test_unwrap_or_default() {
        assert_eq!(unwrap_or_default(Ok(42)), 42);
        assert_eq!(unwrap_or_default(Err("错误".to_string())), 0);
    }

    #[test]
    fn test_and_then_option() {
        assert_eq!(and_then_option(Ok(42)), Some(42));
        assert_eq!(and_then_option(Err("错误".to_string())), None);
    }
}
