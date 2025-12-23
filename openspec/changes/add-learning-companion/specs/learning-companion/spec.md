## ADDED Requirements

### Requirement: 仓库扫描与解析
应用 SHALL 能够扫描并解析符合标准结构的 Rust 学习仓库，提取学习进度信息。

#### Scenario: 扫描仓库结构
- **GIVEN** 用户选择或指定一个学习仓库路径
- **WHEN** 应用执行仓库扫描
- **THEN** SHALL 识别所有学习模块目录（module-XX-*）
- **AND** SHALL 读取每个模块的 README.md、exercises.md、自检清单.md
- **AND** SHALL 读取根目录的 进度.md

#### Scenario: 解析进度文件
- **GIVEN** 仓库包含 进度.md 文件
- **WHEN** 应用解析该文件
- **THEN** SHALL 提取每个模块的完成状态（未开始/进行中/已完成）
- **AND** SHALL 识别已标记的子任务（概念学习、代码示例、练习题等）
- **AND** SHALL 解析完成日期和心得体会

#### Scenario: 解析模块测试结果
- **GIVEN** 模块包含 tests/ 目录
- **WHEN** 应用运行或读取测试结果
- **THEN** SHALL 统计测试通过率
- **AND** SHALL 识别失败的测试用例

### Requirement: 学习进度可视化
应用 SHALL 通过可视化界面展示学习进度，包括进度条、统计图表和里程碑。

#### Scenario: 显示总体进度
- **GIVEN** 用户打开仪表板
- **WHEN** 查看总体进度
- **THEN** SHALL 显示总完成百分比（已完成模块/总模块）
- **AND** SHALL 显示可视化进度条
- **AND** SHALL 显示预计完成时间

#### Scenario: 显示模块详情
- **GIVEN** 用户点击某个模块
- **WHEN** 查看模块详情
- **THEN** SHALL 显示该模块的所有子任务状态
- **AND** SHALL 显示掌握程度评分（0-100%）
- **AND** SHALL 显示已用时间和自检打分

#### Scenario: 显示学习趋势
- **GIVEN** 用户有多次学习记录
- **WHEN** 查看学习趋势
- **THEN** SHALL 显示每日学习时长图表
- **AND** SHALL 显示连续学习天数
- **AND** SHALL 显示历史最高连续天数

### Requirement: 定时学习提醒
应用 SHALL 支持配置定时学习提醒，通过系统通知发送提醒消息。

#### Scenario: 配置提醒时间
- **GIVEN** 用户进入设置页面
- **WHEN** 配置每日提醒时间
- **THEN** SHALL 支持选择具体时间点
- **AND** SHALL 支持选择星期几（工作日/周末）
- **AND** SHALL 支持设置提醒间隔（天）

#### Scenario: 发送学习提醒
- **GIVEN** 用户已配置提醒规则
- **WHEN** 到达提醒时间且应用在运行
- **THEN** SHALL 发送系统通知
- **AND** 通知 SHALL 包含鼓励性文字
- **AND** 通知 SHALL 包含当前学习进度摘要

#### Scenario: 智能提醒
- **GIVEN** 用户启用智能提醒模式
- **WHEN** 检测到用户长时间未学习
- **THEN** SHALL 根据历史学习模式计算最佳提醒时间
- **AND** SHALL 在该时间发送提醒

### Requirement: 智能练习系统
应用 SHALL 能够根据学习模块生成练习题，检查答案，并分析掌握程度。

#### Scenario: 生成模块练习
- **GIVEN** 用户选择一个已学习的模块
- **WHEN** 请求练习题
- **THEN** SHALL 从模块的 exercises.md 和 tests/ 提取题目
- **AND** SHALL 支持多种题型（选择题、判断题、代码填空）
- **AND** SHALL 随机排序题目避免重复

#### Scenario: 检查练习答案
- **GIVEN** 用户完成练习题
- **WHEN** 提交答案
- **THEN** SHALL 自动判分并显示结果
- **AND** SHALL 标注错误答案并显示正确答案
- **AND** SHALL 显示得分百分比

#### Scenario: 分析掌握程度
- **GIVEN** 用户完成练习
- **WHEN** 分析掌握情况
- **THEN** SHALL 计算总体得分（0-100%）
- **AND** SHALL 识别薄弱知识点
- **AND** SHALL 推荐需要复习的内容

