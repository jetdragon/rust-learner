# 文件操作

## 模块简介

本模块介绍Python的文件I/O操作，包括读写文件、目录操作和路径处理。

## 学习目标

完成本模块后，你将能够：

- 读写文本文件和二进制文件
- 使用 `with` 语句管理文件资源
- 处理目录和文件路径
- 使用 `pathlib` 进行路径操作

## 核心概念

### 文件读写

```python
# 写文件
with open("output.txt", "w", encoding="utf-8") as f:
    f.write("Hello")

# 读文件
with open("input.txt", "r", encoding="utf-8") as f:
    content = f.read()
```

### pathlib

现代的路径操作库：

```python
from pathlib import Path

path = Path("data/file.txt")
path.write_text("content")
content = path.read_text()
```

## 代码示例

位于 `examples/` 目录：
- `file_io.py` - 文件I/O演示
- `path_ops.py` - 路径操作演示

## 练习题

查看 [exercises.md](exercises.md)。

## 综合练习

查看 [综合练习.md](综合练习.md)。

## 自检清单

查看 [自检清单.md](自检清单.md)。
