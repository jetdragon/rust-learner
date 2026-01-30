// 05-错误处理 模块集成测试

use module_05_error_handling::*;

// ========== 练习 1: 基本 Result 处理 ==========
#[test]
fn test_safe_divide_function() {
    assert_eq!(safe_divide(10, 2), Ok(5));
    assert_eq!(safe_divide(10, 0), Err(MathError::DivisionByZero));
    assert_eq!(safe_divide(-10, 2), Ok(-5));
}

// ========== 练习 2: Option 转换 ==========
#[test]
fn test_option_to_result() {
    assert_eq!(option_to_result(Some(5), "错误".to_string()), Ok(5));
    assert_eq!(
        option_to_result(None, "错误".to_string()),
        Err("错误".to_string())
    );
    assert_eq!(option_to_result_if_let(Some(5), "错误".to_string()), Ok(5));
    assert_eq!(
        option_to_result_combinator(Some(5), "错误".to_string()),
        Ok(5)
    );
}

// ========== 练习 3: 平方根计算 ==========
#[test]
fn test_safe_sqrt() {
    assert_eq!(safe_sqrt(9.0), Ok(3.0));
    assert!(safe_sqrt(-1.0).is_err());
    assert_eq!(safe_sqrt(0.0), Ok(0.0));
}

// ========== 练习 4: 错误传播 ==========
#[test]
fn test_parse_and_divide() {
    assert_eq!(parse_and_divide("10", "2"), Ok(5));
    assert!(parse_and_divide("abc", "2").is_err());
    assert!(parse_and_divide("10", "0").is_err());
}

// ========== 练习 5: 链式操作 ==========
#[test]
fn test_double_if_positive() {
    assert_eq!(double_if_positive(5), Ok(10));
    assert_eq!(double_if_positive(0), Err("值必须为正数".to_string()));
    assert_eq!(double_if_positive(-5), Err("值必须为正数".to_string()));
}

#[test]
fn test_validate_and_double() {
    assert_eq!(validate_and_double(Some(5)), Ok(10));
    assert!(validate_and_double(Some(-1)).is_err());
    assert!(validate_and_double(None).is_err());
}

// ========== 练习 7: 使用 unwrap_or ==========
#[test]
fn test_get_first_or_default() {
    assert_eq!(get_first_or_default(&[1, 2, 3]), 1);
    assert_eq!(get_first_or_default(&[]), 0);
    assert_eq!(get_first_or_default_lazy(&[1, 2, 3]), 1);
}

// ========== 练习 8: 使用 map 转换值 ==========
#[test]
fn test_square_if_positive() {
    assert_eq!(square_if_positive(Some(5)), Some(25));
    assert_eq!(square_if_positive(Some(-5)), Some(25));
    assert_eq!(square_if_positive(None), None);
}

// ========== 练习 9: 组合多个 Result ==========
#[test]
fn test_sum_results() {
    assert_eq!(sum_results(Ok(5), Ok(3)), Ok(8));
    assert_eq!(
        sum_results(Ok(5), Err("错误".to_string())),
        Err("错误".to_string())
    );
    assert_eq!(
        sum_results(Err("错误".to_string()), Ok(3)),
        Err("错误".to_string())
    );
}

#[test]
fn test_sum_all_results() {
    assert_eq!(sum_all_results(vec![Ok(1), Ok(2), Ok(3)]), Ok(6));
    assert!(sum_all_results(vec![Ok(1), Err("e".to_string())]).is_err());
}

// ========== 自定义错误测试 ==========

#[test]
fn test_divide_with_custom_error() {
    assert_eq!(divide_with_custom_error(10, 2), Ok(5));
    assert_eq!(
        divide_with_custom_error(10, 0),
        Err(MathError::DivisionByZero)
    );
}

#[test]
fn test_sqrt_with_custom_error() {
    assert_eq!(sqrt_with_custom_error(9.0), Ok(3.0));
    assert_eq!(
        sqrt_with_custom_error(-1.0),
        Err(MathError::NegativeSquareRoot)
    );
}

#[test]
fn test_math_error_display() {
    assert_eq!(format!("{}", MathError::DivisionByZero), "除数不能为零");
    assert_eq!(
        format!("{}", MathError::NegativeSquareRoot),
        "不能计算负数的平方根"
    );
    assert_eq!(format!("{}", MathError::OutOfRange), "数值超出范围");
    assert_eq!(format!("{}", MathError::Other("msg".to_string())), "msg");
}

