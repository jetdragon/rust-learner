"""
练习 6：BMI 计算器 - 参考解答
"""

def calculate_bmi(weight_kg, height_m):
    """
    计算 BMI（身体质量指数）
    
    参数:
        weight_kg: 体重（千克）
        height_m: 身高（米）
    
    返回值:
        float: BMI 指数（保留 2 位小数）
    
    BMI 公式: weight (kg) / height² (m)
    """
    if height_m <= 0:
        raise ValueError("身高必须大于 0")
    return round(weight_kg / (height_m ** 2), 2)


def bmi_category(bmi):
    """
    根据 BMI 值返回分类
    
    参数:
        bmi: BMI 指数
    
    返回值:
        str: 分类字符串
    """
    if bmi < 18.5:
        return "体重过轻"
    elif bmi < 25.0:
        return "正常"
    elif bmi < 30.0:
        return "超重"
    else:
        return "肥胖"


def health_report(weight_kg, height_m):
    """
    生成完整的健康报告
    
    参数:
        weight_kg: 体重（千克）
        height_m: 身高（米）
    
    返回值:
        str: 完整的健康报告字符串
    """
    bmi = calculate_bmi(weight_kg, height_m)
    category = bmi_category(bmi)
    
    report = f"""
BMI 健康报告
{'='* 50}
体重: {weight_kg} kg
身高: {height_m} m
BMI: {bmi} ({category})
{'='* 50}
"""
    return report


if __name__ == "__main__":
    # 测试用例
    print("测试 BMI 计算器函数:\n")
    
    print("测试 calculate_bmi:")
    assert calculate_bmi(70, 1.75) == 22.86
    print(f"calculate_bmi(70, 1.75) = {calculate_bmi(70, 1.75)}")
    
    print("\n测试 bmi_category:")
    print(f"bmi_category(22.86) = {bmi_category(22.86)}")
    print(f"bmi_category(17.0) = {bmi_category(17.0)}")
    print(f"bmi_category(28.0) = {bmi_category(28.0)}")
    print(f"bmi_category(32.0) = {bmi_category(32.0)}")
    
    print("\n测试 health_report:")
    report = health_report(70, 1.75)
    print(report)
    
    # 验证报告包含关键信息
    assert "22.86" in report
    assert "正常" in report
    assert "70" in report
    assert "1.75" in report
