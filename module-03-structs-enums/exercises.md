# 练习题

本模块包含以下练习题，帮助你巩固 Rust 结构体和枚举的知识。

## 练习 1：定义结构体

**难度**：简单

**描述**：

定义一个表示书籍的结构体。

**要求**：

- 创建 `Book` 结构体，包含以下字段：
  - `title: String`
  - `author: String`
  - `pages: u32`
  - `available: bool`
- 创建一个实例并打印其信息

**提示**：

<details>
<summary>查看提示</summary>

```rust
struct Book {
    title: String,
    author: String,
    pages: u32,
    available: bool,
}
```
</details>

**验证**：

```bash
cargo test -p module-03-structs-enums -- test_book_struct
```

---

## 练习 2：结构体方法

**难度**：简单

**描述**：

为矩形结构体添加计算面积和判断是否为正方形的方法。

**代码框架**：

```rust
// exercises/rectangle.rs

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // TODO: 实现 area 方法

    // TODO: 实现 is_square 方法
    // 返回 true 如果宽高相等
}

fn main() {
    let rect = Rectangle { width: 10, height: 20 };
    println!("面积: {}", rect.area());
    println!("是否为正方形: {}", rect.is_square());
}
```

**要求**：

- `area(&self) -> u32` - 计算面积
- `is_square(&self) -> bool` - 判断是否为正方形

**验证**：

```bash
cargo test -p module-03-structs-enums -- test_rectangle_methods
```

---

## 练习 3：定义枚举

**难度**：简单

**描述**：

定义一个表示交通信号灯的枚举。

**要求**：

- 创建 `TrafficLight` 枚举，有三个变体：
  - `Red`
  - `Yellow`
  - `Green`
- 实现一个函数返回信号灯持续的时间（秒）

**提示**：

<details>
<summary>查看提示</summary>

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn duration(light: &TrafficLight) -> u32 {
    match light {
        TrafficLight::Red => 30,
        TrafficLight::Yellow => 3,
        TrafficLight::Green => 25,
    }
}
```
</details>

**验证**：

```bash
cargo test -p module-03-structs-enums -- test_traffic_light
```

---

## 练习 4：带数据的枚举

**难度**：中等

**描述**：

定义一个表示几何图形的枚举，并实现计算面积的方法。

**要求**：

- 创建 `Shape` 枚举，包含以下变体：
  - `Circle { radius: f64 }`
  - `Rectangle { width: f64, height: f64 }`
  - `Triangle { base: f64, height: f64 }`
- 实现 `area(&self) -> f64` 方法
  - 圆形面积：π * r²
  - 矩形面积：宽 × 高
  - 三角形面积：0.5 × 底 × 高

**提示**：

<details>
<summary>查看提示</summary>

```rust
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => 3.14159 * radius * radius,
            // ...
        }
    }
}
```
</details>

**验证**：

```bash
cargo test -p module-03-structs-enums -- test_shape_area
```

---

## 练习 5：Option 类型使用

**难度**：中等

**描述**：

使用 Option 类型处理可能不存在的值。

**要求**：

- 创建 `find_second_largest` 函数
  - 参数：`&[i32]` - 数字切片
  - 返回：`Option<i32>` - 第二大的值
  - 如果数组元素少于 2 个，返回 `None`

**提示**：

<details>
<summary>查看提示</summary>

- 先找出最大值
- 再找出小于最大值的最大值
- 使用 Option 处理元素不足的情况
</details>

**验证**：

```bash
cargo test -p module-03-structs-enums -- test_find_second_largest
```

---

## 练习 6：Result 类型使用

**难度**：中等

**描述**：

使用 Result 类型处理可能失败的除法运算。

**要求**：

- 创建 `safe_divide` 函数
  - 参数：`a: f64`, `b: f64`
  - 返回：`Result<f64, String>`
  - 如果除数为 0，返回 `Err("除数不能为零".to_string())`
  - 否则返回 `Ok(a / b)`

**提示**：

<details>
<summary>查看提示</summary>

```rust
fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("除数不能为零"))
    } else {
        Ok(a / b)
    }
}
```
</details>

**验证**：

```bash
cargo test -p module-03-structs-enums -- test_safe_divide
```

---

## 练习 7：match 穷尽性

**难度**：中等

**描述**：

修复以下未穷尽匹配的代码。

**代码框架**：

```rust
// exercises/match_exhaustiveness.rs

