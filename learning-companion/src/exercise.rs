//! 练习系统模块
//!
//! 生成练习题、判分和分析掌握程度

use crate::repo::LearningRepo;
use anyhow::Result;
use rand::seq::SliceRandom;

/// 练习题目
#[derive(Debug, Clone, PartialEq)]
pub struct Question {
    pub question_type: QuestionType,
    pub prompt: String,
    pub options: Option<Vec<String>>,
    pub correct_answer: String,
    pub explanation: String,
    pub topic: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum QuestionType {
    MultipleChoice,
    TrueFalse,
    FillInBlank,
}

/// 练习会话
pub struct PracticeSession {
    pub module_id: String,
    pub questions: Vec<Question>,
    pub answers: Vec<usize>,
}

impl PracticeSession {
    pub fn new(module_id: String, questions: Vec<Question>) -> Self {
        Self {
            module_id,
            questions,
            answers: Vec::new(),
        }
    }

    pub fn score(&self) -> f32 {
        if self.questions.is_empty() {
            return 0.0;
        }

        let correct = self
            .answers
            .iter()
            .zip(self.questions.iter())
            .filter(|(answer, q)| {
                // 简化处理：假设答案索引对应正确选项
                *answer == &q.correct_answer.parse().unwrap_or(0)
            })
            .count();

        (correct as f32 / self.questions.len() as f32) * 100.0
    }

