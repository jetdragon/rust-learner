# 练习题

本模块包含以下练习题，帮助你巩固 Python 基础知识。

## 练习 1：温度转换

**难度**：简单

**描述**：

编写一个函数，将摄氏温度转换为华氏温度。

**要求**：

- 函数名：`celsius_to_fahrenheit`
- 参数：一个 `float` 或 `int` 类型的摄氏温度
- 返回值：对应的华氏温度（`float`）
- 转换公式：`F = C × 9/5 + 32`

**验证**：

```python
assert celsius_to_fahrenheit(0) == 32
assert celsius_to_fahrenheit(100) == 212
assert abs(celsius_to_fahrenheit(37) - 98.6) < 0.1
```

---

## 练习 2：计算列表平均值

**难度**：简单

**描述**：

编写一个函数，计算数字列表的平均值。

**要求**：

- 函数名：`average`
- 参数：一个 `list` 类型的数字列表
- 返回值：平均值（`float`）
- 处理空列表时返回 `0.0`

**验证**：

```python
assert average([1, 2, 3, 4, 5]) == 3.0
assert average([10, 20, 30]) == 20.0
assert average([]) == 0.0
assert abs(average([1, 2]) - 1.5) < 0.01
```

---

## 练习 3：元组解构

**难度**：中等

**描述**：

编写一个函数，接收一个二维坐标元组，返回两点之间的距离。

**要求**：

- 函数名：`distance`
- 参数：两个坐标元组 `(x, y)` 和 `(x2, y2)`
- 返回值：距离（`float`）
- 距离公式：`√((x2-x)² + (y2-y)²)`

**验证**：

```python
assert abs(distance((0, 0), (3, 4)) - 5.0) < 0.01
assert abs(distance((1, 1), (1, 1)) - 0.0) < 0.01
assert abs(distance((0, 0), (0, 5)) - 5.0) < 0.01
```

---

## 练习 4：列表操作

**难度**：中等

**描述**：

实现一个列表处理函数，执行多个操作。

**要求**：

- 函数名：`process_list`
- 参数：一个数字列表
- 返回值：包含以下结果的元组 `(sorted_list, max, min, sum)`：
  1. 排序后的列表（升序）
  2. 最大值
  3. 最小值
  4. 所有元素的和

**验证**：

```python
result = process_list([3, 1, 4, 1, 5, 9, 2, 6, 5])
assert result[0] == [1, 1, 2, 3, 4, 5, 5, 6, 9]
assert result[1] == 9
assert result[2] == 1
assert result[3] == 36
```

---

## 练习 5：字典操作

**难度**：中等

**描述**：

编写一个函数来统计字符串中每个字符出现的次数。

**要求**：

- 函数名：`count_characters`
- 参数：一个字符串
- 返回值：字典，键为字符，值为出现次数

**验证**：

```python
result = count_characters("hello")
assert result['h'] == 1
assert result['e'] == 1
assert result['l'] == 2
assert result['o'] == 1
```

---

## 练习 6：BMI 计算器

**难度**：中等

**描述**：

创建一个 BMI（身体质量指数）计算器函数。

**要求**：

1. 创建函数 `calculate_bmi(weight_kg, height_m)`
   - BMI = weight (kg) / height² (m)
   - 返回 BMI 值（保留2位小数）

2. 创建函数 `bmi_category(bmi)`
   - 根据BMI值返回分类（字符串）：
   - `< 18.5`: "体重过轻"
   - `18.5 - 24.9`: "正常"
   - `25.0 - 29.9`: "超重"
   - `>= 30.0`: "肥胖"

3. 创建函数 `health_report(weight_kg, height_m)`
   - 计算BMI并获取分类
   - 返回完整的健康报告字符串

**验证**：

```python
assert calculate_bmi(70, 1.75) == 22.86
assert bmi_category(22.86) == "正常"
assert bmi_category(17.0) == "体重过轻"
assert bmi_category(28.0) == "超重"
assert bmi_category(32.0) == "肥胖"

report = health_report(70, 1.75)
assert "22.86" in report
assert "正常" in report
```

---

## 综合练习

查看 [综合练习.md](综合练习.md) 进行综合项目实践。

---

## 如何使用练习

1. 打开练习文件（如 `exercise1_temperature.py`）
2. 查看题目要求
3. 实现函数，替换 `pass` 或 `raise NotImplementedError`
4. 运行测试验证实现

查看 `solutions/` 目录获取参考答案。

---

## 难度级别

| 难度 | 练习 |
|--------|--------|
| 简单 | 练习 1, 练习 2 |
| 中等 | 练习 3, 练习 4, 练习 5, 练习 6 |
