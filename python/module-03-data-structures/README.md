# 数据结构 (Data Structures)

## 模块简介

本模块深入介绍 Python 的核心数据结构：列表（List）、元组（Tuple）、字典（Dictionary）和集合（Set）。这些是 Python 程序中最常用的数据类型，掌握它们对于编写高效、Pythonic 的代码至关重要。

数据结构是编程的基石，选择合适的数据结构可以：
- 提高代码可读性和维护性
- 优化程序性能
- 简化复杂问题的解决方案
- 让代码更加 Pythonic

## 学习目标

完成本模块后，你将能够：

- **深入理解列表**：掌握列表的 13 种常用操作，包括创建、访问、添加、删除、排序、推导式等
- **熟练使用元组**：理解元组的不可变性和应用场景，掌握元组解包和命名元组
- **精通字典操作**：理解字典的 O(1) 查找性能，掌握字典的创建、访问、修改、遍历和高级操作
- **运用集合特性**：利用集合的去重和数学运算特性，处理唯一值和集合操作
- **选择合适的数据结构**：根据不同场景选择最合适的数据类型
- **编写 Pythonic 代码**：使用数据结构特性编写简洁、高效的 Python 代码

## 前置知识

在学习本模块之前，你应该具备：

- Python 基础语法（变量、函数、控制流）
- 理解可变（mutable）和不可变（immutable）的概念
- 了解基本数据类型（整数、浮点数、字符串、布尔值）
- 熟悉索引和切片操作

## 核心概念

### 列表 (List)

**列表** 是 Python 中最常用的数据结构，它是一个**可变、有序**的序列类型。

#### 列表的特点

- **可变**：可以修改、添加、删除元素
- **有序**：保持元素的插入顺序
- **异构**：可以存储不同类型的元素
- **动态**：自动调整大小

#### 创建列表

```python
# 空列表
empty_list = []
empty_list = list()  # 等价写法

# 有初始值的列表
numbers = [1, 2, 3, 4, 5]
fruits = ["apple", "banana", "cherry"]
mixed = [1, "hello", 3.14, True]  # 不同类型

# 从其他可迭代对象创建
chars_from_string = list("hello")  # ['h', 'e', 'l', 'l', 'o']
numbers_from_range = list(range(5))    # [0, 1, 2, 3, 4]
```

#### 访问元素

```python
fruits = ["apple", "banana", "cherry"]

# 索引访问（从 0 开始）
first = fruits[0]    # "apple"
last = fruits[-1]    # "cherry"（负索引从 -1 开始）

# 切片（左闭右开）
first_two = fruits[0:2]   # ["apple", "banana"]
last_two = fruits[-2:]    # ["banana", "cherry"]
all_except_first = fruits[1:]  # ["banana", "cherry"]
```

#### 添加元素

```python
fruits = ["apple", "banana"]

# append() - 在末尾添加单个元素
fruits.append("cherry")  # ["apple", "banana", "cherry"]

# insert() - 在指定位置插入
fruits.insert(1, "blueberry")  # ["apple", "blueberry", "banana", "cherry"]

# extend() - 添加多个元素
fruits.extend(["date", "elderberry"])
# ["apple", "blueberry", "banana", "cherry", "date", "elderberry"]

# 列表连接
more_fruits = ["fig", "grape"]
combined = fruits + more_fruits
```

#### 删除元素

```python
fruits = ["apple", "banana", "cherry", "date"]

# remove() - 删除指定值的元素
fruits.remove("banana")  # ["apple", "cherry", "date"]

# pop() - 删除并返回元素（默认最后一个）
last = fruits.pop()  # 返回 "date"，列表变为 ["apple", "cherry"]

# pop(index) - 删除指定位置的元素
fruits.pop(0)  # 删除 "apple"，列表变为 ["cherry"]

# del 语句
del fruits[0]  # 删除第一个元素

# clear() - 清空列表
fruits.clear()  # []
```

#### 修改元素

