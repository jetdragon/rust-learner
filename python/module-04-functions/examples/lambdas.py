# Lambda 表达式演示

"""
Lambda表达式（匿名函数）是Python的简洁函数定义方式。
本文件演示lambda的各种用法。
"""


def main():
    print("=== Python Lambda 表达式演示 ===\n")

    # ========== 1. Lambda 基础 ==========
    print("1. Lambda 基础")

    # 普通函数
    def square(x):
        return x ** 2

    # 等价的 lambda
    square_lambda = lambda x: x ** 2

    print(f"square(5) = {square(5)}")
    print(f"square_lambda(5) = {square_lambda(5)}")
    print()

    # ========== 2. Lambda 多个参数 ==========
    print("2. Lambda 多个参数")

    add = lambda x, y: x + y
    multiply = lambda x, y: x * y

    print(f"add(3, 5) = {add(3, 5)}")
    print(f"multiply(4, 7) = {multiply(4, 7)}")
    print()

    # ========== 3. Lambda 默认参数 ==========
    print("3. Lambda 默认参数")

    power = lambda x, exp=2: x ** exp

    print(f"power(3) = {power(3)}")      # 3^2 = 9
    print(f"power(3, 3) = {power(3, 3)}")  # 3^3 = 27
    print()

    # ========== 4. Lambda 与内置函数 ==========
    print("4. Lambda 与内置函数")

    numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

    # map - 对每个元素应用函数
    doubled = list(map(lambda x: x * 2, numbers))
    print(f"map(x*2, {numbers})")
    print(f"结果: {doubled}")

    # filter - 过滤元素
    evens = list(filter(lambda x: x % 2 == 0, numbers))
    print(f"\nfilter(x%2==0, {numbers})")
    print(f"结果: {evens}")

    # sorted - 自定义排序
    pairs = [(1, 'one'), (3, 'three'), (2, 'two'), (4, 'four')]
    sorted_pairs = sorted(pairs, key=lambda x: x[1])  # 按字母排序
    print(f"\n排序前: {pairs}")
    print(f"按字母排序: {sorted_pairs}")
    print()

    # ========== 5. Lambda 与列表推导式 ==========
    print("5. Lambda vs 列表推导式")

    # 使用 lambda + map
    result1 = list(map(lambda x: x ** 2, range(1, 6)))

    # 使用列表推导式（更Pythonic）
    result2 = [x ** 2 for x in range(1, 6)]

    print(f"Lambda + map: {result1}")
    print(f"列表推导式: {result2}")
    print("💡 推荐：简单操作优先使用列表推导式")
    print()

    # ========== 6. Lambda 实际应用 ==========
    print("6. Lambda 实际应用")

    # 应用1: 排序字典
    students = [
        {'name': 'Alice', 'score': 95},
        {'name': 'Bob', 'score': 87},
        {'name': 'Charlie', 'score': 92}
    ]

    # 按分数排序
    sorted_by_score = sorted(students, key=lambda s: s['score'], reverse=True)
    print("按分数排序:")
    for student in sorted_by_score:
        print(f"  {student['name']}: {student['score']}")

    # 应用2: 字典排序
    grades = {'Alice': 95, 'Bob': 87, 'Charlie': 92}
    sorted_grades = dict(sorted(grades.items(), key=lambda item: item[1], reverse=True))
    print(f"\n排序后的字典: {sorted_grades}")

    # 应用3: 最大值/最小值
    people = [('Alice', 30), ('Bob', 25), ('Charlie', 35)]
    oldest = max(people, key=lambda p: p[1])
    print(f"\n最年长的人: {oldest}")
    print()

    # ========== 7. Lambda 与函数式编程 ==========
    print("7. Lambda 与 reduce")

    from functools import reduce

    numbers = [1, 2, 3, 4, 5]

    # 求和
    sum_result = reduce(lambda x, y: x + y, numbers)
    print(f"reduce(lambda x,y: x+y, {numbers}) = {sum_result}")

    # 求积
    product = reduce(lambda x, y: x * y, numbers)
    print(f"reduce(lambda x,y: x*y, {numbers}) = {product}")
    print()

    # ========== 8. Lambda 的局限性 ==========
    print("8. Lambda 的局限性")

    print("❌ Lambda 只能包含单个表达式")
    print("❌ Lambda 中不能使用语句（如 if, for, while）")
    print("❌ Lambda 中不能使用 return 或 yield")
    print("❌ 复杂逻辑应该用 def 定义函数")
    print()

    # 错误示例
    print("错误的 lambda 示例:")
    print("  lambda x: if x > 0: return x  # ❌ 语法错误")
    print()
    print("正确的做法:")
    print("  lambda x: x if x > 0 else -x  # ✅ 使用三元表达式")
    print()

    # ========== 9. Lambda 最佳实践 ==========
    print("9. Lambda 最佳实践")

    print("✅ 适合使用 Lambda 的场景:")
    print("  - 简短的单行函数")
    print("  - 作为参数传递给高阶函数（map, filter, sorted等）")
    print("  - 临时使用的简单函数")

    print("\n❌ 不适合使用 Lambda 的场景:")
    print("  - 复杂的逻辑")
    print("  - 需要多行的函数")
    print("  - 需要文档字符串的函数")
    print("  - 需要重用的函数")
    print()

    # ========== 10. 实际案例 ==========
    print("10. 实际案例：数据转换管道")

    data = [
        {'name': ' Alice ', 'age': 30},
        {'name': '  Bob  ', 'age': 25},
        {'name': 'Charlie', 'age': 35}
    ]

    # 处理管道：清理名字、过滤年轻人、按名字排序
    cleaned = list(map(lambda p: {**p, 'name': p['name'].strip()}, data))
    filtered = list(filter(lambda p: p['age'] >= 30, cleaned))
    sorted_data = sorted(filtered, key=lambda p: p['name'])

    print("数据处理管道:")
    print(f"原始: {data}")
    print(f"处理后: {sorted_data}")
    print()

    print("=== 演示完成 ===")
    print()
    print("💡 关键要点:")
    print("  1. Lambda 是匿名函数，用于简短操作")
    print("  2. 语法: lambda parameters: expression")
    print("  3. 优先考虑列表推导式而非 map/filter + lambda")
    print("  4. 复杂逻辑应该用 def 定义普通函数")
    print("  5. Lambda 在排序和数据处理中非常有用")


if __name__ == "__main__":
    main()
