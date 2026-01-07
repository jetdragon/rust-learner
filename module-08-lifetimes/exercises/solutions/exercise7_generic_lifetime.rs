//! # 练习 7: 泛型与生命周期结合 - 解答

use std::fmt::Display;

pub fn announce_and_return<'a, T>(item: &'a T, announcement: &str) -> &'a T
where
    T: Display,
{
    println!("Announcement: {}", announcement);
    item
}

pub struct Ref<'a, T: ?Sized> {
    value: &'a T,
}

impl<'a, T> Ref<'a, T> {
    pub fn new(value: &'a T) -> Self {
        Ref { value }
    }

    pub fn get(&self) -> &'a T {
        self.value
    }
}

fn main() {
    let number = 42;
    let result = announce_and_return(&number, "Here's a number");
    println!("Result: {}", result);

    let text = "Hello";
    let text_ref = Ref::new(text);
    println!("Text: {}", text_ref.get());
}
