//! # 练习 10 解答: 解构嵌套结构
//!
//! **提示**：请先尝试自己实现，再参考此解答！
//!
//! # 实现思路
//!
//! 1. 嵌套模式：在模式中嵌套其他模式
//! 2. 深度解构：可以多层嵌套解构
//! 3. 结构体枚举组合：解构包含枚举的结构体
//! 4. 灵活匹配：同时匹配多个层级

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

/// 描述员工信息
pub fn describe_employee(emp: &Employee) -> String {
    match emp {
        Employee { name, role: Role::Manager { department } } => {
            format!("{} 是 {} 部门经理", name, department)
        }
        Employee { name, role: Role::Engineer { level, language } } => {
            format!("{} 是 {} 级工程师，使用 {}", name, level, language)
        }
        Employee { name, role: Role::Intern } => {
            format!("{} 是实习生", name)
        }
    }
}

/// 判断是否为高级工程师（level >= 5）
pub fn is_senior_engineer(emp: &Employee) -> bool {
    match emp {
        Employee { role: Role::Engineer { level, .. }, .. } => *level >= 5,
        _ => false,
    }
}

/// 获取员工的部门（如果有）
pub fn get_department(emp: &Employee) -> Option<&str> {
    match emp {
        Employee { role: Role::Manager { department }, .. } => Some(department.as_str()),
        _ => None,
    }
}

/// 获取工程师的编程语言
pub fn get_engineering_language(emp: &Employee) -> Option<&str> {
    match emp {
        Employee { role: Role::Engineer { language, .. }, .. } => Some(language.as_str()),
        _ => None,
    }
}

/// 检查是否使用特定语言
pub fn uses_language(emp: &Employee, lang: &str) -> bool {
    match emp {
        Employee { role: Role::Engineer { language, .. }, .. } => language == lang,
        _ => false,
    }
}

fn main() {
    let employees = vec![
        Employee {
            name: "张三".to_string(),
            role: Role::Manager { department: "技术".to_string() },
        },
        Employee {
            name: "李四".to_string(),
            role: Role::Engineer { level: 3, language: "Rust".to_string() },
        },
        Employee {
            name: "王五".to_string(),
            role: Role::Intern,
        },
        Employee {
            name: "赵六".to_string(),
            role: Role::Engineer { level: 5, language: "Python".to_string() },
        },
    ];

    println!("=== 员工信息 ===\n");
    for emp in &employees {
        println!("{}", describe_employee(emp));
        println!("高级工程师: {}", is_senior_engineer(emp));
        println!("部门: {:?}", get_department(emp));
        println!("编程语言: {:?}", get_engineering_language(emp));
        println!("使用 Rust: {}", uses_language(emp, "Rust"));
        println!();
    }

    // 演示嵌套解构
    println!("=== 嵌套解构示例 ===");
    let emp = Employee {
        name: "Alice".to_string(),
        role: Role::Engineer { level: 3, language: "Rust".to_string() },
    };

    match emp {
        Employee { name, role: Role::Engineer { language, level } } => {
            println!("{} 使用 {}，级别 {}", name, language, level);
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe_employee() {
        let manager = Employee {
            name: "Alice".to_string(),
            role: Role::Manager { department: "HR".to_string() },
        };
        assert_eq!(describe_employee(&manager), "Alice 是 HR 部门经理");

        let engineer = Employee {
            name: "Bob".to_string(),
            role: Role::Engineer { level: 5, language: "Python".to_string() },
        };
        assert_eq!(describe_employee(&engineer), "Bob 是 5 级工程师，使用 Python");

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
            role: Role::Engineer { level: 5, language: "Rust".to_string() },
        };
        assert!(is_senior_engineer(&senior));

        let junior = Employee {
            name: "Bob".to_string(),
            role: Role::Engineer { level: 2, language: "Python".to_string() },
        };
        assert!(!is_senior_engineer(&junior));

        let manager = Employee {
            name: "Charlie".to_string(),
            role: Role::Manager { department: "Tech".to_string() },
        };
        assert!(!is_senior_engineer(&manager));
    }

    #[test]
    fn test_get_department() {
        let manager = Employee {
            name: "Alice".to_string(),
            role: Role::Manager { department: "HR".to_string() },
        };
        assert_eq!(get_department(&manager), Some("HR"));

        let engineer = Employee {
            name: "Bob".to_string(),
            role: Role::Engineer { level: 3, language: "Rust".to_string() },
        };
        assert_eq!(get_department(&engineer), None);
    }

    #[test]
    fn test_get_engineering_language() {
        let engineer = Employee {
            name: "Alice".to_string(),
            role: Role::Engineer { level: 3, language: "Rust".to_string() },
        };
        assert_eq!(get_engineering_language(&engineer), Some("Rust"));

        let manager = Employee {
            name: "Bob".to_string(),
            role: Role::Manager { department: "Tech".to_string() },
        };
        assert_eq!(get_engineering_language(&manager), None);
    }

    #[test]
    fn test_uses_language() {
        let rust_engineer = Employee {
            name: "Alice".to_string(),
            role: Role::Engineer { level: 3, language: "Rust".to_string() },
        };
        assert!(uses_language(&rust_engineer, "Rust"));
        assert!(!uses_language(&rust_engineer, "Python"));

        let python_engineer = Employee {
            name: "Bob".to_string(),
            role: Role::Engineer { level: 5, language: "Python".to_string() },
        };
        assert!(uses_language(&python_engineer, "Python"));
    }

    #[test]
    fn test_wildcard_in_nested_pattern() {
        // 在嵌套模式中使用通配符
        let emp = Employee {
            name: "Alice".to_string(),
            role: Role::Engineer { level: 3, language: "Rust".to_string() },
        };

        // 忽略 name 和其他字段
        match emp {
            Employee { role: Role::Engineer { language, .. }, .. } => {
                assert_eq!(language, "Rust");
            }
            _ => panic!("应该是工程师"),
        }
    }

    #[test]
    fn test_deep_nesting() {
        // 深度嵌套的模式匹配
        let company = vec![
            Employee {
                name: "Alice".to_string(),
                role: Role::Manager { department: "Tech".to_string() },
            },
            Employee {
                name: "Bob".to_string(),
                role: Role::Engineer { level: 5, language: "Rust".to_string() },
            },
        ];

        // 匹配向量中特定索引的特定模式
        match company.get(1) {
            Some(Employee { name, role: Role::Engineer { level, .. }, .. }) => {
                assert_eq!(name, "Bob");
                assert_eq!(*level, 5);
            }
            _ => panic!("应该找到 Bob"),
        }
    }

    #[test]
    fn test_or_patterns_in_nested_structures() {
        // 在嵌套结构中使用或模式
        let emp1 = Employee {
            name: "Alice".to_string(),
            role: Role::Engineer { level: 3, language: "Rust".to_string() },
        };

        let result = match emp1 {
            Employee { role: Role::Manager { .. }, .. } => "manager",
            Employee { role: Role::Engineer { level: l @ 1..=3, .. }, .. } => "junior",
            Employee { role: Role::Engineer { level: l @ 4..=5, .. }, .. } => "senior",
            Employee { role: Role::Engineer { .. }, .. } => "expert",
            Employee { role: Role::Intern, .. } => "intern",
        };

        assert_eq!(result, "junior");
    }
}
