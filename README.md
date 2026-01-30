# 多语言学习路径 - Learn Multiple Languages

一个为中文学习者设计的多语言编程学习项目，支持 Rust、Python、Go、C、C++、Java 六种语言。

## 项目简介

本项目通过精心设计的模块，从基础到实战，帮助你系统掌握多种编程语言。每种语言包含 12 个渐进式学习模块，每个模块包含：

- 📖 详细的中文概念讲解
- 💻 可运行的代码示例
- ✏️ 练习题及参考答案
- ✔️ 学习效果验证机制

## 支持的编程语言

### ✅ Rust (12 modules) - 已完成
```
module-01-basics      → 变量、数据类型、函数
module-02-ownership    → Rust 核心概念
module-03-structs-enums → 自定义数据类型
module-04-lifetimes    → 引用的有效性（重要！）
module-05-patterns     → 强大的控制流工具
module-06-error-handling → Result 和 Option
module-07-collections  → Vec、HashMap 等
module-08-traits-generics → Trait 与泛型
module-09-concurrency  → 线程与消息传递
module-10-project      → 综合应用
module-11-smart-pointers → 智能指针
module-12-iterators    → 迭代器
```

### ✅ Python (12 modules) - 已完成
```
python-01-basics        → 变量、数据类型、函数
python-02-control-flow  → 条件、循环、异常处理
python-03-data-structures → 列表、元组、字典、集合
python-04-functions     → 高级函数、装饰器、lambda
python-05-oop          → 类、继承、多态
python-06-modules      → 模块、包、导入机制
python-07-files        → 文件 I/O、路径处理
python-08-errors       → 异常处理、错误管理
python-09-iterators    → 迭代器、生成器
python-10-concurrency  → 多线程、asyncio
python-11-testing      → pytest、单元测试
python-12-project      → 综合项目实践
```

### ✅ Go (12 modules) - 已完成
```
go-01-basics           → 变量、类型、函数
go-02-types            → 结构体、指针、接口
go-03-methods          → 方法、receiver
go-04-concurrency      → goroutines、channels
go-05-packages         → 包、导入、模块化
go-06-files            → 文件 I/O、io 包
go-07-errors           → 错误处理
go-08-testing          → go test、table-driven tests
go-09-web              → net/http、HTTP 服务器
go-10-database         → sql、database 包
go-11-reflection       → reflect 包
go-12-project          → 综合项目
```

### ⏳ C (12 modules) - 待完成（需要安装工具链）
```
c-01-basics            → 变量、类型、函数
c-02-pointers          → 指针、地址、解引用
c-03-memory            → malloc、free、内存管理
c-04-structs           → 结构体、联合体
c-05-files             → 文件 I/O、stdio
c-06-strings           → 字符串处理、string.h
c-07-arrays-strings    → 数组、字符串操作
c-08-preprocessor      → 宏、预处理指令
c-09-debugging         → gdb、调试技巧
c-10-build-system      → Makefile、编译选项
c-11-advanced          → 位操作、内联汇编
c-12-project           → 综合项目
```

### ⏳ C++ (12 modules) - 待完成（需要安装工具链）
```
cpp-01-basics           → 变量、类型、函数
cpp-02-oop             → 类、继承、多态
cpp-03-stl             → 容器、算法、迭代器
cpp-04-templates       → 函数模板、类模板
cpp-05-smart-pointers  → unique_ptr、shared_ptr
cpp-06-move-semantics  → 移动语义、右值引用
cpp-07-exceptions      → 异常处理
cpp-08-files           → 文件 I/O、fstream
cpp-09-lambdas         → lambda 表达式
cpp-10-concurrency     → thread、mutex、条件变量
cpp-11-modern-cpp      → C++11/14/17 特性
cpp-12-project         → 综合项目
```

### ⏳ Java (12 modules) - 待完成（需要安装工具链）
```
java-01-basics          → 变量、类型、控制流
java-02-oop            → 类、继承、接口
java-03-collections     → List、Set、Map
java-04-streams        → Stream API、函数式编程
java-05-exceptions     → 异常处理
java-06-files          → NIO、文件操作
java-07-generics       → 泛型
java-08-concurrency    → Thread、ExecutorService
java-09-jdbc           → 数据库连接
java-10-spring-intro   → Spring 基础（可选）
java-11-testing        → JUnit、Mockito
java-12-project        → 综合项目
```

## 快速开始

### Rust 模块

