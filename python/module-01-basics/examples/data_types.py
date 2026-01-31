# 数据类型演示

def main() -> None:
    print("=== Python 数据类型演示 ===\n")

    # 1. 整数类型
    print("1. 整数类型 (int)")
    age = 25
    count = 1000000
    negative = -42

    print(f"age = {age}, type = {type(age).__name__}")
    print(f"count = {count}, type = {type(count).__name__}")
    print(f"negative = {negative}, type = {type(negative).__name__}")

    # 大整数
    big_number = 10 ** 100
    big_str = str(big_number)
    print(f"大整数: {big_str[:20]}... (前20位)\n")

    # 2. 浮点数类型
    print("2. 浮点数类型 (float)")
    price = 99.99
    pi = 3.141592653589793
    scientific = 1.23e-5

    print(f"price = {price}, type = {type(price).__name__}")
    print(f"pi = {pi}, type = {type(pi).__name__}")
    print(f"scientific = {scientific}, type = {type(scientific).__name__}")

    # 浮点数运算
    result = 0.1 + 0.2
    print(f"0.1 + 0.2 = {result} (注意浮点精度问题)\n")

    # 3. 布尔类型
    print("3. 布尔类型 (bool)")
    is_active = True
    is_deleted = False
    comparison = 5 > 3

    print(f"is_active = {is_active}, type = {type(is_active).__name__}")
    print(f"is_deleted = {is_deleted}, type = {type(is_deleted).__name__}")
    print(f"comparison (5 > 3) = {comparison}, type = {type(comparison).__name__}")

    # 布尔运算
    print(f"True and False = {True and False}")
    print(f"True or False = {True or False}")
    print(f"not True = {not True}\n")

    # 4. 字符串类型
    print("4. 字符串类型 (str)")
    name = "Python"
    greeting = 'Hello World'

    print(f"name = {name}, type = {type(name).__name__}")
    print(f"greeting = {greeting}, type = {type(greeting).__name__}")

    # 字符串索引和切片
    text = "Hello, World!"
    print(f"原始: {text}")
    print(f"text[0] = {text[0]}")
    print(f"text[-1] = {text[-1]}")
    print(f"text[7:12] = {text[7:12]}")
    print(f"text[:5] = {text[:5]}")
    print(f"text[7:] = {text[7:]}")
    print(f"len(text) = {len(text)}")

    # 字符串方法
    print(f"text.lower() = {text.lower()}")
    print(f"text.upper() = {text.upper()}")
    print(f"text.split() = {text.split()}")
    print(f"text.replace('World', 'Python') = {text.replace('World', 'Python')}\n")

    # 多行字符串
    multiline = """这是一个
多行字符串
使用三个引号"""
    print(f"多行字符串:\n{multiline}\n")

    # 5. 列表类型
    print("5. 列表类型 (list)")
    fruits = ["apple", "banana", "cherry"]
    numbers = [1, 2, 3, 4, 5]
    mixed = [1, "hello", 3.14, True]

    print(f"fruits = {fruits}, type = {type(fruits).__name__}")
    print(f"numbers = {numbers}, type = {type(numbers).__name__}")
    print(f"mixed = {mixed}, type = {type(mixed).__name__}")

    # 列表操作
    print(f"fruits[0] = {fruits[0]}")
    print(f"fruits[-1] = {fruits[-1]}")
    print(f"fruits[1:3] = {fruits[1:3]}")
    print(f"len(fruits) = {len(fruits)}")

    fruits.append("orange")
    print(f"添加后: {fruits}")

    fruits.remove("banana")
    print(f"删除后: {fruits}")

    fruits.sort()
    print(f"排序后: {fruits}\n")

    # 6. 元组类型
    print("6. 元组类型 (tuple)")
    point = (3, 4)
    colors = ("red", "green", "blue")
    single = (42,)

    print(f"point = {point}, type = {type(point).__name__}")
    print(f"colors = {colors}, type = {type(colors).__name__}")
    print(f"single = {single}, type = {type(single).__name__}")

    # 元组解包
    x, y = point
    print(f"解包: x = {x}, y = {y}")

    r, g, b = colors
    print(f"解包: r = {r}, g = {g}, b = {b}\n")

    # 元组不可变
    print("元组是不可变的\n")

    # 7. 字典类型
    print("7. 字典类型 (dict)")
    person = {
        "name": "Alice",
        "age": 30,
        "city": "Beijing"
    }

    print(f"person = {person}, type = {type(person).__name__}")
    print(f"person['name'] = {person['name']}")
    print(f"person.get('age') = {person.get('age')}")
    print(f"person.get('country', 'N/A') = {person.get('country', 'N/A')}")

    # 字典操作
    person["email"] = "alice@example.com"
    print(f"添加 email: {person}")

    del person["city"]
    print(f"删除 city: {person}")

    print(f"keys: {list(person.keys())}")
    print(f"values: {list(person.values())}")
    print(f"items: {list(person.items())}\n")

    # 8. 集合类型
    print("8. 集合类型 (set)")
    unique_numbers = {1, 2, 3, 2, 1}
    letters = {"a", "b", "c", "b", "a"}

    print(f"unique_numbers = {unique_numbers}, type = {type(unique_numbers).__name__}")
    print(f"letters = {letters}, type = {type(letters).__name__}")

    # 集合操作
    set1 = {1, 2, 3, 4}
    set2 = {3, 4, 5, 6}

    print(f"set1 = {set1}")
    print(f"set2 = {set2}")
    print(f"union: {set1 | set2}")
    print(f"intersection: {set1 & set2}")
    print(f"difference: {set1 - set2}")
    print(f"symmetric difference: {set1 ^ set2}")

    # 集合方法
    set1.add(5)
    print(f"添加 5: {set1}")

    set1.remove(1)
    print(f"删除 1: {set1}\n")

    # 9. 类型检查
    print("9. 类型检查和 isinstance")
    value = 42

    print(f"value = {value}")
    print(f"type(value) = {type(value)}")
    print(f"isinstance(value, int) = {isinstance(value, int)}")
    print(f"isinstance(value, (int, float)) = {isinstance(value, (int, float))}")
    print(f"isinstance(value, str) = {isinstance(value, str)}")

    # None 类型
    print(f"\nNone type: type(None) = {type(None)}, isinstance(None, type(None)) = {isinstance(None, type(None))}")

    print("\n=== 演示完成 ===")

if __name__ == "__main__":
    main()