```python
fruits = ["apple", "banana", "cherry"]

# 直接赋值
fruits[0] = "apricot"  # ["apricot", "banana", "cherry"]

# 修改多个元素（切片赋值）
fruits[1:3] = ["blueberry", "cranberry"]
# ["apricot", "blueberry", "cranberry"]
```

#### 列表排序

```python
numbers = [3, 1, 4, 1, 5, 9, 2, 6]

# sort() - 原地排序（修改原列表）
numbers.sort()  # 升序: [1, 1, 2, 3, 4, 5, 6, 9]
numbers.sort(reverse=True)  # 降序: [9, 6, 5, 4, 3, 2, 1, 1]

# sorted() - 返回新列表（不修改原列表）
numbers = [3, 1, 4, 1, 5]
sorted_numbers = sorted(numbers)
# numbers: [3, 1, 4, 1, 5]（未改变）
# sorted_numbers: [1, 1, 3, 4, 5]
```

#### 列表推导式 (List Comprehension)

**列表推导式** 是创建列表的简洁方式，它结合了 `for` 循环和列表创建。

```python
# 基本语法：[expression for item in iterable]
squares = [x**2 for x in range(1, 6)]  # [1, 4, 9, 16, 25]

# 带条件的列表推导式
evens = [x for x in range(10) if x % 2 == 0]  # [0, 2, 4, 6, 8]

# 嵌套列表推导式（矩阵）
matrix = [[i*j for j in range(3)] for i in range(3)]
# [[0, 0, 0], [0, 1, 2], [0, 2, 4]]
```

#### 列表作为栈和队列

```python
# 栈（LIFO - Last In First Out）
stack = []
stack.append("A")
stack.append("B")
top = stack.pop()  # "B"

# 队列（FIFO - First In First Out）
from collections import deque
queue = deque(["A", "B"])
queue.append("C")
front = queue.popleft()  # "A"
```

#### 性能注意事项

| 操作 | 时间复杂度 | 说明 |
|------|-----------|------|
| `append()` | O(1) | 快 |
| `pop()` | O(1) | 快 |
| `pop(0)` | O(n) | 慢（需要移动所有元素） |
| `insert()` | O(n) | 慢 |
| `index()` | O(n) | 线性搜索 |
| `in` | O(n) | 线性搜索 |

**建议**：频繁在头部操作时使用 `collections.deque`

### 元组 (Tuple)

**元组** 是**不可变、有序**的序列类型，一旦创建就不能修改。

#### 元组的特点

- **不可变**：创建后不能修改（这是与列表最大的区别）
- **有序**：保持元素的插入顺序
- **轻量**：比列表占用更少内存
- **可哈希**：可以作为字典的键或集合的元素

#### 创建元组

```python
# 空元组
empty = ()
empty = tuple()  # 等价写法

# 单个元素（注意逗号！）
single = (42,)  # 必须有逗号，否则会被当作括号表达式

# 多个元素
point = (3, 4)
coordinates = (1, 2, 3, 4, 5)

# 不带括号（元组打包）
auto_tuple = 1, 2, 3  # 等价于 (1, 2, 3)
```

#### 元组解包

```python
# 基本解包
point = (3, 4)
x, y = point  # x=3, y=4

# 交换变量
a, b = 10, 20
a, b = b, a  # a=20, b=10

# 多余元素（使用 *）
first, *middle, last = (1, 2, 3, 4, 5)
# first=1, middle=[2, 3, 4], last=5

# 嵌套解包
nested = (1, (2, 3), 4)
a, (b, c), d = nested  # a=1, b=2, c=3, d=4
```

#### 命名元组 (NamedTuple)

```python
from collections import namedtuple

Person = namedtuple("Person", ["name", "age", "city"])

alice = Person("Alice", 30, "Beijing")
print(alice.name)    # "Alice"
print(alice.age)     # 30
print(alice.city)    # "Beijing"

# 命名元组也支持解包
name, age, city = alice
```

#### 元组的优势

