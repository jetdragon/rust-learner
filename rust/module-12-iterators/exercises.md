# 练习题

本模块包含以下练习题，帮助你巩固 Rust 迭代器的知识。

## 练习 1：Iterator trait 基础

**难度**：简单

**描述**：

实现一个自定义迭代器，生成指定范围内的数字。

**代码框架**：

```rust
// exercises/iterator_basics.rs

struct Counter {
    current: u32,
    max: u32,
}

impl Counter {
    // TODO: 创建新的 Counter
    fn new(max: u32) -> Self {
        todo!()
    }
}

// TODO: 实现 Iterator trait
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

fn main() {
    let counter = Counter::new(5);
    for i in counter {
        println!("{}", i);  // 应该打印 1, 2, 3, 4, 5
    }
}
```

**要求**：

- 实现 `Iterator` trait
- 定义关联类型 `Item`
- 实现 `next()` 方法
- 返回 `Some(value)` 直到达到最大值，然后返回 `None`

**提示**：

<details>
<summary>查看提示</summary>

- `type Item = u32;` 定义迭代器产生值的类型
- `next()` 返回 `Option<Item>`
- 使用 `self.current` 跟踪当前位置
- 当达到 `self.max` 时返回 `None`
</details>

**验证**：

```bash
cargo test -p module-12-iterators -- test_iterator_basics
```

---

## 练习 2：消费者适配器 - sum 和 product

**难度**：简单

**描述**：

使用消费者适配器计算数组的总和和乘积。

**代码框架**：

```rust
// exercises/consumer_adapters.rs

fn sum_and_product(numbers: &[i32]) -> (i32, i32) {
    // TODO: 使用迭代器计算 sum 和 product
    todo!()
}

fn main() {
    let numbers = &[1, 2, 3, 4, 5];
    let (sum, product) = sum_and_product(numbers);
    println!("Sum: {}, Product: {}", sum, product);
}
```

**要求**：

- 使用 `.iter()` 创建迭代器
- 使用 `.sum()` 计算总和
- 使用 `.product()` 计算乘积

**提示**：

<details>
<summary>查看提示</summary>

- `numbers.iter()` 创建引用迭代器
- `.sum()` 消费迭代器并返回总和
- `.product()` 消费迭代器并返回乘积
- 需要指定类型：`.sum::<i32>()` 或通过上下文推断
</details>

**验证**：

```bash
cargo test -p module-12-iterators -- test_sum_product
```

---

## 练习 3：迭代器适配器 - map 和 filter

**难度**：中等

**描述**：

使用 map 和 filter 处理数据。

**代码框架**：

```rust
// exercises/iterator_adapters.rs

fn process_numbers(numbers: &[i32]) -> Vec<i32> {
    // TODO: 过滤偶数，然后每个元素平方
    // 示例: [1, 2, 3, 4, 5] -> [4, 16]
    todo!()
}

fn main() {
    let numbers = &[1, 2, 3, 4, 5, 6, 7, 8];
    let result = process_numbers(numbers);
    println!("处理结果: {:?}", result);
}
```

**要求**：

- 使用 `.iter()` 创建迭代器
- 使用 `.filter()` 过滤偶数
- 使用 `.map()` 对每个元素平方
- 使用 `.collect()` 收集结果

**提示**：

<details>
<summary>查看提示</summary>

- 链式调用：`numbers.iter().filter(...).map(...).collect()`
- filter 的闭包：`|&x| x % 2 == 0`
- map 的闭包：`|&x| x * x`
- 注意闭包接收的是引用
</details>

**验证**：

```bash
cargo test -p module-12-iterators -- test_map_filter
```

---

## 练习 4：enumerate 和 zip

**难度**：中等

**描述**：

使用 enumerate 和 zip 组合多个序列。

**代码框架**：

```rust
// exercises/enumerate_zip.rs

fn index_and_zip(names: &[&str], scores: &[i32]) -> Vec<String> {
    // TODO: 使用 enumerate 和 zip 创建格式化的字符串
    // 示例: ["0. Alice: 95", "1. Bob: 88"]
    todo!()
}

fn main() {
    let names = &["Alice", "Bob", "Charlie"];
    let scores = &[95, 88, 92];
    let result = index_and_zip(names, scores);
    for s in result {
        println!("{}", s);
    }
}
```

**要求**：

- 使用 `.enumerate()` 添加索引
- 使用 `.zip()` 组合 names 和 scores
- 格式化为 `"index. name: score"` 形式

**提示**：

<details>
<summary>查看提示</summary>

- `names.iter().enumerate()` 产生 `(index, &name)`
- `enumerate().zip(scores)` 组合三个序列
- 格式化：`format!("{}. {}: {}", idx, name, score)`
</details>

**验证**：

```bash
cargo test -p module-12-iterators -- test_enumerate_zip
```

---

## 练习 5：fold 和 reduce

**难度**：中等

**描述**：

使用 fold 和 reduce 实现累积计算。

**代码框架**：

