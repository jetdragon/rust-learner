# 多线程和并发演示

"""
本文件演示Python的多线程和并发编程基础。
注意：由于GIL（全局解释器锁），Python多线程适合I/O密集型任务。
"""

import threading
import time
from concurrent.futures import ThreadPoolExecutor, as_completed


def main():
    print("=== Python 并发编程演示 ===\n")

    # ========== 1. 基础线程 ==========
    print("1. 创建和启动线程")

    def worker(name: str, duration: int):
        """工作线程函数"""
        print(f"线程 {name} 开始工作")
        time.sleep(duration)
        print(f"线程 {name} 完成工作（耗时{duration}秒）")

    # 创建线程
    thread1 = threading.Thread(target=worker, args=("A", 2))
    thread2 = threading.Thread(target=worker, args=("B", 1))

    # 启动线程
    thread1.start()
    thread2.start()

    # 等待线程完成
    thread1.join()
    thread2.join()
    print("所有线程完成")
    print()

    # ========== 2. 线程子类 ==========
    print("2. 继承 Thread 类")

    class MyThread(threading.Thread):
        """自定义线程类"""

        def __init__(self, name: str, count: int):
            super().__init__()
            self.name = name
            self.count = count

        def run(self):
            """线程执行的方法"""
            for i in range(self.count):
                print(f"{self.name}: 计数 {i+1}")
                time.sleep(0.5)

    thread = MyThread("计数线程", 3)
    thread.start()
    thread.join()
    print()

    # ========== 3. 线程同步 - Lock ==========
    print("3. 线程同步 - Lock（互斥锁）")

    counter = 0
    lock = threading.Lock()

    def increment():
        """增加计数器"""
        global counter
        for _ in range(100000):
            with lock:  # 获取锁
                counter += 1

    # 创建多个线程
    threads = []
    for _ in range(5):
        t = threading.Thread(target=increment)
        threads.append(t)
        t.start()

    # 等待所有线程
    for t in threads:
        t.join()

    print(f"最终计数器值: {counter}")
    print("(使用Lock确保线程安全)")
    print()

    # ========== 4. 线程间通信 - Queue ==========
    print("4. 线程间通信 - Queue")

    import queue

    def producer(q: queue.Queue):
        """生产者"""
        for i in range(5):
            item = f"项目 {i}"
            q.put(item)
            print(f"生产: {item}")
            time.sleep(0.5)

    def consumer(q: queue.Queue):
        """消费者"""
        while True:
            item = q.get()
            if item == "DONE":
                break
            print(f"消费: {item}")
            time.sleep(0.3)
            q.task_done()

    # 创建队列
    q = queue.Queue()

    # 创建生产者和消费者线程
    prod_thread = threading.Thread(target=producer, args=(q,))
    cons_thread = threading.Thread(target=consumer, args=(q,))

    prod_thread.start()
    cons_thread.start()

    prod_thread.join()
    q.put("DONE")  # 发送结束信号
    cons_thread.join()
    print()

    # ========== 5. ThreadPoolExecutor ==========
    print("5. 线程池（ThreadPoolExecutor）")

    def task(name: str) -> str:
        """任务函数"""
        print(f"执行任务 {name}")
        time.sleep(1)
        return f"{name} 完成"

    # 使用线程池
    with ThreadPoolExecutor(max_workers=3) as executor:
        # 提交任务
        futures = {
            executor.submit(task, f"Task-{i}"): f"Task-{i}"
            for i in range(5)
        }

        # 获取结果
        for future in as_completed(futures):
            task_name = futures[future]
            try:
                result = future.result()
                print(f"结果: {result}")
            except Exception as e:
                print(f"{task_name} 异常: {e}")
    print()

    # ========== 6. GIL 说明 ==========
    print("6. Python GIL（全局解释器锁）")
    print("GIL 说明:")
    print("  - 同一时刻只有一个线程执行Python字节码")
    print("  - 多线程不适合CPU密集型任务")
    print("  - 多线程适合I/O密集型任务（网络、文件）")
    print()
    print("解决方案:")
    print("  - CPU密集: 使用 multiprocessing（多进程）")
    print("  - I/O密集: 使用 threading（多线程）")
    print("  - 现代: 使用 asyncio（异步I/O）")
    print()

    # ========== 7. 线程安全的工具 ==========
    print("7. 线程安全的数据结构")

    # 普通字典（不安全）
    # normal_dict = {}

    # 线程安全字典
    from collections import defaultdict

    safe_dict = defaultdict(int)
    lock = threading.Lock()

    def increment_dict(key: str):
        for _ in range(1000):
            with lock:
                safe_dict[key] += 1

    threads = []
    for key in ["A", "B", "C"]:
        t = threading.Thread(target=increment_dict, args=(key,))
        threads.append(t)
        t.start()

    for t in threads:
        t.join()

    print("线程安全字典:")
    for key, value in safe_dict.items():
        print(f"  {key}: {value}")
    print()

    print("=== 演示完成 ===")
    print()
    print("💡 多线程最佳实践:")
    print("  1. I/O密集型用线程，CPU密集型用进程")
    print("  2. 使用 ThreadPoolExecutor 管理线程")
    print("  3. 使用 Lock 避免竞态条件")
    print("  4. 使用 Queue 进行线程间通信")
    print("  5. 避免共享状态，优先使用消息传递")


if __name__ == "__main__":
    main()
