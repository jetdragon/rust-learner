# 10-综合项目 - 练习

## 练习 1: 实现基本数据结构 (简单)

**任务**: 创建 Todo 结构体和 Priority 枚举

**要求**:
1. 定义 `Todo` 结构体，包含以下字段：
   - `id: u32` - 唯一标识符
   - `title: String` - 任务标题
   - `description: Option<String>` - 可选的描述
   - `completed: bool` - 完成状态
   - `priority: Priority` - 优先级
   - `tags: Vec<String>` - 标签列表
   - `created_at: DateTime<Utc>` - 创建时间

2. 定义 `Priority` 枚举，包含三个变体：
   - `Low`
   - `Medium`
   - `High`

3. 为 `Priority` 实现：
   - `Display` trait，返回中文显示（"低"/"中"/"高"）
   - `FromStr` trait，支持从字符串解析
   - `PartialOrd` trait，定义优先级比较

**提示**:
- 使用 `#[derive(Debug, Clone)]` 自动实现常用 trait
- 为枚举使用 `#[repr(u8)]` 优化内存布局

---

## 练习 2: 实现 TodoList 管理器 (简单)

**任务**: 创建 TodoList 结构体来管理多个 Todo

**要求**:
1. 定义 `TodoList` 结构体：
   ```rust
   pub struct TodoList {
       todos: Vec<Todo>,
       next_id: u32,
   }
   ```

2. 实现以下方法：
   - `new()` - 创建空列表
   - `add(title, description, priority, tags)` - 添加新任务，返回 ID
   - `get(id) -> Option<&Todo>` - 根据 ID 获取任务
   - `get_mut(id) -> Option<&mut Todo>` - 获取可变引用
   - `remove(id) -> bool` - 删除任务，返回是否成功
   - `complete(id) -> Result<(), Error>` - 标记任务完成
   - `uncomplete(id) -> Result<(), Error>` - 取消完成
   - `len() -> usize` - 返回任务数量
   - `is_empty() -> bool` - 是否为空
   - `iter() -> impl Iterator<Item = &Todo>` - 返回迭代器

**提示**:
- 使用 `filter()` 和 `find()` 查找任务
- 考虑使用 `retain()` 或其他 Vec 方法删除元素

---

## 练习 3: 实现过滤器 (中等)

**任务**: 创建任务过滤功能

**要求**:
1. 定义 `Filter` 枚举来表示不同的过滤条件：
   ```rust
   pub enum Filter {
       All,
       Completed,
       Active,
       Priority(Priority),
       Tag(String),
       Search(String),
       And(Box<Filter>, Box<Filter>),
       Or(Box<Filter>, Box<Filter>),
   }
   ```

2. 为 `Filter` 实现：
   - `matches(&Todo) -> bool` 方法
   - `apply<'a, I: Iterator<Item = &'a Todo>>(I) -> Vec<&'a Todo>` 方法

3. 在 `TodoList` 中添加：
   - `filter(&Filter) -> Vec<&Todo>` 方法
   - `find_by_tag(&str) -> Vec<&Todo>`
   - `find_by_priority(Priority) -> Vec<&Todo>`
   - `search(&str) -> Vec<&Todo>` - 在标题和描述中搜索

**提示**:
- 使用闭包简化匹配逻辑
- 考虑使用 `str::contains()` 进行模糊搜索
- 组合过滤器时注意短路求值

---

## 练习 4: 实现排序功能 (中等)

**任务**: 添加任务排序功能

**要求**:
1. 定义 `SortBy` 枚举：
   ```rust
   pub enum SortBy {
       Id,
       Title,
       CreatedAt,
       Priority,
       Custom(Box<dyn Fn(&Todo, &Todo) -> std::cmp::Ordering>),
   }
   ```

2. 实现 `sort(&mut self, by: SortBy)` 方法

3. 实现 `sorted(&self, by: SortBy) -> Vec<&Todo>` 方法（不修改原列表）

**提示**:
- 使用 `Vec::sort_by()` 或 `Vec::sort_by_key()`
- 为 Priority 实现 `Ord` trait
- 使用 `clone()` 为自定义排序保持所有权

---

## 练习 5: 实现数据持久化 (中等)

**任务**: 将任务列表保存到文件和从文件加载

