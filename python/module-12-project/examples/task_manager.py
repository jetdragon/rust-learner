#!/usr/bin/env python3
"""
命令行任务管理器 - 综合项目示例

一个简单的CLI任务管理工具，演示Python的综合应用。
功能：添加任务、列出任务、标记完成、保存到文件
"""

import json
import os
from pathlib import Path
from typing import List, Dict, Optional
from datetime import datetime


class Task:
    """任务类"""

    def __init__(self, title: str, description: str = ""):
        self.title = title
        self.description = description
        self.completed = False
        self.created_at = datetime.now().isoformat()
        self.completed_at: Optional[str] = None

    def mark_completed(self):
        """标记为完成"""
        self.completed = True
        self.completed_at = datetime.now().isoformat()

    def to_dict(self) -> Dict:
        """转换为字典"""
        return {
            "title": self.title,
            "description": self.description,
            "completed": self.completed,
            "created_at": self.created_at,
            "completed_at": self.completed_at
        }

    @classmethod
    def from_dict(cls, data: Dict) -> "Task":
        """从字典创建"""
        task = cls(data["title"], data.get("description", ""))
        task.completed = data.get("completed", False)
        task.created_at = data.get("created_at", datetime.now().isoformat())
        task.completed_at = data.get("completed_at")
        return task


class TaskManager:
    """任务管理器"""

    def __init__(self, data_file: str = "tasks.json"):
        self.data_file = data_file
        self.tasks: List[Task] = []
        self.load_tasks()

    def add_task(self, title: str, description: str = "") -> Task:
        """添加新任务"""
        task = Task(title, description)
        self.tasks.append(task)
        self.save_tasks()
        print(f"✓ 添加任务: {title}")
        return task

    def list_tasks(self, show_completed: bool = False) -> None:
        """列出任务"""
        if not self.tasks:
            print("没有任务")
            return

        print("\n任务列表:")
        print("-" * 60)

        for i, task in enumerate(self.tasks, 1):
            if not show_completed and task.completed:
                continue

            status = "✓" if task.completed else " "
            print(f"{i}. [{status}] {task.title}")
            if task.description:
                print(f"   描述: {task.description}")
            print()

    def complete_task(self, task_number: int) -> bool:
        """标记任务完成"""
        if 1 <= task_number <= len(self.tasks):
            task = self.tasks[task_number - 1]
            task.mark_completed()
            self.save_tasks()
            print(f"✓ 任务完成: {task.title}")
            return True
        else:
            print("✗ 任务编号无效")
            return False

    def delete_task(self, task_number: int) -> bool:
        """删除任务"""
        if 1 <= task_number <= len(self.tasks):
            task = self.tasks.pop(task_number - 1)
            self.save_tasks()
            print(f"✓ 删除任务: {task.title}")
            return True
        else:
            print("✗ 任务编号无效")
            return False

    def save_tasks(self) -> None:
        """保存到文件"""
        data = [task.to_dict() for task in self.tasks]
        with open(self.data_file, "w", encoding="utf-8") as f:
            json.dump(data, f, ensure_ascii=False, indent=2)

    def load_tasks(self) -> None:
        """从文件加载"""
        if os.path.exists(self.data_file):
            with open(self.data_file, "r", encoding="utf-8") as f:
                data = json.load(f)
                self.tasks = [Task.from_dict(item) for item in data]

    def get_stats(self) -> Dict[str, int]:
        """获取统计信息"""
        total = len(self.tasks)
        completed = sum(1 for t in self.tasks if t.completed)
        pending = total - completed
        return {
            "total": total,
            "completed": completed,
            "pending": pending
        }


def print_menu():
    """打印菜单"""
    print("\n" + "=" * 60)
    print("任务管理器")
    print("=" * 60)
    print("1. 添加任务")
    print("2. 列出任务")
    print("3. 完成任务")
    print("4. 删除任务")
    print("5. 查看统计")
    print("6. 退出")
    print("=" * 60)


def main():
    """主函数"""
    manager = TaskManager()

    while True:
        print_menu()
        choice = input("请选择操作 (1-6): ").strip()

        if choice == "1":
            # 添加任务
            title = input("任务标题: ").strip()
            if not title:
                print("标题不能为空")
                continue

            description = input("任务描述（可选）: ").strip()
            manager.add_task(title, description)

        elif choice == "2":
            # 列出任务
            show_all = input("显示已完成任务？(y/n): ").strip().lower() == "y"
            manager.list_tasks(show_completed)

        elif choice == "3":
            # 完成任务
            manager.list_tasks()
            try:
                num = int(input("输入任务编号: "))
                manager.complete_task(num)
            except ValueError:
                print("请输入有效的数字")

        elif choice == "4":
            # 删除任务
            manager.list_tasks(show_completed=True)
            try:
                num = int(input("输入要删除的任务编号: "))
                manager.delete_task(num)
            except ValueError:
                print("请输入有效的数字")

        elif choice == "5":
            # 统计
            stats = manager.get_stats()
            print(f"\n统计:")
            print(f"  总任务: {stats['total']}")
            print(f"  已完成: {stats['completed']}")
            print(f"  待完成: {stats['pending']}")

            if stats['total'] > 0:
                completion_rate = (stats['completed'] / stats['total']) * 100
                print(f"  完成率: {completion_rate:.1f}%")

        elif choice == "6":
            # 退出
            print("再见！")
            break

        else:
            print("无效的选择，请重试")


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("\n\n程序已中断")
    except Exception as e:
        print(f"发生错误: {e}")
