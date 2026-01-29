# 迭代器与生成器

## 模块简介

本模块介绍Python的迭代器模式、生成器表达式和迭代协议。

## 学习目标

完成本模块后，你将能够：

- 理解迭代器协议
- 使用生成器表达式
- 创建自定义迭代器和生成器
- 掌握迭代工具

## 核心概念

### 生成器函数

使用 `yield` 关键字：

```python
def my_generator():
    yield 1
    yield 2
    yield 3
```

### 生成器表达式

简洁的语法：

```python
squares = (x**2 for x in range(10))
```

## 代码示例

位于 `examples/` 目录：
- `generators.py` - 生成器演示
- `iterators.py` - 迭代器演示

## 练习题

查看 [exercises.md](exercises.md)。

## 综合练习

查看 [综合练习.md](综合练习.md)。

## 自检清单

查看 [自检清单.md](自检清单.md)。
