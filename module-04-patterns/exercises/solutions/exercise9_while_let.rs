//! # 练习 9 解答: while let 循环
//!
//! **提示**：请先尝试自己实现，再参考此解答！
//!
//! # 实现思路
//!
//! 1. while let：持续匹配模式直到失败
//! 2. 递归结构：适合遍历递归数据结构
//! 3. 可变借用：需要可变引用来修改指针
//! 4. 循环条件：只要模式匹配成功就继续

/// 使用 while let 遍历链表
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

/// 使用 while let 遍历并打印所有元素
pub fn print_list(list: &List) {
    let mut current = list;

    while let List::Cons(val, next) = current {
        println!("{}", val);
        current = next;
    }

    println!("链表遍历完成");
}

/// 计算链表长度
pub fn list_length(list: &List) -> usize {
    let mut current = list;
    let mut count = 0;

    while let List::Cons(_, next) = current {
        count += 1;
        current = next;
    }

    count
}

/// 计算链表元素之和
pub fn sum_list(list: &List) -> i32 {
    let mut current = list;
    let mut sum = 0;

    while let List::Cons(val, next) = current {
        sum += val;
        current = next;
    }

    sum
}

/// 查找链表中是否包含某个值
pub fn contains(list: &List, target: i32) -> bool {
    let mut current = list;

    while let List::Cons(val, next) = current {
        if val == target {
            return true;
        }
        current = next;
    }

    false
}

/// 获取链表的最后一个元素
pub fn last_element(list: &List) -> Option<i32> {
    let mut current = list;
    let mut last_val = None;

    while let List::Cons(val, next) = current {
        last_val = Some(val);
        current = next;
    }

    last_val
}

/// 创建链表的辅助函数
pub fn create_list(values: &[i32]) -> List {
    let mut list = List::Nil;
    for &val in values.iter().rev() {
        list = List::Cons(val, Box::new(list));
    }
    list
}

fn main() {
    let list = create_list(&[1, 2, 3, 4, 5]);

    println!("=== 链表遍历 ===\n");
    print_list(&list);

    println!("\n长度: {}", list_length(&list));
    println!("求和: {}", sum_list(&list));
    println!("包含3: {}", contains(&list, 3));
    println!("包含10: {}", contains(&list, 10));
    println!("最后一个元素: {:?}", last_element(&list));

    // 空链表示例
    println!("\n=== 空链表 ===");
    let empty: List = List::Nil;
    println!("空链表长度: {}", list_length(&empty));
    println!("空链表求和: {}", sum_list(&empty));

    // 单元素链表
    println!("\n=== 单元素链表 ===");
    let single = List::Cons(42, Box::new(List::Nil));
    println!("长度: {}", list_length(&single));
    println!("最后一个元素: {:?}", last_element(&single));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_length() {
        let list = create_list(&[1, 2, 3]);
        assert_eq!(list_length(&list), 3);

        assert_eq!(list_length(&List::Nil), 0);
    }

    #[test]
    fn test_sum_list() {
        let list = create_list(&[1, 2, 3]);
        assert_eq!(sum_list(&list), 6);

        assert_eq!(sum_list(&List::Nil), 0);
    }

    #[test]
    fn test_contains() {
        let list = create_list(&[1, 2, 3, 4, 5]);
        assert!(contains(&list, 3));
        assert!(!contains(&list, 10));
        assert!(!contains(&List::Nil, 1));
    }

    #[test]
    fn test_last_element() {
        let list = create_list(&[1, 2, 3, 4, 5]);
        assert_eq!(last_element(&list), Some(5));

        assert_eq!(last_element(&List::Nil), None);

        let single = List::Cons(42, Box::new(List::Nil));
        assert_eq!(last_element(&single), Some(42));
    }

    #[test]
    fn test_create_list() {
        let list = create_list(&[1, 2, 3]);
        assert_eq!(sum_list(&list), 6);
        assert_eq!(list_length(&list), 3);
    }

    #[test]
    fn test_while_let_with_counter() {
        // 使用 while let 和计数器
        let list = create_list(&[10, 20, 30, 40, 50]);
        let mut current = &list;
        let mut count = 0;

        while let List::Cons(val, next) = current {
            assert!(val > 0);
            current = next;
            count += 1;
        }

        assert_eq!(count, 5);
    }

    #[test]
    fn test_while_let_early_exit() {
        // 提前退出 while let
        let list = create_list(&[1, 2, 3, 4, 5]);
        let mut current = &list;
        let mut found = false;

        while let List::Cons(val, next) = current {
            if val == 3 {
                found = true;
                break;
            }
            current = next;
        }

        assert!(found);
    }
}
