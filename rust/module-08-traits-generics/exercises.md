# 07-Trait 与泛型 - 练习题

## 基础练习

### 1. 定义和实现 Trait

定义一个 `Shape` trait，包含计算面积和周长的方法。为 `Rectangle` 和 `Circle` 实现 `Shape`。

```rust
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Circle {
    radius: f64,
}

// TODO: 为 Rectangle 和 Circle 实现 Shape
```

### 2. 泛型结构体

实现一个泛型的 `Pair` 结构体，可以存储任意类型的两个值，并提供方法交换它们。

```rust
struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    // TODO: 实现 new 方法

    // TODO: 实现 swap 方法
}

// TODO: 为实现 Display 的类型实现 println 方法
impl<T: Display> Pair<T> {
    fn print(&self) {
        // ...
    }
}
```

### 3. Trait Bound

实现一个泛型函数，找出列表中最大的元素，并打印它。

```rust
fn print_largest<T: PartialOrd + Display>(list: &[T]) {
    // TODO: 实现此函数
}
```

### 4. 生命周期练习

修复以下代码的生命周期问题：

```rust
fn first_word(s: &str) -> &str {
    // TODO: 修复此函数
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
```

## 进阶练习

### 5. 关联类型

定义一个 `Container` trait，使用关联类型而非泛型。

```rust
trait Container {
    type Item;

    fn get(&self, index: usize) -> Option<&Self::Item>;
    fn push(&mut self, item: Self::Item);
    fn len(&self) -> usize;
}
```

### 6. 返回 impl Trait

实现一个函数，返回某种类型的迭代器。

```rust
fn get_iterator() -> impl Iterator<Item = i32> {
    // TODO: 返回一个迭代器
}
```

### 7. 多重 Trait Bound

实现一个函数，要求类型同时实现多个 Trait。

```rust
fn compare_and_print<T: Display + Debug + PartialEq>(a: &T, b: &T) {
    // TODO: 实现此函数
}
```

### 8. Trait 对象

使用 Trait 对象创建一个异构集合。

```rust
trait Animal {
    fn make_sound(&self) -> &str;
}

struct Dog;
struct Cat;

// TODO: 为 Dog 和 Cat 实现 Animal
// TODO: 创建一个 Vec<Box<dyn Animal>> 并调用每个动物的 make_sound
```

## 挑战练习

### 9. 运算符重载

为自定义类型实现运算符。

```rust
use std::ops::Add;

struct Point {
    x: i32,
    y: i32,
}

// TODO: 实现 Add trait，使得 Point + Point = Point
// TODO: 实现 Add trait，使得 Point + i32 = Point
```

### 10. 自定义 Derive Macro

实现一个简单的派生宏示例。

```rust
// 研究并实现一个简单的 #[derive(TraitName)] 宏
```

### 11. 高阶 Trait Bound

使用 `where` 子句实现复杂的 Trait Bound。

```rust
fn complex_function<T, U, V>(t: T, u: U, v: V)
where
    T: Iterator<Item = U>,
    U: Clone + Debug,
    V: Fn(&U) -> String,
{
    // TODO: 实现此函数
}
```

### 12. 生命周期 + 泛型

结合生命周期和泛型实现一个结构体。

```rust
struct Ref<'a, T: 'a> {
    value: &'a T,
}

impl<'a, T> Ref<'a, T> {
    // TODO: 实现 new 方法
    // TODO: 实现 get 方法
}
```

## 实战练习

### 13. 泛型缓存

实现一个简单的泛型缓存结构。

```rust
struct Cache<T>
where
    T: Clone,
{
    value: Option<T>,
}

impl<T: Clone> Cache<T> {
    // TODO: 实现 get, set, clear 方法
}
```

### 14. Trait 链

实现一个可以链式调用的 Trait。

```rust
trait Builder {
    type Output;

    fn build(self) -> Self::Output;
}

// TODO: 实现多个可以链接的 Builder
```

### 15. 比较器

实现一个泛型比较器，支持不同的比较策略。

```rust
trait Comparator<T> {
    fn compare(&self, a: &T, b: &T) -> Ordering;
}

// TODO: 实现几种不同的比较器
```

## 测试验证

为每个练习编写测试：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shape_area() {
        let rect = Rectangle { width: 10.0, height: 5.0 };
        assert!((rect.area() - 50.0).abs() < 0.001);
    }

    // TODO: 为每个练习添加测试
}
```

## 学习检查

完成练习后，检查你是否理解：

- [ ] Trait 和接口的区别
- [ ] 如何定义和实现 Trait
- [ ] 泛型语法和使用场景
- [ ] Trait Bound 的各种写法
- [ ] 生命周期基本规则
- [ ] 关联类型 vs 泛型
- [ ] Trait 对象的使用
- [ ] 运算符重载
- [ ] 标准库常用 Trait