enum Color {
    Red,
    Blue,
    Green,
    Yellow,
}

fn color_name(color: &Color) -> &str {
    match color {
        Color::Red => "红色",
        Color::Blue => "蓝色",
        // TODO: 添加缺失的分支
    }
}
```

**要求**：

- 添加缺失的分支使 match 穷尽
- 处理所有可能的 Color 变体

**验证**：

```bash
cargo test -p module-03-structs-enums -- test_match_exhaustiveness
```

---

## 练习 8：if let 使用

**难度**：简单

**描述**：

使用 if let 简化 Option 的处理。

**代码框架**：

```rust
// exercises/if_let_practice.rs

fn main() {
    let some_value: Option<i32> = Some(7);

    // 使用 if let 简化以下 match
    match some_value {
        Some(i) => println!("值为: {}", i),
        None => println!("没有值"),
    }
}
```

**要求**：

- 用 `if let` 简化上面的 match
- 保持相同的功能

**提示**：

<details>
<summary>查看提示</summary>

```rust
if let Some(i) = some_value {
    println!("值为: {}", i);
} else {
    println!("没有值");
}
```
</details>

---

## 练习 9：嵌套结构体

**难度**：困难

**描述**：

创建嵌套结构体来表示公司和员工信息。

**要求**：

- 定义 `Address` 结构体：
  - `street: String`
  - `city: String`
  - `zip_code: String`

- 定义 `Employee` 结构体：
  - `name: String`
  - `position: String`
  - `address: Address`

- 定义 `Company` 结构体：
  - `name: String`
  - `employees: Vec<Employee>`

- 为 `Company` 实现 `add_employee` 方法
- 为 `Company` 实现 `employee_count` 方法

**验证**：

```bash
cargo test -p module-03-structs-enums -- test_nested_structs
```

---

## 练习 10：综合应用 - 状态机

**难度**：困难

**描述**：

使用枚举和结构体实现简单的博客文章状态机。

**要求**：

1. 定义 `Post` 结构体：
   - `title: String`
   - `content: String`
   - `state: PostState`

2. 定义 `PostState` 枚举：
   - `Draft`
   - `PendingReview`
   - `Published`

3. 为 `Post` 实现方法：
   - `new(title: String, content: String) -> Self` - 创建草稿
   - `request_review(&mut self)` - 请求审核
   - `approve(&mut self)` - 批准发布
   - `content(&self) -> Option<&str>` - 获取内容（只有已发布才能查看）

4. 状态转换规则：
   - Draft → PendingReview → Published
   - 只能按顺序转换

**验证**：

```bash
cargo test -p module-03-structs-enums -- test_blog_post
```

---

## 综合练习：库存管理系统

**难度**：困难

**描述**：

创建一个简单的库存管理系统，综合运用结构体和枚举。

**项目需求**：

1. 定义 `Product` 结构体：
   - `id: u32`
   - `name: String`
   - `price: f64`
   - `stock: u32`

2. 定义 `InventoryAction` 枚举：
   - `Add { product: Product, quantity: u32 }`
   - `Remove { product_id: u32, quantity: u32 }`
   - `Update { product_id: u32, new_price: Option<f64> }`

3. 定义 `Inventory` 结构体：
   - `products: Vec<Product>`

4. 为 `Inventory` 实现方法：
   - `new() -> Self`
   - `apply_action(&mut self, action: InventoryAction) -> Result<(), String>`
   - `find_product(&self, id: u32) -> Option<&Product>`
   - `total_value(&self) -> f64`
   - `list_products(&self)`

**验收标准**：

- [ ] 所有结构体和枚举定义正确
- [ ] 方法实现完整
- [ ] 正确处理错误情况
- [ ] 通过所有单元测试
- [ ] 代码通过 clippy 检查

**验证**：

```bash
cargo test -p module-03-structs-enums -- test_inventory
```

---

## 参考答案

完成练习后，可以查看 `exercises/solutions/` 目录中的参考答案对比学习。

⚠️ **注意**：建议先自己完成练习，再查看参考答案。
