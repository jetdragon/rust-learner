# 练习文件使用说明

本目录包含 Rust 基础练习题，每个练习都有模板文件和参考解答。

## 目录结构

```
exercises/
├── README.md                    # 本文件
├── exercise1_temperature.rs     # 练习 1: 温度转换（模板）
├── exercise2_average.rs         # 练习 2: 数组平均值（模板）
├── exercise3_distance.rs        # 练习 3: 距离计算（模板）
├── exercise4_shadowing.rs       # 练习 4: 变量隐藏（模板）
├── exercise5_functional.rs      # 练习 5: 函数式编程（模板）
├── exercise6_bmi.rs             # 练习 6: BMI 计算器（模板）
└── solutions/                   # 参考解答目录
    ├── exercise1_temperature.rs
    ├── exercise2_average.rs
    ├── exercise3_distance.rs
    ├── exercise4_shadowing.rs
    ├── exercise5_functional.rs
    └── exercise6_bmi.rs
```

## 如何使用练习文件

### 方法 1: 直接编辑练习文件

1. 打开练习文件（如 `exercise1_temperature.rs`）
2. 找到 `TODO:` 注释标记的位置
3. 实现相应的功能，替换 `unimplemented!()` 宏
4. 运行测试验证实现：
   ```bash
   cargo test -p module-01-basics --test celsius_to_fahrenheit
   ```

### 方法 2: 作为独立的可执行文件运行

对于包含 `main` 函数的练习（如 exercise4 和 exercise6）：

```bash
# 运行 shadowing 练习
cargo build -p module-01-basics --bin exercise4_shadowing
cargo run -p module-01-basics --bin exercise4_shadowing

# 运行 BMI 计算器
cargo run -p module-01-basics --bin exercise6_bmi
```

### 方法 3: 在模块的 lib.rs 中调用

可以在 `src/lib.rs` 中添加测试来调用练习函数：

```rust
#[cfg(test)]
mod exercise_tests {
    // 引入练习模块中的函数
    use crate::exercises::exercise1_temperature::celsius_to_fahrenheit;

    #[test]
    fn test_exercise_1() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
    }
}
```

## 查看参考解答

完成练习后，可以查看 `solutions/` 目录中的参考解答：

```bash
# 查看练习 1 的解答
cat solutions/exercise1_temperature.rs
```

⚠️ **注意**：建议先自己完成练习，再查看参考答案！

## 练习进度

- [ ] 练习 1: 温度转换
- [ ] 练习 2: 数组平均值
- [ ] 练习 3: 距离计算
- [ ] 练习 4: 变量隐藏
- [ ] 练习 5: 函数式编程
- [ ] 练习 6: BMI 计算器

## 测试所有练习

```bash
# 运行所有练习相关的测试
cargo test -p module-01-basics

# 只运行练习模块的测试
cargo test -p module-01-basics --test exercises
```

## 常见问题

### Q: 练习文件无法编译？

A: 练习文件中的函数使用了 `unimplemented!()` 宏，这是故意的。你需要实现 TODO 标记的部分，然后删除 `unimplemented!()` 调用。

### Q: 如何验证我的实现是否正确？

A: 每个练习文件都包含内置测试。实现完成后，运行 `cargo test` 即可验证。

### Q: 可以直接复制解答代码吗？

A: 不建议！复制代码无法真正学到东西。请先自己尝试实现，遇到困难时再参考解答，理解解题思路。

### Q: 练习的难度如何选择？

A: 建议按顺序完成：
1. **简单**：练习 1、练习 2
2. **中等**：练习 3、练习 4、练习 6
3. **困难**：练习 5
