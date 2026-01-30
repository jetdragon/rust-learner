# 练习题

本模块包含以下练习题，帮助你巩固 Rust 所有权系统的知识。

## 练习 1：所有权转移

**难度**：简单

**描述**：

理解并修复以下代码中的所有权错误。

**代码框架**：

```rust
// exercises/ownership_transfer.rs

fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s1);  // TODO: 修复这里的错误
}
```

**要求**：

- 理解为什么这段代码会编译失败
- 修复代码，使其能正确编译运行
- 解释错误原因

**提示**：

<details>
<summary>查看提示</summary>

- String 赋值时发生移动，s1 不再有效
- 可以在 println! 之前就使用 s1，或者克隆 s1
</details>

**验证**：

```bash
cargo test -p module-02-ownership -- test_ownership_transfer
```

---

## 练习 2：函数与所有权

**难度**：简单

**描述**：

编写函数，演示所有权如何在函数间转移。

**要求**：

- 创建一个 `take_ownership` 函数，接收 String 参数并打印
- 创建一个 `make_copy` 函数，接收 i32 参数并打印
- 在 main 中调用这两个函数
- 演示 String 被移动后不能再使用
- 演示 i32 被拷贝后仍可使用

**提示**：

<details>
<summary>查看提示</summary>

- String 实现时不包含 Copy trait
- i32 实现了 Copy trait
- 移动后原变量失效，拷贝后原变量仍有效
</details>

**验证**：

```bash
cargo test -p module-02-ownership -- test_function_ownership
```

---

## 练习 3：引用与借用

**难度**：中等

**描述**：

编写函数，使用引用计算字符串长度，而不获取所有权。

**要求**：

- 创建 `calculate_length` 函数，接收字符串引用 `&String`
- 返回字符串长度 `usize`
- 在 main 中调用该函数，并证明原字符串仍可使用

**提示**：

<details>
<summary>查看提示</summary>

- 使用 `&String` 创建引用
- 使用 `.len()` 方法获取长度
- 引用不会获取所有权
</details>

**验证**：

```bash
cargo test -p module-02-ownership -- test_calculate_length
```

---

## 练习 4：可变引用

**难度**：中等

**描述**：

使用可变引用修改字符串内容。

**代码框架**：

```rust
// exercises/mutable_reference.rs

fn main() {
    let s = String::from("hello");
    append_world(&s);  // TODO: 修复函数签名和实现
    println!("{}", s);  // 应输出 "hello, world"
}

// TODO: 实现 append_world 函数
```

**要求**：

- 修改 append_world 函数，使用可变引用
- 修改 main 中的变量声明，使其可变
- 确保程序输出 "hello, world"

**提示**：

<details>
<summary>查看提示</summary>

- 使用 `mut` 关键字声明可变变量
- 使用 `&mut T` 创建可变引用
- 可变引用需要原变量也是可变的
</details>

**验证**：

```bash
cargo test -p module-02-ownership -- test_mutable_reference
```

---

## 练习 5：借用规则

**难度**：中等

**描述**：

修复以下违反借用规则的代码。

**代码框架**：

```rust
// exercises/borrow_rules.rs

fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;  // TODO: 修复这里的错误

    println!("{}, {}, {}", r1, r2, r3);
}
```

**要求**：

- 理解为什么这段代码会编译失败
- 修复代码，使其能正确编译
- 保持所有三个引用都被使用

**提示**：

<details>
<summary>查看提示</summary>

- 不可变引用和可变引用不能同时存在
- 可以在不可变引用作用域结束后再创建可变引用
- 使用 {} 创建作用域
</details>

**验证**：

```bash
cargo test -p module-02-ownership -- test_borrow_rules
```

---

## 练习 6：字符串切片

**难度**：简单

**描述**：

编写函数，使用字符串切片获取单词的第一个单词。

**代码框架**：

```rust
// exercises/first_word.rs

fn first_word(s: &String) -> &str {
    // TODO: 实现此函数
    // 返回第一个单词（以空格分隔）
    // 如果没有空格，返回整个字符串
}

fn main() {
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("第一个单词是: {}", word);
}
```

