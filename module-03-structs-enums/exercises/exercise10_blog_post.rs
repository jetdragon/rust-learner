//! # 练习 10: 综合应用 - 博客文章状态机
//!
//! **难度**: 困难
//! **预计时间**: 40 分钟
//!
//! **前置知识**:
//! - 结构体定义和实现
//! - 枚举定义和 match
//! - 所有权和借用
//!
//! **学习目标**:
//! - 学习如何使用枚举实现状态机
//! - 理解状态转换的概念
//! - 掌握复杂的业务逻辑实现

/// 文章状态枚举
///
/// # TODO
///
/// 定义 PostState 枚举，包含三个变体：
/// - Draft（草稿）
/// - PendingReview（待审核）
/// - Published（已发布）
pub enum PostState {
    Draft,
    PendingReview,
    Published,
}

/// 博客文章结构体
///
/// # TODO
///
/// 定义 Post 结构体，包含：
/// - title: String
/// - content: String
/// - state: PostState
pub struct Post {
    pub title: String,
    pub content: String,
    pub state: PostState,
}

impl Post {
    /// TODO: 创建新文章（草稿状态）
    pub fn new(title: String, content: String) -> Self {
        unimplemented!()
    }

    /// TODO: 请求审核
    ///
    /// 只能从 Draft 转换到 PendingReview
    pub fn request_review(&mut self) -> Result<(), String> {
        unimplemented!()
    }

    /// TODO: 批准发布
    ///
    /// 只能从 PendingReview 转换到 Published
    pub fn approve(&mut self) -> Result<(), String> {
        unimplemented!()
    }

    /// TODO: 获取文章内容
    ///
    /// 只有 Published 状态才能查看内容
    /// 其他状态返回 None
    pub fn content(&self) -> Option<&str> {
        unimplemented!()
    }

    /// TODO: 拒绝审核（回到草稿）
    ///
    /// 从 PendingReview 回到 Draft
    pub fn reject(&mut self) -> Result<(), String> {
        unimplemented!()
    }

    /// 获取当前状态
    pub fn state(&self) -> &PostState {
        &self.state
    }

    /// 返回状态描述
    pub fn state_description(&self) -> &str {
        match self.state {
            PostState::Draft => "草稿",
            PostState::PendingReview => "待审核",
            PostState::Published => "已发布",
        }
    }

    /// TODO: 编辑文章
    ///
    /// 只有草稿状态可以编辑
    pub fn edit(&mut self, new_title: String, new_content: String) -> Result<(), String> {
        unimplemented!()
    }
}

