# 08-生命周期与高级类型 - 练习题

## 基础练习

### 1. 基础生命周期

实现一个函数，返回两个字符串切片中较长的一个。

```rust
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // TODO: 实现此函数
}
```

### 2. 结构体生命周期

定义一个持有引用的结构体。

```rust
pub struct Context<'a> {
    name: &'a str,
    value: &'a str,
}

impl<'a> Context<'a> {
    pub fn new(name: &'a str, value: &'a str) -> Self {
        // TODO: 实现
    }

    pub fn get_name(&self) -> &str {
        // TODO: 实现
    }

    pub fn get_value(&self) -> &str {
        // TODO: 实现
    }
}
```

### 3. 方法中的生命周期

```rust
pub struct Text<'a> {
    content: &'a str,
}

impl<'a> Text<'a> {
    pub fn new(content: &'a str) -> Self {
        // TODO: 实现
    }

    pub fn get_first_word(&self) -> &str {
        // TODO: 返回第一个单词
    }

    pub fn trim_and_return(&self) -> &str {
        // TODO: 去除首尾空格并返回
    }
}
```

### 4. 多个生命周期参数

```rust
pub fn compare_and_return<'a, 'b>(x: &'a str, y: &'b str, flag: bool) -> &'a str {
    // TODO: 如果 flag 为 true 返回 x，否则返回 x (必须返回 'a 生命周期的引用)
}
```

## 进阶练习

### 5. 泛型与生命周期结合

```rust
pub struct Pair<'a, T: 'a> {
    first: &'a T,
    second: &'a T,
}

impl<'a, T: 'a> Pair<'a, T> {
    pub fn new(first: &'a T, second: &'a T) -> Self {
        // TODO: 实现
    }

    pub fn first(&self) -> &T {
        // TODO: 实现
    }

    pub fn second(&self) -> &T {
        // TODO: 实现
    }

    pub fn swap(&self) -> (&T, &T) {
        // TODO: 返回交换后的元组
    }
}
```

### 6. 生命周期子类型

```rust
pub fn choose_str<'a, 'b>(x: &'a str, y: &'b str) -> &'a str
where
    'b: 'a,
{
    // TODO: 如果 x 非空返回 x，否则返回 y
}
```

### 7. 迭代器中的生命周期

```rust
pub struct SplitIter<'a> {
    text: &'a str,
    delimiter: char,
    position: usize,
}

impl<'a> SplitIter<'a> {
    pub fn new(text: &'a str, delimiter: char) -> Self {
        // TODO: 实现
    }
}

impl<'a> Iterator for SplitIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: 实现迭代逻辑
    }
}
```

### 8. 生命周期边界

```rust
pub struct Ref<'a, T: 'a> {
    value: &'a T,
}

impl<'a, T: 'a> Ref<'a, T> {
    pub fn new(value: &'a T) -> Self {
        // TODO: 实现
    }

    pub fn get(&self) -> &T {
        // TODO: 实现
    }

    pub fn map<U, F>(&self, f: F) -> Option<U>
    where
        F: FnOnce(&T) -> U,
    {
        // TODO: 应用函数到引用的值
    }
}
```

## 挑战练习

### 9. 解析器

实现一个简单的字符串解析器。

```rust
pub struct Parser<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        // TODO: 实现
    }

    pub fn current(&self) -> Option<char> {
        // TODO: 返回当前位置的字符
    }

    pub fn advance(&mut self) -> Option<char> {
        // TODO: 前进一个字符并返回之前的字符
    }

    pub fn parse_word(&mut self) -> Option<&'a str> {
        // TODO: 解析一个单词
    }

    pub fn parse_until(&mut self, delimiter: char) -> Option<&'a str> {
        // TODO: 解析直到遇到分隔符
    }
}
```

### 10. 链表

实现一个简单的链表，注意生命周期。