```rust
// exercises/fold_reduce.rs

fn calculate_average(numbers: &[i32]) -> f64 {
    // TODO: 使用 fold 计算平均值
    // 提示: 需要计算总和和元素数量
    todo!()
}

fn concatenate(strings: &[&str]) -> String {
    // TODO: 使用 reduce 连接字符串（用逗号分隔）
    todo!()
}

fn main() {
    let numbers = &[1, 2, 3, 4, 5];
    let avg = calculate_average(numbers);
    println!("平均数: {}", avg);

    let strings = &["hello", "world", "rust"];
    let concat = concatenate(strings);
    println!("连接: {}", concat);
}
```

**要求**：

- `calculate_average`: 使用 fold 计算总和，然后除以数量
- `concatenate`: 使用 reduce 连接字符串

**提示**：

<details>
<summary>查看提示</summary>

```rust
// fold 版本
fn calculate_average(numbers: &[i32]) -> f64 {
    let (sum, count) = numbers.iter().fold((0, 0), |(sum, count), &x| {
        (sum + x, count + 1)
    });
    sum as f64 / count as f64
}

// reduce 版本
fn concatenate(strings: &[&str]) -> String {
    strings.iter().reduce(|acc, x| format!("{}, {}", acc, x))
        .unwrap_or_else(|| String::new())
}
```

</details>

**验证**：

```bash
cargo test -p module-12-iterators -- test_fold_reduce
```

---

## 练习 6：flat_map 和 flatten

**难度**：中等

**描述**：

使用 flat_map 和 flatten 扁平化嵌套结构。

**代码框架**：

```rust
// exercises/flat_flatten.rs

fn flatten_nested(vecs: &[Vec<i32>]) -> Vec<i32> {
    // TODO: 扁平化二维向量
    // 示例: [[1, 2], [3, 4, 5], [6]] -> [1, 2, 3, 4, 5, 6]
    todo!()
}

fn expand_ranges(ranges: &[(i32, i32)]) -> Vec<i32> {
    // TODO: 使用 flat_map 展开范围
    // 示例: [(1, 3), (5, 7)] -> [1, 2, 3, 5, 6, 7]
    todo!()
}

fn main() {
    let nested = &[
        vec
![1, 2],
        vec
![3, 4, 5],
        vec
![6]
    ];
    println!("扁平化: {:?}", flatten_nested(nested));

    let ranges = &[(1, 3), (5, 7)];
    println!("展开范围: {:?}", expand_ranges(ranges));
}
```

**要求**：

- `flatten_nested`: 使用 `.flatten()` 扁平化
- `expand_ranges`: 使用 `.flat_map()` 展开范围

**提示**：

<details>
<summary>查看提示</summary>

```rust
fn flatten_nested(vecs: &[Vec<i32>]) -> Vec<i32> {
    vecs.iter().flatten().copied().collect()
}

fn expand_ranges(ranges: &[(i32, i32)]) -> Vec<i32> {
    ranges.iter()
        .flat_map(|(start, end)| *start..=*end)
        .collect()
}
```

</details>

**验证**：

```bash
cargo test -p module-12-iterators -- test_flat_flatten
```

---

## 练习 7：take, skip, take_while, skip_while

**难度**：中等

**描述**：

使用分页相关的迭代器适配器。

**代码框架**：

```rust
// exercises/take_skip.rs

fn paginate(items: &[i32>, page: usize, per_page: usize) -> Vec<i32> {
    // TODO: 返回指定页的数据
    // 示例: items=[1..=20], page=1, per_page=5 -> [6, 7, 8, 9, 10]
    todo!()
}

fn take_until_positive(numbers: &[i32]) -> Vec<i32> {
    // TODO: 使用 take_while 获取正数直到遇到非正数
    // 示例: [1, 2, 3, -1, 4, 5] -> [1, 2, 3]
    todo!()
}

fn skip_prefix(numbers: &[i32]) -> Vec<i32> {
    // TODO: 使用 skip_while 跳过前面的零
    // 示例: [0, 0, 0, 1, 2, 3] -> [1, 2, 3]
    todo!()
}

fn main() {
    let items: Vec<i32> = (1..=20).collect();
    let page1 = paginate(&items, 1, 5);
    println!("第一页: {:?}", page1);

    let nums = &[1, 2, 3, -1, 4, 5];
    println!("正数: {:?}", take_until_positive(nums));

    let nums = &[0, 0, 0, 1, 2, 3];
    println!("跳过零: {:?}", skip_prefix(nums));
}
```

**要求**：

- `paginate`: 使用 `skip()` 和 `take()`
- `take_until_positive`: 使用 `take_while()`
- `skip_prefix`: 使用 `skip_while()`

**提示**：

<details>
<summary>查看提示</summary>

```rust
fn paginate(items: &[i32], page: usize, per_page: usize) -> Vec<i32> {
    items.iter()
        .skip((page - 1) * per_page)
        .take(per_page)
        .copied()
        .collect()
}

fn take_until_positive(numbers: &[i32]) -> Vec<i32> {
    numbers.iter()
        .take_while(|&&x| x > 0)
        .copied()
        .collect()
}

fn skip_prefix(numbers: &[i32]) -> Vec<i32> {
    numbers.iter()
        .skip_while(|&&x| x == 0)
        .copied()
        .collect()
}
```

