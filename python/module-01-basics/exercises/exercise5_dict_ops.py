"""
练习 5：字典操作

请实现 count_characters 函数
"""

def count_characters(text):
    """
    统计字符串中每个字符出现的次数
    
    参数:
        text: 字符串
    
    返回值:
        dict: 键为字符，值为出现次数
    """
    # TODO: 实现字符统计
    raise NotImplementedError("请实现 count_characters 函数")


if __name__ == "__main__":
    # 测试用例
    print("测试 count_characters 函数:\n")
    
    test_cases = [
        ("hello", {'h': 1, 'e': 1, 'l': 2, 'o': 1}),
        ("abc", {'a': 1, 'b': 1, 'c': 1}),
        ("programming", {'p': 2, 'r': 2, 'o': 2, 'g': 2, 'a': 1, 'm': 1, 'i': 1, 'n': 1}),
    ]
    
    for text, expected in test_cases:
        result = count_characters(text)
        print(f"\n输入: '{text}'")
        print(f"结果: {result}")
        print(f"期望: {expected}")