// ========== 浮点数除法测试 ==========

#[test]
fn test_safe_divide_float() {
    assert_eq!(safe_divide_float(10.0, 2.0), Ok(5.0));
    assert_eq!(
        safe_divide_float(10.0, 0.0),
        Err("除数不能为零".to_string())
    );
    assert_eq!(safe_divide_float(-10.0, 2.0), Ok(-5.0));
}

// ========== 解析测试 ==========

#[test]
fn test_parse_and_sqrt() {
    assert_eq!(parse_and_sqrt("9"), Ok(3.0));
    assert!(parse_and_sqrt("-1").is_err());
    assert!(parse_and_sqrt("abc").is_err());
}

// ========== 错误上下文测试 ==========

#[test]
fn test_divide_with_context() {
    assert_eq!(divide_with_context(10, 2), Ok(5));
    assert_eq!(
        divide_with_context(10, 0),
        Err("除法失败 (10/0): 除数不能为零".to_string())
    );
}

#[test]
fn test_parse_divide_with_context() {
    assert_eq!(parse_divide_with_context("10", "2"), Ok(5));
    assert!(parse_divide_with_context("abc", "2").is_err());
    assert!(parse_divide_with_context("10", "0").is_err());
}

// ========== 链式计算测试 ==========

#[test]
fn test_calculate_chain() {
    assert_eq!(calculate_chain("16"), Ok(8.0));
    assert!(calculate_chain("-1").is_err());
    assert!(calculate_chain("abc").is_err());
}

// ========== 综合测试 ==========

#[test]
fn test_comprehensive_error_handling() {
    // 测试安全除法
    assert_eq!(safe_divide(20, 4), Ok(5));

    // 测试 Option 转换
    assert_eq!(option_to_result(Some(42), "no value".to_string()), Ok(42));

    // 测试平方根
    assert_eq!(safe_sqrt(16.0), Ok(4.0));

    // 测试解析和除法
    assert_eq!(parse_and_divide("100", "5"), Ok(20));

    // 测试验证和翻倍
    assert_eq!(validate_and_double(Some(10)), Ok(20));
}

// ========== 边界条件测试 ==========

#[test]
fn test_edge_cases() {
    // 测试除法边界
    assert_eq!(safe_divide(i32::MAX, 1), Ok(i32::MAX));
    assert_eq!(safe_divide(i32::MIN, 1), Ok(i32::MIN));

    // 测试空向量
    assert_eq!(get_first_or_default(&[]), 0);

    // 测试零值
    assert_eq!(safe_sqrt(0.0), Ok(0.0));

    // 测试空 Option
    assert!(square_if_positive(None).is_none());
}

// ========== 错误类型比较 ==========

#[test]
fn test_error_equality() {
    assert_eq!(MathError::DivisionByZero, MathError::DivisionByZero);
    assert_ne!(MathError::DivisionByZero, MathError::NegativeSquareRoot);

    let error1 = MathError::Other("test".to_string());
    let error2 = MathError::Other("test".to_string());
    assert_eq!(error1, error2);
}

// ========== 复杂错误处理测试 ==========

#[test]
fn test_complex_error_chaining() {
    // 测试多步操作中的错误传播
    fn complex_operation(input: &str) -> Result<i32, String> {
        let n: i32 = input.parse().map_err(|_| "解析失败")?;
        if n <= 0 {
            return Err("数字必须为正".to_string());
        }
        let result = safe_divide(100, n).map_err(|e| e.to_string())?;
        Ok(result)
    }

    assert_eq!(complex_operation("10"), Ok(10));
    assert!(complex_operation("-5").is_err());
    assert!(complex_operation("0").is_err());
    assert!(complex_operation("abc").is_err());
}

#[test]
fn test_map_and_map_err() {
    let result: Result<i32, &str> = Ok(5);
    assert_eq!(result.map(|x| x * 2), Ok(10));

    let result: Result<i32, &str> = Err("error");
    assert_eq!(result.map(|x| x * 2), Err("error"));

    let result: Result<i32, &str> = Err("IO错误");
    let mapped = result.map_err(|e| format!("严重: {}", e));
    assert_eq!(mapped, Err("严重: IO错误".to_string()));
}
