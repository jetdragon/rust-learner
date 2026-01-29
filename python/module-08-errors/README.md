# 异常处理

## 模块简介

本模块介绍Python的异常处理机制，包括try-except、自定义异常和上下文管理器。

## 学习目标

完成本模块后，你将能够：

- 使用 try-except 捕获异常
- 抛出自定义异常
- 使用 finally 确保资源释放
- 编写上下文管理器

## 核心概念

### try-except-else-finally

```python
try:
    result = 10 / 0
except ZeroDivisionError:
    print("不能除以零")
else:
    print(f"结果是: {result}")
finally:
    print("清理资源")
```

### 自定义异常

```python
class CustomError(Exception):
    pass

raise CustomError("自定义错误")
```

## 代码示例

位于 `examples/` 目录：
- `exceptions.py` - 异常处理演示
- `custom_errors.py` - 自定义异常

## 练习题

查看 [exercises.md](exercises.md)。

## 综合练习

查看 [综合练习.md](综合练习.md)。

## 自检清单

查看 [自检清单.md](自检清单.md)。
