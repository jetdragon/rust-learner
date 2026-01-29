"""
练习 6：BMI 计算器

请实现以下三个函数：
1. calculate_bmi(weight_kg, height_m)
2. bmi_category(bmi)
3. health_report(weight_kg, height_m)
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
    # TODO: 实现 BMI 计算
    raise NotImplementedError("请实现 calculate_bmi 函数")


def bmi_category(bmi):
    """
    根据 BMI 值返回分类
    
    参数:
        bmi: BMI 指数
    
    返回值:
        str: 分类字符串
    """
    # TODO: 实现 BMI 分类
    raise NotImplementedError("请实现 bmi_category 函数")


def health_report(weight_kg, height_m):
    """
    生成完整的健康报告
    
    参数:
        weight_kg: 体重（千克）
        height_m: 身高（米）
    
    返回值:
        str: 完整的健康报告字符串
    """
    # TODO: 实现健康报告生成
    raise NotImplementedError("请实现 health_report 函数")


if __name__ == "__main__":
    # 测试用例
    print("测试 BMI 计算器函数:\n")
    
    print("测试 calculate_bmi:")
    assert calculate_bmi(70, 1.75) == pytest.approx(22.86, rel=0.01)
    print(f"calculate_bmi(70, 1.75) = {calculate_bmi(70, 1.75):.2f}")
    
    print("\n测试 bmi_category:")
    print(f"bmi_category(22.86) = {bmi_category(22.86)}")
    print(f"bmi_category(17.0) = {bmi_category(17.0)}")
    print(f"bmi_category(28.0) = {bmi_category(28.0)}")
    print(f"bmi_category(32.0) = {bmi_category(32.0)}")
    
    print("\n测试 health_report:")
    report = health_report(70, 1.75)
    print(report)
