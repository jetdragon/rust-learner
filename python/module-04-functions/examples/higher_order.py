# 高阶函数和内置函数演示

"""
高阶函数是接受函数作为参数或返回函数的函数。
本文件演示Python的内置高阶函数和函数式编程模式。
"""


def main():
    print("=== Python 高阶函数和函数式编程演示 ===\n")

    # ========== 1. map 函数 ==========
    print("1. map() - 对每个元素应用函数")

    numbers = [1, 2, 3, 4, 5]

    # 使用 lambda
    squared = list(map(lambda x: x ** 2, numbers))
    print(f"map(x^2, {numbers}) = {squared}")

    # 使用普通函数
    def cube(x):
        return x ** 3

    cubed = list(map(cube, numbers))
    print(f"map(x^3, {numbers}) = {cubed}")

    # 多个可迭代对象
    list1 = [1, 2, 3]
    list2 = [10, 20, 30]
    added = list(map(lambda x, y: x + y, list1, list2))
    print(f"map(x+y, {list1}, {list2}) = {added}")
    print()

    # ========== 2. filter 函数 ==========
    print("2. filter() - 过滤元素")

    numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

    # 过滤偶数
    evens = list(filter(lambda x: x % 2 == 0, numbers))
    print(f"filter(偶数, {numbers}) = {evens}")

    # 过滤大于5的数
    above_5 = list(filter(lambda x: x > 5, numbers))
    print(f"filter(>5, {numbers}) = {above_5}")

    # 使用普通函数
    def is_prime(n):
        if n < 2:
            return False
        for i in range(2, int(n ** 0.5) + 1):
            if n % i == 0:
                return False
        return True

    primes = list(filter(is_prime, numbers))
    print(f"filter(质数, {numbers}) = {primes}")
    print()

    # ========== 3. reduce 函数 ==========
    print("3. reduce() - 累积计算")

    from functools import reduce

    numbers = [1, 2, 3, 4, 5]

    # 求和
    total = reduce(lambda x, y: x + y, numbers)
    print(f"reduce(+, {numbers}) = {total}")

    # 求积
    product = reduce(lambda x, y: x * y, numbers)
    print(f"reduce(*, {numbers}) = {product}")

    # 找最大值
    maximum = reduce(lambda x, y: x if x > y else y, numbers)
    print(f"reduce(max, {numbers}) = {maximum}")

    # 字符串连接
    words = ['Hello', ' ', 'World', '!']
    sentence = reduce(lambda x, y: x + y, words)
    print(f"reduce(concat, {words}) = {sentence}")
    print()

    # ========== 4. sorted 函数 ==========
    print("4. sorted() - 自定义排序")

    # 按绝对值排序
    numbers = [-5, 3, -1, 7, -4, 2]
    sorted_by_abs = sorted(numbers, key=abs)
    print(f"按绝对值排序: {numbers} -> {sorted_by_abs}")

    # 按字符串长度排序
    words = ['apple', 'pie', 'a', 'banana']
    sorted_by_len = sorted(words, key=len)
    print(f"按长度排序: {words} -> {sorted_by_len}")

    # 复杂对象排序
    students = [
        {'name': 'Alice', 'score': 95, 'age': 20},
        {'name': 'Bob', 'score': 87, 'age': 22},
        {'name': 'Charlie', 'score': 92, 'age': 21}
    ]

    # 按分数降序
    by_score = sorted(students, key=lambda s: s['score'], reverse=True)
    print(f"\n按分数排序:")
    for s in by_score:
        print(f"  {s['name']}: {s['score']}")
    print()

    # ========== 5. max/min 函数 ==========
    print("5. max/min() - 使用 key 参数")

    # 找最长的单词
    words = ['apple', 'banana', 'pie', 'watermelon']
    longest = max(words, key=len)
    print(f"最长单词: {longest}")

    # 找字典中值最大的项
    grades = {'Alice': 95, 'Bob': 87, 'Charlie': 92}
    top_student = max(grades.items(), key=lambda item: item[1])
    print(f"最高分学生: {top_student}")
    print()

    # ========== 6. any/all 函数 ==========
    print("6. any/all() - 逻辑判断")

    numbers = [2, 4, 6, 8, 10]

    # any - 是否有元素满足条件
    has_even = any(x % 2 == 0 for x in numbers)
    print(f"any(偶数): {has_even}")

    has_odd = any(x % 2 != 0 for x in numbers)
    print(f"any(奇数): {has_odd}")

    # all - 是否所有元素都满足条件
    all_even = all(x % 2 == 0 for x in numbers)
    print(f"all(偶数): {all_even}")

    all_positive = all(x > 0 for x in numbers)
    print(f"all(正数): {all_positive}")
    print()

    # ========== 7. zip 函数 ==========
    print("7. zip() - 并行迭代")

    names = ['Alice', 'Bob', 'Charlie']
    ages = [25, 30, 35]
    cities = ['Beijing', 'Shanghai', 'Guangzhou']

    # 配对多个列表
    people = list(zip(names, ages, cities))
    print(f"zip(names, ages, cities):")
    for person in people:
        print(f"  {person[0]}, {person[1]}岁, {person[2]}")

    # 创建字典
    person_dict = dict(zip(names, ages))
    print(f"\ndict(zip(names, ages)): {person_dict}")
    print()

    # ========== 8. enumerate 函数 ==========
    print("8. enumerate() - 获取索引")

    fruits = ['apple', 'banana', 'cherry']

    print("使用 enumerate:")
    for index, fruit in enumerate(fruits, start=1):
        print(f"  {index}. {fruit}")

    # 创建带索引的字典
    indexed_fruits = {f"fruit_{i}": fruit for i, fruit in enumerate(fruits)}
    print(f"\n带索引的字典: {indexed_fruits}")
    print()

    # ========== 9. 函数组合 ==========
    print("9. 函数组合（管道）")

    def compose(*functions):
        """函数组合：从右到左执行"""
        return reduce(lambda f, g: lambda x: f(g(x)), functions)

    # 定义简单函数
    add_one = lambda x: x + 1
    multiply_two = lambda x: x * 2
    square = lambda x: x ** 2

    # 组合函数: square(multiply_two(add_one(x)))
    combined = compose(square, multiply_two, add_one)
    result = combined(3)
    print(f"compose(square, *2, +1)(3) = {result}")
    print(f"计算过程: (3 + 1) * 2 ^ 2 = 16")
    print()

    # ========== 10. 偏函数 (Partial Functions) ==========
    print("10. 偏函数 - 固定部分参数")

    from functools import partial

    def power(base, exp):
        return base ** exp

    # 固定 base 为 2
    power_of_two = partial(power, 2)
    print(f"power_of_two(3) = {power_of_two(3)}")  # 2^3 = 8
    print(f"power_of_two(5) = {power_of_two(5)}")  # 2^5 = 32

    # 固定 exp 为 2
    square_partial = partial(power, exp=2)
    print(f"square_partial(4) = {square_partial(4)}")  # 4^2 = 16
    print()

    # ========== 11. 柯里化 (Currying) ==========
    print("11. 柯里化 - 多参数函数转换为单参数链")

    def add(a):
        def add_b(b):
            return a + b
        return add_b

    # 逐步调用
    add_5 = add(5)
    result = add_5(3)
    print(f"add(5)(3) = {result}")

    # 一次性调用
    result2 = add(10)(20)
    print(f"add(10)(20) = {result2}")
    print()

    # ========== 12. 惰性求值 ==========
    print("12. 惰性求值 - 生成器表达式")

    # 生成器表达式（惰性）
    lazy_squares = (x ** 2 for x in range(1000000))
    print(f"生成器: {lazy_squares}")
    print(f"前5个: {list(next(lazy_squares) for _ in range(5))}")

    # 列表推导式（立即求值）
    eager_squares = [x ** 2 for x in range(10)]
    print(f"\n列表: {eager_squares}")
    print()

    # ========== 13. 实际应用案例 ==========
    print("13. 实际应用：学生成绩处理")

    students = [
        {'name': 'Alice', 'math': 95, 'english': 87, 'science': 92},
        {'name': 'Bob', 'math': 78, 'english': 85, 'science': 80},
        {'name': 'Charlie', 'math': 88, 'english': 92, 'science': 90}
    ]

    # 计算每个学生的平均分
    def calc_average(student):
        scores = [student['math'], student['english'], student['science']]
        return sum(scores) / len(scores)

    students_with_avg = list(map(
        lambda s: {**s, 'average': calc_average(s)},
        students
    ))

    # 过滤平均分>=85的学生
    top_students = list(filter(
        lambda s: s['average'] >= 85,
        students_with_avg
    ))

    # 按平均分排序
    ranked = sorted(top_students, key=lambda s: s['average'], reverse=True)

    print("学生成绩排名（平均分≥85）:")
    for i, student in enumerate(ranked, 1):
        print(f"{i}. {student['name']}: {student['average']:.1f}")
    print()

    print("=== 演示完成 ===")
    print()
    print("💡 函数式编程的优势:")
    print("  1. 代码简洁，表达力强")
    print("  2. 便于并行处理")
    print("  3. 减少可变状态")
    print("  4. 更容易测试和推理")
    print()
    print("⚠️  注意:")
    print("  - 过度使用会降低可读性")
    print("  - Python 不是纯函数式语言")
    print("  - 合理使用，与命令式风格结合")


if __name__ == "__main__":
    main()
