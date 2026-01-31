# 列表 (List) 操作演示

"""
列表是Python中最常用的数据结构，可变、有序。
本文件演示列表的常用操作。
"""


def main():
    print("=== Python 列表操作演示 ===\n")

    # 1. 创建列表
    print("1. 创建列表")
    fruits = ["apple", "banana", "cherry"]
    numbers = [1, 2, 3, 4, 5]
    mixed = [1, "hello", 3.14, True]  # 混合类型

    print(f"水果列表: {fruits}")
    print(f"数字列表: {numbers}")
    print(f"混合列表: {mixed}\n")

    # 2. 访问元素
    print("2. 访问元素")
    print(f"第一个水果: {fruits[0]}")  # apple
    print(f"最后一个水果: {fruits[-1]}")  # cherry
    print(f"切片 fruits[1:3]: {fruits[1:3]}")  # ['banana', 'cherry']
    print()

    # 3. 添加元素
    print("3. 添加元素")
    fruits.append("orange")
    print(f"append('orange'): {fruits}")

    fruits.insert(1, "blueberry")
    print(f"insert(1, 'blueberry'): {fruits}")

    fruits2 = ["grape", "mango"]
    fruits.extend(fruits2)
    print(f"extend(['grape', 'mango']): {fruits}\n")

    # 4. 删除元素
    print("4. 删除元素")
    removed = fruits.pop()  # 删除最后一个
    print(f"pop() 删除了: {removed}")
    print(f"现在的列表: {fruits}")

    fruits.remove("banana")  # 删除指定值
    print(f"remove('banana'): {fruits}")

    # 清空列表
    # fruits.clear()  # 取消注释以清空
    print()

    # 5. 修改元素
    print("5. 修改元素")
    fruits[0] = "Apricot"  # 修改第一个元素
    print(f"修改后 fruits[0] = 'Apricot': {fruits}\n")

    # 6. 列表排序
    print("6. 列表排序")
    nums = [3, 1, 4, 1, 5, 9, 2, 6]
    print(f"原始列表: {nums}")

    nums.sort()  # 升序排序（原位修改）
    print(f"sort() 升序: {nums}")

    nums.sort(reverse=True)  # 降序排序
    print(f"sort(reverse=True) 降序: {nums}")

    # sorted() 返回新列表（不修改原列表）
    nums2 = [3, 1, 4, 1, 5]
    sorted_nums = sorted(nums2)
    print(f"sorted() 不修改原列表: {nums2} -> {sorted_nums}\n")

    # 7. 列表推导式 (List Comprehension)
    print("7. 列表推导式")
    squares = [x**2 for x in range(1, 6)]
    print(f"[x**2 for x in range(1, 6)]: {squares}")

    evens = [x for x in range(10) if x % 2 == 0]
    print(f"[x for x in range(10) if x % 2 == 0]: {evens}\n")

    # 8. 常用方法
    print("8. 常用方法")
    nums = [1, 2, 3, 2, 1, 2, 3, 4]
    print(f"列表: {nums}")
    print(f"len(nums): {len(nums)}")
    print(f"nums.count(2): {nums.count(2)}")  # 计数
    print(f"nums.index(3): {nums.index(3)}")  # 首次出现位置
    print(f"2 in nums: {2 in nums}")  # 成员检查
    print()

    # 9. 列表作为栈 (Stack)
    print("9. 列表作为栈 (LIFO)")
    stack = []
    stack.append("A")
    stack.append("B")
    stack.append("C")
    print(f"栈: {stack}")
    print(f"pop(): {stack.pop()}")
    print(f"pop(): {stack.pop()}")
    print(f"剩余: {stack}\n")

    # 10. 列表作为队列 (Queue)
    print("10. 列表作为队列 (FIFO - 效率较低，建议用 collections.deque)")
    from collections import deque

    queue = deque(["A", "B", "C"])
    print(f"队列: {list(queue)}")
    queue.append("D")  # 入队
    print(f"入队 D: {list(queue)}")
    print(f"出队: {queue.popleft()}")  # 出队
    print(f"剩余: {list(queue)}\n")

    # 11. 列表复制
    print("11. 列表复制（注意浅拷贝问题）")
    original = [1, 2, [3, 4]]
    shallow = original.copy()  # 浅拷贝
    # shallow = original[:]  # 另一种浅拷贝方式
    print(f"原始: {original}")
    print(f"浅拷贝: {shallow}")

    # 修改嵌套列表
    shallow[2][0] = 99
    print(f"修改嵌套后 - 原始: {original}")
    print(f"修改嵌套后 - 浅拷贝: {shallow}")
    print("注意：浅拷贝会共享嵌套对象！\n")

    # 12. 多维列表
    print("12. 多维列表")
    matrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ]
    print(f"矩阵: {matrix}")
    print(f"第二行: {matrix[1]}")
    print(f"matrix[1][2]: {matrix[1][2]}")

    # 矩阵转置
    transposed = [[row[i] for row in matrix] for i in range(3)]
    print(f"转置: {transposed}\n")

    # 13. 列表性能注意事项
    print("13. 列表性能注意事项")
    print("操作 | 时间复杂度")
    print("-" * 30)
    print("append()  | O(1) - 快")
    print("pop()     | O(1) - 快")
    print("pop(0)    | O(n) - 慢（需要移动所有元素）")
    print("insert()  | O(n) - 慢")
    print("index()   | O(n)")
    print("count()   | O(n)")
    print("in        | O(n)")
    print()
    print("建议：频繁在头部操作时使用 collections.deque\n")

    print("=== 演示完成 ===")


if __name__ == "__main__":
    main()
