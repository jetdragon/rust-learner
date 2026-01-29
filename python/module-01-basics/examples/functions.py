# 函数演示

def main():
    print("=== Python 函数演示 ===\n")

    # 1. 基本函数定义
    print("1. 基本函数定义")
    
    def greet():
        """简单的打招呼函数"""
        return "Hello, World!"
    
    message = greet()
    print(f"greet() -> {message}\n")

    # 2. 带参数的函数
    print("2. 带参数的函数")
    
    def greet_person(name):
        """向指定的人打招呼"""
        return f"Hello, {name}!"
    
    print(f"greet_person('Alice') -> {greet_person('Alice')}")
    print(f"greet_person('Bob') -> {greet_person('Bob')}\n")

    # 3. 多个参数
    print("3. 多个参数")
    
    def describe(name, age, city):
        """描述一个人的信息"""
        return f"{name}, {age}岁, 来自{city}"
    
    print(f"describe('Alice', 25, 'Beijing') -> {describe('Alice', 25, 'Beijing')}\n")

    # 4. 默认参数
    print("4. 默认参数")
    
    def greet_with_default(name, greeting="Hello"):
        """带默认问候语的函数"""
        return f"{greeting}, {name}!"
    
    print(f"greet_with_default('Alice') -> {greet_with_default('Alice')}")
    print(f"greet_with_default('Bob', 'Hi') -> {greet_with_default('Bob', 'Hi')}\n")

    # 5. 关键字参数
    print("5. 关键字参数")
    
    def create_user(name, age, city="Unknown"):
        """创建用户信息"""
        return f"User: {name}, Age: {age}, City: {city}"
    
    # 位置参数
    print(f"位置参数: {create_user('Alice', 25)}")
    # 关键字参数（顺序不重要）
    print(f"关键字参数: {create_user(age=25, name='Alice')}")
    print(f"混合: {create_user('Alice', age=25, city='Shanghai')}\n")

    # 6. 返回值
    print("6. 返回值")
    
    def add(a, b):
        """加法函数"""
        return a + b
    
    def add_and_multiply(a, b, c):
        """先加再乘"""
        return (a + b) * c
    
    result = add(3, 4)
    print(f"add(3, 4) = {result}")
    
    result2 = add_and_multiply(2, 3, 4)
    print(f"add_and_multiply(2, 3, 4) = {result2}")
    
    # 多个返回值（元组）
    def divide_and_remainder(a, b):
        """返回商和余数"""
        return a // b, a % b
    
    quotient, remainder = divide_and_remainder(10, 3)
    print(f"10 / 3: 商 = {quotient}, 余 = {remainder}\n")

    # 7. 文档字符串
    print("7. 函数文档字符串")
    
    def calculate_bmi(weight_kg, height_m):
        """
        计算BMI指数
        
        Args:
            weight_kg: 体重（千克）
            height_m: 身高（米）
        
        Returns:
            float: BMI指数
        
        Example:
            >>> calculate_bmi(70, 1.75)
            22.857142857142858
        """
        return weight_kg / (height_m ** 2)
    
    bmi = calculate_bmi(70, 1.75)
    print(f"BMI = {bmi:.2f}")
    print(f"文档: {calculate_bmi.__doc__[:100]}...\n")

    # 8. 函数作为参数
    print("8. 函数作为参数")
    
    def apply_operation(a, b, operation):
        """应用操作函数"""
        return operation(a, b)
    
    def multiply(x, y):
        return x * y
    
    def power(x, y):
        return x ** y
    
    print(f"apply_operation(3, 4, multiply) = {apply_operation(3, 4, multiply)}")
    print(f"apply_operation(2, 3, power) = {apply_operation(2, 3, power)}\n")

    # 9. Lambda 函数
    print("9. Lambda 函数")
    
    square = lambda x: x ** 2
    print(f"square(5) = {square(5)}")
    
    add_lambda = lambda x, y: x + y
    print(f"add_lambda(3, 4) = {add_lambda(3, 4)}")
    
    # 结合内置函数使用 lambda
    numbers = [1, 2, 3, 4, 5]
    doubled = list(map(lambda x: x * 2, numbers))
    print(f"map(lambda x: x * 2, [1,2,3,4,5]) = {doubled}")
    
    even = list(filter(lambda x: x % 2 == 0, numbers))
    print(f"filter(lambda x: x % 2 == 0, [1,2,3,4,5]) = {even}\n")

    # 10. 作用域
    print("10. 变量作用域")
    
    global_var = "全局变量"
    
    def outer_function():
        outer_var = "外部函数变量"
        
        def inner_function():
            inner_var = "内部函数变量"
            print(f"  inner_function: global_var = {global_var}")
            print(f"  inner_function: outer_var = {outer_var}")
            print(f"  inner_function: inner_var = {inner_var}")
        
        print("  outer_function: 开始")
        inner_function()
        print(f"  outer_function: outer_var = {outer_var}")
        # print(f"  outer_function: inner_var = {inner_var}")  # NameError
        print("  outer_function: 结束")
    
    print("主函数: 开始")
    outer_function()
    print(f"主函数: global_var = {global_var}")
    # print(f"主函数: outer_var = {outer_var}")  # NameError
    print("主函数: 结束\n")

    # 11. 函数递归
    print("11. 函数递归")
    
    def factorial(n):
        """计算阶乘（递归实现）"""
        if n <= 1:
            return 1
        return n * factorial(n - 1)
    
    result = factorial(5)
    print(f"factorial(5) = {result}")
    print(f"计算过程: 5! = 5 * 4! = 5 * 4 * 3! = ... = {result}\n")

    # 12. *args 和 **kwargs
    print("12. 可变参数")
    
    def sum_all(*args):
        """计算所有参数的和"""
        return sum(args)
    
    print(f"sum_all(1, 2, 3, 4, 5) = {sum_all(1, 2, 3, 4, 5)}")
    print(f"sum_all(10, 20, 30) = {sum_all(10, 20, 30)}")
    
    def print_info(**kwargs):
        """打印所有关键字参数"""
        for key, value in kwargs.items():
            print(f"  {key}: {value}")
    
    print("print_info(name='Alice', age=25, city='Beijing'):")
    print_info(name='Alice', age=25, city='Beijing')
    
    print("\n=== 演示完成 ===")

if __name__ == "__main__":
    main()
