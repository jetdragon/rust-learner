# 继承 (Inheritance) 演示

"""
继承是面向对象编程的核心概念，允许代码重用和层次化设计。
本文件演示Python的继承机制。
"""


def main():
    print("=== Python 继承演示 ===\n")

    # ========== 1. 基础继承 ==========
    print("1. 基础继承")

    class Animal:
        """动物基类"""

        def __init__(self, name: str, age: int):
            self.name = name
            self.age = age

        def speak(self) -> str:
            """发出声音"""
            return "某种声音"

        def info(self) -> str:
            """基本信息"""
            return f"{self.name}，{self.age}岁"

    class Dog(Animal):
        """狗类（继承自Animal）"""
        def speak(self) -> str:
            """重写父类方法"""
            return "汪汪！"

        def fetch(self) -> str:
            """新方法（狗特有）"""
            return f"{self.name}去捡球了"

    # 创建子类对象
    dog = Dog("旺财", 3)
    print(f"dog.name: {dog.name}")
    print(f"dog.speak(): {dog.speak()}")
    print(f"dog.info(): {dog.info()}")
    print(f"dog.fetch(): {dog.fetch()}")
    print()

    # ========== 2. super() 函数 ==========
    print("2. super() 函数 - 调用父类方法")

    class Vehicle:
        """车辆基类"""
        def __init__(self, brand: str, speed: int):
            self.brand = brand
            self.speed = speed

        def description(self) -> str:
            return f"{self.brand}，速度{self.speed}km/h"

    class Car(Vehicle):
        """汽车类"""
        def __init__(self, brand: str, speed: int, fuel_type: str):
            # 使用super()调用父类的__init__
            super().__init__(brand, speed)
            self.fuel_type = fuel_type

        def description(self) -> str:
            # 扩展父类方法
            base_desc = super().description()
            return f"{base_desc}，燃油类型：{self.fuel_type}"

    car = Car("特斯拉", 200, "电动")
    print(car.description())
    print()

    # ========== 3. 方法解析顺序 (MRO) ==========
    print("3. 方法解析顺序 (MRO)")

    class A:
        def method(self):
            print("A.method()")
            return "A"

    class B(A):
        def method(self):
            print("B.method()")
            return "B"

    class C(A):
        def method(self):
            print("C.method()")
            return "C"

    class D(B, C):
        def method(self):
            print("D.method() 调用链:")
            print(f"  D -> C -> B -> A")
            print(f"MRO: {[c.__name__ for c in D.__mro__]}")
            return "D"

    d = D()
    d.method()
    print()

    # ========== 4. 多继承 ==========
    print("4. 多继承")

    class Flyable:
        """可飞行接口"""
        def fly(self) -> str:
            return "飞行中..."

    class Swimmable:
        """可游泳接口"""
        def swim(self) -> str:
            return "游泳中..."

    class Duck(Flyable, Swimmable):
        """鸭子类（多继承）"""
        def __init__(self, name: str):
            self.name = name

        def quack(self) -> str:
            return f"{self.name}：嘎嘎！"

    duck = Duck("唐老鸭")
    print(duck.fly())
    print(duck.swim())
    print(duck.quack())
    print()

    # ========== 5. 抽象基类 ==========
    print("5. 抽象基类（ABC）")

    from abc import ABC, abstractmethod

    class Shape(ABC):
        """形状抽象基类"""

        @abstractmethod
        def area(self) -> float:
            """计算面积（抽象方法）"""
            pass

        @abstractmethod
        def perimeter(self) -> float:
            """计算周长（抽象方法）"""
            pass

    class Rectangle(Shape):
        """矩形类"""
        def __init__(self, width: float, height: float):
            self.width = width
            self.height = height

        def area(self) -> float:
            return self.width * self.height

        def perimeter(self) -> float:
            return 2 * (self.width + self.height)

    class Circle(Shape):
        """圆形类"""
        import math

        def __init__(self, radius: float):
            self.radius = radius

        def area(self) -> float:
            return Circle.math.pi * self.radius ** 2

        def perimeter(self) -> float:
            return 2 * Circle.math.pi * self.radius

    # 使用具体类
    rect = Rectangle(5, 3)
    circle = Circle(2)

    print(f"矩形面积: {rect.area()}")
    print(f"矩形周长: {rect.perimeter()}")
    print(f"圆形面积: {circle.area():.2f}")
    print()

    # ========== 6. Mixin 模式 ==========
    print("6. Mixin 模式（代码复用）")

    class LoggableMixin:
        """可日志记录的Mixin"""
        def log(self, message: str):
            print(f"[LOG] {self.__class__.__name__}: {message}")

    class TimestampMixin:
        """时间戳Mixin"""
        import time

        def get_timestamp(self) -> str:
            return TimestampMixin.time.ctime()

    class User(LoggableMixin, TimestampMixin):
        """用户类（使用多个Mixin）"""
        def __init__(self, username: str):
            self.username = username

        def login(self):
            self.log(f"用户 {self.username} 登录于 {self.get_timestamp()}")

    user = User("Alice")
    user.login()
    print()

    print("=== 演示完成 ===")
    print()
    print("💡 继承的最佳实践:")
    print("  1. 优先使用组合而非继承")
    print("  2. 明确使用super()调用父类方法")
    print("  3. 多继承要谨慎，注意MRO顺序")
    print("  4. 使用抽象基类定义接口")
    print("  5. Mixin用于代码复用，不应单独使用")


if __name__ == "__main__":
    main()
