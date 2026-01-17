//! # 练习 10: 解构嵌套结构
//!
//! **难度**: 困难
//! **预计时间**: 20 分钟

/// 员工角色
#[derive(Debug)]
pub enum Role {
    Manager { department: String },
    Engineer { level: u32, language: String },
    Intern,
}

/// 员工
#[derive(Debug)]
pub struct Employee {
    pub name: String,
    pub role: Role,
}

/// TODO: 描述员工信息
///
/// 匹配各种嵌套情况：
/// - Manager { department } => "{name} 是 {department} 部门经理"
/// - Engineer { level, language } => "{name} 是 {level} 级工程师，使用 {language}"
/// - Intern => "{name} 是实习生"
pub fn describe_employee(emp: &Employee) -> String {
    match emp {
        Employee {
            name,
            role: Role::Manager { department },
        } => {
            format!("{} 是 {} 部门经理", name, department)
        }
        Employee {
            name,
            role: Role::Engineer { level, language },
        } => {
            format!("{} 是 {} 级工程师，使用 {}", name, level, language)
        }
        Employee {
            name,
            role: Role::Intern,
        } => {
            format!("{} 是实习生", name)
        }
    }
}

/// TODO: 判断是否为高级工程师（level >= 5）
pub fn is_senior_engineer(emp: &Employee) -> bool {
    unimplemented!()
}

fn main() {
    let employees = vec![
        Employee {
            name: "张三".to_string(),
            role: Role::Manager {
                department: "技术".to_string(),
            },
        },
        Employee {
            name: "李四".to_string(),
            role: Role::Engineer {
                level: 3,
                language: "Rust".to_string(),
            },
        },
        Employee {
            name: "王五".to_string(),
            role: Role::Intern,
        },
    ];

    println!("=== 员工信息 ===\n");
    for emp in &employees {
        println!("{}", describe_employee(emp));
        println!("高级工程师: {}\n", is_senior_engineer(emp));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe_employee() {
        let manager = Employee {
            name: "Alice".to_string(),
            role: Role::Manager {
                department: "HR".to_string(),
            },
        };
        assert_eq!(describe_employee(&manager), "Alice 是 HR 部门经理");

        let engineer = Employee {
            name: "Bob".to_string(),
            role: Role::Engineer {
                level: 5,
                language: "Python".to_string(),
            },
        };
        assert_eq!(
            describe_employee(&engineer),
            "Bob 是 5 级工程师，使用 Python"
        );

        let intern = Employee {
            name: "Charlie".to_string(),
            role: Role::Intern,
        };
        assert_eq!(describe_employee(&intern), "Charlie 是实习生");
    }

    #[test]
    fn test_is_senior_engineer() {
        let senior = Employee {
            name: "Alice".to_string(),
            role: Role::Engineer {
                level: 5,
                language: "Rust".to_string(),
            },
        };
        assert!(is_senior_engineer(&senior));

        let junior = Employee {
            name: "Bob".to_string(),
            role: Role::Engineer {
                level: 2,
                language: "Python".to_string(),
            },
        };
        assert!(!is_senior_engineer(&junior));

        let manager = Employee {
            name: "Charlie".to_string(),
            role: Role::Manager {
                department: "Tech".to_string(),
            },
        };
        assert!(!is_senior_engineer(&manager));
    }
}
