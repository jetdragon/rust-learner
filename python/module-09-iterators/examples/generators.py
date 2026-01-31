# 迭代器和生成器演示

"""
本文件演示Python的迭代器协议、生成器函数和惰性求值。
"""


def main():
    print("=== Python 迭代器和生成器演示 ===\n")

    # ========== 1. 迭代器基础 ==========
    print("1. 可迭代对象 vs 迭代器")

    # 可迭代对象（Iterable）
    my_list = [1, 2, 3, 4, 5]
    print(f"列表是可迭代的: {hasattr(my_list, '__iter__')}")

    # 获取迭代器
    my_iterator = iter(my_list)
    print(f"迭代器: {my_iterator}")

    # 使用迭代器
    print(f"next(my_iterator): {next(my_iterator)}")
    print(f"next(my_iterator): {next(my_iterator)}")
    print()

    # ========== 2. 自定义迭代器 ==========
    print("2. 自定义迭代器类")

    class Countdown:
        """倒计时迭代器"""

        def __init__(self, start):
            self.start = start

        def __iter__(self):
            """返回迭代器对象"""
            return self

        def __next__(self):
            """返回下一个值"""
            if self.start <= 0:
                raise StopIteration
            value = self.start
            self.start -= 1
            return value

    countdown = Countdown(5)
    print("倒计时:")
    for num in countdown:
        print(f"  {num}")
    print()

    # ========== 3. 生成器函数 ==========
    print("3. 生成器函数（使用 yield）")

    def simple_generator():
        """简单的生成器"""
        yield 1
        yield 2
        yield 3

    gen = simple_generator()
    print(f"生成器对象: {gen}")
    print(f"next(gen): {next(gen)}")
    print(f"next(gen): {next(gen)}")
    print(f"next(gen): {next(gen)}")
    # print(f"next(gen): {next(gen)}")  # StopIteration
    print()

    # ========== 4. 生成器表达式 ==========
    print("4. 生成器表达式（类似列表推导）")

    # 列表推导（立即求值）
    list_comp = [x**2 for x in range(5)]
    print(f"列表推导: {list_comp}")

    # 生成器表达式（惰性求值）
    gen_expr = (x**2 for x in range(5))
    print(f"生成器: {gen_expr}")
    print(f"转换为列表: {list(gen_expr)}")
    print()

    # ========== 5. 无限序列生成器 ==========
    print("5. 无限序列生成器")

    def fibonacci():
        """斐波那契数列生成器"""
        a, b = 0, 1
        while True:
            yield a
            a, b = b, a + b

    fib = fibonacci()
    print("斐波那契数列（前10个）:")
    for _ in range(10):
        print(f"  {next(fib)}")
    print()

    # ========== 6. 生成器管道 ==========
    print("6. 生成器管道（链式处理）")

    def integers():
        """无限整数序列"""
        i = 0
        while True:
            yield i
            i += 1

    def take(n, iterable):
        """取前n个元素"""
        for i, x in enumerate(iterable):
            if i >= n:
                break
            yield x

    def is_even(iterable):
        """过滤偶数"""
        for x in iterable:
            if x % 2 == 0:
                yield x

    # 管道：integers -> is_even -> take
    result = list(take(5, is_even(integers())))
    print(f"前5个偶数: {result}")
    print()

    # ========== 7. yield from - 委托生成器 ==========
    print("7. yield from - 委托给子生成器")

    def sub_generator():
        yield "子生成器: A"
        yield "子生成器: B"

    def main_generator():
        yield "主: 1"
        yield from sub_generator()
        yield "主: 2"

    for value in main_generator():
        print(f"  {value}")
    print()

    # ========== 8. 生成器方法 ==========
    print("8. 生成器的方法")

    def range_generator(n):
        """范围生成器"""
        for i in range(n):
            yield i

    gen = range_generator(5)
    print(f"生成器对象: {gen}")

    # send() - 向生成器发送值
    def echo_generator():
        """回显生成器"""
        while True:
            received = yield
            print(f"收到: {received}")

    echo = echo_generator()
    next(echo)  # 启动生成器
    echo.send("Hello")
    echo.send("World")

    # close() - 关闭生成器
    gen.close()
    print("生成器已关闭")

    # throw() - 向生成器抛出异常
    # gen.throw(ValueError("测试异常"))
    print()

    # ========== 9. 惰性求值的优势 ==========
    print("9. 惰性求值的优势")

    import sys

    # 列表（占用大量内存）
    big_list = [x for x in range(1000000)]
    print(f"列表大小: {sys.getsizeof(big_list)} 字节")

    # 生成器（占用很少内存）
    big_gen = (x for x in range(1000000))
    print(f"生成器大小: {sys.getsizeof(big_gen)} 字节")
    print("💡 生成器节省内存！")
    print()

    # ========== 10. 实际应用：读取大文件 ==========
    print("10. 实际应用：逐行读取大文件")

    def read_lines(filename):
        """逐行生成器"""
        with open(filename, "r", encoding="utf-8") as f:
            for line in f:
                yield line.strip()

    # 创建测试文件
    import tempfile
    import os

    temp_file = tempfile.NamedTemporaryFile(mode="w", delete=False, encoding="utf-8")
    temp_name = temp_file.name
    for i in range(5):
        temp_file.write(f"行 {i+1}\n")
    temp_file.close()

    # 使用生成器读取
    print("逐行读取:")
    for i, line in enumerate(read_lines(temp_name), 1):
        print(f"  {i}. {line}")

    os.unlink(temp_name)
    print()

    print("=== 演示完成 ===")
    print()
    print("💡 生成器的优势:")
    print("  1. 节省内存（惰性求值）")
    print("  2. 表示无限序列")
    print("  3. 流水线处理")
    print("  4. 代码简洁")
    print()
    print("💡 使用场景:")
    print("  - 处理大文件")
    print("  - 无限数据流")
    print("  - 数据管道")
    print("  - 协程和异步编程")


if __name__ == "__main__":
    main()
