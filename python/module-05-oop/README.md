# 面向对象编程 (OOP)

## 模块简介

本模块介绍Python的面向对象编程概念，包括类、继承、封装和多态。

## 学习目标

完成本模块后，你将能够：

- 定义类和对象
- 理解封装、继承和多态
- 使用实例和类方法
- 掌握特殊方法

## 核心概念

### 类定义

```python
class Person:
    def __init__(self, name, age):
        self.name = name
        self.age = age
    
    def greet(self):
        return f"Hello, {self.name}!"
```

### 继承

子类可以继承父类的属性和方法：

```python
class Student(Person):
    def __init__(self, name, age, student_id):
        super().__init__(name, age)
        self.student_id = student_id
```

### 封装

使用私有属性（`__` 前缀）和方法访问控制。

### 多态

不同类实现相同方法，但行为不同。

## 代码示例

位于 `examples/` 目录：
- `classes.py` - 类定义演示
- `inheritance.py` - 继承演示
- `polymorphism.py` - 多态演示

## 练习题

查看 [exercises.md](exercises.md)。

## 综合练习

查看 [综合练习.md](综合练习.md)。

## 自检清单

查看 [自检清单.md](自检清单.md)。
