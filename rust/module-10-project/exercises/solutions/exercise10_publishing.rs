//! # 练习 10: 发布到 crates.io - 解答
//!
//! 难度: 中等
//! 时间: 20 分钟
//! 学习目标: 准备包发布

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn main() {
    println!("2 + 3 = {}", add(2, 3));
    println!("2 * 3 = {}", multiply(2, 3));

    println!("\n准备发布的检查清单:");
    println!("1. [ ] 在 Cargo.toml 中设置唯一的包名");
    println!("2. [ ] 添加作者和描述信息");
    println!("3. [ ] 设置许可证 (MIT 或 Apache-2.0)");
    println!("4. [ ] 添加仓库链接");
    println!("5. [ ] 编写文档注释");
    println!("6. [ ] 运行 cargo test 确保所有测试通过");
    println!("7. [ ] 运行 cargo clippy 检查代码质量");
    println!("8. [ ] 运行 cargo fmt 格式化代码");
    println!("9. [ ] 运行 cargo publish --dry-run 预检查");
    println!("10. [ ] 运行 cargo publish 发布");
}