- **数据保护**：不可变性确保数据不被意外修改
- **性能**：比列表略快，占用内存更少
- **字典键**：可以作为字典的键（列表不行）
- **多返回值**：函数返回多个值时常用元组

#### 使用场景

```python
# 1. 固定配置
CONFIG = ("localhost", 8080, "utf-8")

# 2. 字典键
location = {}  # 空字典
location[(40.7128, -74.0060)] = "New York"  # 元组作为键
# 列表不能作为字典键（TypeError: unhashable type: 'list'）

# 3. 函数多返回值
def get_user_info(user_id):
    # 查询数据库...
    return "Alice", 30, "Beijing"  # 返回元组

name, age, city = get_user_info(123)
```

### 字典 (Dictionary)

**字典** 是**键值对（Key-Value）**的集合，提供 O(1) 的查找复杂度。

#### 字典的特点

- **键唯一**：每个键只能出现一次
- **无序**（Python 3.7+ 保持插入顺序）
- **可变**：可以添加、删除、修改键值对
- **快速查找**：O(1) 的平均查找时间

#### 创建字典

```python
# 空字典
empty = {}
empty = dict()  # 等价写法

# 直接创建
person = {
    "name": "Alice",
    "age": 30,
    "city": "Beijing"
}

# 使用 dict() 构造函数
person2 = dict(name="Bob", age=25, city="Shanghai")

# 从键值对列表创建
data = [("name", "Charlie"), ("age", 35)]
person3 = dict(data)
```

#### 访问和修改

```python
person = {"name": "Alice", "age": 30}

# 访问元素
print(person["name"])  # "Alice"
print(person.get("age"))  # 30
print(person.get("email", "N/A"))  # "N/A"（键不存在时返回默认值）

# 添加/修改元素
person["email"] = "alice@example.com"  # 添加
person["age"] = 31  # 修改

# 删除元素
del person["email"]
value = person.pop("age")  # 删除并返回值
person.clear()  # 清空字典
```

#### 遍历字典

```python
person = {"name": "Alice", "age": 30, "city": "Beijing"}

# 遍历键
for key in person:
    print(key)

# 遍历值
for value in person.values():
    print(value)

# 遍历键值对
for key, value in person.items():
    print(f"{key}: {value}")

# 检查键是否存在
if "name" in person:
    print("name 键存在")
```

#### 字典方法

```python
person = {"name": "Alice", "age": 30}

# keys() - 所有键
keys = list(person.keys())  # ["name", "age"]

# values() - 所有值
values = list(person.values())  # ["Alice", 30]

# items() - 所有键值对
items = list(person.items())  # [("name", "Alice"), ("age", 30)]

# update() - 更新字典
person.update({"city": "Beijing", "email": "alice@example.com"})

# setdefault() - 键不存在时设置默认值
email = person.setdefault("phone", "N/A")

# pop() - 删除并返回值
age = person.pop("age")  # 30
```

#### 字典推导式

```python
# 基本语法：{key_expr: value_expr for item in iterable}
squares = {x: x**2 for x in range(1, 6)}
# {1: 1, 2: 4, 3: 9, 4: 16, 5: 25}

# 带条件的字典推导式
evens = {x: x**2 for x in range(10) if x % 2 == 0}
# {0: 0, 2: 4, 4: 16, 6: 36, 8: 64}
```

#### 字典变体

```python
from collections import defaultdict, OrderedDict

# defaultdict - 自动设置默认值
dd = defaultdict(int)  # 默认值为 0
dd["count"] += 1  # 不需要先检查键是否存在

# OrderedDict - 保持插入顺序（Python 3.7+ dict 已有序）
od = OrderedDict()
od["first"] = 1
od["second"] = 2
```

### 集合 (Set)

**集合** 是**无序、唯一**元素集合，自动去重。

#### 集合的特点

- **唯一性**：自动去除重复元素
- **无序**：不保持插入顺序（但Python 3.7+ 实际上是有序的）
- **可变**：可以添加和删除元素
- **元素要求**：元素必须是可哈希的（不可变）

