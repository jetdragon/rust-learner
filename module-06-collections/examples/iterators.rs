// examples/iterators.rs
// 迭代器演示

fn main() {
    println!("=== 迭代器演示 ===\n");

    // 1. map
    println!("1. map:");
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("  {:?}", doubled);

    // 2. filter
    println!("\n2. filter:");
    let evens: Vec<&i32> = numbers.iter().filter(|x| *x % 2 == 0).collect();
    println!("  {:?}", evens);

    // 3. filter_map
    println!("\n3. filter_map:");
    let parsed: Vec<Option<i32>> = vec!["1", "abc", "3"]
        .iter()
        .map(|s| s.parse::<i32>().ok())
        .collect();
    println!("  {:?}", parsed);

    // 4. fold
    println!("\n4. fold:");
    let sum: i32 = numbers.iter().fold(0, |acc, x| acc + x);
    println!("  sum = {}", sum);

    // 5. any/all
    println!("\n5. any/all:");
    println!("  any > 3: {}", numbers.iter().any(|&x| x > 3));
    println!("  all > 0: {}", numbers.iter().all(|&x| x > 0));

    // 6. find
    println!("\n6. find:");
    let found = numbers.iter().find(|&&x| x > 2);
    println!("  first > 2: {:?}", found);

    // 7. enumerate
    println!("\n7. enumerate:");
    for (i, val) in numbers.iter().enumerate() {
        println!("  index {}: {}", i, val);
    }

    // 8. take/skip
    println!("\n8. take/skip:");
    let taken: Vec<&i32> = numbers.iter().take(3).collect();
    println!("  take(3): {:?}", taken);

    let skipped: Vec<&i32> = numbers.iter().skip(2).collect();
    println!("  skip(2): {:?}", skipped);

    // 9. chain
    println!("\n9. chain:");
    let combined: Vec<&i32> = numbers.iter().chain(numbers.iter().rev()).take(7).collect();
    println!("  {:?}", combined);

    // 10. zip
    println!("\n10. zip:");
    let names = vec!["Alice", "Bob", "Charlie"];
    let scores = vec![10, 20, 30];
    let pairs: Vec<_> = names.iter().zip(scores.iter()).collect();
    println!("  {:?}", pairs);

    // 11. sum/product
    println!("\n11. sum/product:");
    println!("  sum: {}", numbers.iter().sum::<i32>());
    println!("  product: {}", numbers.iter().product::<i32>());

    // 12. min/max
    println!("\n12. min/max:");
    println!("  min: {:?}", numbers.iter().min());
    println!("  max: {:?}", numbers.iter().max());
}
