// examples/iterator_adapters.rs
// 迭代器适配器演示

fn main() {
    println!("=== 迭代器适配器演示 ===\n");

    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // map, filter
    let doubled_evens: Vec<_> = v.iter().map(|x| x * 2).filter(|x| x > &10).collect();
    println!(" doubled_evens: {:?}", doubled_evens);

    // take, skip
    let first_five: Vec<_> = v.iter().take(5).cloned().collect();
    let last_five: Vec<_> = v.iter().skip(5).cloned().collect();
    println!(" first_five: {:?}", first_five);
    println!(" last_five: {:?}", last_five);

    // enumerate
    for (i, val) in v.iter().enumerate().take(5) {
        println!("  [{}]: {}", i, val);
    }

    // zip
    let v2 = vec!["a", "b", "c", "d", "e"];
    let zipped: Vec<_> = v.iter().zip(v2.iter()).take(3).collect();
    println!(" zipped: {:?}", zipped);
}
