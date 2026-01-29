# Go 基础

## 模块简介

欢迎来到 Go 之旅！本模块将带你了解 Go 的基础知识，包括变量、类型、函数和控制流。

## 学习目标

完成本模块后，你将能够：

- 理解 Go 的变量声明和类型推断
- 掌握基本数据类型（整型、浮点型、字符串、布尔）
- 编写简单的 Go 程序
- 理解包结构和导入

## 核心概念

### 变量声明

Go 使用 `var` 关键字声明变量，也可以使用 `:=` 短变量声明。

```go
var x int = 5
y := "hello"
```

### 基本类型

```go
var (
    i     int     = 42
    f     float64 = 3.14
    b     bool    = true
    s     string  = "hello"
)
```

### 函数

```go
func add(a, b int) int {
    return a + b
}
```

## 代码示例

位于 `examples/` 目录。

## 练习题

查看 [exercises.md](exercises.md)。

## 自检清单

查看 [自检清单.md](自检清单.md)。

## 综合练习

查看 [综合练习.md](综合练习.md)。

## 下一步

完成本模块后，继续学习：

- [module-02-types](../module-02-types/) - 类型系统详解