    pub fn weak_topics(&self) -> Vec<String> {
        let mut topics = Vec::new();

        for (answer, question) in self.answers.iter().zip(self.questions.iter()) {
            if *answer != question.correct_answer.parse().unwrap_or(0) {
                topics.push(question.topic.clone());
            }
        }

        topics
    }
}

/// 生成基础入门模块的练习题
pub fn generate_basics_questions(count: usize) -> Vec<Question> {
    let all_questions = vec![
        Question {
            question_type: QuestionType::MultipleChoice,
            prompt: "Rust 中声明不可变变量的关键字是什么？".to_string(),
            options: Some(vec![
                "let".to_string(),
                "var".to_string(),
                "const".to_string(),
                "mut".to_string(),
            ]),
            correct_answer: "0".to_string(),
            explanation: "let 是 Rust 中声明变量的关键字，默认创建不可变变量。".to_string(),
            topic: "变量声明".to_string(),
        },
        Question {
            question_type: QuestionType::MultipleChoice,
            prompt: "如何声明一个可变变量？".to_string(),
            options: Some(vec![
                "let mut x = 5;".to_string(),
                "let x = 5;".to_string(),
                "var x = 5;".to_string(),
                "const mut x = 5;".to_string(),
            ]),
            correct_answer: "0".to_string(),
            explanation: "使用 let mut 关键字可以声明可变变量。".to_string(),
            topic: "可变性".to_string(),
        },
        Question {
            question_type: QuestionType::TrueFalse,
            prompt: "Rust 中，默认情况下变量是不可变的。".to_string(),
            options: Some(vec!["正确".to_string(), "错误".to_string()]),
            correct_answer: "0".to_string(),
            explanation: "是的，Rust 默认变量不可变，这是为了安全性。".to_string(),
            topic: "可变性".to_string(),
        },
        Question {
            question_type: QuestionType::MultipleChoice,
            prompt: "i32 类型的整数范围是多少？".to_string(),
            options: Some(vec![
                "-128 到 127".to_string(),
                "0 到 255".to_string(),
                "-2^31 到 2^31-1".to_string(),
                "-2^63 到 2^63-1".to_string(),
            ]),
            correct_answer: "2".to_string(),
            explanation: "i32 是 32 位有符号整数，范围是 -2^31 到 2^31-1。".to_string(),
            topic: "数据类型".to_string(),
        },
        Question {
            question_type: QuestionType::MultipleChoice,
            prompt: "Rust 中哪个类型表示布尔值？".to_string(),
            options: Some(vec![
                "bool".to_string(),
                "boolean".to_string(),
                "bit".to_string(),
                "flag".to_string(),
            ]),
            correct_answer: "0".to_string(),
            explanation: "Rust 使用 bool 类型表示布尔值，值为 true 或 false。".to_string(),
            topic: "数据类型".to_string(),
        },
        Question {
            question_type: QuestionType::TrueFalse,
            prompt: "元组可以包含不同类型的值。".to_string(),
            options: Some(vec!["正确".to_string(), "错误".to_string()]),
            correct_answer: "0".to_string(),
            explanation: "是的，元组可以将不同类型的值组合在一起。".to_string(),
            topic: "复合类型".to_string(),
        },
        Question {
            question_type: QuestionType::MultipleChoice,
            prompt: "函数返回值的表达式应该：".to_string(),
            options: Some(vec![
                "以分号结尾".to_string(),
                "不以分号结尾".to_string(),
                "使用 return 关键字".to_string(),
                "使用 yield 关键字".to_string(),
            ]),
            correct_answer: "1".to_string(),
            explanation: "Rust 中，表达式不以分号结尾会自动作为返回值。".to_string(),
            topic: "函数".to_string(),
        },
        Question {
            question_type: QuestionType::MultipleChoice,
            prompt: "如何访问元组的第一个元素？".to_string(),
            options: Some(vec![
                "tuple.1".to_string(),
                "tuple.0".to_string(),
                "tuple[0]".to_string(),
                "tuple.first()".to_string(),
            ]),
            correct_answer: "1".to_string(),
            explanation: "使用 tuple.0 访问元组的第一个元素（索引从 0 开始）。".to_string(),
            topic: "复合类型".to_string(),
        },
        Question {
            question_type: QuestionType::TrueFalse,
            prompt: "数组在 Rust 中可以有不同类型的元素。".to_string(),
            options: Some(vec!["正确".to_string(), "错误".to_string()]),
            correct_answer: "1".to_string(),
            explanation: "错误。数组的所有元素必须是相同类型。".to_string(),
            topic: "复合类型".to_string(),
        },
        Question {
            question_type: QuestionType::MultipleChoice,
            prompt: "char 类型在 Rust 中占用多少字节？".to_string(),
            options: Some(vec![
                "1 字节".to_string(),
                "2 字节".to_string(),
                "4 字节".to_string(),
                "8 字节".to_string(),
            ]),
            correct_answer: "2".to_string(),
            explanation: "Rust 的 char 类型是 Unicode 字符，占用 4 字节。".to_string(),
            topic: "数据类型".to_string(),
        },
    ];

    // 随机选择指定数量的问题
    let mut rng = rand::thread_rng();
    let mut indices: Vec<usize> = (0..all_questions.len()).collect();
    indices.shuffle(&mut rng);

    indices
        .iter()
        .take(count.min(all_questions.len()))
        .map(|&i| all_questions[i].clone())
        .collect()
}

/// 运行练习测试
pub fn run_practice(_repo: &LearningRepo, module_id: &str, count: usize) -> Result<()> {
    println!("\n╔════════════════════════════════════════╗");
    println!("║        📝 Rust 学习伴侣 - 练习测试      ║");
    println!("╚════════════════════════════════════════╝\n");

    let questions = match module_id {
        "module-01-basics" | "01-基础入门" | "basics" => generate_basics_questions(count),
        _ => {
            println!("❌ 暂不支持该模块的练习题");
            println!("💡 目前支持：module-01-basics（基础入门）");
            return Ok(());
        }
    };

    if questions.is_empty() {
        println!("❌ 没有可用的练习题");
        return Ok(());
    }

    let mut session = PracticeSession::new(module_id.to_string(), questions);
    let mut correct_count = 0;

    for (i, q) in session.questions.iter().enumerate() {
        println!(
            "📚 题目 {}/{} - [{}]",
            i + 1,
            session.questions.len(),
            q.topic
        );
        println!("{}\n", q.prompt);

        if let Some(options) = &q.options {
            for (j, opt) in options.iter().enumerate() {
                println!("  {}. {}", j + 1, opt);
            }
        }

        print!("\n你的答案 (输入选项编号): ");
        use std::io::{self, Write};
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let answer: usize = input.trim().parse().unwrap_or(0);

        // 记录答案（转换为 0-indexed）
        let answer_index = if answer > 0 { answer - 1 } else { 0 };
        session.answers.push(answer_index);

        println!();
        if answer_index == q.correct_answer.parse().unwrap_or(0) {
            println!("✅ 正确！");
            correct_count += 1;
        } else {
            println!("❌ 错误！");
        }

        println!("💡 解析：{}\n", q.explanation);
        println!("{}\n", "─".repeat(50));
    }

    // 显示结果
    let score = session.score();
    let weak = session.weak_topics();

    println!("\n╔════════════════════════════════════════╗");
    println!("║            📊 测试结果                ║");
    println!("╚════════════════════════════════════════╝");
    println!(
        "\n正确率：{:.1}% ({}/{})",
        score,
        correct_count,
        session.questions.len()
    );

    if score >= 95.0 {
        println!("🎉 太棒了！掌握程度：优秀");
    } else if score >= 80.0 {
        println!("👍 做得不错！掌握程度：良好");
    } else if score >= 60.0 {
        println!("💪 继续努力！掌握程度：及格");
    } else {
        println!("📚 需要复习，加油！");
    }

    if !weak.is_empty() {
        println!("\n建议复习的知识点：");
        for topic in &weak {
            println!("  • {}", topic);
        }
    }

    // 记录结果
    crate::db::record_practice_result(
        module_id,
        session.questions.len() as u32,
        correct_count as u32,
        score,
        weak,
    )?;

    Ok(())
}
