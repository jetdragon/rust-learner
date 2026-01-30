// examples/vec_basics.rs
// Vec 基础演示

use module_06_collections::{
    double_elements, filter_even, find_index, find_largest, frequency_map, group_by_parity,
    sort_descending, top_n, vec_filter_positive, vec_sum,
};

fn main() {
    println!("=== Vec 基础演示 ===\n");

    // 1. 创建 Vec
    println!("1. 创建 Vec:");
    let mut vec1 = Vec::new();
    vec1.push(1);
    vec1.push(2);

    let vec2 = vec![1, 2, 3];
    println!("  vec1 = {:?}", vec1);
    println!("  vec2 = {:?}", vec2);

    // 2. 访问元素
    println!("\n2. 访问元素:");
    let vec = vec![10, 20, 30];
    println!("  vec[0] = {}", vec[0]);
    println!("  vec.get(1) = {:?}", vec.get(1));
    println!("  vec.get(10) = {:?}", vec.get(10));

    // 3. 求和
    println!("\n3. 求和:");
    let numbers = vec![1, 2, 3, 4, 5];
    println!("  sum = {}", vec_sum(&numbers));

    // 4. 过滤
    println!("\n4. 过滤:");
    println!("  正数: {:?}", vec_filter_positive(&numbers));
    println!("  偶数: {:?}", filter_even(&numbers));

    // 5. 转换
    println!("\n5. 转换:");
    println!("  翻倍: {:?}", double_elements(&numbers));

    // 6. 查找
    println!("\n6. 查找:");
    println!("  最大值: {:?}", find_largest(&numbers));
    println!("  索引3: {:?}", find_index(&numbers, 3));

    // 7. 分组
    println!("\n7. 分组:");
    let (evens, odds) = group_by_parity(&numbers);
    println!("  偶数: {:?}", evens);
    println!("  奇数: {:?}", odds);

    // 8. 排序
    println!("\n8. 排序:");
    println!("  降序: {:?}", sort_descending(vec![3, 1, 2]));

    // 9. 频率
    println!("\n9. 频率:");
    let data = vec![1, 2, 2, 3, 3, 3];
    println!("  频率: {:?}", frequency_map(&data));

    // 10. Top N
    println!("\n10. Top N:");
    println!("  前3: {:?}", top_n(&data, 3));
}
