# 异常处理演示

"""
本文件演示Python的异常处理机制和错误管理最佳实践。
"""


def main():
    print("=== Python 异常处理演示 ===\n")

    # ========== 1. 基础异常处理 ==========
    print("1. 基础 try-except")

    try:
        result = 10 / 0
    except ZeroDivisionError as e:
        print(f"捕获异常: {e}")
        print(f"异常类型: {type(e).__name__}")

    # ========== 2. 多个 except 块 ==========
    print("\n2. 处理多种异常")

    def safe_divide(a, b):
        try:
            return a / b
        except ZeroDivisionError:
            print("❌ 除数不能为0")
            return None
        except TypeError:
            print("❌ 参数必须是数字")
            return None

    print(f"safe_divide(10, 2): {safe_divide(10, 2)}")
    print(f"safe_divide(10, 0): {safe_divide(10, 0)}")
    print(f"safe_divide('10', '2'): {safe_divide('10', '2')}")

    # ========== 3. else 和 finally ==========
    print("\n3. else 和 finally 块")

    try:
        result = 10 / 2
    except ZeroDivisionError:
        print("除零错误")
    else:
        print(f"✅ 计算成功，结果: {result}")
    finally:
        print("🔄 无论如何都执行（finally）")

    # ========== 4. 捕获异常信息 ==========
    print("\n4. 获取详细的异常信息")

    import traceback

    try:
        result = 10 / 0
    except Exception as e:
        print(f"异常信息: {e}")
        print(f"异常类型: {type(e).__name__}")
        print("\n完整堆栈跟踪:")
        traceback.print_exc()

    # ========== 5. 主动抛出异常 ==========
    print("\n5. raise - 主动抛出异常")

    def validate_age(age):
        if age < 0:
            raise ValueError("年龄不能为负数")
        if age > 150:
            raise ValueError("年龄不真实")
        return age

    try:
        validate_age(-5)
    except ValueError as e:
        print(f"验证失败: {e}")

    # ========== 6. 自定义异常 ==========
    print("\n6. 自定义异常类")

    class InsufficientFundsError(Exception):
        """余额不足异常"""
        def __init__(self, balance, amount):
            self.balance = balance
            self.amount = amount
            super().__init__(f"余额 ${balance} 不足以支付 ${amount}")

    def withdraw(balance, amount):
        if amount > balance:
            raise InsufficientFundsError(balance, amount)
        return balance - amount

    try:
        withdraw(100, 150)
    except InsufficientFundsError as e:
        print(f"自定义异常: {e}")

    # ========== 7. 异常链 ==========
    print("\n7. 异常链（raise from）")

    def process_data(data):
        try:
            int(data)
        except ValueError as e:
            # 保留原始异常信息
            raise TypeError("数据格式错误") from e

    try:
        process_data("abc")
    except TypeError as e:
        print(f"当前异常: {e}")
        print(f"原始异常: {e.__cause__}")

    # ========== 8. 上下文管理器进行清理 ==========
    print("\n8. 上下文管理器确保资源释放")

    class FileManager:
        def __init__(self, filename):
            self.filename = filename

        def __enter__(self):
            print("打开文件...")
            self.file = open(self.filename, "w")
            return self.file

        def __exit__(self, exc_type, exc_val, exc_tb):
            print("关闭文件...")
            if self.file:
                self.file.close()
            # 返回 False 表示异常不处理，返回 True 表示异常已处理
            return False

    # 使用
    import tempfile
    import os

    temp_file = tempfile.NamedTemporaryFile(delete=False)
    temp_name = temp_file.name
    temp_file.close()

    try:
        with FileManager(temp_name) as f:
            f.write("测试内容")
            # 模拟异常
            # raise ValueError("测试异常")
    except Exception as e:
        print(f"捕获到异常: {e}")
    finally:
        os.unlink(temp_name)

    # ========== 9. 常见异常类型 ==========
    print("\n9. 常见内置异常类型")

    exceptions = {
        "Exception": "所有异常的基类",
        "TypeError": "类型错误",
        "ValueError": "值不正确",
        "KeyError": "字典键不存在",
        "IndexError": "索引超出范围",
        "AttributeError": "属性不存在",
        "FileNotFoundError": "文件不存在",
        "ZeroDivisionError": "除零错误",
        "ImportError": "导入失败",
        "RuntimeError": "运行时错误"
    }

    for exc, desc in exceptions.items():
        print(f"  {exc}: {desc}")

    # ========== 10. 最佳实践 ==========
    print("\n10. 异常处理最佳实践")
    print("✅ 推荐做法:")
    print("  1. 只捕获能处理的异常")
    print("  2. 使用具体的异常类型而非裸 except")
    print("  3. finally 用于资源清理")
    print("  4. 自定义异常提供更多上下文")
    print("  5. 记录异常日志")
    print()
    print("❌ 避免:")
    print("  1. except: （裸except）- 吞掉所有异常")
    print("  2. except Exception: - 过于宽泛")
    print("  3. pass - 忽略异常")
    print("  4. 用异常控制正常流程")

    print("\n=== 演示完成 ===")


if __name__ == "__main__":
    main()
