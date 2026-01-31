# 异常处理演示

"""
本文件演示Python的异常处理机制：try-except-else-finally
"""


def main() -> None:
    print("=== Python 异常处理演示 ===\n")

    # ========== 1. 基础 try-except ==========
    print("1. 基础 try-except")

    try:
        result = 10 / 2
        print(f"10 / 2 = {result}")
    except ZeroDivisionError:
        print("不能除以零")

    try:
        result = 10 / 0
    except ZeroDivisionError as e:
        print(f"捕获异常: {e}")
    print()

    # ========== 2. 捕获多种异常 ==========
    print("2. 捕获多种异常")

    def safe_divide(a: int, b: int) -> None:
        """安全的除法函数"""
        try:
            result = a / b
        except ZeroDivisionError:
            print("❌ 除数不能为0")
        except TypeError:
            print("❌ 参数必须是数字")
        else:
            print(f"✅ 结果: {result}")

    safe_divide(10, 2)
    safe_divide(10, 0)
    safe_divide("10", "2")
    print()

    # ========== 3. else 和 finally ==========
    print("3. else 和 finally 子句")

    try:
        result = 10 / 2
    except ZeroDivisionError:
        print("除零错误")
    else:
        print(f"✅ 计算成功: {result}")
    finally:
        print("🔄 无论如何都执行")
    print()

    # ========== 4. 获取异常信息 ==========
    print("4. 获取异常详细信息")

    try:
        number = int("abc")
    except ValueError as e:
        print(f"异常类型: {type(e).__name__}")
        print(f"异常信息: {e}")
        print(f"异常参数: {e.args}")
    print()

    # ========== 5. 主动抛出异常 ==========
    print("5. raise - 主动抛出异常")

    def validate_age(age: int) -> None:
        """验证年龄"""
        if age < 0:
            raise ValueError("年龄不能为负数")
        if age > 150:
            raise ValueError("年龄不真实")
        print(f"✅ 年龄 {age} 有效")

    try:
        validate_age(-5)
    except ValueError as e:
        print(f"❌ {e}")
    print()

    # ========== 6. 自定义异常 ==========
    print("6. 自定义异常类")

    class InvalidEmailError(Exception):
        """无效邮箱异常"""
        def __init__(self, email: str):
            self.email = email
            super().__init__(f"无效的邮箱地址: {email}")

    def validate_email(email: str) -> None:
        """验证邮箱"""
        if "@" not in email:
            raise InvalidEmailError(email)
        print(f"✅ 邮箱 {email} 有效")

    try:
        validate_email("invalid-email")
    except InvalidEmailError as e:
        print(f"❌ {e}")
    print()

    # ========== 7. try-except-else 完整示例 ==========
    print("7. 完整的异常处理结构")

    def process_file(filename: str) -> None:
        """处理文件（模拟）"""
        try:
            print(f"打开文件: {filename}")
            # 模拟文件操作
            if filename == "missing.txt":
                raise FileNotFoundError(f"文件不存在: {filename}")
        except FileNotFoundError as e:
            print(f"❌ {e}")
        except PermissionError:
            print("❌ 没有权限")
        except Exception as e:
            print(f"❌ 未知错误: {e}")
        else:
            print("✅ 文件处理成功")
        finally:
            print("🔄 清理资源")

    process_file("test.txt")
    process_file("missing.txt")
    print()

    # ========== 8. 异常链 ==========
    print("8. 异常链（raise from）")

    def parse_number(text: str) -> int:
        """解析数字"""
        try:
            return int(text)
        except ValueError as e:
            # 保留原始异常
            raise TypeError(f"'{text}' 不是有效的数字") from e

    try:
        parse_number("abc")
    except TypeError as e:
        print(f"当前异常: {e}")
        print(f"原始异常: {e.__cause__}")
    print()

    # ========== 9. 多个异常处理 ==========
    print("9. 处理多个异常")

    def handle_data(data) -> None:
        """处理数据"""
        try:
            # 可能抛出多种异常的操作
            result = int(data)
            print(f"转换结果: {result}")
        except (ValueError, TypeError) as e:
            print(f"❌ 类型错误: {e}")
        except Exception as e:
            print(f"❌ 其他错误: {e}")

    handle_data("123")
    handle_data("abc")
    print()

    # ========== 10. 警告处理 ==========
    print("10. 警告（Warning）")

    import warnings

    # 发出警告
    warnings.warn("这是一个警告", UserWarning)

    # 捕获警告
    with warnings.catch_warnings(record=True) as w:
        warnings.simplefilter("always")
        warnings.warn("另一个警告", DeprecationWarning)
        if w:
            print(f"⚠️  捕获到警告: {w[0].message}")
    print()

    # ========== 11. 资源清理 ==========
    print("11. 使用 with 进行资源清理")

    class File:
        """模拟文件类"""
        def __init__(self, name: str):
            self.name = name

        def __enter__(self):
            print(f"打开 {self.name}")
            return self

        def __exit__(self, exc_type, exc_val, exc_tb):
            print(f"关闭 {self.name}")
            if exc_type:
                print(f"发生异常: {exc_type.__name__}")
            return False  # 不处理异常

    # 使用上下文管理器
    with File("test.txt") as f:
        print(f"操作 {f.name}")
    print("文件已自动关闭")
    print()

    # ========== 12. 最佳实践 ==========
    print("12. 异常处理最佳实践")
    print("✅ 推荐做法:")
    print("  1. 只捕获能处理的异常")
    print("  2. 使用具体的异常类型")
    print("  3. finally 用于资源清理")
    print("  4. 提供有用的错误信息")
    print("  5. 记录异常日志")
    print()
    print("❌ 避免:")
    print("  1. except: （裸except）")
    print("  2. 吞掉异常（pass）")
    print("  3. 过于宽泛的异常捕获")
    print("  4. 用异常处理正常流程")

    print("\n=== 演示完成 ===")


if __name__ == "__main__":
    main()
