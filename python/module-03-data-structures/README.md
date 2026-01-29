# 数据结构

## 模块简介

本模块介绍Python的核心数据结构：列表、元组、字典和集合。这些是Python程序中最常用的数据类型。

## 学习目标

完成本模块后，你将能够：

- 理解列表的可变性和常用操作
- 掌握元组的不可变性
- 熟练使用字典进行键值对存储
- 了解集合的去重特性
- 知道何时使用哪种数据类型

## 核心概念

### 列表

列表是Python中最常用的数据结构，可变、有序：

```python
fruits = ["apple", "banana"]
fruits.append("orange")
fruits.sort()
```

### 元组

元组是不可变的有序序列：

```python
point = (3, 4)
x, y = point  # 解包
```

### 字典

字典存储键值对，O(1)查找复杂度：

```python
person = {"name": "Alice", "age": 25}
person["email"] = "alice@example.com"
```

### 集合

集合自动去重，用于需要唯一值的场景：

```python
unique_numbers = {1, 2, 2, 3}
```

## 代码示例

本模块的代码示例位于 `examples/` 目录：

- `lists.py` - 列表操作演示
- `tuples_dicts.py` - 元组和字典演示
- `sets.py` - 集合演示

运行示例：

```bash
python3 python/module-03-data-structures/examples/lists.py
```

## 练习题

查看 [exercises.md](exercises.md) 完成练习题。

## 综合练习

查看 [综合练习.md](综合练习.md) 进行综合实践。

## 自检清单

完成学习后，使用 [自检清单.md](自检清单.md) 检查掌握情况。

## 常见问题

### Q: 列表和元组的区别？

A: 列表可变，元组不可变。如果需要修改数据，用列表；如果数据固定，用元组。

### Q: 字典和集合的区别？

A: 字典存储键值对；集合只存储唯一值。

## 下一步

完成本模块后，继续学习：

- [module-04-functions](../module-04-functions/) - 高级函数
