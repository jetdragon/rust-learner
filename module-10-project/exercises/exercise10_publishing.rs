//! # 练习 10: 发布到 crates.io
//!
//! 难度: 中等
//! 时间: 20 分钟
//! 前置知识: Cargo 基础
//! 学习目标:
//!   - 准备包发布
//!   - 理解发布流程

// 1. 准备 Cargo.toml
/*
[package]
name = "my_crate"  # 在 crates.io 上唯一的名称
version = "0.1.0"  # 遵循语义化版本
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
description = "A brief description"
license = "MIT"  # 或 Apache-2.0 等 SPDX 许可证
repository = "https://github.com/yourusername/my_crate"

[dependencies]
# 依赖项
*/

// 2. 添加文档
//! # My Crate
//!
//! 这是一个示例库，展示如何发布到 crates.io
//!
//! ## 功能
//!
//! - 功能 1
//! - 功能 2
//!
//! ## 使用示例
//!
//! ```
//! use my_crate::add;
//!
//! let result = add(2, 3);
//! assert_eq!(result, 5);
//! ```

// 3. 实现库功能
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// 4. 发布步骤
/*
1. 注册 crates.io 账号
2. 运行: cargo login
3. 检查包: cargo publish --dry-run
4. 发布: cargo publish
5. 访问 https://crates.io/crates/my_crate
*/

// 5. 版本管理
/*
0.1.0 -> 0.2.0: 新增功能（但可能有不兼容变更）
0.2.0 -> 0.2.1: Bug 修复（向后兼容）
0.2.1 -> 1.0.0: 稳定版本发布
*/

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
