// examples/consumer_adapters.rs
// 消费者适配器演示

fn main() {
    println!("=== 消费者适配器演示 ===\n");

    // 1. collect()
    println!("1. collect():");
    let v = vec![1, 2, 3, 4, 5];
    let squared: Vec<i32> = v.iter().map(|x| x * x).collect();
    println!("  平方: {:?}", squared);

    // 2. sum(), product()
    println!("\n2. sum(), product():");
    let v = vec![1, 2, 3, 4, 5];
    println!("  sum: {}", v.iter().sum::<i32>());
    println!("  product: {}", v.iter().product::<i32>());

    // 3. min(), max()
    println!("\n3. min(), max():");
    let v = vec![3, 1, 4, 1, 5];
    println!("  min: {:?}", v.iter().min());
    println!("  max: {:?}", v.iter().max());

    // 4. find(), position()
    println!("\n4. find(), position():");
    let v = vec![1, 2, 3, 4, 5];
    println!("  find(3): {:?}", v.iter().find(|&&x| x == 3));
    println!("  position(3): {:?}", v.iter().position(|&x| x == 3));

    // 5. any(), all()
    println!("\n5. any(), all():");
    let v = vec![1, 2, 3, 4, 5];
    println!("  any even: {}", v.iter().any(|&x| x % 2 == 0));
    println!("  all positive: {}", v.iter().all(|&x| x > 0));

    // 6. fold(), reduce()
    println!("\n6. fold(), reduce():");
    let v = vec![1, 2, 3, 4, 5];
    println!("  fold: {}", v.iter().fold(0, |acc, &x| acc + x));
    println!("  reduce: {:?}", v.iter().reduce(|acc, &x| *acc + x));

    // 7. count(), last(), nth()
    println!("\n7. count(), last(), nth():");
    let v = vec![1, 2, 3, 4, 5];
    println!("  count: {}", v.iter().count());
    println!("  last: {:?}", v.iter().last());
    println!("  nth(2): {:?}", v.iter().nth(2));
}
