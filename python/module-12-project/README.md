# 综合项目：命令行任务管理器

## 模块简介

欢迎来到 Python 学习之旅的终点站！本模块是一个**综合项目实践**，将运用前 11 个模块所学的所有知识，开发一个功能完整的命令行（CLI）任务管理工具。

这个项目不仅是练习，更是一个**真实可用的应用程序**，展示了 Python 在实际开发中的强大能力。

## 项目目标

完成本项目后，你将：

- 🎯 **整合运用** 前 11 个模块的知识（基础、控制流、数据结构、函数、OOP、模块、文件、异常、迭代器、并发、测试）
- 🛠️ **开发完整 CLI 应用**：从设计到实现到测试
- 📊 **实现数据持久化**：将任务保存到 JSON 文件
- 🔧 **实践软件工程**：模块化设计、错误处理、文档编写
- 📚 **理解真实项目结构**：如何组织一个可维护的 Python 项目
- 💪 **建立编程信心**：通过完成完整项目获得成就感

## 前置知识

在开始本项目之前，请确保已掌握：

- ✅ **基础语法**：变量、数据类型、控制流（module 01-02）
- ✅ **数据结构**：列表、字典、元组、集合（module 03）
- ✅ **函数编程**：函数定义、参数、返回值、lambda（module 04）
- ✅ **面向对象**：类、对象、继承、封装（module 05）
- ✅ **模块系统**：import、包、路径处理（module 06）
- ✅ **文件 I/O**：读写文件、pathlib、JSON 处理（module 07）
- ✅ **异常处理**：try-except、自定义异常（module 08）
- ✅ **迭代器/生成器**：生成器函数、yield（module 09）

可选知识（加分项）：
- 🔀 **并发编程**：threading、asyncio（module 10）
- ✅ **测试框架**：pytest、单元测试（module 11）

## 项目概述

### 我们要构建什么？

一个**命令行任务管理器**（Task Manager CLI），功能包括：

1. ✅ **添加任务**：创建新任务，指定标题和描述
2. 📋 **列出任务**：查看所有待完成的任务
3. ✅ **完成任务**：标记任务为已完成
4. ❌ **删除任务**：移除不需要的任务
5. 📊 **查看统计**：显示任务完成统计
6. 💾 **数据持久化**：任务自动保存到 JSON 文件
7. 🔄 **数据恢复**：程序启动时自动加载已保存的任务

### 为什么是命令行应用？

命令行应用（CLI）是 Python 的强项，因为：

- 🚀 **快速开发**：无需复杂的 GUI 框架，专注于核心逻辑
- 🔧 **易于测试**：可以通过标准输入输出进行自动化测试
- 💻 **跨平台**：Windows、Linux、macOS 都能运行
- 🛠️ **实用性强**：服务器管理、自动化脚本、DevOps 工具都是 CLI
- 📚 **学习价值**：涵盖编程的核心概念（数据结构、算法、设计模式）

## 项目架构

### 文件结构

```
python/module-12-project/
├── README.md              # 本文件（项目说明）
├── exercises.md           # 练习题（具体功能扩展）
├── 自检清单.md            # 项目完成检查清单
├── 综合练习.md            # 项目背景和需求
├── examples/
│   └── task_manager.py   # 完整实现（220+ 行）
├── tests/
│   └── test_12.py         # 单元测试
├── pyproject.toml         # 包配置
└── setup.py               # 安装脚本
```

### 代码架构

本项目采用**面向对象设计**，包含两个核心类：

#### 1. Task 类（任务实体）

```python
class Task:
    """任务类 - 表示单个任务"""

    def __init__(self, title: str, description: str = ""):
        self.title = title              # 任务标题
        self.description = description    # 任务描述
        self.completed = False          # 是否完成
        self.created_at = ...          # 创建时间
        self.completed_at = None       # 完成时间

    def mark_completed(self):
        """标记为已完成"""

    def to_dict(self) -> Dict:
        """转换为字典（用于 JSON 序列化）"""

    @classmethod
    def from_dict(cls, data: Dict) -> "Task":
        """从字典创建任务（用于 JSON 反序列化）"""
```

**设计要点**：
- ✅ 使用类型提示（PEP 484）
- ✅ 属性封装（面向对象）
- ✅ 序列化方法（持久化支持）

#### 2. TaskManager 类（管理器）

