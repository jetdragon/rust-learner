// examples/iterator_basics.rs
// Iterator trait 基础演示

fn main() {
    println!("=== Iterator trait 基础演示 ===\n");

    // 1. Iterator 基础
    println!("1. Iterator 基础:");
    let v = vec![1, 2, 3, 4, 5];
    let mut iter = v.iter();

    println!("  使用 next() 遍历:");
    while let Some(val) = iter.next() {
        println!("    {}", val);
    }

    // 2. iter(), iter_mut(), into_iter()
    println!("\n2. iter(), iter_mut(), into_iter():");

    // iter() - 不可变引用
    let v = vec![1, 2, 3];
    for val in v.iter() {
        println!("  iter(): {}", val);
    }

    // iter_mut() - 可变引用
    let mut v = vec![1, 2, 3];
    for val in v.iter_mut() {
        *val *= 2;
    }
    println!("  iter_mut() 后: {:?}", v);

    // into_iter() - 消耗所有权
    let v = vec![1, 2, 3];
    for val in v.into_iter() {
        println!("  into_iter(): {}", val);
    }

    // 3. for 循环使用迭代器
    println!("\n3. for 循环使用迭代器:");
    let v = vec!["a", "b", "c"];
    for (i, item) in v.iter().enumerate() {
        println!("  [{}]: {}", i, item);
    }

    // 4. 惰性求值
    println!("\n4. 惰性求值:");
    let lazy_iter = (1..).filter(|&x| x % 2 == 0);
    println!("  创建迭代器（未计算）");

    let first_few: Vec<_> = lazy_iter.take(5).collect();
    println!("  只计算前 5 个: {:?}", first_few);
}
