"""
练习 4：列表操作 - 参考解答
"""

def process_list(numbers):
    """
    处理数字列表，返回排序后的列表、最大值、最小值、和
    
    参数:
        numbers: 数字列表
    
    返回值:
        tuple: (sorted_list, max, min, sum)
            - sorted_list: 排序后的列表（升序）
            - max: 最大值
            - min: 最小值（空列表返回 None）
            - sum: 所有元素的和
    """
    if not numbers:
        return ([], None, None, 0)
    
    sorted_list = sorted(numbers)
    return (sorted_list, max(sorted_list), min(sorted_list), sum(sorted_list))


if __name__ == "__main__":
    # 测试用例
    print("测试 process_list 函数:\n")
    
    test_cases = [
        ([3, 1, 4, 1, 5, 9, 2, 6, 5], (sorted, max, min, sum)),
        ([], (list, None, None, 0)),
    ]
    
    for numbers, expected_type in test_cases:
        print(f"\n输入: {numbers}")
        result = process_list(numbers)
        print(f"结果: {result}")
