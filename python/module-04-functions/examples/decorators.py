# 装饰器 (Decorators) 演示

"""
装饰器是Python的强大特性，用于修改或增强函数的行为。
本文件演示装饰器的创建和使用。
"""


def main():
    print("=== Python 装饰器演示 ===\n")

    # ========== 1. 基础装饰器 ==========
    print("1. 基础装饰器")

    def my_decorator(func):
        """简单的装饰器"""
        def wrapper():
            print("调用函数前...")
            result = func()
            print("调用函数后...")
            return result
        return wrapper

    @my_decorator
    def say_hello():
        return "Hello!"

    print("调用 @my_decorator 装饰的函数:")
    print(say_hello())
    print()

    # ========== 2. 装饰器保留原函数信息 ==========
    print("2. 装饰器保留原函数信息")

    from functools import wraps

    def better_decorator(func):
        """使用 functools.wraps 保留原函数信息"""
        @wraps(func)
        def wrapper(*args, **kwargs):
            print(f"调用 {func.__name__}...")
            return func(*args, **kwargs)
        return wrapper

    @better_decorator
    def greet(name: str) -> str:
        """向某人打招呼"""
        return f"Hello, {name}!"

    print(f"函数名: {greet.__name__}")
    print(f"函数文档: {greet.__doc__}")
    print(greet("Alice"))
    print()

    # ========== 3. 带参数的装饰器 ==========
    print("3. 带参数的装饰器")

    def repeat(times: int):
        """重复执行函数多次的装饰器工厂"""
        def decorator(func):
            @wraps(func)
            def wrapper(*args, **kwargs):
                results = []
                for _ in range(times):
                    result = func(*args, **kwargs)
                    results.append(result)
                return results
            return wrapper
        return decorator

    @repeat(times=3)
    def say_hi(name: str) -> str:
        return f"Hi, {name}!"

    print("调用 @repeat(times=3) 装饰的函数:")
    results = say_hi("Bob")
    for i, result in enumerate(results, 1):
        print(f"  第{i}次: {result}")
    print()

    # ========== 4. 实际应用：计时装饰器 ==========
    print("4. 实际应用：计时装饰器")

    import time

    def timer(func):
        """测量函数执行时间的装饰器"""
        @wraps(func)
        def wrapper(*args, **kwargs):
            start = time.time()
            result = func(*args, **kwargs)
            end = time.time()
            print(f"{func.__name__}() 执行时间: {end - start:.4f}秒")
            return result
        return wrapper

    @timer
    def slow_function():
        """模拟耗时操作"""
        time.sleep(0.1)
        return "完成"

    print("测量函数执行时间:")
    result = slow_function()
    print(f"返回值: {result}\n")

    # ========== 5. 实际应用：日志装饰器 ==========
    print("5. 实际应用：日志装饰器")

    def logger(func):
        """记录函数调用的装饰器"""
        @wraps(func)
        def wrapper(*args, **kwargs):
            print(f"[LOG] 调用 {func.__name__}()")
            print(f"[LOG] 参数: args={args}, kwargs={kwargs}")
            result = func(*args, **kwargs)
            print(f"[LOG] 返回值: {result}")
            return result
        return wrapper

    @logger
    def add(a: int, b: int) -> int:
        """加法运算"""
        return a + b

    print("日志装饰器:")
    add(3, 5)
    print()

    # ========== 6. 实际应用：缓存装饰器 ==========
    print("6. 实际应用：缓存装饰器")

    def memoize(func):
        """缓存函数结果，避免重复计算"""
        cache = {}

        @wraps(func)
        def wrapper(*args):
            if args not in cache:
                cache[args] = func(*args)
                print(f"计算 {func.__name__}{args}")
            else:
                print(f"从缓存读取 {func.__name__}{args}")
            return cache[args]
        return wrapper

    @memoize
    def fibonacci(n: int) -> int:
        """斐波那契数列（带缓存）"""
        if n < 2:
            return n
        return fibonacci(n - 1) + fibonacci(n - 2)

    print("斐波那契数列（带缓存）:")
    print(f"fibonacci(10) = {fibonacci(10)}")
    print()

    # ========== 7. 实际应用：调试装饰器 ==========
    print("7. 实际应用：调试装饰器")

    def debug(func):
        """打印函数调用信息的调试装饰器"""
        @wraps(func)
        def wrapper(*args, **kwargs):
            args_repr = [repr(a) for a in args]
            kwargs_repr = [f"{k}={v!r}" for k, v in kwargs.items()]
            signature = ", ".join(args_repr + kwargs_repr)
            print(f"调用 {func.__name__}({signature})")
            result = func(*args, **kwargs)
            print(f"{func.__name__}() 返回 {result!r}")
            return result
        return wrapper

    @debug
    def calculate(a: int, b: int) -> int:
        """计算两数之积"""
        return a * b

    print("调试装饰器:")
    calculate(5, 3)
    print()

    # ========== 8. 多个装饰器 ==========
    print("8. 多个装饰器（装饰器堆叠）")

    def make_bold(func):
        @wraps(func)
        def wrapper(*args, **kwargs):
            return f"<b>{func(*args, **kwargs)}</b>"
        return wrapper

    def make_italic(func):
        @wraps(func)
        def wrapper(*args, **kwargs):
            return f"<i>{func(*args, **kwargs)}</i>"
        return wrapper

    @make_bold
    @make_italic
    def greet_text():
        return "Hello"

    print("多个装饰器（从下往上应用）:")
    print(f"结果: {greet_text()}")
    print("解释: 先 italic，再 bold")
    print()

    # ========== 9. 类作为装饰器 ==========
    print("9. 类作为装饰器")

    class CountCalls:
        """统计函数调用次数的类装饰器"""
        def __init__(self, func):
            self.func = func
            self.count = 0

        def __call__(self, *args, **kwargs):
            self.count += 1
            print(f"{self.func.__name__} 被调用了 {self.count} 次")
            return self.func(*args, **kwargs)

    @CountCalls
    def process_data():
        return "数据处理中..."

    print("类装饰器 - 统计调用次数:")
    process_data()
    process_data()
    process_data()
    print()

    # ========== 10. 带状态的装饰器 ==========
    print("10. 带状态的装饰器")

    def counter(func):
        """统计函数调用次数的函数装饰器"""
        def wrapper(*args, **kwargs):
            wrapper.count += 1
            print(f"调用次数: {wrapper.count}")
            return func(*args, **kwargs)
        wrapper.count = 0
        return wrapper

    @counter
    def do_work():
        return "工作中..."

    print("带状态的装饰器:")
    do_work()
    do_work()
    do_work()
    print()

    # ========== 11. 可选参数的装饰器 ==========
    print("11. 可选参数的装饰器")

    def smart_decorator(func=None, *, option="default"):
        """可以使用带或不带参数的装饰器"""
        def decorator(f):
            @wraps(f)
            def wrapper(*args, **kwargs):
                print(f"装饰器选项: {option}")
                return f(*args, **kwargs)
            return wrapper

        if func is None:
            # 带参数调用 @smart_decorator(option="value")
            return decorator
        else:
            # 不带参数调用 @smart_decorator
            return decorator(func)

    @smart_decorator
    def func1():
        return "函数1"

    @smart_decorator(option="custom")
    def func2():
        return "函数2"

    print("可选参数的装饰器:")
    func1()
    func2()
    print()

    # ========== 12. 装饰器最佳实践 ==========
    print("12. 装饰器最佳实践")
    print("✅ 使用 functools.wraps 保留原函数信息")
    print("✅ 装饰器应该透明，不改变函数签名")
    print("✅ 考虑使用装饰器工厂实现可配置的装饰器")
    print("✅ 装饰器应该有清晰的文档字符串")
    print("✅ 复杂装饰器可以考虑使用类实现")
    print()

    print("=== 演示完成 ===")


if __name__ == "__main__":
    main()
