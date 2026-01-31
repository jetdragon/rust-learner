# 条件语句演示

"""
本文件演示Python的条件语句：if-elif-else和match-case（Python 3.10+）
"""


def main() -> None:
    print("=== Python 条件语句演示 ===\n")

    # ========== 1. if 语句 ==========
    print("1. 基本 if 语句")

    age = 18
    if age >= 18:
        print("你已经成年了")
    print()

    # ========== 2. if-else 语句 ==========
    print("2. if-else 语句")

    temperature = 25
    if temperature > 30:
        print("天气很热")
    else:
        print("天气还不错")
    print()

    # ========== 3. if-elif-else 语句 ==========
    print("3. if-elif-else 语句")

    score = 85
    if score >= 90:
        grade = "A"
    elif score >= 80:
        grade = "B"
    elif score >= 70:
        grade = "C"
    elif score >= 60:
        grade = "D"
    else:
        grade = "F"

    print(f"分数 {score} 对应等级: {grade}")
    print()

    # ========== 4. 嵌套条件 ==========
    print("4. 嵌套条件语句")

    age = 25
    has_license = True

    if age >= 18:
        if has_license:
            print("可以开车")
        else:
            print("需要先考驾照")
    else:
        print("还未成年")
    print()

    # ========== 5. 条件表达式（三元运算符）==========
    print("5. 条件表达式")

    age = 20
    # 传统的 if-else
    if age >= 18:
        status1 = "成年"
    else:
        status1 = "未成年"

    # 条件表达式（更简洁）
    status2 = "成年" if age >= 18 else "未成年"

    print(f"status1: {status1}")
    print(f"status2: {status2}")
    print()

    # ========== 6. 逻辑运算符 ==========
    print("6. 逻辑运算符")

    age = 25
    has_id = True
    has_ticket = True

    # and - 所有条件都为真
    if age >= 18 and has_id:
        print("可以进入场所")

    # or - 任一条件为真
    if has_ticket or has_id:
        print("有有效凭证")

    # not - 取反
    if not age < 18:
        print("不是未成年")
    print()

    # ========== 7. match-case 语句（Python 3.10+）==========
    print("7. match-case 模式匹配（Python 3.10+）")

    day = "星期一"

    match day:
        case "星期一" | "星期二" | "星期三" | "星期四" | "星期五":
            print("工作日")
        case "星期六" | "星期日":
            print("周末")
        case _:
            print("其他")

    # 带模式的 match
    print("\n带模式的 match:")

    point = (3, 4)

    match point:
        case (0, 0):
            print("原点")
        case (0, y):
            print(f"在Y轴上: {y}")
        case (x, 0):
            print(f"在X轴上: {x}")
        case (x, y) if x == y:
            print(f"对角线: ({x}, {y})")
        case (x, y):
            print(f"点: ({x}, {y})")
    print()

    # ========== 8. 真值测试 ==========
    print("8. 真值测试（Truthiness）")

    # 在Python中，以下值被视为False：
    false_values = [
        False,
        None,
        0,
        0.0,
        "",
        [],
        {},
        set()
    ]

    print("假值示例:")
    for value in false_values:
        print(f"  {repr(value)}: {bool(value)}")

    # 其他值都是True
    print("\n真值示例:")
    true_values = [True, 1, -1, 0.1, "hello", [1], {"a": 1}]
    for value in true_values:
        print(f"  {repr(value)}: {bool(value)}")
    print()

    # ========== 9. 短路求值 ==========
    print("9. 短路求值")

    def expensive_function() -> bool:
        print("  执行耗时函数...")
        return True

    # and 短路：第一个为False，第二个不执行
    print("and 短路:")
    if False and expensive_function():
        pass
    print("  第一个为False，第二个函数未执行")

    # or 短路：第一个为True，第二个不执行
    print("\nor 短路:")
    if True or expensive_function():
        pass
    print("  第一个为True，第二个函数未执行")
    print()

    # ========== 10. 最佳实践 ==========
    print("10. 条件语句最佳实践")
    print("✅ 推荐做法:")
    print("  1. 使用 is None 而非 == None")
    print("  2. 使用 if x 而非 if x == True")
    print("  3. 先处理常见/简单情况")
    print("  4. 使用有意义的变量名")
    print("  5. 避免过深的嵌套")
    print()
    print("❌ 避免:")
    print("  1. if x == True: → 应该用 if x:")
    print("  2. if x != None: → 应该用 if x is not None:")
    print("  3. 过多的嵌套层级")
    print("  4. 复杂的条件表达式")

    print("\n=== 演示完成 ===")


if __name__ == "__main__":
    main()
