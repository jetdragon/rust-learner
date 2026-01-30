// examples/cons_list.rs
// 递归类型（Cons List）演示

use std::rc::Rc;

fn main() {
    println!("=== 递归类型（Cons List）演示 ===\n");

    // 1. 为什么需要 Box？
    println!("1. 为什么递归类型需要 Box:");
    // 以下代码无法编译（Rust 无法确定大小）：
    //
    // enum List {
    //     Cons(i32, List),  // 编译错误！
    //     Nil,
    // }
    //
    // 使用 Box 解决：

    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::{Cons, Nil};

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("  List: {:?}", list);

    // 2. 链表基本操作
    println!("\n2. 链表基本操作:");

    // 创建列表
    let list = Cons(5, Box::new(Cons(10, Box::new(Cons(15, Box::new(Nil))))));
    println!("  创建列表: {:?}", list);

    // 遍历列表
    print!("  遍历列表: ");
    let mut current = &list;
    loop {
        match current {
            Cons(val, tail) => {
                print!("{} -> ", val);
                current = tail;
            }
            Nil => {
                println!("Nil");
                break;
            }
        }
    }

    // 3. 计算列表长度
    println!("\n3. 计算列表长度:");
    fn list_length(list: &List) -> usize {
        match list {
            Cons(_, tail) => 1 + list_length(tail),
            Nil => 0,
        }
    }

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("  列表长度: {}", list_length(&list));

    // 4. 列表求和
    println!("\n4. 列表求和:");
    fn list_sum(list: &List) -> i32 {
        match list {
            Cons(val, tail) => val + list_sum(tail),
            Nil => 0,
        }
    }

    println!("  列表和: {}", list_sum(&list));

    // 5. 反转列表
    println!("\n5. 反转列表:");
    fn reverse_list(list: List) -> List {
        fn helper(current: List, acc: List) -> List {
            match current {
                Cons(val, tail) => helper(*tail, Cons(val, Box::new(acc))),
                Nil => acc,
            }
        }
        helper(list, Nil)
    }

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let reversed = reverse_list(list);
    println!("  反转后: {:?}", reversed);

    // 6. 共享列表尾部（Rc）
    println!("\n6. 共享列表尾部（Rc）:");

    #[derive(Debug)]
    enum RcList {
        Cons(i32, Rc<RcList>),
        Nil,
    }

    use RcList::{Cons, Nil};

    let shared_tail = Rc::new(Cons(10, Rc::new(Cons(20, Rc::new(Nil)))));
    println!("  共享尾部: {:?}", shared_tail);

    let list_a = Cons(1, Rc::clone(&shared_tail));
    let list_b = Cons(2, Rc::clone(&shared_tail));
    let list_c = Cons(3, Rc::clone(&shared_tail));

    println!("  list_a: {:?}", list_a);
    println!("  list_b: {:?}", list_b);
    println!("  list_c: {:?}", list_c);
    println!("  共享尾部的引用计数: {}", Rc::strong_count(&shared_tail));

    // 7. 实际应用：树结构
    println!("\n7. 实际应用：树结构");

    #[derive(Debug)]
    enum Tree {
        Leaf(i32),
        Node(Box<Tree>, Box<Tree>),
    }

    use Tree::{Leaf, Node};

    // 创建树
    //       5
    //      / \
    //     3   7
    //    / \
    //   1   4
    let tree = Node(
        Box::new(Node(
            Box::new(Node(Box::new(Leaf(1)), Box::new(Leaf(4)))),
            Box::new(Leaf(3)),
        )),
        Box::new(Leaf(7)),
    );
    println!("  树: {:?}", tree);

    // 树的深度
    fn tree_depth(tree: &Tree) -> usize {
        match tree {
            Leaf(_) => 1,
            Node(left, right) => 1 + tree_depth(left).max(tree_depth(right)),
        }
    }

    println!("  树的深度: {}", tree_depth(&tree));
}