#### 创建集合

```python
# 空集合（注意：不能用 {}，这是空字典）
empty = set()

# 从列表创建（自动去重）
numbers = [1, 2, 2, 3, 3, 3]
unique = set(numbers)  # {1, 2, 3}

# 直接创建
fruits = {"apple", "banana", "cherry"}
```

#### 集合运算

```python
set_a = {1, 2, 3, 4, 5}
set_b = {4, 5, 6, 7, 8}

# 并集（Union）- 所有元素
union = set_a | set_b
# 或 set_a.union(set_b)
# {1, 2, 3, 4, 5, 6, 7, 8}

# 交集（Intersection）- 共同元素
intersection = set_a & set_b
# 或 set_a.intersection(set_b)
# {4, 5}

# 差集（Difference）- A中有B中没有的
difference = set_a - set_b
# 或 set_a.difference(set_b)
# {1, 2, 3}

# 对称差集（Symmetric Difference）- 不在交集中的元素
symmetric = set_a ^ set_b
# 或 set_a.symmetric_difference(set_b)
# {1, 2, 3, 6, 7, 8}
```

#### 集合方法

```python
fruits = {"apple", "banana"}

# add() - 添加元素
fruits.add("cherry")

# update() - 添加多个元素
fruits.update(["date", "elderberry"])

# remove() - 删除元素（不存在会报错）
fruits.remove("banana")

# discard() - 删除元素（不存在不报错）
fruits.discard("fig")

# pop() - 随机删除并返回元素
random_fruit = fruits.pop()

# clear() - 清空集合
fruits.clear()
```

#### 集合推导式

```python
# 基本语法：{expr for item in iterable}
squares = {x**2 for x in range(5)}  # {0, 1, 4, 9, 16}

# 带条件
evens = {x for x in range(10) if x % 2 == 0}  # {0, 2, 4, 6, 8}
```

#### 冻结集合 (frozenset)

**frozenset** 是不可变的集合，可以作为字典的键或集合的元素。

```python
fs = frozenset([1, 2, 3, 2, 1])
print(fs)  # frozenset({1, 2, 3})

# frozenset 不可修改
# fs.add(4)  # AttributeError
```

#### 使用场景

```python
# 1. 去重
words = ["apple", "banana", "apple", "cherry"]
unique_words = list(set(words))  # ["banana", "cherry", "apple"]

# 2. 成员检查（快速）
allowed = {"Alice", "Bob", "Charlie"}
if "Alice" in allowed:  # O(1) 快速查找
    print("Allowed")

# 3. 数学集合运算
students_a = {"Alice", "Bob", "Charlie"}
students_b = {"Bob", "David", "Emma"}
both = students_a & students_b  # 同时选修两门课的学生
```

#### 性能对比

```python
import time

# 列表查找（O(n)）
list_data = list(range(100000))
start = time.time()
_ = 99999 in list_data
list_time = time.time() - start

# 集合查找（O(1)）
set_data = set(range(100000))
start = time.time()
_ = 99999 in set_data
set_time = time.time() - start

print(f"列表查找: {list_time:.6f}秒")
print(f"集合查找: {set_time:.6f}秒")
print(f"集合快了 {list_time/set_time:.1f} 倍！")
```

## 数据结构选择指南

| 场景 | 推荐类型 | 原因 |
|------|----------|------|
| 需要修改的有序序列 | `list` | 可变、有序、索引访问 |
| 固定配置、函数返回值 | `tuple` | 不可变、轻量、可解包 |
| 键值对存储、快速查找 | `dict` | O(1) 查找、语义清晰 |
| 去重、成员检查、集合运算 | `set` | 自动去重、O(1) 查找 |
| 需要同时存储重复和唯一值 | `list + set` | 列表保持顺序，集合用于检查 |

## 代码示例

本模块的代码示例位于 `examples/` 目录：

