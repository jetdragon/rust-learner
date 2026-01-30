pub fn box_basics_example() {
    let _b = Box::new(5);
}

pub fn cons_list_example() {
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::{Cons, Nil};

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
}

pub fn rc_example() {
    use std::rc::Rc;
    let a = Rc::new(5);
    let b = Rc::clone(&a);
    println!("a = {}, b = {}", a, b);
}

pub fn arc_example() {
    use std::sync::Arc;
    let a = Arc::new(5);
    let b = Arc::clone(&a);
    println!("a = {}, b = {}", a, b);
}

pub fn refcell_example() {
    use std::cell::RefCell;
    let data = RefCell::new(5);
    *data.borrow_mut() += 1;
    println!("data = {}", data.borrow());
}
