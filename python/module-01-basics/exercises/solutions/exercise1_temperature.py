"""
练习 1：温度转换 - 参考解答
"""

def celsius_to_fahrenheit(celsius):
    """
    将摄氏温度转换为华氏温度
    
    参数:
        celsius: 摄氏温度（float 或 int）
    
    返回值:
        float: 华氏温度
    
    转换公式: F = C × 9/5 + 32
    """
    return celsius * 9 / 5 + 32


if __name__ == "__main__":
    # 测试用例
    print("测试 celsius_to_fahrenheit 函数:\n")
    
    test_cases = [
        (0, 32),
        (100, 212),
        (37, 98.6),
        (-40, -40)
    ]
    
    for celsius, expected_f in test_cases:
        result = celsius_to_fahrenheit(celsius)
        print(f"{celsius}°C -> {result:.2f}°F (期望: {expected_f:.2f}°F)")