**要求**：

- 实现 `first_word` 函数
- 返回字符串切片 `&str`
- 处理边界情况（没有空格）

**提示**：

<details>
<summary>查看提示</summary>

- 使用 `.bytes()` 方法遍历字节
- 使用 `.position()` 找到空格位置
- 使用切片语法 `&s[0..index]`
</details>

**验证**：

```bash
cargo test -p module-02-ownership -- test_first_word
```

---

## 练习 7：数组切片

**难度**：简单

**描述**：

编写函数，返回数组的中间元素切片。

**要求**：

- 创建 `middle_slice` 函数，接收 i32 数组引用
- 返回中间三个元素的切片
- 处理数组长度小于 3 的情况

**提示**：

<details>
<summary>查看提示</summary>

- 使用 `&a[start..end]` 创建数组切片
- 检查数组长度
- 返回 `&[i32]` 类型
</details>

**验证**：

```bash
cargo test -p module-02-ownership -- test_array_slice
```

---

## 练习 8：悬垂引用

**难度**：困难

**描述**：

识别并修复以下代码中的悬垂引用问题。

**代码框架**：

```rust
// exercises/dangle.rs

fn dangle() -> &String {  // TODO: 修复这个函数
    let s = String::from("hello");
    &s
}

fn main() {
    let s = dangle();
    println!("{}", s);
}
```

**要求**：

- 理解为什么会产生悬垂引用
- 修复 `dangle` 函数，使其返回有效值
- 两种修复方案：
  1. 直接返回 String（移动所有权）
  2. 传入引用作为参数

**提示**：

<details>
<summary>查看提示</summary>

- s 在函数结束时被释放
- 返回的引用指向已释放的内存
- 方案1：返回 String 而非 &String
- 方案2：让调用者传入 String 引用
</details>

**验证**：

```bash
cargo test -p module-02-ownership -- test_dangle
```

---

## 练习 9：综合应用 - 字符串处理

**难度**：困难

**描述**：

编写一个字符串处理程序，综合运用所有权、引用和切片知识。

**要求**：

1. 创建 `remove_first_word` 函数
   - 参数：`&mut String`
   - 功能：删除字符串的第一个单词
   - 返回：被删除的单词（String）

2. 创建 `capitalize_first` 函数
   - 参数：`&mut str`
   - 功能：将第一个字母大写
   - 返回：`&str`

3. 创建 `get_words` 函数
   - 参数：`&str`
   - 功能：返回所有单词的向量
   - 返回：`Vec<&str>`

**验证**：

```bash
cargo test -p module-02-ownership -- test_string_processing
```

---

## 综合练习：文本统计工具

**难度**：困难

**描述**：

创建一个文本统计工具，综合运用本模块的所有知识。

**项目需求**：

实现 `TextStats` 结构体和相关功能：

```rust
pub struct TextStats {
    pub word_count: usize,
    pub char_count: usize,
    pub avg_word_length: f64,
}
```

**功能要求**：

1. `analyze(text: &str) -> TextStats`
   - 分析文本并返回统计信息
   - 使用切片和引用，不获取所有权

2. `longest_word<'a>(text: &'a str) -> &'a str`
   - 找到文本中最长的单词
   - 使用生命周期标注

3. `replace_word(text: &mut String, from: &str, to: &str) -> bool`
   - 替换文本中的单词
   - 返回是否成功替换

**验收标准**：

- [ ] 所有函数能正确编译
- [ ] 统计结果准确
- [ ] 不发生不必要的拷贝
- [ ] 正确处理生命周期
- [ ] 通过所有单元测试

**提示**：

<details>
<summary>查看提示</summary>

1. 使用 `split_whitespace()` 分割单词
2. 使用 `chars().count()` 计算字符数
3. 生命周期标注确保引用有效
4. 使用 `replace()` 方法替换字符串
</details>

**验证**：

```bash
cargo test -p module-02-ownership -- test_text_stats
```

---

## 参考答案

完成练习后，可以查看 `exercises/solutions/` 目录中的参考答案对比学习。

⚠️ **注意**：建议先自己完成练习，再查看参考答案。
