"""
练习 2：计算列表平均值 - 参考解答
"""

def average(numbers):
    """
    计算数字列表的平均值
    
    参数:
        numbers: 数字列表
    
    返回值:
        float: 平均值，空列表返回 0.0
    """
    if not numbers:
        return 0.0
    return sum(numbers) / len(numbers)


if __name__ == "__main__":
    # 测试用例
    print("测试 average 函数:\n")
    
    test_cases = [
        ([1, 2, 3, 4, 5], 3.0),
        ([10, 20, 30], 20.0),
        ([], 0.0),
    ]
    
    for numbers, expected in test_cases:
        result = average(numbers)
        print(f"{numbers} 的平均值: {result} (期望: {expected})")
