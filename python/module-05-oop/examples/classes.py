# 面向对象编程基础 - 类和对象演示

"""
本文件演示Python面向对象编程的核心概念：类、对象、属性和方法。
"""


def main():
    print("=== Python 面向对象编程演示 ===\n")

    # ========== 1. 类定义和对象创建 ==========
    print("1. 类定义和对象创建")

    class Person:
        """人类"""

        # 类属性（所有实例共享）
        species = "人类"

        def __init__(self, name: str, age: int):
            """构造方法 - 初始化对象"""
            self.name = name  # 实例属性
            self.age = age

        def greet(self) -> str:
            """实例方法 - 打招呼"""
            return f"你好，我是{self.name}"

        def introduce(self) -> str:
            """自我介绍"""
            return f"我叫{self.name}，今年{self.age}岁"

    # 创建对象（实例化）
    person1 = Person("Alice", 30)
    person2 = Person("Bob", 25)

    print(f"person1.name: {person1.name}")
    print(f"person1.species: {person1.species}")
    print(person1.greet())
    print(person1.introduce())
    print()

    # ========== 2. 实例属性 vs 类属性 ==========
    print("2. 实例属性 vs 类属性")

    print(f"person1.species: {person1.species}")
    print(f"person2.species: {person2.species}")

    # 修改类属性（影响所有实例）
    Person.species = "智人"
    print(f"修改后 person1.species: {person1.species}")
    print(f"修改后 person2.species: {person2.species}")

    # 添加实例属性（只影响当前实例）
    person1.species = "人类变种"
    print(f"person1.species: {person1.species}")  # 实例属性
    print(f"person2.species: {person2.species}")  # 类属性
    print()

    # ========== 3. 方法类型 ==========
    print("3. 方法类型")

    class Calculator:
        """计算器类"""

        def __init__(self, value: float = 0.0):
            self.value = value

        # 实例方法（最常用）
        def add(self, x: float) -> float:
            """实例方法 - 第一个参数是self"""
            self.value += x
            return self.value

        # 类方法（操作类本身）
        @classmethod
        def from_string(cls, s: str):
            """从字符串创建计算器"""
            value = float(s)
            return cls(value)

        # 静态方法（不访问实例或类）
        @staticmethod
        def is_positive(x: float) -> bool:
            """静态方法 - 不需要self或cls"""
            return x > 0

    # 使用实例方法
    calc = Calculator(10.0)
    print(f"calc.add(5): {calc.add(5)}")

    # 使用类方法
    calc2 = Calculator.from_string("20.0")
    print(f"calc2.value: {calc2.value}")

    # 使用静态方法
    print(f"Calculator.is_positive(5): {Calculator.is_positive(5)}")
    print(f"Calculator.is_positive(-3): {Calculator.is_positive(-3)}")
    print()

    # ========== 4. 封装（私有属性）==========
    print("4. 封装 - 私有属性")

    class BankAccount:
        """银行账户类"""

        def __init__(self, owner: str, balance: float):
            self.owner = owner
            self.__balance = balance  # 私有属性（双下划线）

        def deposit(self, amount: float):
            """存款"""
            if amount > 0:
                self.__balance += amount
                print(f"存款 {amount} 元")
            else:
                print("存款金额必须大于0")

        def withdraw(self, amount: float):
            """取款"""
            if 0 < amount <= self.__balance:
                self.__balance -= amount
                print(f"取款 {amount} 元")
            else:
                print("余额不足或金额无效")

        def get_balance(self) -> float:
            """获取余额（通过方法访问私有属性）"""
            return self.__balance

        # 属性装饰器（提供更优雅的访问方式）
        @property
        def balance(self) -> float:
            """balance 属性的 getter"""
            return self.__balance

    account = BankAccount("Alice", 1000.0)
    print(f"账户余额: {account.get_balance()}")

    account.deposit(500)
    account.withdraw(200)
    print(f"当前余额: {account.balance}")  # 使用 @property
    print()

    # ========== 5. 特殊方法（魔术方法）==========
    print("5. 特殊方法（魔术方法）")

    class Vector:
        """向量类"""

        def __init__(self, x: float, y: float):
            self.x = x
            self.y = y

        # 字符串表示
        def __str__(self) -> str:
            """友好的字符串表示（print时调用）"""
            return f"Vector({self.x}, {self.y})"

        def __repr__(self) -> str:
            """开发者用的字符串表示"""
            return f"Vector(x={self.x}, y={self.y})"

        # 比较运算
        def __eq__(self, other) -> bool:
            """相等比较"""
            if not isinstance(other, Vector):
                return False
            return self.x == other.x and self.y == other.y

        # 算术运算
        def __add__(self, other):
            """加法运算"""
            return Vector(self.x + other.x, self.y + other.y)

        def __mul__(self, scalar: float):
            """标量乘法"""
            return Vector(self.x * scalar, self.y * scalar)

        # 长度
        def __len__(self) -> int:
            """向量长度（整数部分）"""
            return int((self.x ** 2 + self.y ** 2) ** 0.5)

        # 布尔值
        def __bool__(self) -> bool:
            """是否为零向量"""
            return self.x != 0 or self.y != 0

        # 索引访问
        def __getitem__(self, index: int) -> float:
            """支持索引访问"""
            if index == 0:
                return self.x
            elif index == 1:
                return self.y
            else:
                raise IndexError("Vector index out of range")

    v1 = Vector(3, 4)
    v2 = Vector(1, 2)

    print(f"v1: {v1}")
    print(f"repr(v2): {repr(v2)}")
    print(f"v1 == v2: {v1 == v2}")
    print(f"v1 + v2: {v1 + v2}")
    print(f"v1 * 2: {v1 * 2}")
    print(f"len(v1): {len(v1)}")
    print(f"bool(v1): {bool(v1)}")
    print(f"v1[0]: {v1[0]}, v1[1]: {v1[1]}")
    print()

    # ========== 6. 类的组合 ==========
    print("6. 类的组合（组合优于继承）")

    class Engine:
        """引擎类"""
        def __init__(self, power: int):
            self.power = power

        def start(self):
            return f"{self.power}马力引擎启动"

    class Car:
        """汽车类"""
        def __init__(self, brand: str, engine: Engine):
            self.brand = brand
            self.engine = engine  # 组合：Car包含Engine

        def start(self):
            return f"{self.brand}汽车 - {self.engine.start()}"

    # 创建对象
    engine = Engine(200)
    car = Car("宝马", engine)
    print(car.start())
    print()

    print("=== 演示完成 ===")


if __name__ == "__main__":
    main()
