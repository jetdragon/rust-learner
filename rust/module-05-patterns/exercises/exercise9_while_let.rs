//! # 练习 9: while let 循环
//!
//! **难度**: 中等
//! **预计时间**: 15 分钟

/// 使用 while let 遍历链表
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

/// TODO: 使用 while let 遍历并打印所有元素
pub fn print_list(list: &List) {
    let mut current = list;

    while let List::Cons(val, next) = current {
        println!("{}", val);
        current = next;
    }
}

/// TODO: 计算链表长度
pub fn list_length(list: &List) -> usize {
    unimplemented!()
}

/// TODO: 计算链表元素之和
pub fn sum_list(list: &List) -> i32 {
    unimplemented!()
}

fn main() {
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    println!("=== 链表遍历 ===\n");
    print_list(&list);

    println!("\n长度: {}", list_length(&list));
    println!("求和: {}", sum_list(&list));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_length() {
        let list = List::Cons(
            1,
            Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
        );
        assert_eq!(list_length(&list), 3);

        assert_eq!(list_length(&List::Nil), 0);
    }

    #[test]
    fn test_sum_list() {
        let list = List::Cons(
            1,
            Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
        );
        assert_eq!(sum_list(&list), 6);

        assert_eq!(sum_list(&List::Nil), 0);
    }
}
