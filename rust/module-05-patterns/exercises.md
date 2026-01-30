# 练习题

## 模式匹配练习

完成以下练习以巩固模式匹配的知识。

### 练习 1: 基本 match 表达式

实现一个函数，根据分数返回评级：

```rust
fn grade(score: u32) -> &'static str {
    // 返回评级：
    // 90-100: "优秀"
    // 80-89: "良好"
    // 60-79: "及格"
    // 0-59: "不及格"
    // 其他: "无效分数"
    todo!()
}

#[test]
fn test_grade() {
    assert_eq!(grade(95), "优秀");
    assert_eq!(grade(85), "良好");
    assert_eq!(grade(70), "及格");
    assert_eq!(grade(50), "不及格");
}
```

### 练习 2: 匹配 Option

实现一个函数，安全地获取向量中的第二个元素：

```rust
fn second<T>(vec: &Vec<T>) -> Option<&T> {
    todo!()
}

#[test]
fn test_second() {
    assert_eq!(second(&vec![1, 2, 3]), Some(&2));
    assert_eq!(second(&vec![1]), None);
    assert_eq!(second(&Vec::<i32>::new()), None);
}
```

### 练习 3: 解构结构体

```rust
struct Point {
    x: i32,
    y: i32,
}

fn quadrant(p: &Point) -> &'static str {
    // 返回点所在的象限：
    // (+, +): "第一象限"
    // (-, +): "第二象限"
    // (-, -): "第三象限"
    // (+, -): "第四象限"
    // (0, 0): "原点"
    // 在轴上: "坐标轴"
    todo!()
}

#[test]
fn test_quadrant() {
    assert_eq!(quadrant(&Point { x: 1, y: 1 }), "第一象限");
    assert_eq!(quadrant(&Point { x: -1, y: 1 }), "第二象限");
    assert_eq!(quadrant(&Point { x: 0, y: 0 }), "原点");
    assert_eq!(quadrant(&Point { x: 0, y: 5 }), "坐标轴");
}
```

### 练习 4: 匹配枚举

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn action(light: TrafficLight) -> &'static str {
    // 返回对应动作：
    // Red: "停止"
    // Yellow: "准备"
    // Green: "通行"
    todo!()
}

#[test]
fn test_action() {
    assert_eq!(action(TrafficLight::Red), "停止");
}
```

### 练习 5: 使用守卫

```rust
fn describe_number(n: i32) -> &'static str {
    match n {
        // 正偶数: "正偶数"
        // 正奇数: "正奇数"
        // 负偶数: "负偶数"
        // 负奇数: "负奇数"
        // 0: "零"
        _ => "其他"
    }
}

#[test]
fn test_describe_number() {
    assert_eq!(describe_number(4), "正偶数");
    assert_eq!(describe_number(3), "正奇数");
    assert_eq!(describe_number(-4), "负偶数");
    assert_eq!(describe_number(0), "零");
}
```

### 练习 6: @ 绑定

```rust
fn age_category(age: u32) -> String {
    match age {
        // 使用 @ 绑定，同时匹配范围和保留值
        // 0-12: 返回 "儿童: {age}岁"
        // 13-17: 返回 "少年: {age}岁"
        // 18-64: 返回 "成人: {age}岁"
        // 65+: 返回 "老人: {age}岁"
        _ => "未知".to_string()
    }
}

#[test]
fn test_age_category() {
    assert_eq!(age_category(10), "儿童: 10岁");
    assert_eq!(age_category(15), "少年: 15岁");
    assert_eq!(age_category(30), "成人: 30岁");
    assert_eq!(age_category(70), "老人: 70岁");
}
```

### 练习 7: 忽略值

```rust
fn sum_first_three(tuple: (i32, i32, i32, i32, i32)) -> i32 {
    // 只计算前三个元素的和，忽略后两个
    todo!()
}

#[test]
fn test_sum_first_three() {
    assert_eq!(sum_first_three((1, 2, 3, 4, 5)), 6);
    assert_eq!(sum_first_three((10, 20, 30, 40, 50)), 60);
}
```

### 练习 8: if let 简化

将以下 match 表达式改为 if let：

```rust
fn print_if_positive(value: Option<i32>) {
    // 使用 if let 简化
    match value {
        Some(n) if n > 0 => println!("正数: {}", n),
        _ => println!("不是正数"),
    }
}

// 再写一个只关心 None 的版本
fn print_if_none(value: Option<i32>) {
    // 使用 if let 或 let else
    todo!()
}
```

### 练习 9: while let 循环

```rust
// 使用 while let 遍历链表
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn print_list(list: &List) {
    // 使用 while let 遍历并打印所有元素
    todo!()
}
```

### 练习 10: 解构嵌套结构

```rust
struct Employee {
    name: String,
    role: Role,
}

enum Role {
    Manager { department: String },
    Engineer { level: u32, language: String },
    Intern,
}

fn describe_employee(emp: &Employee) -> String {
    match emp {
        // 匹配各种嵌套情况
        Employee { name, role: Role::Manager { department } } => {
            format!("{} 是 {} 部门经理", name, department)
        }
        // 完成其他模式
        _ => "未知".to_string()
    }
}
```

## 提示

1. 使用 `..=` 表示包含范围
2. 使用 `|` 匹配多个模式
3. 使用 `_` 忽略不需要的值
4. 使用 `..` 忽略剩余部分
5. 使用 `if` 作为守卫添加额外条件
6. 使用 `@` 绑定值到变量

## 答案

查看 [src/lib.rs](src/lib.rs) 中的实现和测试。
