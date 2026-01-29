# 变量和动态类型演示

def main():
    print("=== 变量和动态类型演示 ===\n")

    # 1. 基本变量声明
    print("1. 基本变量声明")
    x = 5
    y = "Hello"
    z = 3.14
    is_true = True

    print(f"x = {x}, type = {type(x).__name__}")
    print(f"y = {y}, type = {type(y).__name__}")
    print(f"z = {z}, type = {type(z).__name__}")
    print(f"is_true = {is_true}, type = {type(is_true).__name__}\n")

    # 2. 动态类型演示
    print("2. 动态类型演示")
    dynamic_var = 42
    print(f"初始: dynamic_var = {dynamic_var}, type = {type(dynamic_var).__name__}")

    dynamic_var = "现在是字符串"
    print(f"修改后: dynamic_var = {dynamic_var}, type = {type(dynamic_var).__name__}")

    dynamic_var = [1, 2, 3]
    print(f"再修改: dynamic_var = {dynamic_var}, type = {type(dynamic_var).__name__}\n")

    # 3. 类型转换
    print("3. 类型转换")
    num_str = "100"
    num = int(num_str)
    print(f"字符串 '{num_str}' 转为整数: {num}, type = {type(num).__name__}")

    pi_str = "3.14159"
    pi = float(pi_str)
    print(f"字符串 '{pi_str}' 转为浮点: {pi}, type = {type(pi).__name__}")

    # 字符串拼接
    age = 25
    message = "我今年 " + str(age) + " 岁"
    print(f"字符串拼接: {message}")

    # f-string 推荐
    message2 = f"我今年 {age} 岁"
    print(f"f-string: {message2}\n")

    # 4. 变量命名规范
    print("4. 变量命名规范")
    user_name = "Alice"       # 蛇形命名法
    user_age = 30
    total_score = 95

    print(f"user_name: {user_name}")
    print(f"user_age: {user_age}")
    print(f"total_score: {total_score}")

    # 不推荐的命名
    # User = "Bob"  # PEP 8 不建议
    # totalScore = 100  # 应该用下划线
    print()

    # 5. 变量删除
    print("5. 变量删除")
    temp_var = "这个变量会被删除"
    print(f"删除前: temp_var = {temp_var}")

    del temp_var
    # print(temp_var)  # 这行会报错: NameError
    print("temp_var 已被删除\n")

    print("=== 演示完成 ===")

if __name__ == "__main__":
    main()
