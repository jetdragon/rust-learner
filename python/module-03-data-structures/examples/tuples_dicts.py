# 元组和字典 (Tuple and Dict) 演示

"""
元组是不可变的有序序列，字典是键值对存储。
本文件演示元组和字典的常用操作。
"""


def main():
    print("=== Python 元组和字典演示 ===\n")

    # ========== 元组 (Tuple) ==========
    print("【元组 Tuple】\n")

    # 1. 创建元组
    print("1. 创建元组")
    point = (3, 4)
    coordinates = (1, 2, 3, 4, 5)
    single = (42,)  # 注意：单个元素需要逗号
    empty = ()

    print(f"点: {point}")
    print(f"坐标: {coordinates}")
    print(f"单个元素: {single} (type: {type(single).__name__})")
    print(f"空元组: {empty}\n")

    # 2. 元组解包 (Tuple Unpacking)
    print("2. 元组解包")
    x, y = point
    print(f"point = {point}")
    print(f"x = {x}, y = {y}")

    # 交换变量
    a, b = 10, 20
    print(f"交换前: a={a}, b={b}")
    a, b = b, a
    print(f"交换后: a={a}, b={b}\n")

    # 3. 访问元素
    print("3. 访问元素（类似列表）")
    print(f"coordinates[0]: {coordinates[0]}")
    print(f"coordinates[1:3]: {coordinates[1:3]}")
    print(f"coordinates[-1]: {coordinates[-1]}\n")

    # 4. 元组不可变性
    print("4. 元组不可变性")
    print("尝试修改元组...")
    try:
        point[0] = 5  # 这会报错
    except TypeError as e:
        print(f"❌ TypeError: {e}")
    print("元组创建后不能修改（不可变）\n")

    # 5. 命名元组 (Named Tuple)
    print("5. 命名元组")
    from collections import namedtuple

    Person = namedtuple("Person", ["name", "age", "city"])
    alice = Person("Alice", 30, "Beijing")
    print(f"Person: {alice}")
    print(f"姓名: {alice.name}")
    print(f"年龄: {alice.age}")
    print()

    # ========== 字典 (Dict) ==========
    print("【字典 Dictionary】\n")

    # 6. 创建字典
    print("6. 创建字典")
    person = {
        "name": "Alice",
        "age": 30,
        "city": "Beijing"
    }
    print(f"person = {person}\n")

    # 7. 访问元素
    print("7. 访问元素")
    print(f"person['name']: {person['name']}")

    # 使用 get() 方法（更安全）
    print(f"person.get('age'): {person.get('age')}")
    print(f"person.get('email', 'N/A'): {person.get('email', 'N/A')}")  # 不存在时返回默认值
    print()

    # 8. 添加和修改
    print("8. 添加和修改元素")
    person["email"] = "alice@example.com"  # 添加
    print(f"添加 email: {person}")

    person["age"] = 31  # 修改
    print(f"修改 age: {person}\n")

    # 9. 删除元素
    print("9. 删除元素")
    removed = person.pop("city")
    print(f"pop('city') 删除了: {removed}")
    print(f"现在的字典: {person}")

    # del person["email"]  # 另一种删除方式
    # person.clear()  # 清空字典
    print()

    # 10. 字典方法
    print("10. 常用字典方法")
    scores = {"Alice": 95, "Bob": 87, "Charlie": 92}
    print(f"字典: {scores}")
    print(f"scores.keys(): {list(scores.keys())}")
    print(f"scores.values(): {list(scores.values())}")
    print(f"scores.items(): {list(scores.items())}")
    print(f"'Bob' in scores: {'Bob' in scores}")
    print()

    # 11. 遍历字典
    print("11. 遍历字典")
    print("方式1: 遍历键")
    for key in scores:
        print(f"  {key}: {scores[key]}")

    print("\n方式2: 遍历键值对")
    for key, value in scores.items():
        print(f"  {key}: {value}")

    print("\n方式3: 只遍历值")
    for value in scores.values():
        print(f"  分数: {value}")
    print()

    # 12. 字典推导式
    print("12. 字典推导式")
    squares = {x: x**2 for x in range(1, 6)}
    print(f"{x: x**2 for x in range(1, 6)}: {squares}")

    # 过滤
    even_squares = {k: v for k, v in squares.items() if k % 2 == 0}
    print(f"偶数的平方: {even_squares}\n")

    # 13. 嵌套字典
    print("13. 嵌套字典")
    employees = {
        "001": {"name": "Alice", "dept": "IT"},
        "002": {"name": "Bob", "dept": "HR"}
    }
    print(f"employees['001']['name']: {employees['001']['name']}")
    print()

    # 14. 字典合并 (Python 3.9+)
    print("14. 字典合并")
    dict1 = {"a": 1, "b": 2}
    dict2 = {"c": 3, "d": 4}

    # 方法1: 使用 ** 运算符
    merged1 = {**dict1, **dict2}
    print(f"{{**dict1, **dict2}}: {merged1}")

    # 方法2: 使用 | 运算符 (Python 3.9+)
    merged2 = dict1 | dict2
    print(f"dict1 | dict2: {merged2}")

    # 方法3: update() 方法
    dict3 = dict1.copy()
    dict3.update(dict2)
    print(f"dict1.copy().update(dict2): {dict3}\n")

    # 15. defaultdict (默认值字典)
    print("15. defaultdict - 自动设置默认值")
    from collections import defaultdict

    # 普通字典需要检查键是否存在
    normal_dict = {}
    normal_dict["a"] = normal_dict.get("a", 0) + 1
    print(f"普通字典累加: {normal_dict}")

    # defaultdict 自动设置默认值
    dd = defaultdict(int)  # int() 默认返回 0
    dd["a"] += 1
    dd["b"] += 1
    dd["a"] += 1
    print(f"defaultdict: {dict(dd)}")  # {'a': 2, 'b': 1}
    print()

    # 16. OrderedDict (有序字典)
    print("16. OrderedDict - 保持插入顺序")
    from collections import OrderedDict

    od = OrderedDict()
    od["first"] = 1
    od["second"] = 2
    od["third"] = 3
    print(f"OrderedDict: {list(od.keys())}")  # ['first', 'second', 'third']
    print()

    # 17. 字典性能
    print("17. 字典性能")
    print("操作 | 时间复杂度")
    print("-" * 30)
    print("get/[]   | O(1) 平均 - 快")
    print("set/[]   | O(1) 平均 - 快")
    print("delete   | O(1) 平均 - 快")
    print("in       | O(1) 平均 - 快")
    print("遍历     | O(n)")
    print()
    print("字典是Python中最强大的数据结构之一，提供O(1)的查找性能！\n")

    print("=== 演示完成 ===")


if __name__ == "__main__":
    main()
