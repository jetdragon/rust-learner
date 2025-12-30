// 02-所有权系统 模块测试

// ========== 练习 1: 所有权转移 ==========
#[test]
fn test_ownership_transfer() {
    let s1 = String::from("hello");
    let s2 = s1;
    // s1 不再有效
    assert_eq!(s2, "hello");

    // Copy 类型
    let x = 5;
    let y = x;
    assert_eq!(x, 5);  // x 仍然有效
    assert_eq!(y, 5);
}

// ========== 练习 2: 函数与所有权 ==========
#[test]
fn test_function_ownership() {
    use module_02_ownership::{take_ownership, makes_copy};

    let s = String::from("hello");
    take_ownership(s);
    // s 不再有效

    let x = 5;
    makes_copy(x);
    assert_eq!(x, 5);  // x 仍然有效（Copy 类型）
}

// ========== 练习 3: 引用与借用 ==========
#[test]
fn test_calculate_length() {
    use module_02_ownership::calculate_length;

    let s = String::from("hello");
    let len = calculate_length(&s);
    assert_eq!(len, 5);
    assert_eq!(s, "hello");  // s 仍然有效
}

// ========== 练习 4: 可变引用 ==========
#[test]
fn test_mutable_reference() {
    use module_02_ownership::change;

    let mut s = String::from("hello");
    change(&mut s);
    assert_eq!(s, "hello, world");
}

// ========== 练习 5: 借用规则 ==========
#[test]
fn test_borrow_rules() {
    let mut s = String::from("hello");

    // 多个不可变引用
    let r1 = &s;
    let r2 = &s;
    assert_eq!(r1, r2);

    // 不可变引用不再使用后，可以创建可变引用
    let r3 = &mut s;
    r3.push_str(" world");
    assert_eq!(s, "hello world");
}

// ========== 练习 6: 字符串切片 ==========
#[test]
fn test_first_word() {
    use module_02_ownership::first_word;

    let s = String::from("hello world");
    assert_eq!(first_word(&s), "hello");

    let s = String::from("hello");
    assert_eq!(first_word(&s), "hello");
}

// ========== 练习 7: 数组切片 ==========
#[test]
fn test_array_slice() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..4];
    assert_eq!(slice, &[2, 3, 4]);
}

// ========== 练习 8: 悬垂引用（正确做法） ==========
#[test]
fn test_no_dangle() {
    use module_02_ownership::no_dangle;

    let s = no_dangle();
    assert_eq!(s, "hello");
}

// ========== 所有权规则测试 ==========
#[test]
fn test_ownership_rules() {
    use module_02_ownership::{gives_ownership, takes_and_gives_back};

    // 规则 1: 每个值都有一个所有者
    let s = String::from("rule1");

    // 规则 2: 同一时间只能有一个所有者
    let s1 = String::from("rule2");
    let s2 = s1;
    assert_eq!(s2, "rule2");

    // 规则 3: 所有者超出作用域时值被丢弃
    {
        let s3 = String::from("rule3");
        assert_eq!(s3, "rule3");
    }
    // s3 已被丢弃

    // 函数转移所有权
    let s4 = gives_ownership();
    assert_eq!(s4, "yours");

    let s5 = String::from("hello");
    let s6 = takes_and_gives_back(s5);
    assert_eq!(s6, "hello, world");
}

// ========== Copy 类型测试 ==========
#[test]
fn test_copy_types() {
    // 整数是 Copy 类型
    let x: i32 = 5;
    let y = x;
    assert_eq!(x, 5);
    assert_eq!(y, 5);

    // 布尔是 Copy 类型
    let b1: bool = true;
    let b2 = b1;
    assert_eq!(b1, true);
    assert_eq!(b2, true);

    // 浮点是 Copy 类型
    let f1: f64 = 3.14;
    let f2 = f1;
    assert_eq!(f1, 3.14);
    assert_eq!(f2, 3.14);

    // 字符是 Copy 类型
    let c1: char = 'a';
    let c2 = c1;
    assert_eq!(c1, 'a');
    assert_eq!(c2, 'a');

    // 元组（如果所有元素都是 Copy）
    let t1 = (1, 2, 3);
    let t2 = t1;
    assert_eq!(t1, (1, 2, 3));
    assert_eq!(t2, (1, 2, 3));
}