```python
class TaskManager:
    """任务管理器 - 管理所有任务"""

    def __init__(self, data_file: str = "tasks.json"):
        self.data_file = data_file      # JSON 文件路径
        self.tasks: List[Task] = []      # 任务列表
        self.load_tasks()               # 加载已保存的任务

    def add_task(self, title: str, description: str = "") -> Task:
        """添加新任务"""

    def list_tasks(self, show_completed: bool = False) -> None:
        """列出任务"""

    def complete_task(self, task_number: int) -> bool:
        """完成任务"""

    def delete_task(self, task_number: int) -> bool:
        """删除任务"""

    def save_tasks(self) -> None:
        """保存到 JSON 文件"""

    def load_tasks(self) -> None:
        """从 JSON 文件加载"""

    def get_stats(self) -> Dict[str, int]:
        """获取统计信息"""
```

**设计要点**：
- ✅ 单一职责原则（只负责任务管理）
- ✅ 依赖注入（data_file 路径可配置）
- ✅ 错误处理（文件不存在、JSON 解析错误等）
- ✅ 类型提示（便于 IDE 自动补全）

## 实现细节

### 1. 数据持久化

使用 **JSON 格式**存储任务数据：

```json
[
  {
    "title": "完成作业",
    "description": "数学、英语、物理",
    "completed": true,
    "created_at": "2026-01-31T10:00:00",
    "completed_at": "2026-01-31T12:00:00"
  },
  {
    "title": "学习Python",
    "description": "完成模块1-12",
    "completed": false,
    "created_at": "2026-01-31T10:00:00",
    "completed_at": null
  }
]
```

**为什么用 JSON？**
- ✅ 人类可读
- ✅ Python 内置支持（`json` 模块）
- ✅ 跨语言支持
- ✅ 比 pickle 更安全

### 2. 用户交互

**主菜单系统**：
```python
while True:
    print_menu()
    choice = input("请选择操作 (1-6): ").strip()

    if choice == "1":  # 添加任务
        title = input("任务标题: ")
        description = input("任务描述（可选）: ")
        manager.add_task(title, description)

    elif choice == "2":  # 列出任务
        manager.list_tasks()

    # ... 其他选项

    elif choice == "6":  # 退出
        print("再见！")
        break
```

**输入验证**：
- 检查空输入（标题不能为空）
- 验证数字输入（任务编号）
- 处理 Ctrl+C（KeyboardInterrupt）

### 3. 错误处理

**关键错误场景**：

```python
# 1. 文件不存在（首次运行）
def load_tasks(self) -> None:
    if os.path.exists(self.data_file):
        with open(self.data_file, "r", encoding="utf-8") as f:
            data = json.load(f)
            self.tasks = [Task.from_dict(item) for item in data]
    else:
        self.tasks = []  # 首次运行，没有数据

# 2. 无效的任务编号
def complete_task(self, task_number: int) -> bool:
    if 1 <= task_number <= len(self.tasks):
        # 执行操作
        return True
    else:
        print("✗ 任务编号无效")
        return False

# 3. JSON 解析错误
try:
    data = json.load(f)
except json.JSONDecodeError:
    print("✗ 文件格式错误，使用空任务列表")
    self.tasks = []
```

### 4. 代码组织

**模块化设计**：
- Task 类：数据模型
- TaskManager 类：业务逻辑
- main() 函数：用户界面

**优点**：
- 职责分离（易于维护）
- 可测试性（每个类可单独测试）
- 可扩展性（添加新功能容易）

## 扩展功能建议

完成基础功能后，你可以尝试以下扩展：

### 简单扩展

1. **任务优先级**：
   - 添加 priority 字段（high, medium, low）
   - 按优先级排序显示

2. **任务分类**：
   - 添加 category 字段（工作、学习、生活）
   - 按类别筛选显示

3. **搜索功能**：
   - 按标题搜索任务
   - 支持模糊匹配

### 中级扩展

4. **任务编辑**：
   - 修改任务标题或描述
   - 使用新的 menu 选项

5. **批量操作**：
   - 标记全部为已完成
   - 删除已完成的任务
   - 清空所有任务

6. **截止日期**：
   - 添加 due_date 字段
   - 显示过期任务
   - 按截止日期排序

### 高级扩展

7. **标签系统**：
   - 一个任务可以有多个标签
   - 按标签筛选

8. **子任务**：
   - 一个任务可以分解为多个子任务
   - 显示主任务和子任务的进度

9. **导入/导出**：
   - 导出任务到 CSV
   - 从 CSV 导入任务

10. **多文件支持**：
    - 支持多个任务列表文件
    - 文件切换功能

## 代码示例

完整的实现位于 `examples/task_manager.py` (220+ 行代码)。

运行示例：

```bash
cd python/module-12-project
python3 examples/task_manager.py
```