**前置要求:**
- 安装 [Rust](https://www.rust-lang.org/tools/install) (1.75 或更高版本)

```bash
# 构建所有 Rust 模块
cargo build

# 构建特定模块
cargo build -p module-01-basics

# 运行示例
cargo run -p module-01-basics --bin variables

# 运行测试
cargo test
cargo test -p module-01-basics
```

### Python 模块

**前置要求:**
- Python 3.8 或更高版本
- pytest (用于测试)

```bash
# 进入 Python 模块目录
cd python/python-01-basics

# 运行示例
python3 examples/variables.py

# 运行测试
pytest tests/ -v

# 安装测试依赖
pip install pytest pytest-cov
```

### Go 模块

**前置要求:**
- Go 1.21 或更高版本

```bash
# 进入 Go 模块目录
cd go/go-01-basics

# 运行示例
go run examples/variables.go

# 运行测试
go test ./...
```

### C/C++ 模块

**前置要求:**
- GCC 或 Clang 编译器
- CMake (用于构建)

```bash
# 进入模块目录
cd c/c-01-basics
mkdir build && cd build
cmake ..
cmake --build .
./examples/variables
```

### Java 模块

**前置要求:**
- JDK 17 或更高版本
- Maven

```bash
# 进入模块目录
cd java/java-01-basics

# 编译和运行
mvn compile
mvn exec:java -Dexec.mainClass="Example"
```

## 学习进度

查看 [进度.md](进度.md) 追踪你的学习进展。

## 🤖 学习伴侣

本项目附带一个 **学习伴侣 CLI 工具**，帮助你：

- 📊 **可视化进度** - 带进度条的仪表板
- 📝 **智能练习** - 自动生成练习题并判分
- ⏰ **学习提醒** - 系统通知定时提醒
- 🏆 **成就系统** - 保持学习动力
- 💾 **数据追踪** - 持久化学习记录

### 安装学习伴侣

```bash
cd learning-companion
cargo build --release
```

### 使用学习伴侣

```bash
# 查看学习仪表板
cargo run --release -- dashboard

# 更新学习进度
cargo run --release -- update -m module-01-basics -t concept

# 开始练习测试
cargo run --release -- practice -m module-01-basics -c 5

# 设置学习提醒（每天 20:00）
cargo run --release -- remind -H 20 -M 0

# 查看成就
cargo run --release -- achievements
```

详细文档请查看 [learning-companion/README.md](learning-companion/README.md)。

## 项目结构

```
learn_rust/
├── rust/                   # Rust 模块目录
│   ├── module-01-basics/
│   ├── module-02-ownership/
│   ├── module-03-structs-enums/
│   ├── module-04-lifetimes/
│   ├── module-05-patterns/
│   ├── module-06-error-handling/
│   ├── module-07-collections/
│   ├── module-08-traits-generics/
│   ├── module-09-concurrency/
│   ├── module-10-project/
│   ├── module-11-smart-pointers/
│   └── module-12-iterators/
├── python/                 # Python 模块目录
│   ├── python-01-basics/
│   ├── python-02-control-flow/
│   └── ...                 # Python 模块 03-12
├── go/                     # Go 模块目录
│   ├── go-01-basics/
│   ├── go-02-types/
│   └── ...                 # Go 模块 03-12
├── c/                      # C 模块目录（待完成）
├── cpp/                    # C++ 模块目录（待完成）
├── java/                   # Java 模块目录（待完成）
├── learning-companion/     # 学习伴侣 CLI 工具
├── learning-companion-web/ # 学习伴侣 Web 版
└── README.md              # 本文件
```

## 多语言学习建议

### 学习顺序推荐

1. **初学者推荐**: Rust → Python → Go
   - Rust: 系统编程基础，理解内存管理
   - Python: 快速上手，学习通用编程概念
   - Go: 并发编程，系统级开发

2. **面向对象学习路径**: Java → C++ → Python
   - Java: 完整的 OOP 体系
   - C++: 高级 OOP 特性
   - Python: 简洁的 OOP 实现

3. **系统编程路径**: C → Rust → Go
   - C: 底层内存管理
   - Rust: 安全的系统编程
   - Go: 简洁的并发系统

### 语言特性对比

| 特性 | Rust | Python | Go | C | C++ | Java |
|------|------|--------|-----|---|-----|------|
| 内存管理 | 所有权 + 借用 | GC | GC | 手动 | 手动 + RAII | GC |
| 类型系统 | 强类型 + 静态 | 强类型 + 动态 | 强类型 + 静态 | 弱类型 + 静态 | 强类型 + 静态 | 强类型 + 静态 |
| 并发模型 | async + channels | asyncio + threading | goroutines + channels | pthread | thread + futures | Thread + ExecutorService |
| 编译速度 | 慢 | 快（JIT） | 快 | 快 | 慢（模板） | 慢（JVM 启动） |
| 运行速度 | 快 | 中等 | 快 | 最快 | 快 | 中等 |
| 学习曲线 | 陡峭 | 平缓 | 中等 | 陡峭 | 陡峭 | 中等 |

## 安装工具链

### 当前需要安装的工具链（完成 C/C++/Java 模块）

如果想要完成 C、C++、Java 模块，需要安装相应的工具链：

#### C/C++
```bash
# Windows (使用 MSYS2 或 MinGW)
# 安装 GCC 和 G++

# Linux
sudo apt-get install gcc g++ cmake

# macOS
xcode-select --install
```

#### Java
```bash
# 安装 OpenJDK 17+
# Windows: 从 Oracle 或 Adoptium 下载
# Linux
sudo apt-get install openjdk-17-jdk maven

# macOS
brew install openjdk@17 maven
```

验证安装：
```bash
gcc --version      # C 编译器
g++ --version      # C++ 编译器
javac -version     # Java 编译器
mvn -version       # Maven 构建工具
```

## 贡献指南

欢迎贡献新的学习内容！请查看 [CONTRIBUTING.md](CONTRIBUTING.md) 了解详情。

## 许可证

MIT License
