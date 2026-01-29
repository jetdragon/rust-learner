# 并发编程

## 模块简介

本模块介绍Python的并发编程，包括多线程、异步I/O和进程管理。

## 学习目标

完成本模块后，你将能够：

- 创建和管理线程
- 理解GIL（全局解释器锁）
- 使用 asyncio 进行异步编程
- 了解进程间通信

## 核心概念

### 多线程

```python
import threading

def worker():
    print("Working")

t = threading.Thread(target=worker)
t.start()
```

### asyncio

现代异步编程框架：

```python
import asyncio

async def main():
    await asyncio.sleep(1)
    print("Done")
```

## 代码示例

位于 `examples/` 目录：
- `threading.py` - 线程演示
- `asyncio_demo.py` - 异步演示

## 练习题

查看 [exercises.md](exercises.md)。

## 综合练习

查看 [综合练习.md](综合练习.md)。

## 自检清单

查看 [自检清单.md](自检清单.md)。
