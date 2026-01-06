//! # 练习 10 解答: 博客文章状态机
//!
//! **提示**：请先尝试自己实现，再参考此解答！
//!
//! # 实现思路
//!
//! 1. 状态机：使用枚举表示状态
//! 2. 状态转换：控制状态之间的转换规则
//! 3. 封装：确保只有通过方法才能改变状态
//! 4. 验证：返回 Result 表示转换是否成功

/// 文章状态枚举
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PostState {
    Draft,
    PendingReview,
    Published,
}

/// 博客文章结构体
pub struct Post {
    pub title: String,
    pub content: String,
    pub state: PostState,
}

impl Post {
    /// 创建新文章（草稿状态）
    ///
    /// # 实现思路
    /// 新文章默认为草稿状态
    pub fn new(title: String, content: String) -> Self {
        Post {
            title,
            content,
            state: PostState::Draft,
        }
    }

    /// 请求审核
    ///
    /// # 实现思路
    /// 只能从 Draft 转换到 PendingReview
    pub fn request_review(&mut self) -> Result<(), String> {
        match self.state {
            PostState::Draft => {
                self.state = PostState::PendingReview;
                Ok(())
            }
            _ => Err(format!("无法从 {:?} 转换到待审核", self.state)),
        }
    }

    /// 批准发布
    ///
    /// # 实现思路
    /// 只能从 PendingReview 转换到 Published
    pub fn approve(&mut self) -> Result<(), String> {
        match self.state {
            PostState::PendingReview => {
                self.state = PostState::Published;
                Ok(())
            }
            _ => Err(format!("无法从 {:?} 转换到已发布", self.state)),
        }
    }

    /// 获取文章内容
    ///
    /// # 实现思路
    /// 只有 Published 状态才能查看内容
    pub fn content(&self) -> Option<&str> {
        match self.state {
            PostState::Published => Some(&self.content),
            _ => None,
        }
    }

    /// 拒绝审核（回到草稿）
    ///
    /// # 实现思路
    /// 从 PendingReview 回到 Draft
    pub fn reject(&mut self) -> Result<(), String> {
        match self.state {
            PostState::PendingReview => {
                self.state = PostState::Draft;
                Ok(())
            }
            _ => Err(format!("无法从 {:?} 转换到草稿", self.state)),
        }
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

    /// 编辑文章
    ///
    /// # 实现思路
    /// 只有草稿状态可以编辑
    pub fn edit(&mut self, new_title: String, new_content: String) -> Result<(), String> {
        match self.state {
            PostState::Draft => {
                self.title = new_title;
                self.content = new_content;
                Ok(())
            }
            _ => Err("只有草稿状态可以编辑".to_string()),
        }
    }

    /// 检查是否可以编辑
    pub fn is_editable(&self) -> bool {
        matches!(self.state, PostState::Draft)
    }

    /// 检查内容是否可见
    pub fn is_content_visible(&self) -> bool {
        matches!(self.state, PostState::Published)
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

    #[test]
    fn test_is_editable() {
        let mut post = Post::new(
            String::from("Title"),
            String::from("Content"),
        );

        assert!(post.is_editable());

        post.request_review().unwrap();
        assert!(!post.is_editable());

        post.reject().unwrap();
        assert!(post.is_editable());
    }

    #[test]
    fn test_is_content_visible() {
        let mut post = Post::new(
            String::from("Title"),
            String::from("Content"),
        );

        assert!(!post.is_content_visible());

        post.request_review().unwrap();
        assert!(!post.is_content_visible());

        post.approve().unwrap();
        assert!(post.is_content_visible());
    }
}
