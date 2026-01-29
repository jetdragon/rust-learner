import pytest
import sys
import os

# 添加父目录到路径以导入 exercises
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

from exercises.exercise1_temperature import celsius_to_fahrenheit
from exercises.exercise2_average import average
from exercises.exercise3_distance import distance


def test_exercise1_temperature():
    """测试练习 1：温度转换"""
    assert celsius_to_fahrenheit(0) == 32
    assert celsius_to_fahrenheit(100) == 212
    assert abs(celsius_to_fahrenheit(37) - 98.6) < 0.1
    assert abs(celsius_to_fahrenheit(-40) - 40) < 0.1


def test_exercise2_average():
    """测试练习 2：计算列表平均值"""
    assert average([1, 2, 3, 4, 5]) == 3.0
    assert average([10, 20, 30]) == 20.0
    assert average([]) == 0.0
    assert abs(average([1, 2]) - 1.5) < 0.01


def test_exercise3_distance():
    """测试练习 3：元组解构"""
    assert abs(distance((0, 0), (3, 4)) - 5.0) < 0.01
    assert abs(distance((1, 1), (1, 1)) - 0.0) < 0.01
    assert abs(distance((0, 0), (0, 5)) - 5.0) < 0.01
    assert abs(distance((0, 0), (12, 5)) - 13.0) < 0.01


class TestExercise4ListOps:
    """测试练习 4：列表操作"""
    
    def test_sort_and_stats(self):
        from exercises.exercise4_list_ops import process_list
        result = process_list([3, 1, 4, 1, 5, 9, 2, 6, 5])
        assert result[0] == [1, 1, 2, 3, 4, 5, 5, 6, 9]
        assert result[1] == 9
        assert result[2] == 1
        assert result[3] == 36
    
    def test_empty_list(self):
        from exercises.exercise4_list_ops import process_list
        result = process_list([])
        assert result[0] == []
        assert result[1] is None  # 空列表的最大值
        assert result[2] is None  # 空列表的最小值
        assert result[3] == 0


class TestExercise5DictOps:
    """测试练习 5：字典操作"""
    
    def test_count_characters(self):
        from exercises.exercise5_dict_ops import count_characters
        result = count_characters("hello")
        assert result['h'] == 1
        assert result['e'] == 1
        assert result['l'] == 2
        assert result['o'] == 1
    
    def test_count_all_unique(self):
        from exercises.exercise5_dict_ops import count_characters
        result = count_characters("abc")
        assert result['a'] == 1
        assert result['b'] == 1
        assert result['c'] == 1


class TestExercise6BMI:
    """测试练习 6：BMI 计算器"""
    
    def test_calculate_bmi(self):
        from exercises.exercise6_bmi import calculate_bmi
        assert calculate_bmi(70, 1.75) == 22.86
        assert abs(calculate_bmi(60, 1.70) - 20.76) < 0.01
    
    def test_bmi_category(self):
        from exercises.exercise6_bmi import bmi_category
        assert bmi_category(22.86) == "正常"
        assert bmi_category(17.0) == "体重过轻"
        assert bmi_category(28.0) == "超重"
        assert bmi_category(32.0) == "肥胖"
    
    def test_health_report(self):
        from exercises.exercise6_bmi import health_report
        report = health_report(70, 1.75)
        assert "22.86" in report
        assert "正常" in report


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
