# 练习题

## 错误处理练习

完成以下练习以巩固错误处理的知识。

### 练习 1: 基础 Result 处理

实现一个安全除法函数：

```rust
pub fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
    todo!()
}

#[test]
fn test_safe_divide() {
    assert_eq!(safe_divide(10, 2), Ok(5));
    assert_eq!(safe_divide(10, 0), Err("除数不能为零".to_string()));
}
```

### 练习 2: Option 转换

将 Option 转换为 Result：

```rust
pub fn option_to_result(opt: Option<i32>, err: String) -> Result<i32, String> {
    todo!()
}

#[test]
fn test_option_to_result() {
    assert_eq!(option_to_result(Some(5), "错误".to_string()), Ok(5));
    assert_eq!(option_to_result(None, "错误".to_string()), Err("错误".to_string()));
}
```

### 练习 3: 平方根计算

实现带错误处理的平方根：

```rust
pub fn safe_sqrt(n: f64) -> Result<f64, String> {
    todo!()
}

#[test]
fn test_safe_sqrt() {
    assert_eq!(safe_sqrt(9.0), Ok(3.0));
    assert!(safe_sqrt(-1.0).is_err());
}
```

### 练习 4: 错误传播

使用 ? 操作符简化代码：

```rust
pub fn parse_and_divide(a_str: &str, b_str: &str) -> Result<i32, String> {
    // 提示：使用 ? 传播错误
    todo!()
}

#[test]
fn test_parse_and_divide() {
    assert_eq!(parse_and_divide("10", "2"), Ok(5));
    assert!(parse_and_divide("abc", "2").is_err());
    assert!(parse_and_divide("10", "0").is_err());
}
```

### 练习 5: 链式操作

使用 and_then 处理多个 Result：

```rust
pub fn double_if_positive(n: i32) -> Result<i32, String> {
    // 如果 n 为正数，返回 2*n，否则返回错误
    todo!()
}

pub fn validate_and_double(v: Option<i32>) -> Result<i32, String> {
    // 将 Option 转为 Result，然后调用 double_if_positive
    todo!()
}

#[test]
fn test_validate_and_double() {
    assert_eq!(validate_and_double(Some(5)), Ok(10));
    assert!(validate_and_double(Some(-1)).is_err());
    assert!(validate_and_double(None).is_err());
}
```

### 练习 6: 自定义错误

定义并使用自定义错误类型：

```rust
#[derive(Debug, PartialEq)]
pub enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
}

impl MathError {
    pub fn description(&self) -> &str {
        match self {
            MathError::DivisionByZero => "除数不能为零",
            MathError::NegativeSquareRoot => "不能计算负数的平方根",
        }
    }
}

pub fn divide_with_custom_error(a: i32, b: i32) -> Result<i32, MathError> {
    todo!()
}

pub fn sqrt_with_custom_error(n: f64) -> Result<f64, MathError> {
    todo!()}
```

### 练习 7: 使用 unwrap_or

```rust
pub fn get_first_or_default(vec: &Vec<i32>) -> i32 {
    // 使用 unwrap_or 返回第一个元素，如果没有则返回 0
    todo!()
}

#[test]
fn test_get_first_or_default() {
    assert_eq!(get_first_or_default(&vec![1, 2, 3]), 1);
    assert_eq!(get_first_or_default(&vec![]), 0);
}
```

### 练习 8: 使用 map 转换值

```rust
pub fn square_if_positive(n: i32) -> Option<i32> {
    // 使用 map 对 Some 值进行平方
    todo!()
}

#[test]
fn test_square_if_positive() {
    assert_eq!(square_if_positive(Some(5)), Some(25));
    assert_eq!(square_if_positive(None), None);
}
```

### 练习 9: 组合多个 Result

```rust
pub fn sum_results(a: Result<i32, String>, b: Result<i32, String>) -> Result<i32, String> {
    // 如果两个都是 Ok，返回和；否则返回第一个错误
    todo!()
}

#[test]
fn test_sum_results() {
    assert_eq!(sum_results(Ok(5), Ok(3)), Ok(8));
    assert_eq!(sum_results(Ok(5), Err("错误".to_string())), Err("错误".to_string()));
}
```

### 练习 10: 错误上下文

```rust
pub fn read_config_value(path: &str, key: &str) -> Result<String, String> {
    // 模拟：读取文件，解析 JSON，获取值
    // 在每个步骤添加错误上下文
    // 例如：文件读取失败 -> "无法读取配置文件 {path}: 原错误"
    todo!()
}
```

## 提示

1. 使用 `match` 处理 Result 和 Option
2. 使用 `?` 简化错误传播
3. 使用 `unwrap_or`, `unwrap_or_else` 提供默认值
4. 使用 `map`, `and_then`, `or_else` 链式操作
5. 自定义错误类型应该实现 `Debug` 和 `Display`

## 答案

查看 [src/lib.rs](src/lib.rs) 中的实现和测试。
