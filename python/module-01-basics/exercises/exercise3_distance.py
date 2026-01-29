"""
练习 3：元组解构

请实现 distance 函数
"""

import math

def distance(point1, point2):
    """
    计算两点之间的距离
    
    参数:
        point1: 元组 (x, y)
        point2: 元组 (x2, y2)
    
    返回值:
        float: 距离
    距离公式: √((x2-x)² + (y2-y)²)
    """
    # TODO: 实现距离计算
    raise NotImplementedError("请实现 distance 函数")


if __name__ == "__main__":
    # 测试用例
    print("测试 distance 函数:\n")
    
    test_cases = [
        ((0, 0), (3, 4), 5.0),
        ((1, 1), (1, 1), 0.0),
        ((0, 0), (0, 5), 5.0),
    ]
    
    for p1, p2, expected in test_cases:
        result = distance(p1, p2)
        print(f"{p1} 到 {p2} 的距离: {result:.2f} (期望: {expected:.2f})")