### Requirement: 学习阶段解锁
应用 SHALL 根据掌握程度控制学习阶段解锁，确保充分掌握后再进入下一模块。

#### Scenario: 计算掌握程度
- **GIVEN** 用户完成一个模块的学习
- **WHEN** 计算掌握程度
- **THEN** SHALL 综合以下因素：
  - 练习题完成率（30%权重）
  - 测试通过率（30%权重）
  - 自检打分（20%权重）
  - 综合练习完成情况（20%权重）
- **AND** SHALL 生成 0-100% 的掌握分数

#### Scenario: 解锁下一模块
- **GIVEN** 用户完成当前模块且掌握程度 ≥95%
- **WHEN** 尝试进入下一模块
- **THEN** SHALL 允许访问下一模块内容
- **AND** SHALL 标记当前模块为"已完成"
- **AND** SHALL 记录完成日期

#### Scenario: 阻止提前进入
- **GIVEN** 用户掌握程度 <95%
- **WHEN** 尝试进入下一模块
- **THEN** SHALL 显示提示信息
- **AND** SHALL 建议需要复习的内容
- **AND** SHALL 不允许访问下一模块

### Requirement: 激励与成就系统
应用 SHALL 提供激励机制，包括成就系统、鼓励消息和里程碑庆祝。

#### Scenario: 成就解锁
- **GIVEN** 用户达成特定条件
- **WHEN** 满足成就要求
- **THEN** SHALL 自动解锁对应成就
- **AND** SHALL 显示成就解锁通知
- **AND** 支持的成就包括：
  - 首次完成模块
  - 连续学习 7/30/100 天
  - 完美通过测试（100%）
  - 代码质量大师（clippy 零警告）

#### Scenario: 显示鼓励消息
- **GIVEN** 用户完成学习活动
- **WHEN** 达成一定进度或里程碑
- **THEN** SHALL 显示鼓励性消息
- **AND** 消息内容 SHALL 根据完成情况变化
- **AND** SHALL 包含正面肯定和进一步学习建议

#### Scenario: 连续学习追踪
- **GIVEN** 用户每天有学习活动
- **WHEN** 记录学习会话
- **THEN** SHALL 累加连续学习天数
- **AND** SHALL 显示当前连续天数
- **AND** SHALL 在中断时从 0 重新计算
- **AND** SHALL 记录历史最高连续天数

### Requirement: 数据持久化
应用 SHALL 将所有学习数据持久化存储在本地数据库中。

#### Scenario: 保存学习记录
- **GIVEN** 用户进行任何学习活动
- **WHEN** 活动结束或应用关闭
- **THEN** SHALL 自动保存到本地 SQLite 数据库
- **AND** SHALL 保存内容包括：进度、练习结果、学习时长、设置等

#### Scenario: 加载历史数据
- **GIVEN** 用户重新启动应用
- **WHEN** 应用初始化
- **THEN** SHALL 从数据库加载所有历史数据
- **AND** SHALL 恢复之前的学习状态

#### Scenario: 数据导出
- **GIVEN** 用户需要备份或迁移数据
- **WHEN** 请求数据导出
- **THEN** SHALL 支持导出为 JSON 格式
- **AND** SHALL 包含所有学习记录和统计数据

### Requirement: 配置与设置
应用 SHALL 提供设置界面，允许用户自定义应用行为。

#### Scenario: 配置仓库路径
- **GIVEN** 用户首次使用或更换仓库
- **WHEN** 配置学习仓库路径
- **THEN** SHALL 支持浏览选择目录
- **AND** SHALL 验证目录是否为有效学习仓库

#### Scenario: 配置提醒设置
- **GIVEN** 用户进入提醒设置
- **WHEN** 配置提醒选项
- **THEN** SHALL 支持开关提醒功能
- **AND** SHALL 支持自定义提醒时间
- **AND** SHALL 支持自定义提醒消息

#### Scenario: 配置学习目标
- **GIVEN** 用户有学习计划
- **WHEN** 设置学习目标
- **THEN** SHALL 支持设置每日学习时长目标
- **AND** SHALL 支持设置预期完成日期
- **AND** SHALL 根据目标调整进度提示
