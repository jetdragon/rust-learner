# 多态和封装演示

"""
多态和封装是面向对象编程的重要概念。
本文件演示Python的多态实现和封装技术。
"""


def main():
    print("=== Python 多态和封装演示 ===\n")

    # ========== 1. 方法重写多态 ==========
    print("1. 方法重写多态")

    class Animal:
        """动物基类"""
        def speak(self) -> str:
            return "动物发出声音"

    class Dog(Animal):
        """狗类"""
        def speak(self) -> str:
            return "汪汪！"

    class Cat(Animal):
        """猫类"""
        def speak(self) -> str:
            return "喵喵~"

    class Cow(Animal):
        """牛类"""
        def speak(self) -> str:
            return "哞哞！"

    # 多态：同一接口，不同实现
    animals = [Dog(), Cat(), Cow()]
    print("多态调用:")
    for animal in animals:
        print(f"  {animal.__class__.__name__}: {animal.speak()}")
    print()

    # ========== 2. 鸭子类型 (Duck Typing) ==========
    print("2. 鸭子类型（如果它走起来像鸭子...）")

    class Duck:
        def quack(self):
            print("嘎嘎！")

    class Person:
        def quack(self):
            print("我在模仿鸭子的声音")

    def make_it_quack(thing):
        """不检查类型，只要有quack()方法就行"""
        thing.quack()

    duck = Duck()
    person = Person()

    print("鸭子类型 - 不关心类型，只关心行为:")
    make_it_quack(duck)
    make_it_quack(person)
    print()

    # ========== 3. 抽象基类多态 ==========
    print("3. 抽象基类强制接口")

    from abc import ABC, abstractmethod

    class PaymentProcessor(ABC):
        """支付处理器抽象基类"""

        @abstractmethod
        def process_payment(self, amount: float) -> bool:
            """处理支付（抽象方法）"""
            pass

        @abstractmethod
        def refund(self, amount: float) -> bool:
            """退款（抽象方法）"""
            pass

    class CreditCardProcessor(PaymentProcessor):
        """信用卡支付处理器"""
        def process_payment(self, amount: float) -> bool:
            print(f"信用卡支付 ${amount}")
            return True

        def refund(self, amount: float) -> bool:
            print(f"信用卡退款 ${amount}")
            return True

    class PayPalProcessor(PaymentProcessor):
        """PayPal支付处理器"""
        def process_payment(self, amount: float) -> bool:
            print(f"PayPal支付 ${amount}")
            return True

        def refund(self, amount: float) -> bool:
            print(f"PayPal退款 ${amount}")
            return True

    # 多态使用
    processors = [CreditCardProcessor(), PayPalProcessor()]

    print("多态支付处理:")
    for processor in processors:
        processor.process_payment(100)
        processor.refund(50)
    print()

    # ========== 4. 运算符重载多态 ==========
    print("4. 运算符重载")

    class Money:
        """金额类"""
        def __init__(self, amount: float, currency: str = "USD"):
            self.amount = amount
            self.currency = currency

        def __add__(self, other):
            """重载 + 运算符"""
            if not isinstance(other, Money):
                raise TypeError("只能与Money对象相加")
            if self.currency != other.currency:
                raise ValueError("货币类型不同")
            return Money(self.amount + other.amount, self.currency)

        def __mul__(self, multiplier: float):
            """重载 * 运算符"""
            return Money(self.amount * multiplier, self.currency)

        def __str__(self) -> str:
            return f"{self.amount:.2f} {self.currency}"

    # 多态运算
    m1 = Money(100, "USD")
    m2 = Money(50, "USD")
    m3 = m1 + m2
    m4 = m1 * 1.5

    print(f"m1: {m1}")
    print(f"m2: {m2}")
    print(f"m1 + m2: {m3}")
    print(f"m1 * 1.5: {m4}")
    print()

    # ========== 5. 封装 - 私有属性 ==========
    print("5. 封装 - 私有属性")

    class Employee:
        """员工类"""
        def __init__(self, name: str, salary: float):
            self.name = name
            # 私有属性（名称改写）
            self.__salary = salary

        def get_salary(self) -> float:
            """获取薪资"""
            return self.__salary

        def set_salary(self, new_salary: float):
            """设置薪资（带验证）"""
            if new_salary > 0:
                self.__salary = new_salary
                print(f"薪资已更新为 ${new_salary}")
            else:
                print("薪资必须大于0")

        # 名称改写演示
        def __private_method(self):
            """私有方法"""
            print("这是私有方法")

        def public_method(self):
            """公开方法调用私有方法"""
            self.__private_method()

    emp = Employee("Alice", 50000)
    print(f"初始薪资: ${emp.get_salary()}")

    emp.set_salary(60000)
    print(f"更新后: ${emp.get_salary()}")

    # 尝试直接访问（会失败）
    print("尝试访问私有属性:")
    print(f"emp.__salary: {hasattr(emp, '__salary')}")  # False（名称改写）
    print(f"emp._Employee__salary: {emp._Employee__salary}")  # True（改写后的名称）
    print()

    # ========== 6. 属性装饰器（更好的封装）==========
    print("6. 属性装饰器（Pythonic封装）")

    class Temperature:
        """温度类"""
        def __init__(self, celsius: float = 0):
            self._celsius = celsius

        @property
        def celsius(self) -> float:
            """摄氏度 getter"""
            return self._celsius

        @celsius.setter
        def celsius(self, value: float):
            """摄氏度 setter"""
            if value < -273.15:
                raise ValueError("温度不能低于绝对零度")
            self._celsius = value

        @property
        def fahrenheit(self) -> float:
            """华氏度 getter（只读属性）"""
            return self._celsius * 9/5 + 32

        @fahrenheit.setter
        def fahrenheit(self, value: float):
            """华氏度 setter"""
            self._celsius = (value - 32) * 5/9

    temp = Temperature(25)
    print(f"摄氏度: {temp.celsius}°C")
    print(f"华氏度: {temp.fahrenheit:.1f}°F")

    temp.celsius = 30
    print(f"\n更新后: {temp.celsius}°C")

    temp.fahrenheit = 86
    print(f"通过华氏度设置: {temp.celsius}°C")
    print()

    # ========== 7. 多态的实际应用 ==========
    print("7. 实际应用：插件系统")

    class Plugin(ABC):
        """插件抽象基类"""
        @abstractmethod
        def execute(self, data: str) -> str:
            pass

    class UpperCasePlugin(Plugin):
        """转大写插件"""
        def execute(self, data: str) -> str:
            return data.upper()

    class ReversePlugin(Plugin):
        """反转插件"""
        def execute(self, data: str) -> str:
            return data[::-1]

    class LengthPlugin(Plugin):
        """长度插件"""
        def execute(self, data: str) -> str:
            return f"长度: {len(data)}"

    # 插件管理器（多态使用）
    class PluginManager:
        def __init__(self):
            self.plugins = []

        def register(self, plugin: Plugin):
            self.plugins.append(plugin)

        def process(self, data: str):
            """多态处理：应用所有插件"""
            results = []
            for plugin in self.plugins:
                result = plugin.execute(data)
                results.append(result)
            return results

    # 使用插件系统
    manager = PluginManager()
    manager.register(UpperCasePlugin())
    manager.register(ReversePlugin())
    manager.register(LengthPlugin())

    text = "Hello World"
    print(f"原文: {text}")
    print(f"插件处理结果: {manager.process(text)}")
    print()

    print("=== 演示完成 ===")
    print()
    print("💡 多态的优势:")
    print("  1. 代码灵活性和可扩展性")
    print("  2. 减少 if-else 条件判断")
    print("  3. 易于维护和测试")
    print("  4. 支持开放封闭原则（对扩展开放，对修改封闭）")
    print()
    print("💡 封装的优势:")
    print("  1. 隐藏实现细节")
    print("  2. 保护数据完整性")
    print("  3. 降低耦合度")
    print("  4. 提供清晰的接口")


if __name__ == "__main__":
    main()
