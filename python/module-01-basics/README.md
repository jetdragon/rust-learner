# 基础入门

## 模块简介

欢迎来到 Python 之旅！本模块将带你了解 Python 的基础知识，包括变量、数据类型和函数。这些是构建任何 Python 程序的基石。

## 学习目标

完成本模块后，你将能够：

- 理解 Python 的变量系统和动态类型
- 掌握基本数据类型（整数、浮点数、布尔、字符串）
- 了解复合数据类型（列表、元组、字典、集合）
- 编写和使用函数
- 理解注释和代码组织

## 前置知识

在学习本模块之前，你应该具备：

- 基本的编程概念了解（变量、函数等）
- 熟悉命令行基本操作

## 核心概念

### 变量与动态类型

Python 是动态类型语言，变量的类型在运行时自动确定。

```python
# 变量声明
x = 5           # int 类型
y = "hello"     # str 类型
z = 3.14        # float 类型
```

**为什么是动态类型？**

- 代码简洁，不需要声明类型
- 灵活性高，同一个变量可以存储不同类型的值
- 开发效率高，快速原型

**类型转换**：
```python
# 显式转换
num_str = "123"
num = int(num_str)  # 123

float_num = float("3.14")  # 3.14
```

### 数据类型

Python 内置了丰富的数据类型。

#### 标量类型

**整数类型**：
```python
age = 25
count = 1000000
```

**浮点类型**：
```python
price = 99.99
pi = 3.14159
```

**布尔类型**：
```python
is_active = True
is_deleted = False
```

**字符串类型**：
```python
name = "Python"
greeting = 'Hello World'
multiline = """这是一个
多行字符串"""
```

**字符串操作**：
```python
text = "Hello, World!"
print(text[0])       # H
print(text[-1])      # !
print(text[7:12])   # World
print(len(text))     # 13
```

#### 复合数据类型

**列表**：
```python
fruits = ["apple", "banana", "cherry"]
fruits.append("orange")
print(fruits[0])    # apple
```

**元组**：
```python
point = (3, 4)
x, y = point  # 解包
print(x, y)  # 3 4
```

**字典**：
```python
person = {
    "name": "Alice",
    "age": 30,
    "city": "Beijing"
}
print(person["name"])  # Alice
```

**集合**：
```python
unique_numbers = {1, 2, 3, 2, 1}  # 自动去重
print(unique_numbers)  # {1, 2, 3}
```

### 函数

函数是 Python 的核心概念，用于组织和重用代码。

```python
def greet(name):
    """向指定的人打招呼"""
    return f"Hello, {name}!"

# 调用函数
message = greet("Alice")
print(message)  # Hello, Alice!
```

**参数和返回值**：
```python
def add(x, y):
    """计算两个数的和"""
    return x + y

def calculate_bmi(weight_kg, height_m):
    """计算BMI指数"""
    return weight_kg / (height_m ** 2)
```

**默认参数**：
```python
def greet(name, greeting="Hello"):
    return f"{greeting}, {name}!"

print(greet("Alice"))           # Hello, Alice!
print(greet("Bob", "Hi"))      # Hi, Bob!
```

**关键字参数**：
```python
def describe_person(name, age, city):
    print(f"{name}, {age}, {city}")

describe_person(name="Alice", age=30, city="Beijing")
describe_person(city="Shanghai", name="Bob", age=25)
```

### 语句 vs 表达式

Python 中的区别：

- **语句**：执行操作，不返回值（如赋值、import）
- **表达式**：计算并返回值

```python
# 语句
x = 10       # 赋值语句
if x > 5:    # if 语句
    print("Big")

# 表达式
3 + 4          # 返回 7
len("hello")    # 返回 5
```

### 注释

```python
# 单行注释
# 这是另一个注释

"""
多行注释
也叫文档字符串
"""

def example():
    """
    函数文档字符串
    通常在函数开头
    """
    pass
```

## 代码示例

本模块的代码示例位于 `examples/` 目录：

- `variables.py` - 变量和动态类型的演示
- `data_types.py` - 数据类型的演示
- `functions.py` - 函数的演示

运行示例：

```bash
# 运行特定示例
python3 python/module-01-basics/examples/variables.py

# 或者
cd python/module-01-basics
python3 examples/variables.py
```

## 练习题

查看 [exercises.md](exercises.md) 完成练习题。

## 综合练习

查看 [综合练习.md](综合练习.md) 进行综合实践。

## 自检清单

完成学习后，使用 [自检清单.md](自检清单.md) 检查掌握情况。

## 常见问题

### Q: Python 是动态类型，会不会有性能问题？

A: 动态类型确实会带来一些性能开销，但：
- Python 的解释器做了大量优化
- 对于大多数应用，这个开销可以忽略
- 真正的性能瓶颈通常在算法和 I/O 上

### Q: 列表和元组的区别是什么？

A:
- **列表**：可变，可以增删改元素 `[]`
- **元组**：不可变，创建后不能修改 `()`
- 性能：元组略快，但差别很小
- 使用：需要修改用列表，固定数据用元组

### Q: 什么时候应该使用字典？

A: 当你需要：
- 键值对存储数据
- 快速查找（O(1) 复杂度）
- 语义化的数据结构

### Q: 函数的参数传递是值传递还是引用传递？

A: Python 是"对象引用传递"：
- 不可变对象（int, str, tuple）：行为像值传递
- 可变对象（list, dict, tuple）：行为像引用传递

## 下一步

完成本模块后，继续学习：

- [module-02-control-flow](../module-02-control-flow/) - 控制流语句
