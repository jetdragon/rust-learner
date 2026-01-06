//! # 练习 1: 基本 match 表达式
//!
//! **难度**: 入门
//! **预计时间**: 10 分钟

/// 根据分数返回评级
///
/// # TODO
///
/// 返回评级：
/// - 90-100: "优秀"
/// - 80-89: "良好"
/// - 60-79: "及格"
/// - 0-59: "不及格"
/// - 其他: "无效分数"
pub fn grade(score: u32) -> &'static str {
    match score {
        90..=100 => "优秀",
        80..=89 => "良好",
        60..=79 => "及格",
        0..=59 => "不及格",
        _ => "无效分数",
    }
}

fn main() {
    let scores = [95, 85, 70, 50, 105];

    for score in scores {
        println!("分数 {}: {}", score, grade(score));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grade() {
        assert_eq!(grade(95), "优秀");
        assert_eq!(grade(85), "良好");
        assert_eq!(grade(70), "及格");
        assert_eq!(grade(50), "不及格");
    }

    #[test]
    fn test_boundary() {
        assert_eq!(grade(90), "优秀");
        assert_eq!(grade(89), "良好");
        assert_eq!(grade(60), "及格");
        assert_eq!(grade(59), "不及格");
    }

    #[test]
    fn test_invalid() {
        assert_eq!(grade(101), "无效分数");
        assert_eq!(grade(200), "无效分数");
    }
}