```rust
pub enum Node<'a, T> {
    Leaf,
    Branch { value: T, next: &'a Node<'a, T> },
}

impl<'a, T> Node<'a, T> {
    pub fn leaf() -> Self {
        // TODO: 实现
    }

    pub fn branch(value: T, next: &'a Node<'a, T>) -> Self {
        // TODO: 实现
    }

    pub fn iter(&'a self) -> NodeIter<'a, T> {
        // TODO: 返回迭代器
    }
}

pub struct NodeIter<'a, T> {
    current: Option<&'a Node<'a, T>>,
}

impl<'a, T> Iterator for NodeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: 实现迭代逻辑
    }
}
```

### 11. 类型别名练习

```rust
// 定义类型别名
type StrResult<'a> = Result<&'a str, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    Empty,
    Invalid,
}

pub fn parse_string(input: &str) -> StrResult {
    // TODO: 解析字符串，返回类型别名
}

pub fn parse_prefix(input: &str, prefix: &str) -> StrResult {
    // TODO: 检查前缀并返回剩余部分
}
```

### 12. Cow (Clone on Write)

使用 `std::borrow::Cow` 实现条件克隆。

```rust
use std::borrow::Cow;

pub fn to_uppercase<'a>(s: &'a str) -> Cow<'a, str> {
    // TODO: 如果已经有大写字母，返回借用的 str
    // 否则返回拥有的 String
}

pub fn maybe_clone<'a>(s: &'a str, should_clone: bool) -> Cow<'a, str> {
    // TODO: 根据 should_clone 决定是否克隆
}
```

## 实战练习

### 13. 配置文件解析器

```rust
pub struct ConfigParser<'a> {
    content: &'a str,
}

impl<'a> ConfigParser<'a> {
    pub fn new(content: &'a str) -> Self {
        // TODO: 实现
    }

    pub fn get_value(&self, key: &str) -> Option<&'a str> {
        // TODO: 解析 key=value 格式
    }

    pub fn get_section(&self, section: &str) -> Option<&'a str> {
        // TODO: 解析 [section] 格式
    }
}
```

### 14. 字符串视图

```rust
pub struct StringView<'a> {
    parts: Vec<&'a str>,
}

impl<'a> StringView<'a> {
    pub fn new(s: &'a str) -> Self {
        // TODO: 按行分割
    }

    pub fn lines(&self) -> &[&'a str] {
        // TODO: 返回行切片
    }

    pub fn filter<F>(&self, predicate: F) -> Vec<&'a str>
    where
        F: Fn(&&'a str) -> bool,
    {
        // TODO: 过滤行
    }
}
```

### 15. 生命周期闭包

```rust
pub struct LazyEvaluator<'a, T, F>
where
    F: FnOnce() -> T,
{
    value: Option<T>,
    evaluator: Option<F>,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a, T, F> LazyEvaluator<'a, T, F>
where
    F: FnOnce() -> T,
{
    pub fn new(evaluator: F) -> Self {
        // TODO: 实现
    }

    pub fn evaluate(&mut self) -> &T {
        // TODO: 延迟求值，只执行一次
    }
}
```

## 测试验证

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        let s1 = "long string";
        let s2 = "short";
        assert_eq!(longest(s1, s2), s1);
    }

    #[test]
    fn test_context() {
        let name = String::from("test");
        let value = String::from("value");
        let ctx = Context::new(&name, &value);
        assert_eq!(ctx.get_name(), "test");
    }

    // TODO: 为每个练习添加测试
}
```

## 学习检查

完成练习后，检查你是否理解：

- [ ] 生命周期的基本概念
- [ ] 三条生命周期省略规则
- [ ] 结构体中的生命周期标注
- [ ] 多个生命周期参数
- [ ] 生命周期子类型关系
- [ ] 生命周期与泛型结合
- [ ] 静态生命周期的使用
- [ ] 类型别名的定义和使用
- [ ] Never 类型 `!` 的含义
- [ ] DST 的基本概念
