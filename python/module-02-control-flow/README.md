# 控制流 (Control Flow)

## 模块简介

本模块介绍 Python 的控制流语句，包括条件判断、循环和异常处理。这些是程序逻辑的核心构建块。

## 学习目标

完成本模块后，你将能够：

- 使用 if-elif-else 进行条件判断
- 使用 match-case 进行模式匹配
- 使用 for 和 while 循环
- 理解 break, continue, pass 的用法
- 掌握异常处理
- 使用上下文管理器

## 核心概念

### if-elif-else 条件语句

```python
age = 25

if age < 18:
    print("未成年")
elif age < 60:
    print("成年")
else:
    print("老年")
```

### match-case 模式匹配

Python 3.10+ 引入了 match-case，类似其他语言的 switch:

```python
day = "星期一"

match day:
    case "星期一" | "星期二" | "星期三" | "星期四" | "星期五":
        print("工作日")
    case "星期六" | "星期日":
        print("周末")
    case _:
        print("其他")
```

### for 循环

```python
# 遍历列表
fruits = ["apple", "banana", "cherry"]
for fruit in fruits:
    print(f"我喜欢{fruit}")

# 带索引
for index, fruit in enumerate(fruits):
    print(f"{index}: {fruit}")

# 字典遍历
person = {"name": "Alice", "age": 25}
for key, value in person.items():
    print(f"{key}: {value}")
```

### while 循环

```python
count = 5
while count > 0:
    print(f"倒计时: {count}")
    count -= 1
```

### 循环控制

```python
# break - 跳出循环
for i in range(10):
    if i == 5:
        break
    print(i)

# continue - 跳过本次迭代
for i in range(5):
    if i % 2 == 0:
        continue
    print(f"奇数: {i}")

# pass - 占位符
if condition:
    pass  # 待实现
```

### 异常处理

```python
try:
    result = 10 / 0
except ZeroDivisionError:
    print("不能除以零")
except Exception as e:
    print(f"发生错误: {e}")
else:
    print("没有异常发生")
finally:
    print("无论如何都会执行")
```

### 上下文管理器

```python
# with 语句自动管理资源
with open("file.txt", "w", encoding="utf-8") as f:
    f.write("Hello")

# 文件在退出 with 块时自动关闭
```

## 代码示例

本模块的代码示例位于 `examples/` 目录：

- `if_statements.py` - 条件语句演示
- `loops.py` - 循环演示
- `exception_handling.py` - 异常处理演示

运行示例：

```bash
python3 python/module-02-control-flow/examples/if_statements.py
```

## 练习题

查看 [exercises.md](exercises.md) 完成练习题。

## 综合练习

查看 [综合练习.md](综合练习.md) 进行综合实践。

## 自检清单

完成学习后，使用 [自检清单.md](自检清单.md) 检查掌握情况。

## 常见问题

### Q: match-case 和 if-elif-else 应该用哪个？

A: 
- match-case 更适合：多个可能的值，模式匹配
- if-elif-else 更适合：范围判断、复杂条件
- Python 3.10+ 才支持 match-case

### Q: for 和 while 循环有什么区别？

A:
- for 循环：已知次数，遍历序列（列表、元组、字典）
- while 循环：不确定次数，满足条件时继续

### Q: 什么时候应该使用异常处理？

A: 当代码可能抛出异常时：
- 文件操作
- 网络请求
- 除法运算
- 数据库操作

### Q: pass 语句有什么用？

A: pass 是占位符：
- 用于空函数或空代码块
- 用于暂时不实现的代码
- 保持语法正确性

## 下一步

完成本模块后，继续学习：

- [module-03-data-structures](../module-03-data-structures/) - 数据结构