- `lists.py` (177 行) - 列表的 13 种操作演示
- `tuples_dicts.py` (209 行) - 元组解包、命名元组、字典操作
- `sets.py` (195 行) - 集合运算和实际应用

运行示例：

```bash
# 列表操作
python3 python/module-03-data-structures/examples/lists.py

# 元组和字典
python3 python/module-03-data-structures/examples/tuples_dicts.py

# 集合
python3 python/module-03-data-structures/examples/sets.py
```

## 练习题

查看 [exercises.md](exercises.md) 完成练习题。

练习题包括：
1. 列表操作基础
2. 列表推导式
3. 元组解包
4. 字典操作
5. 字典嵌套
6. 集合运算

## 综合练习

查看 [综合练习.md](综合练习.md) 进行综合实践。

综合练习是一个**学生成绩管理系统**，综合运用列表、字典、元组和集合：
- 使用列表存储学生对象
- 使用字典记录学生信息
- 使用集合进行课程统计
- 使用元组作为不可变配置

## 自检清单

完成学习后，使用 [自检清单.md](自检清单.md) 检查掌握情况。

自检清单包含：
- [ ] 能创建和操作列表（添加、删除、排序、切片）
- [ ] 理解元组的不可变性和应用场景
- [ ] 熟练使用字典进行键值对存储和查找
- [ ] 掌握集合的自动去重和数学运算
- [ ] 能选择合适的数据结构解决问题
- [ ] 理解各种操作的时间复杂度

## 常见问题

### Q: 列表和元组的区别？

A:
| 特性 | 列表 | 元组 |
|------|------|------|
| 可变性 | 可变 | 不可变 |
| 语法 | `[1, 2]` | `(1, 2)` |
| 性能 | 略慢，占用更多内存 | 略快，占用更少内存 |
| 用途 | 需要修改的数据 | 固定配置、函数多返回值 |
| 哈希 | 不能作为字典键 | 可以作为字典键 |

### Q: 何时使用字典而非列表？

A: 使用字典的场景：
- 需要通过键快速查找值（O(1) vs O(n)）
- 数据有明确的键值对应关系
- 需要语义化的访问方式（`person["name"]` vs `person[0]`）
- 需要去重（使用字典的键）

使用列表的场景：
- 需要保持元素的插入顺序
- 元素有顺序关系（如时间序列）
- 需要通过索引访问
- 有重复元素

### Q: 集合和列表的区别？

A:
- 集合：自动去重、无序、快速成员检查（O(1)）
- 列表：允许重复、有序、索引访问

### Q: 为什么单元素元组需要逗号？

A: Python 需要区分元组和括号表达式：
- `(42)` 是整数 42（括号可以省略）
- `(42,)` 是元组（逗号使其成为元组）

这是 Python 的设计选择，使得创建单元素元组需要显式的逗号。

## 性能提示

1. **成员检查**：
   - 列表：`if x in my_list` - O(n) 慢
   - 集合：`if x in my_set` - O(1) 快
   - 建议：频繁查找时先转换为集合

2. **头部操作**：
   - `list.pop(0)` - O(n) 慢（需要移动所有元素）
   - `deque.popleft()` - O(1) 快（使用 collections.deque）

3. **字符串拼接**：
   - 使用 `str.join()` 而不是 `+` 循环拼接

## 下一步

完成本模块后，继续学习：

- [module-04-functions](../module-04-functions/) - 高级函数、装饰器、lambda
- [module-05-oop](../module-05-oop/) - 面向对象编程
- [module-07-files](../module-07-files/) - 文件和数据持久化

## 进阶主题

学完本模块后，你还可以探索：

- **`collections.deque`** - 双端队列，适合头部操作
- **`collections.Counter`** - 计数器，统计元素出现次数
- **`collections.ChainMap`** - 链式字典，多个字典的视图
- **`collections.defaultdict`** - 带默认值的字典
- **`heapq`** - 堆队列算法
- **`bisect`** - 二分查找算法

这些进阶数据结构在特定场景下非常有用！