</details>

**验证**：

```bash
cargo test -p module-12-iterators -- test_take_skip
```

---

## 练习 8：cloned 和 copied

**难度**：简单

**描述**：

理解并使用 cloned 和 copied 转换引用为拥有的值。

**代码框架**：

```rust
// exercises/cloned_copied.rs

fn copy_integers(numbers: &[i32]) -> Vec<i32> {
    // TODO: 使用 copied 转换
    todo!()
}

fn clone_strings(strings: &[String]) -> Vec<String> {
    // TODO: 使用 cloned 转换
    todo!()
}

fn main() {
    let numbers = &[1, 2, 3, 4, 5];
    let copied_nums = copy_integers(numbers);
    println!("Copied: {:?}", copied_nums);

    let strings = &[
        "hello".to_string(),
        "world".to_string(),
        "rust".to_string(),
    ];
    let cloned_strings = clone_strings(strings);
    println!("Cloned: {:?}", cloned_strings);
}
```

**要求**：

- `copy_integers`: 使用 `.copied()`（i32 实现了 Copy）
- `clone_strings`: 使用 `.cloned()`（String 实现了 Clone）

**提示**：

<details>
<summary>查看提示</summary>

- `.copied()` 只适用于实现了 Copy 的类型
- `.cloned()` 调用 `clone()` 方法
- 两者都会从 `&T` 转换为 `T`
- copied 比 cloned 更快（内存拷贝 vs 深度克隆）
</details>

**验证**：

```bash
cargo test -p module-12-iterators -- test_cloned_copied
```

---

## 练习 9：复杂迭代器链

**难度**：困难

**描述**：

组合多个迭代器方法解决复杂问题。

**代码框架**：

```rust
// exercises/complex_chain.rs

fn process_data(data: &[String]) -> Vec<String> {
    // TODO: 实现以下处理流程
    // 1. 过滤空字符串
    // 2. 转换为小写
    // 3. 过滤长度 < 3 的字符串
    // 4. 添加索引前缀："1. xxx", "2. yyy"
    // 5. 只取前 5 个
    todo!()
}

fn calculate_word_stats(text: &str) -> (usize, usize, String) {
    // TODO: 返回 (单词数, 平均长度, 最长单词)
    // 需要使用多个迭代器方法
    todo!()
}

fn main() {
    let data = &[
        "Hello".to_string(),
        "".to_string(),
        "World".to_string(),
        "rust".to_string(),
        "".to_string(),
        "Programming".to_string(),
    ];
    println!("处理结果: {:?}", process_data(data));

    let text = "The quick brown fox jumps over the lazy dog";
    let (count, avg_len, longest) = calculate_word_stats(text);
    println!("单词数: {}, 平均长度: {}, 最长: {}", count, avg_len, longest);
}
```

**要求**：

- `process_data`: 使用迭代器链实现数据处理流程
- `calculate_word_stats`: 使用迭代器方法计算统计信息

**提示**：

<details>
<summary>查看提示</summary>

```rust
fn process_data(data: &[String]) -> Vec<String> {
    data.iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_lowercase())
        .filter(|s| s.len() >= 3)
        .enumerate()
        .map(|(i, s)| format!("{}. {}", i + 1, s))
        .take(5)
        .collect()
}

fn calculate_word_stats(text: &str) -> (usize, usize, String) {
    let words: Vec<&str> = text.split_whitespace().collect();

    let count = words.len();

    let avg_len = if count > 0 {
        words.iter().map(|w| w.len()).sum::<usize>() / count
    } else {
        0
    };

    let longest = words.iter()
        .max_by_key(|w| w.len())
        .unwrap_or(&"")
        .to_string();

    (count, avg_len, longest)
}
```

</details>

**验证**：

```bash
cargo test -p module-12-iterators -- test_complex_chain
```

---

## 练习 10：自定义迭代器 - 斐波那契数列

**难度**：困难

**描述**：

实现一个生成斐波那契数列的迭代器。

**代码框架**：

```rust
// exercises/fibonacci.rs

struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

// TODO: 实现 Iterator trait
impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

fn main() {
    println!("斐波那契数列前 10 项:");
    for (i, fib) in Fibonacci::new().take(10).enumerate() {
        println!("F({}) = {}", i, fib);
    }
}
```

**要求**：

- 实现 `Iterator` trait
- 生成斐波那契数列: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, ...
- 支持所有迭代器适配器

**提示**：

<details>
<summary>查看提示</summary>

```rust
impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.a;
        let next = self.a.wrapping_add(self.b);

        self.a = self.b;
        self.b = next;

        Some(current)
    }
}
```

- 使用 `wrapping_add` 处理溢出
- 标准的斐波那契: F(0)=0, F(1)=1, F(n)=F(n-1)+F(n-2)
</details>

**验证**：

```bash
cargo test -p module-12-iterators -- test_fibonacci
```

---

## 参考答案

完成练习后，可以查看 `exercises/solutions/` 目录中的参考答案对比学习。

⚠️ **注意**：建议先自己完成练习，再查看参考答案。
