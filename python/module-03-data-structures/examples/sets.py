# 集合 (Set) 操作演示

"""
集合是Python中用于存储唯一值的数据结构，支持数学集合运算。
本文件演示集合的常用操作。
"""


def main():
    print("=== Python 集合 (Set) 演示 ===\n")

    # 1. 创建集合
    print("1. 创建集合")
    # 空集合必须用 set()，{} 是空字典
    empty_set = set()
    print(f"空集合: {empty_set} (type: {type(empty_set).__name__})")

    # 从列表创建（自动去重）
    numbers = [1, 2, 2, 3, 3, 3, 4]
    unique_numbers = set(numbers)
    print(f"列表 {numbers} -> 集合 {unique_numbers}")

    # 直接创建
    fruits = {"apple", "banana", "cherry", "apple"}  # 重复的自动去除
    print(f"水果集合: {fruits}")
    print()

    # 2. 添加和删除元素
    print("2. 添加和删除元素")
    colors = {"red", "green", "blue"}
    print(f"初始: {colors}")

    colors.add("yellow")  # 添加单个元素
    print(f"add('yellow'): {colors}")

    colors.update(["purple", "orange"])  # 添加多个元素
    print(f"update(['purple', 'orange']): {colors}")

    colors.remove("red")  # 删除元素（不存在会报错）
    print(f"remove('red'): {colors}")

    colors.discard("green")  # 删除元素（不存在不报错）
    print(f"discard('green'): {colors}")

    popped = colors.pop()  # 随机删除并返回
    print(f"pop() 删除了: {popped}")
    print(f"剩余: {colors}\n")

    # 3. 集合运算（数学运算）
    print("3. 集合运算")
    set_a = {1, 2, 3, 4, 5}
    set_b = {4, 5, 6, 7, 8}

    print(f"集合A: {set_a}")
    print(f"集合B: {set_b}\n")

    # 并集 (Union) - 所有元素
    print("并集 (A ∪ B) - 所有元素")
    union_ab = set_a | set_b  # 或 set_a.union(set_b)
    print(f"A | B: {union_ab}\n")

    # 交集 (Intersection) - 共同元素
    print("交集 (A ∩ B) - 共同元素")
    inter_ab = set_a & set_b  # 或 set_a.intersection(set_b)
    print(f"A & B: {inter_ab}\n")

    # 差集 (Difference) - A中有B中没有的
    print("差集 (A - B) - A中有B中没有的")
    diff_ab = set_a - set_b  # 或 set_a.difference(set_b)
    print(f"A - B: {diff_ab}\n")

    # 对称差集 (Symmetric Difference) - 不在交集中的元素
    print("对称差集 (A △ B) - 不在交集中的元素")
    sym_diff = set_a ^ set_b  # 或 set_a.symmetric_difference(set_b)
    print(f"A ^ B: {sym_diff}\n")

    # 4. 集合关系判断
    print("4. 集合关系判断")
    set1 = {1, 2, 3}
    set2 = {1, 2, 3, 4, 5}
    set3 = {1, 2, 3}

    print(f"set1: {set1}")
    print(f"set2: {set2}")
    print(f"set3: {set3}\n")

    print(f"set1.issubset(set2): {set1.issubset(set2)}")  # set1 ⊆ set2
    print(f"set2.issuperset(set1): {set2.issuperset(set1)}")  # set2 ⊇ set1
    print(f"set1 == set3: {set1 == set3}")  # 相等
    print(f"set1.isdisjoint(set2): {set1.isdisjoint({4, 5})}")  # 是否不相交
    print()

    # 5. 集合推导式
    print("5. 集合推导式")
    squares = {x**2 for x in range(5)}
    print(f"{{x**2 for x in range(5)}}: {squares}")

    evens = {x for x in range(10) if x % 2 == 0}
    print(f"偶数集合: {evens}\n")

    # 6. 冻结集合 (frozenset) - 不可变集合
    print("6. 冻结集合 (frozenset)")
    fs = frozenset([1, 2, 3, 2, 1])
    print(f"frozenset: {fs}")
    print("尝试修改冻结集合...")
    try:
        fs.add(4)  # 这会报错
    except AttributeError as e:
        print(f"❌ AttributeError: {e}")
    print("frozenset 创建后不能修改\n")

    # 7. 实际应用示例
    print("7. 实际应用示例")

    # 示例1: 去重
    print("示例1: 列表去重")
    words = ["apple", "banana", "apple", "cherry", "banana"]
    unique_words = list(set(words))
    print(f"原始: {words}")
    print(f"去重: {unique_words}\n")

    # 示例2: 查找重复元素
    print("示例2: 查找列表中的重复元素")
    from collections import Counter

    data = [1, 2, 3, 2, 4, 5, 3, 1, 2]
    counter = Counter(data)
    duplicates = {item for item, count in counter.items() if count > 1}
    print(f"列表: {data}")
    print(f"重复的元素: {duplicates}\n")

    # 示例3: 权限管理
    print("示例3: 权限管理")
    admin_permissions = {"read", "write", "delete", "admin"}
    user_permissions = {"read", "write"}

    print(f"管理员权限: {admin_permissions}")
    print(f"普通用户权限: {user_permissions}")
    print(f"用户有管理员权限吗: {user_permissions.issuperset(admin_permissions)}")
    print(f"用户缺少的权限: {admin_permissions - user_permissions}\n")

    # 示例4: 标签系统
    print("示例4: 文章标签系统")
    article1_tags = {"python", "programming", "tutorial"}
    article2_tags = {"python", "data-science"}
    article3_tags = {"javascript", "web", "tutorial"}

    all_tags = article1_tags | article2_tags | article3_tags
    common_tags = article1_tags & article2_tags
    print(f"所有标签: {all_tags}")
    print(f"文章1和2的共同标签: {common_tags}\n")

    # 8. 集合性能
    print("8. 集合性能")
    print("操作 | 时间复杂度")
    print("-" * 30)
    print("add/removediscard | O(1) 平均")
    print("in (成员检查)    | O(1) 平均 - 比列表快很多！")
    print("union/intersect  | O(len(s) + len(t))")
    print()
    print("为什么集合的 'in' 比列表快？")
    print("- 列表: O(n) - 需要遍历整个列表")
    print("- 集合: O(1) - 基于哈希表，直接计算位置")
    print()

    # 性能对比演示
    import time

    large_list = list(range(100000))
    large_set = set(range(100000))

    start = time.time()
    _ = 99999 in large_list
    list_time = time.time() - start

    start = time.time()
    _ = 99999 in large_set
    set_time = time.time() - start

    print(f"列表查找耗时: {list_time:.6f}秒")
    print(f"集合查找耗时: {set_time:.6f}秒")
    print(f"集合快了 {list_time/set_time:.1f} 倍！\n")

    # 9. 集合的限制
    print("9. 集合的限制")
    print("集合中的元素必须是可哈希的（不可变）:")
    print("✅ 可以: int, float, str, tuple (不可变)")
    print("❌ 不可以: list, dict, set (可变)")
    print()

    valid_set = {1, 2.5, "hello", (1, 2)}  # tuple 可以
    print(f"有效集合: {valid_set}")

    try:
        invalid_set = {[1, 2], {3, 4}}  # list 和 set 不可以
    except TypeError as e:
        print(f"❌ TypeError: {e}")
    print()

    print("=== 演示完成 ===")
    print()
    print("💡 集合的最佳使用场景:")
    print("  1. 需要去重时")
    print("  2. 需要快速成员检查时 (in 操作)")
    print("  3. 需要数学集合运算时（并集、交集等）")
    print("  4. 需要消除重复项时")


if __name__ == "__main__":
    main()
