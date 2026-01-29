"""
练习 4：列表操作

请实现 process_list 函数
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
    # TODO: 实现列表处理
    raise NotImplementedError("请实现 process_list 函数")


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
