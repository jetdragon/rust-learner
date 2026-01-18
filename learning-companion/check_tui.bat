@echo off
echo ========================================
echo TUI 问题自动检测工具
echo ========================================
echo.

echo [1] 检查 tui.rs 中的 println! 使用（污染 TUI）
echo ----------------------------------------
findstr /n "println!" src\tui.rs | findstr /v "//"
if %errorlevel% equ 0 (
    echo [警告] 发现 println! 使用，这些会破坏 TUI
) else (
    echo [通过] 未发现 println!
)
echo.

echo [2] 检查 progress.rs 中的打印（污染 TUI）
echo ----------------------------------------
findstr /n "println!" src\progress.rs | findstr /v "//"
if %errorlevel% equ 0 (
    echo [警告] 发现 println! 使用，应该使用返回值
) else (
    echo [通过] 未发现 println!
)
echo.

echo [3] 检查直接状态赋值（可能返回逻辑错误）
echo ----------------------------------------
findstr /n "self.state = AppState::" src\tui.rs | find /c /v "" > temp_count.txt
set /p count=<temp_count.txt
echo 发现 %count% 处直接状态赋值
echo 建议检查这些位置是否应该使用 push/pop_state
del temp_count.txt
echo.

echo [4] 检查状态栈操作
echo ----------------------------------------
findstr /n "push_state\|pop_state" src\tui.rs | find /c /v "" > stack_count.txt
set /p stack_count=<stack_count.txt
echo 状态栈操作数量: %stack_count%
if %stack_count% lss 10 (
    echo [警告] 状态栈操作较少，可能需要更多使用
)
del stack_count.txt
echo.

echo [5] 检查是否有消息超时清理
echo ----------------------------------------
findstr /n "message_deadline" src\tui.rs | find /c /v "" > msg_count.txt
set /p msg_count=<msg_count.txt
echo 消息超时相关代码: %msg_count% 处
if %msg_count% lss 5 (
    echo [警告] 消息超时处理可能不足
)
del msg_count.txt
echo.

echo [6] 检查未处理的按键 case
echo ----------------------------------------
echo 检查主要状态处理函数的默认分支...
findstr /n "_ =>" src\tui.rs | find /c /v "" > default_count.txt
set /p default_count=<default_count.txt
echo 默认分支数量: %default_count%
if %default_count% lss 10 (
    echo [警告] 可能有未处理的按键
)
del default_count.txt
echo.

echo ========================================
echo 检测完成！
echo ========================================
echo.
echo 建议操作：
echo 1. 查看 TUI_DEBUG_GUIDE.md 了解详细调试方法
echo 2. 运行 TUI 并测试按键响应
echo 3. 使用 Ctrl+C 可以安全退出 TUI
echo.