**使用演示**：
```
============================================================
任务管理器
============================================================
1. 添加任务
2. 列出任务
3. 完成任务
4. 删除任务
5. 查看统计
6. 退出
============================================================
请选择操作 (1-6): 1
任务标题: 学习Python
任务描述（可选）: 完成所有12个模块
✓ 添加任务: 学习Python

请选择操作 (1-6): 5

统计:
  总任务: 1
  已完成: 0
  待完成: 1
  完成率: 0.0%
```

## 练习题

查看 [exercises.md](exercises.md) 完成练习题。

练习题包括：
1. 添加任务优先级
2. 实现任务搜索
3. 添加任务编辑功能
4. 实现批量操作
5. 添加截止日期支持

## 自检清单

完成项目后，使用 [自检清单.md](自检清单.md) 检查掌握情况。

检查点：
- [ ] 理解完整的 OOP 设计（类、对象、方法）
- [ ] 掌握文件 I/O 和 JSON 序列化
- [ ] 实现了异常处理和输入验证
- [ ] 使用类型提示提高代码质量
- [ ] 代码结构清晰、模块化
- [ ] 能够独立扩展新功能

## 常见问题

### Q: 为什么不用数据库（SQLite/MySQL）？

A:
- JSON 文件更简单，适合小到中等规模数据
- 无需额外依赖（Python 内置支持）
- 便于查看和手动编辑
- 本项目重点在编程基础，而非数据库

**进阶**：你可以将本项目改造为使用 SQLite 的版本！

### Q: 如何运行程序？

A:
```bash
# 方法1：直接运行
python3 examples/task_manager.py

# 方法2：作为模块运行
python3 -m examples.task_manager

# 方法3：安装后运行
pip install -e .
task-manager
```

### Q: 数据保存在哪里？

A:
- 默认保存在 `tasks.json`（程序运行目录）
- 每次操作自动保存
- 下次启动自动加载

**数据文件位置**：
```bash
~/.task-manager/tasks.json      # 用户主目录
C:\Users\<用户>\AppData\Local\tasks.json  # Windows (AppData)
```

### Q: 如何添加到系统路径？

A: 创建启动脚本

**Linux/macOS** (`~/bin/task-manager`):
```bash
#!/bin/bash
python3 /path/to/python/module-12-project/examples/task_manager.py "$@"
```

**Windows** (`task-manager.bat`):
```batch
@echo off
python C:\path\to\python\module-12-project\examples\task_manager.py %*
```

## 技术要点总结

### 使用的 Python 知识

| 模块 | 知识点 | 应用位置 |
|------|--------|----------|
| 01-02 | 基础、控制流 | 主循环、条件判断 |
| 03 | 数据结构 | `List[Task]` 存储任务 |
| 04 | 函数 | `add_task()`, `list_tasks()` 等方法 |
| 05 | OOP | Task 类、TaskManager 类 |
| 06 | 模块 | `from datetime import datetime` |
| 07 | 文件、JSON | `save_tasks()`, `load_tasks()` |
| 08 | 异常 | try-except 处理文件错误 |
| 09 | 迭代器 | `for task in self.tasks` |
| 10 | 并发 | 未使用（CLI 不需要） |
| 11 | 测试 | 单元测试文件 |

### 设计模式应用

- **单一职责原则**：Task 负责数据，TaskManager 负责管理
- **封装**：隐藏内部实现细节
- **依赖注入**：data_file 参数可配置
- **开闭原则**：易于扩展（添加新功能无需修改现有代码）

## 项目展示

完成本项目后，你将拥有一个：
- ✅ **真实可用的工具**：管理你的日常任务
- ✅ **完整的代码示例**：展示 Python 最佳实践
- ✅ **项目经验**：可以在简历中展示
- ✅ **学习成果**：验证你已掌握 Python 基础

## 恭喜！

🎉 **恭喜你到达 Python 学习之旅的终点！**

你现在已经掌握了：
- ✅ Python 基础语法和核心概念
- ✅ 面向对象编程思想
- ✅ 文件操作和数据持久化
- ✅ 异常处理和错误管理
- ✅ 完整项目的开发流程

**下一步建议**：
- 🚀 深入学习 Python 高级主题（装饰器、元类、描述符）
- 🌐 学习 Web 框架（Flask、Django、FastAPI）
- 🤖 探索数据科学（NumPy、Pandas、Matplotlib）
- 🔧 开发实际项目（Web 爬虫、自动化脚本、数据分析）

**记住**：编程是一个持续学习的过程，保持好奇心，继续探索！

---

**祝你在 Python 世界的旅程愉快！** 🐍✨