**要求**:
1. 定义自定义错误类型：
   ```rust
   #[derive(Debug)]
   pub enum Error {
       NotFound(u32),
       InvalidInput(String),
       IoError(std::io::Error),
       JsonError(serde_json::Error),
   }

   impl std::error::Error for Error {}
   impl std::fmt::Display for Error { ... }
   ```

2. 实现 `Saveable` trait：
   ```rust
   pub trait Saveable {
       fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Error>;
       fn load<P: AsRef<Path>>(path: P) -> Result<Self, Error>
       where
           Self: Sized;
   }
   ```

3. 为 `TodoList` 实现 `Saveable`：
   - 使用 serde_json 序列化
   - 创建备份文件
   - 验证文件完整性

**提示**:
- 使用 `std::fs::write()` 和 `std::fs::read()`
- 考虑原子写入（先写临时文件，再重命名）
- 添加文件版本检查

---

## 练习 6: 实现命令行界面 (中等)

**任务**: 创建交互式命令行界面

**要求**:
1. 定义 `Command` 枚举：
   ```rust
   pub enum Command {
       Add { title: String, description: Option<String>, priority: Priority },
       List { filter: Filter },
       Complete { id: u32 },
       Delete { id: u32 },
       Save { path: String },
       Load { path: String },
       Quit,
   }
   ```

2. 实现命令解析：
   - `parse(&str) -> Result<Command, ParseError>`
   - 支持多种命令格式（短格式和长格式）

3. 实现 `run()` 函数：
   - 读取用户输入
   - 执行命令
   - 显示结果
   - 处理错误

**提示**:
- 使用 `std::io::stdin()` 读取输入
- 使用 `colored` crate 添加颜色
- 实现命令历史

---

## 练习 7: 添加测试 (困难)

**任务**: 为项目编写全面的测试

**要求**:
1. 单元测试：
   - 测试每个结构体的方法
   - 测试边界条件
   - 测试错误处理

2. 集成测试：
   - 测试完整的用户流程
   - 测试文件读写
   - 测试并发操作

3. 测试覆盖率：
   - 使用 `tarpaulin` 检查覆盖率
   - 目标：至少 80% 覆盖率

**提示**:
- 使用 `#[should_panic]` 测试错误情况
- 使用 `assert_matches!` 检查错误类型
- 为复杂场景编写属性测试

---

## 练习 8: 实现并发操作 (困难)

**任务**: 使用多线程处理任务

**要求**:
1. 实现 `parallel_process` 函数：
   - 使用多个线程处理任务
   - 使用 channel 传递结果
   - 使用 Mutex 保护共享状态

2. 添加批量操作：
   - `complete_all_async()` - 异步完成所有任务
   - `delete_all_async()` - 异步删除所有任务
   - `export_async()` - 异步导出数据

3. 使用 `rayon` 库优化过滤和排序

**提示**:
- 使用 `std::sync::mpsc` 创建 channel
- 使用 `Arc<Mutex<T>>` 共享数据
- 考虑使用 `Barrier` 同步线程

---

## 练习 9: 添加配置系统 (困难)

**任务**: 实现可配置的行为

**要求**:
1. 定义 `Config` 结构体：
   ```rust
   pub struct Config {
       pub default_priority: Priority,
       pub auto_save: bool,
       pub backup_enabled: bool,
       pub max_backups: usize,
       pub date_format: String,
   }
   ```

2. 实现配置加载：
   - 从 TOML 文件加载
   - 从环境变量读取
   - 支持默认值

3. 实现 `Configurable` trait

**提示**:
- 使用 `serde` 和 `toml` crate
- 使用 `dirs` crate 查找配置目录
- 实现配置热重载

---

## 练习 10: 完整应用 (困难)

**任务**: 将所有组件整合成一个完整的应用

**要求**:
1. 创建 `main.rs` 可执行文件：
   - 解析命令行参数
   - 加载配置和任务
   - 运行交互式界面或单命令模式

2. 添加日志系统：
   - 使用 `env_logger` 或 `tracing`
   - 记录重要操作
   - 支持日志级别配置

3. 优化用户体验：
   - 添加帮助信息
   - 实现自动补全
   - 显示进度提示
   - 错误信息本地化

4. 编写文档：
   - API 文档（rustdoc）
   - 用户手册
   - 开发者指南

**提示**:
- 使用 `clap` crate 处理命令行参数
- 使用 `crossterm` 或 `termion` 处理终端
- 考虑使用 `anyhow` 简化错误处理
