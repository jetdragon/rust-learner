# 循环语句演示

"""
本文件演示Python的循环语句：for循环、while循环和循环控制
"""


def main() -> None:
    print("=== Python 循环语句演示 ===\n")

    # ========== 1. for 循环遍历列表 ==========
    print("1. for 循环遍历列表")

    fruits = ["apple", "banana", "cherry"]
    for fruit in fruits:
        print(f"水果: {fruit}")
    print()

    # ========== 2. for 循环遍历字符串 ==========
    print("2. for 循环遍历字符串")

    for char in "Python":
        print(f"字符: {char}")
    print()

    # ========== 3. for 循环遍历字典 ==========
    print("3. for 循环遍历字典")

    person = {"name": "Alice", "age": 30, "city": "Beijing"}

    print("遍历键:")
    for key in person:
        print(f"  {key}")

    print("\n遍历键值对:")
    for key, value in person.items():
        print(f"  {key}: {value}")
    print()

    # ========== 4. range() 函数 ==========
    print("4. range() 函数")

    print("range(5):")
    for i in range(5):
        print(f"  {i}")

    print("\nrange(2, 6):")
    for i in range(2, 6):
        print(f"  {i}")

    print("\nrange(1, 10, 2):")
    for i in range(1, 10, 2):
        print(f"  {i}")
    print()

    # ========== 5. enumerate() - 获取索引 ==========
    print("5. enumerate() - 获取索引和值")

    fruits = ["apple", "banana", "cherry"]
    for index, fruit in enumerate(fruits, start=1):
        print(f"{index}. {fruit}")
    print()

    # ========== 6. while 循环 ==========
    print("6. while 循环")

    count = 5
    while count > 0:
        print(f"倒计时: {count}")
        count -= 1
    print("发射！🚀")
    print()

    # ========== 7. 循环控制：break ==========
    print("7. break - 跳出循环")

    for i in range(10):
        if i == 5:
            break
        print(f"  {i}")
    print("在 i=5 时跳出循环\n")

    # ========== 8. 循环控制：continue ==========
    print("8. continue - 跳过本次迭代")

    for i in range(5):
        if i == 2:
            continue
        print(f"  {i}")
    print("跳过了 i=2\n")

    # ========== 9. 循环控制：pass ==========
    print("9. pass - 占位符")

    for i in range(3):
        if i == 1:
            pass  # 什么都不做
        else:
            print(f"  {i}")
    print("i=1 时使用了 pass\n")

    # ========== 10. while-else 和 for-else ==========
    print("10. 循环的 else 子句")

    # for-else
    print("for-else 示例:")
    for i in range(5):
        print(f"  {i}")
    else:
        print("循环正常完成（未被break）")

    print("\nbreak 的情况:")
    for i in range(5):
        if i == 3:
            break
        print(f"  {i}")
    else:
        print("这不会执行")
    print("循环被break中断\n")

    # ========== 11. 嵌套循环 ==========
    print("11. 嵌套循环")

    print("乘法表（前3行）:")
    for i in range(1, 4):
        for j in range(1, 4):
            print(f"{i}x{j}={i*j:2d}", end="  ")
        print()
    print()

    # ========== 12. 列表推导式 ==========
    print("12. 列表推导式（循环的简洁写法）")

    # 传统方式
    squares1 = []
    for x in range(5):
        squares1.append(x ** 2)

    # 列表推导式
    squares2 = [x ** 2 for x in range(5)]

    print(f"传统循环: {squares1}")
    print(f"列表推导式: {squares2}")

    # 带条件的列表推导式
    evens = [x for x in range(10) if x % 2 == 0]
    print(f"偶数列表: {evens}")
    print()

    # ========== 13. zip() - 并行迭代 ==========
    print("13. zip() - 并行迭代多个序列")

    names = ["Alice", "Bob", "Charlie"]
    ages = [25, 30, 35]
    cities = ["Beijing", "Shanghai", "Guangzhou"]

    for name, age, city in zip(names, ages, cities):
        print(f"{name}, {age}岁, {city}")
    print()

    # ========== 14. reversed() 和 sorted() ==========
    print("14. reversed() 和 sorted()")

    numbers = [3, 1, 4, 1, 5, 9]

    print("原序列:", numbers)
    print("reversed:", list(reversed(numbers)))
    print("sorted:", sorted(numbers))
    print()

    # ========== 15. 无限循环 ==========
    print("15. while True 无限循环（带break）")

    count = 0
    while True:
        print(f"  迭代 {count}")
        count += 1
        if count >= 3:
            break
    print()

    # ========== 16. 最佳实践 ==========
    print("16. 循环最佳实践")
    print("✅ 推荐做法:")
    print("  1. 优先使用 for 循环（Pythonic）")
    print("  2. 使用 enumerate() 获取索引")
    print("  3. 使用 zip() 并行遍历多个序列")
    print("  4. 列表推导式用于简单转换")
    print("  5. 避免在循环中修改正在遍历的序列")
    print()
    print("❌ 避免:")
    print("  1. 在循环中修改列表长度")
    print("  2. 过度使用 while 循环")
    print("  3. 复杂的列表推导式（降低可读性）")
    print("  4. 无限循环（必须有明确的退出条件）")

    print("\n=== 演示完成 ===")


if __name__ == "__main__":
    main()