// ========== 移动语义测试 ==========
#[test]
fn test_move_semantics() {
    // String 不是 Copy 类型
    let s1 = String::from("hello");
    let s2 = s1;  // 移动
    assert_eq!(s2, "hello");
    // s1 不再有效

    // Vec 不是 Copy 类型
    let v1 = vec![1, 2, 3];
    let v2 = v1;  // 移动
    assert_eq!(v2, vec![1, 2, 3]);
    // v1 不再有效
}

// ========== Clone 深拷贝测试 ==========
#[test]
fn test_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();  // 深拷贝
    assert_eq!(s1, "hello");
    assert_eq!(s2, "hello");
}

// ========== 不可变引用测试 ==========
#[test]
fn test_immutable_borrow() {
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    // 多个不可变引用同时存在
    assert_eq!(r1, "hello");
    assert_eq!(r2, "hello");
    assert_eq!(s, "hello");
}

// ========== 可变引用测试 ==========
#[test]
fn test_mutable_borrow() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        r1.push_str(" world");
    }  // r1 作用域结束

    let r2 = &mut s;
    r2.push_str("!");

    assert_eq!(s, "hello world!");
}

// ========== 切片测试 ==========
#[test]
fn test_slices() {
    // 字符串切片
    let s = String::from("hello world");
    let hello = &s[0..5];
    assert_eq!(hello, "hello");

    let world = &s[6..11];
    assert_eq!(world, "world");

    // 数组切片
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    // 完整切片
    let whole = &a[..];
    assert_eq!(whole, &[1, 2, 3, 4, 5]);
}

// ========== 字符串字面量切片测试 ==========
#[test]
fn test_string_literal_slice() {
    use module_02_ownership::first_word_improved;

    let s = "hello world";
    assert_eq!(first_word_improved(s), "hello");
}

// ========== 引用作用域测试 ==========
#[test]
fn test_reference_scope() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // r1 和 r2 最后一次使用后，不再有效

    let r3 = &mut s;
    r3.push_str(" world");
    assert_eq!(s, "hello world");
}

// ========== first_word_improved 测试 ==========
#[test]
fn test_first_word_improved() {
    use module_02_ownership::first_word_improved;

    // 正常情况
    let s = "hello world";
    assert_eq!(first_word_improved(s), "hello");

    // 没有空格
    let s = "hello";
    assert_eq!(first_word_improved(s), "hello");

    // 多个空格
    let s = "hello world rust";
    assert_eq!(first_word_improved(s), "hello");

    // 空字符串
    let s = "";
    assert_eq!(first_word_improved(s), "");
}

// ========== 切片边界测试 ==========
#[test]
fn test_slice_boundaries() {
    let s = String::from("hello");

    // 有效边界
    assert_eq!(&s[0..1], "h");
    assert_eq!(&s[1..5], "ello");
    assert_eq!(&s[..5], "hello");
    assert_eq!(&s[0..], "hello");
    assert_eq!(&s[..], "hello");
}

// ========== 综合测试：所有权 + 引用 + 切片 ==========
#[test]
fn test_comprehensive() {
    use module_02_ownership::{calculate_length, first_word};

    let mut s = String::from("hello world");

    // 使用引用，不转移所有权
    let len = calculate_length(&s);
    assert_eq!(len, 11);

    // 获取切片
    let word = first_word(&s);
    assert_eq!(word, "hello");

    // 修改字符串
    s.push_str("!");
    assert_eq!(s, "hello world!");

    // 仍然可以使用原字符串
    assert_eq!(s.len(), 12);
}
