# 文件 I/O 操作演示

"""
本文件演示Python的文件读写操作和最佳实践。
"""

import os
from pathlib import Path


def main():
    print("=== Python 文件 I/O 演示 ===\n")

    # ========== 1. 基础文件读写 ==========
    print("1. 基础文件读写（使用 with 语句）")

    # 写文件
    filename = "demo.txt"
    with open(filename, "w", encoding="utf-8") as f:
        f.write("Hello, World!\n")
        f.write("这是中文内容\n")
        f.write("Line 3\n")
    print(f"已创建文件: {filename}")

    # 读文件
    with open(filename, "r", encoding="utf-8") as f:
        content = f.read()
    print(f"文件内容:\n{content}")

    # ========== 2. 逐行读取 ==========
    print("\n2. 逐行读取大文件")

    with open(filename, "r", encoding="utf-8") as f:
        print("逐行输出:")
        for i, line in enumerate(f, 1):
            print(f"  行{i}: {line.rstrip()}")

    # ========== 3. 读取模式 ==========
    print("\n3. 不同的读取模式")

    with open(filename, "r", encoding="utf-8") as f:
        # read() - 读取全部
        all_content = f.read()

    with open(filename, "r", encoding="utf-8") as f:
        # readline() - 读取一行
        first_line = f.readline()

    with open(filename, "r", encoding="utf-8") as f:
        # readlines() - 读取所有行到列表
        all_lines = f.readlines()

    print(f"第一行: {first_line.rstrip()}")
    print(f"总行数: {len(all_lines)}")

    # ========== 4. 文件模式 ==========
    print("\n4. 文件打开模式")
    modes = {
        "r": "只读（默认）",
        "w": "写入（覆盖已有文件）",
        "a": "追加（在文件末尾写入）",
        "x": "独占创建（文件不存在才创建）",
        "r+": "读写",
        "b": "二进制模式",
        "t": "文本模式（默认）"
    }
    for mode, desc in modes.items():
        print(f"  '{mode}': {desc}")

    # ========== 5. 追加内容 ==========
    print("\n5. 追加内容")

    with open(filename, "a", encoding="utf-8") as f:
        f.write("追加的新行\n")

    with open(filename, "r", encoding="utf-8") as f:
        print("追加后的内容:")
        print(f.read())

    # ========== 6. pathlib - 现代路径操作 ==========
    print("\n6. pathlib - 现代化的路径操作")

    # 创建路径对象
    file_path = Path("demo.txt")
    print(f"文件名: {file_path.name}")
    print(f"是否存在: {file_path.exists()}")
    print(f"是否为文件: {file_path.is_file()}")
    print(f"文件大小: {file_path.stat().st_size} 字节")

    # 读写文件（更简洁）
    content = file_path.read_text(encoding="utf-8")
    print(f"读取内容（前30字符）: {content[:30]}...")

    # ========== 7. 二进制文件 ==========
    print("\n7. 二进制文件操作")

    binary_file = "binary_data.bin"
    data = b'\x00\x01\x02\x03\x04\x05'

    # 写二进制
    with open(binary_file, "wb") as f:
        f.write(data)

    # 读二进制
    with open(binary_file, "rb") as f:
        read_data = f.read()
    print(f"二进制数据: {read_data}")

    # ========== 8. JSON 文件 ==========
    print("\n8. JSON 文件操作")

    import json

    data = {
        "name": "Alice",
        "age": 30,
        "skills": ["Python", "JavaScript", "SQL"]
    }

    json_file = "data.json"
    with open(json_file, "w", encoding="utf-8") as f:
        json.dump(data, f, ensure_ascii=False, indent=2)
    print(f"已创建JSON文件: {json_file}")

    with open(json_file, "r", encoding="utf-8") as f:
        loaded_data = json.load(f)
    print(f"读取的JSON: {loaded_data}")

    # ========== 9. CSV 文件 ==========
    print("\n9. CSV 文件操作")

    import csv

    csv_file = "users.csv"
    users = [
        ["name", "age", "city"],
        ["Alice", 30, "Beijing"],
        ["Bob", 25, "Shanghai"]
    ]

    # 写CSV
    with open(csv_file, "w", newline="", encoding="utf-8") as f:
        writer = csv.writer(f)
        writer.writerows(users)

    # 读CSV
    with open(csv_file, "r", encoding="utf-8") as f:
        reader = csv.reader(f)
        for row in reader:
            print(f"  {row}")

    # ========== 10. 文件和目录操作 ==========
    print("\n10. 文件和目录操作")

    # 使用 pathlib
    test_dir = Path("test_directory")
    test_dir.mkdir(exist_ok=True)  # 创建目录
    print(f"创建目录: {test_dir}")

    # 列出目录内容
    parent = Path(".")
    py_files = list(parent.glob("*.py"))
    print(f"当前目录的Python文件: {len(py_files)}个")

    # ========== 11. 临时文件 ==========
    print("\n11. 临时文件")

    import tempfile

    # 创建临时文件
    with tempfile.NamedTemporaryFile(mode="w", delete=False, encoding="utf-8") as tmp:
        tmp.write("临时内容")
        temp_name = tmp.name
    print(f"临时文件: {temp_name}")

    # 读取临时文件
    with open(temp_name, "r", encoding="utf-8") as f:
        print(f"临时文件内容: {f.read()}")

    # 清理临时文件
    os.unlink(temp_name)
    print("临时文件已删除")

    # ========== 12. 上下文管理器（自定义）==========
    print("\n12. 自定义上下文管理器")

    from contextlib import contextmanager

    @contextmanager
    def file_manager(filename, mode):
        """自定义文件管理器"""
        print(f"打开文件: {filename}")
        f = open(filename, mode, encoding="utf-8")
        try:
            yield f
        finally:
            f.close()
            print(f"关闭文件: {filename}")

    with file_manager(filename, "r") as f:
        content = f.readline()
    print(f"读取内容: {content.rstrip()}")

    # ========== 13. 清理演示文件 ==========
    print("\n13. 清理演示文件")
    demo_files = [filename, binary_file, json_file, csv_file]
    for f in demo_files:
        if os.path.exists(f):
            os.remove(f)
            print(f"已删除: {f}")

    if test_dir.exists():
        test_dir.rmdir()
        print(f"已删除目录: {test_dir}")

    print("\n=== 演示完成 ===")
    print()
    print("💡 最佳实践:")
    print("  1. 优先使用 pathlib 代替 os.path")
    print("  2. 始终使用 with 语句管理文件资源")
    print("  3. 明确指定 encoding='utf-8'")
    print("  4. 小文件用 read()，大文件逐行读取")
    print("  5. 二进制用 'b' 模式，文本用 't' 模式")


if __name__ == "__main__":
    main()
