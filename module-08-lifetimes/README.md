# 08-生命周期与高级类型

## 学习目标

- 深入理解生命周期参数
- 掌握生命周期省略规则
- 理解静态生命周期
- 学习类型别名和高级类型

## 核心概念

### 1. 生命周期基础

生命周期确保引用在其整个生命周期内有效。

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

### 2. 生命周期注解语法

| 语法 | 含义 |
|------|------|
| `&'a T` | 带生命周期 `'a` 的引用 |
| `&'a mut T` | 带生命周期的可变引用 |
| `<'a>` | 生命周期参数声明 |
| `'static` | 静态生命周期，程序整个运行期间 |

### 3. 生命周期省略规则

编译器会应用三条规则来推断生命周期：

1. **每个引用参数都有自己的生命周期**
   ```rust
   fn foo(x: &str, y: &str) // 等价于: fn foo<'a, 'b>(x: &'a str, y: &'b str)
   ```

2. **如果只有一个输入生命周期，赋给所有输出生命周期**
   ```rust
   fn foo(x: &str) -> &str // 等价于: fn foo<'a>(x: &'a str) -> &'a str
   ```

3. **如果有多个输入生命周期，但其中一个是 `&self` 或 `&mut self`，赋给 `self` 的生命周期**
   ```rust
   impl MyStruct {
       fn get_name(&self) -> &str { ... }
   }
   ```

### 4. 结构体中的生命周期

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

### 5. 生命周期子类型

```rust
fn choose<'a, 'b>(x: &'a str, y: &'b str) -> &'a str
where
    'b: 'a,  // 'b 至少和 'a 一样长
{
    x
}
```

### 6. 静态生命周期

```rust
let s: &'static str = "I have a static lifetime.";
```

### 7. 高级类型

#### 类型别名

```rust
type Kilometers = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;

type Result<T> = std::result::Result<T, std::io::Error>;
```

#### Never 类型 (`!`)

```rust
fn panic_forever() -> ! {
    loop {
        println!("I will never return!");
    }
}

fn never_returns() -> ! {
    panic!("This function never returns!");
}
```

#### 动态大小类型 (DST)

```rust
// str 是 DST
let s: &str = "hello";

// [T] 是 DST
let arr: &[i32] = &[1, 2, 3];
```

## 生命周期与泛型

### 结合泛型参数

```rust
fn ref_eq<'a, T>(x: &'a T, y: &'a T) -> bool
where
    T: PartialEq,
{
    x == y
}
```

### 方法中的生命周期

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

## 生命周期边界

### T: 'a

```rust
struct Ref<'a, T: 'a> {
    value: &'a T,
}
```

表示类型 `T` 必须比生命周期 `'a` 活得更久。

### 示例

```rust
fn print_ref<'a, T>(t: &'a T)
where
    T: Display + 'a,
{
    println!("{}", t);
}
```

## 常见模式

### 1. 解析字符串

```rust
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    s
}
```

### 2. 迭代器适配器

```rust
struct StrIter<'a> {
    s: &'a str,
}

impl<'a> Iterator for StrIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        // ...
    }
}
```

### 3. 多个生命周期

```rust
fn compare<'a, 'b>(x: &'a str, y: &'b str) -> bool {
    x == y
}
```

## 高级主题

### 1. 匿名生命周期 (`'_`)

```rust
fn foo(s: &str) -> &str {
    // 等价于: fn foo<'a>(s: &'a str) -> &'a str
    s
}
```

### 2. 生命周期和 Trait 对象

```rust
trait Animal {
    fn speak(&self) -> &str;
}

struct Dog;
impl Animal for Dog {
    fn speak(&self) -> &str {
        "Woof!"
    }
}
```

### 3. Self 生命周期

```rust
trait Builder {
    fn build(&self) -> Self;
}

fn use_builder<B: Builder>(builder: &B) -> B {
    builder.build()
}
```

## 实践建议

1. **尽量让编译器推断**：大多数情况不需要显式标注
2. **理解三条省略规则**：可以帮助理解为什么不需要标注
3. **`'static` 不是必须的**：优先使用普通生命周期
4. **结构体生命周期**：存储引用时必须标注
5. **函数签名优先**：先把生命周期搞对，再实现逻辑

## 相关资源

- [Rust Book - 生命周期验证](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- [Rust Book - 高级 Trait](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html)
- [Rust Reference - Lifetime Elision](https://doc.rust-lang.org/reference/lifetimes.html#lifetime-elision)
