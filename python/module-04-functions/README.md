# 高级函数

## 模块简介

本模块介绍Python的高级函数概念，包括装饰器、lambda表达式和函数式编程模式。

## 学习目标

完成本模块后，你将能够：

- 编写和使用装饰器
- 掌握lambda表达式的语法
- 理解函数式编程概念
- 使用内置函数进行数据处理

## 核心概念

### 装饰器

装饰器是Python的强大特性，用于修改或增强函数：

```python
def my_decorator(func):
    def wrapper():
        print("调用前")
        result = func()
        print("调用后")
        return result
    return wrapper

@my_decorator
def say_hello():
    return "Hello"
```

### Lambda 表达式

匿名函数，用于简单操作：

```python
square = lambda x: x ** 2
double = lambda x: x * 2
```

### 内置函数

Python提供强大的内置函数：
- `map()`: 对每个元素应用函数
- `filter()`: 过滤元素
- `reduce()`: 累积计算

## 代码示例

位于 `examples/` 目录：
- `decorators.py` - 装饰器演示
- `lambdas.py` - lambda表达式演示
- `builtins.py` - 内置函数演示

## 练习题

查看 [exercises.md](exercises.md)。

## 综合练习

查看 [综合练习.md](综合练习.md)。

## 自检清单

查看 [自检清单.md](自检清单.md)。