fn main() {
    println!("=== 博客文章状态机 ===\n");

    // 创建草稿
    let mut post = Post::new(
        String::from("我的第一篇博客"),
        String::from("这是内容..."),
    );

    println!("状态: {}", post.state_description());
    println!("内容: {:?}", post.content());

    // 尝试查看内容
    match post.content() {
        Some(content) => println!("文章内容: {}", content),
        None => println!("文章尚未发布，无法查看内容"),
    }

    // 编辑文章
    println!("\n--- 编辑文章 ---");
    match post.edit(
        String::from("我的第一篇博客（已编辑）"),
        String::from("这是更新后的内容..."),
    ) {
        Ok(_) => println!("编辑成功"),
        Err(e) => println!("编辑失败: {}", e),
    }

    // 请求审核
    println!("\n--- 请求审核 ---");
    match post.request_review() {
        Ok(_) => println!("已提交审核"),
        Err(e) => println!("提交失败: {}", e),
    }
    println!("状态: {}", post.state_description());

    // 尝试在审核状态下编辑
    match post.edit(
        String::from("试图编辑"),
        String::from("应该失败"),
    ) {
        Ok(_) => println!("编辑成功"),
        Err(e) => println!("编辑失败: {}", e),
    }

    // 拒绝审核
    println!("\n--- 拒绝审核 ---");
    match post.reject() {
        Ok(_) => println!("已拒绝，回到草稿"),
        Err(e) => println!("拒绝失败: {}", e),
    }
    println!("状态: {}", post.state_description());

    // 再次请求审核
    println!("\n--- 再次请求审核 ---");
    let _ = post.request_review();
    println!("状态: {}", post.state_description());

    // 批准发布
    println!("\n--- 批准发布 ---");
    match post.approve() {
        Ok(_) => println!("已发布"),
        Err(e) => println!("发布失败: {}", e),
    }
    println!("状态: {}", post.state_description());

    // 查看内容
    println!("\n--- 查看内容 ---");
    match post.content() {
        Some(content) => println!("文章内容: {}", content),
        None => println!("无法查看内容"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_post_is_draft() {
        let post = Post::new(
            String::from("Title"),
            String::from("Content"),
        );

        assert!(matches!(post.state(), PostState::Draft));
        assert_eq!(post.content(), None);
    }

    #[test]
    fn test_draft_to_pending_review() {
        let mut post = Post::new(
            String::from("Title"),
            String::from("Content"),
        );

        assert!(post.request_review().is_ok());
        assert!(matches!(post.state(), PostState::PendingReview));
    }

    #[test]
    fn test_pending_review_to_published() {
        let mut post = Post::new(
            String::from("Title"),
            String::from("Content"),
        );

        post.request_review().unwrap();
        assert!(post.approve().is_ok());
        assert!(matches!(post.state(), PostState::Published));
    }

    #[test]
    fn test_published_post_has_content() {
        let mut post = Post::new(
            String::from("Title"),
            String::from("Content"),
        );

        post.request_review().unwrap();
        post.approve().unwrap();
        assert_eq!(post.content(), Some("Content"));
    }

    #[test]
    fn test_cannot_skip_review() {
        let mut post = Post::new(
            String::from("Title"),
            String::from("Content"),
        );

        // 不能从草稿直接发布
        assert!(post.approve().is_err());
        assert!(matches!(post.state(), PostState::Draft));
    }

    #[test]
    fn test_reject_returns_to_draft() {
        let mut post = Post::new(
            String::from("Title"),
            String::from("Content"),
        );

        post.request_review().unwrap();
        post.reject().unwrap();
        assert!(matches!(post.state(), PostState::Draft));
    }

    #[test]
    fn test_only_draft_can_edit() {
        let mut post = Post::new(
            String::from("Old Title"),
            String::from("Old Content"),
        );

        // 草稿状态可以编辑
        assert!(post.edit(
            String::from("New Title"),
            String::from("New Content")
        ).is_ok());

        // 待审核状态不能编辑
        post.request_review().unwrap();
        assert!(post.edit(
            String::from("Another Title"),
            String::from("Another Content")
        ).is_err());
    }

    #[test]
    fn test_invalid_state_transitions() {
        let mut post = Post::new(
            String::from("Title"),
            String::from("Content"),
        );

        // 草稿不能批准
        assert!(post.approve().is_err());

        // 草稿不能拒绝
        assert!(post.reject().is_err());

        post.request_review().unwrap();

        // 待审核不能再次请求审核
        assert!(post.request_review().is_err());

        post.approve().unwrap();

        // 已发布不能更改状态
        assert!(post.request_review().is_err());
        assert!(post.approve().is_err());
        assert!(post.reject().is_err());
    }

    #[test]
    fn test_full_workflow() {
        let mut post = Post::new(
            String::from("Title"),
            String::from("Content"),
        );

        // 草稿
        assert!(matches!(post.state(), PostState::Draft));
        assert_eq!(post.content(), None);

        // 编辑草稿
        assert!(post.edit(
            String::from("Updated"),
            String::from("Updated Content")
        ).is_ok());

        // 请求审核
        assert!(post.request_review().is_ok());
        assert!(matches!(post.state(), PostState::PendingReview));

        // 拒绝
        assert!(post.reject().is_ok());
        assert!(matches!(post.state(), PostState::Draft));

        // 再次请求审核
        assert!(post.request_review().is_ok());

        // 批准
        assert!(post.approve().is_ok());
        assert!(matches!(post.state(), PostState::Published));

        // 查看内容
        assert_eq!(post.content(), Some("Updated Content"));
    }

    #[test]
    fn test_state_description() {
        let post = Post::new(
            String::from("Title"),
            String::from("Content"),
        );

        assert_eq!(post.state_description(), "草稿");

        let mut post = Post::new(
            String::from("Title"),
            String::from("Content"),
        );
        post.request_review().unwrap();
        assert_eq!(post.state_description(), "待审核");

        post.approve().unwrap();
        assert_eq!(post.state_description(), "已发布");
    }
}
