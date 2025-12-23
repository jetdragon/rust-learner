# 练习题

本模块包含以下练习题，帮助你巩固 Rust 基础知识。

## 练习 1：温度转换

**难度**：简单

**描述**：

编写一个函数，将摄氏温度转换为华氏温度。

**要求**：

- 函数名：`celsius_to_fahrenheit`
- 参数：一个 `f64` 类型的摄氏温度
- 返回值：对应的华氏温度（`f64`）
- 转换公式：`F = C × 9/5 + 32`

**提示**：

<details>
<summary>查看提示</summary>

- 函数签名：`fn celsius_to_fahrenheit(c: f64) -> f64`
- 记住 Rust 的整数除法会舍去小数部分
</details>

**验证**：

```bash
# 在 exercises 目录创建 temperature.rs 文件
# 运行以下命令测试
cargo test -p 01-基础入门 -- test_temperature
```

---

## 练习 2：数组操作

**难度**：简单

**描述**：

编写一个函数，计算整数数组的平均值。

**要求**：

- 函数名：`average`
- 参数：一个 `i32` 数组的引用 `&[i32]`
- 返回值：平均值（`f64`）
- 处理空数组时返回 `0.0`

**提示**：

<details>
<summary>查看提示</summary>

- 使用迭代器的 `.sum()` 方法计算总和
- 使用 `.len()` 获取数组长度
- 注意类型转换：`as f64`
</details>

**验证**：

```bash
cargo test -p 01-基础入门 -- test_average
```

---

## 练习 3：元组解构

**难度**：中等

**描述**：

编写一个函数，接收一个三维坐标元组，返回到原点的距离。

**要求**：

- 函数名：`distance_to_origin`
- 参数：一个包含三个 `f64` 的元组 `(f64, f64, f64)`
- 返回值：距离（`f64`）
- 距离公式：`√(x² + y² + z²)`

**提示**：

<details>
<summary>查看提示</summary>

- 使用 `.powi(2)` 计算平方
- 使用 `.sqrt()` 计算平方根
- 元组可以通过 `.0`, `.1`, `.2` 访问元素
</details>

**验证**：

```bash
cargo test -p 01-基础入门 -- test_distance
```

---

## 练习 4：变量隐藏与类型转换

**难度**：中等

**描述**：

修复下面的代码，使其能编译并正确运行。你需要运用变量隐藏和类型转换的知识。

**代码框架**：

```rust
// exercises/shadowing.rs

fn main() {
    let number = "42";
    // TODO: 使用变量隐藏将字符串转换为数字
    // 提示: 使用 .parse().expect("...")

    println!("number * 2 = {}", number * 2);
}
```

**要求**：

- 使用变量隐藏（shadowing）
- 正确处理字符串解析
- 输出应为 `number * 2 = 84`

**验证**：

```bash
cargo run -p 01-基础入门 --bin shadowing
```

---

## 练习 5：函数式编程

**难度**：困难

**描述**：

编写一个函数，使用高阶函数模式，判断一个数字是否满足特定条件。

**要求**：

- 创建一个 `is_even` 函数判断偶数
- 创建一个 `is_positive` 函数判断正数
- 创建一个通用的 `check` 函数，接收数字和条件函数
- 使用闭包（closure）简化代码

**提示**：

<details>
<summary>查看提示</summary>

- 函数可以作为参数传递
- 闭包语法：`|参数| 表达式`
- 类型签名示例：`fn(i32) -> bool`
</details>

**验证**：

```bash
cargo test -p 01-基础入门 -- test_functional
```

---

## 综合练习：BMI 计算器

**难度**：中等

**描述**：

创建一个 BMI（身体质量指数）计算器。

**要求**：

1. 创建函数 `calculate_bmi(weight_kg: f64, height_m: f64) -> f64`
   - BMI = weight (kg) / height² (m)

2. 创建函数 `bmi_category(bmi: f64) -> &str`
   - < 18.5: "体重过轻"
   - 18.5 - 24.9: "正常"
   - 25.0 - 29.9: "超重"
   - >= 30.0: "肥胖"

3. 创建函数 `print_health_report(weight_kg: f64, height_m: f64)`
   - 打印完整的健康报告

**验证**：

```bash
cargo test -p 01-基础入门 -- test_bmi
cargo run -p 01-基础入门 --bin bmi_calculator
```

---

## 参考答案

完成练习后，可以查看 `exercises/solutions/` 目录中的参考答案对比学习。

⚠️ **注意**：建议先自己完成练习，再查看参考答案。
