# 模块和包导入演示

"""
本文件演示Python的模块系统、导入机制和包管理。
"""


def main():
    print("=== Python 模块和包演示 ===\n")

    # ========== 1. 导入方式 ==========
    print("1. 不同的导入方式")

    # 导入整个模块
    import math
    print(f"math.pi: {math.pi}")
    print(f"math.sqrt(16): {math.sqrt(16)}")

    # 导入模块并给别名
    import numpy as np  # 如果已安装
    # print(f"np.array([1,2,3]): {np.array([1,2,3])}")

    # 从模块导入特定函数/类
    from datetime import datetime
    now = datetime.now()
    print(f"\n当前时间: {now}")

    # 导入多个
    from sys import version, platform
    print(f"Python版本: {version}")
    print(f"平台: {platform}")

    # 导入所有（不推荐）
    # from math import *
    print()

    # ========== 2. 相对导入 ==========
    print("2. 相对导入（包内部使用）")
    print("在包内:")
    print("  from . import module      # 当前包")
    print("  from .sibling import func  # 同级包")
    print("  from ..parent import Class # 父包")
    print("  from ..grandparent.sub import X  # 父包的子包")
    print()

    # ========== 3. 模块搜索路径 ==========
    print("3. 模块搜索路径")
    import sys
    print("sys.path 包含:")
    for i, path in enumerate(sys.path[:3], 1):
        print(f"  {i}. {path}")
    print()

    # ========== 4. 动态导入 ==========
    print("4. 动态导入")

    # 使用 importlib
    import importlib

    # 动态导入模块
    math_module = importlib.import_module("math")
    print(f"动态导入 math.pi: {math_module.pi}")

    # 条件导入
    module_name = "random"
    try:
        random_module = importlib.import_module(module_name)
        print(f"成功导入 {module_name}")
    except ImportError:
        print(f"无法导入 {module_name}")
    print()

    # ========== 5. 模块属性 ==========
    print("5. 有用的模块属性")

    import os

    print(f"os.__file__: {os.__file__}")
    print(f"os.__name__: {os.__name__}")
    print(f"os.__doc__: {os.__doc__[:50]}...")
    print(f"os.__package__: {os.__package__}")
    print()

    # ========== 6. __name__ == '__main__' ==========
    print("6. __name__ == '__main__' 的作用")
    print("当前文件 __name__:", __name__)
    print("直接运行时: __name__ == '__main__'")
    print("被导入时: __name__ == '模块名'")
    print("这个特性使文件既是脚本又是可导入模块")
    print()

    # ========== 7. 包结构 ==========
    print("7. 包结构示例")
    print("""
mypackage/
├── __init__.py          # 包初始化文件
├── module1.py
├── module2.py
└── subpackage/
    ├── __init__.py
    └── module3.py

导入方式:
- import mypackage
- from mypackage import module1
- from mypackage.subpackage import module3
    """)
    print()

    # ========== 8. 延迟导入 ==========
    print("8. 延迟导入（优化启动时间）")

    def heavy_operation():
        """只在需要时导入重模块"""
        import pandas as pd  # 延迟导入
        return pd.DataFrame({"a": [1, 2, 3]})

    # 函数未调用，pandas未导入
    print("函数未调用，pandas 未加载")

    # 需要时才导入
    # df = heavy_operation()  # 这时才导入pandas
    print()

    # ========== 9. 循环导入问题 ==========
    print("9. 循环导入及解决方案")
    print("问题: module_a 导入 module_b，module_b 导入 module_a")
    print()
    print("解决方案:")
    print("  1. 重构代码，消除循环依赖")
    print("  2. 在函数内导入（延迟导入）")
    print("  3. 使用 import 代替 from ... import")
    print()

    # ========== 10. 第三方包管理 ==========
    print("10. 第三方包管理")

    # 检查包是否已安装
    try:
        import requests
        print(f"requests 已安装，版本: {requests.__version__}")
    except ImportError:
        print("requests 未安装")

    print("\n常用包管理命令:")
    print("  pip install package-name    # 安装")
    print("  pip install -r requirements.txt  # 批量安装")
    print("  pip list                    # 列出已安装")
    print("  pip show package-name       # 查看详情")
    print("  pip uninstall package-name  # 卸载")
    print()

    # ========== 11. 最佳实践 ==========
    print("11. 导入最佳实践")
    print("✅ 推荐顺序:")
    print("  1. 标准库导入 (os, sys, re, etc.)")
    print("  2. 第三方库导入 (numpy, pandas, etc.)")
    print("  3. 本地应用/库导入")
    print()
    print("✅ 导入格式:")
    print("  import os")
    print("  import sys")
    print("  import numpy as np")
    print("  from datetime import datetime")
    print("  from myapp import utils")
    print()
    print("❌ 避免:")
    print("  from module import *  # 污染命名空间")
    print("  循环导入")
    print("  过度使用相对导入")
    print()

    print("=== 演示完成 ===")


if __name__ == "__main__":
    main()
