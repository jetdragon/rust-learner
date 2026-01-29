# 测试

## 模块简介

本模块介绍Python的测试框架和最佳实践，包括单元测试、测试驱动开发（TDD）和覆盖率。

## 学习目标

完成本模块后，你将能够：

- 编写单元测试
- 使用 pytest 框架
- 理解 TDD 开发流程
- 运行测试并查看覆盖率

## 核心概念

### pytest

Python流行的测试框架：

```bash
pytest tests/test_module.py -v
```

### 断言

```python
assert result == expected
assert value in collection
```

### Fixture

测试夹，提供测试数据：

```python
@pytest.fixture
def sample_data():
    return [1, 2, 3]
```

## 代码示例

位于 `examples/` 目录：
- `unit_tests.py` - 单元测试演示
- `tdd_demo.py` - TDD演示

## 练习题

查看 [exercises.md](exercises.md)。

## 综合练习

查看 [综合练习.md](综合练习.md)。

## 自检清单

查看 [自检清单.md](自检清单.md)。
