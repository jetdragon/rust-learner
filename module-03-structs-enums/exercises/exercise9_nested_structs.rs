//! # 练习 9: 嵌套结构体
//!
//! **难度**: 困难
//! **预计时间**: 30 分钟
//!
//! **前置知识**:
//! - 结构体基础定义
//! - 结构体方法
//! - Vec 类型基础
//!
//! **学习目标**:
//! - 学习如何定义嵌套结构体
//! - 理解结构体间的组合关系
//! - 掌握复杂结构的操作方法

/// 地址结构体
///
/// # TODO
///
/// 定义 Address 结构体，包含：
/// - street: String
/// - city: String
/// - zip_code: String
pub struct Address {
    pub street: String,
    pub city: String,
    pub zip_code: String,
}

impl Address {
    /// 创建新地址
    pub fn new(street: String, city: String, zip_code: String) -> Self {
        unimplemented!()
    }

    /// 返回完整地址字符串
    pub fn full_address(&self) -> String {
        unimplemented!()
    }
}

/// 员工结构体
///
/// # TODO
///
/// 定义 Employee 结构体，包含：
/// - name: String
/// - position: String
/// - address: Address（嵌套结构体）
pub struct Employee {
    pub name: String,
    pub position: String,
    pub address: Address,
}

impl Employee {
    /// 创建新员工
    pub fn new(name: String, position: String, address: Address) -> Self {
        unimplemented!()
    }

    /// 返回员工描述
    pub fn describe(&self) -> String {
        unimplemented!()
    }

    /// 升职
    pub fn promote(&mut self, new_position: String) {
        unimplemented!()
    }
}

/// 公司结构体
///
/// # TODO
///
/// 定义 Company 结构体，包含：
/// - name: String
/// - employees: Vec<Employee>
pub struct Company {
    pub name: String,
    pub employees: Vec<Employee>,
}

impl Company {
    /// 创建新公司
    pub fn new(name: String) -> Self {
        unimplemented!()
    }

    /// TODO: 添加员工
    pub fn add_employee(&mut self, employee: Employee) {
        unimplemented!()
    }

    /// TODO: 返回员工数量
    pub fn employee_count(&self) -> usize {
        unimplemented!()
    }

    /// TODO: 根据姓名查找员工
    pub fn find_employee(&self, name: &str) -> Option<&Employee> {
        unimplemented!()
    }

    /// TODO: 列出所有员工姓名
    pub fn list_employees(&self) -> Vec<&str> {
        unimplemented!()
    }

    /// TODO: 移除员工
    pub fn remove_employee(&mut self, name: &str) -> Result<(), String> {
        unimplemented!()
    }
}

fn main() {
    println!("=== 嵌套结构体示例 ===\n");

    // 创建地址
    let address1 = Address::new(
        String::from("123 Main St"),
        String::from("New York"),
        String::from("10001"),
    );

    let address2 = Address::new(
        String::from("456 Oak Ave"),
        String::from("Boston"),
        String::from("02108"),
    );

    // 创建员工
    let mut emp1 = Employee::new(
        String::from("张三"),
        String::from("工程师"),
        address1,
    );

    let emp2 = Employee::new(
        String::from("李四"),
        String::from("设计师"),
        address2,
    );

    println!("{}", emp1.describe());

    // 创建公司并添加员工
    let mut company = Company::new(String::from("Tech Corp"));
    company.add_employee(emp1);
    company.add_employee(emp2);

    println!("\n--- 公司信息 ---");
    println!("公司: {}", company.name);
    println!("员工数量: {}", company.employee_count());

    println!("\n--- 员工列表 ---");
    for name in company.list_employees() {
        println!("- {}", name);
    }

    // 查找员工
    println!("\n--- 查找员工 ---");
    if let Some(emp) = company.find_employee("张三") {
        println!("找到: {}", emp.describe());
    }

    // 升职
    if let Some(emp) = &mut company.employees.iter_mut().find(|e| e.name == "李四") {
        emp.promote(String::from("高级设计师"));
    }

    // 移除员工
    println!("\n--- 移除员工 ---");
    match company.remove_employee("张三") {
        Ok(_) => println!("成功移除员工"),
        Err(e) => println!("移除失败: {}", e),
    }

    println!("剩余员工数: {}", company.employee_count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_creation() {
        let addr = Address::new(
            String::from("123 St"),
            String::from("City"),
            String::from("12345"),
        );

        assert_eq!(addr.street, "123 St");
        assert_eq!(addr.city, "City");
        assert_eq!(addr.zip_code, "12345");
    }

    #[test]
    fn test_address_full() {
        let addr = Address::new(
            String::from("456 Ave"),
            String::from("Boston"),
            String::from("02108"),
        );

        let full = addr.full_address();
        assert!(full.contains("456 Ave"));
        assert!(full.contains("Boston"));
        assert!(full.contains("02108"));
    }

    #[test]
    fn test_employee_creation() {
        let addr = Address::new(
            String::from("123 St"),
            String::from("NYC"),
            String::from("10001"),
        );

        let emp = Employee::new(
            String::from("Alice"),
            String::from("Developer"),
            addr,
        );

        assert_eq!(emp.name, "Alice");
        assert_eq!(emp.position, "Developer");
        assert_eq!(emp.address.city, "NYC");
    }

    #[test]
    fn test_employee_promote() {
        let addr = Address::new(
            String::from("1 St"),
            String::from("City"),
            String::from("00000"),
        );

        let mut emp = Employee::new(
            String::from("Bob"),
            String::from("Junior"),
            addr,
        );

        emp.promote(String::from("Senior"));
        assert_eq!(emp.position, "Senior");
    }

    #[test]
    fn test_company_creation() {
        let company = Company::new(String::from("Test Co"));
        assert_eq!(company.name, "Test Co");
        assert_eq!(company.employee_count(), 0);
    }

    #[test]
    fn test_add_employee() {
        let mut company = Company::new(String::from("Test"));

        let addr = Address::new(
            String::from("1 St"),
            String::from("City"),
            String::from("00000"),
        );

        let emp = Employee::new(
            String::from("Alice"),
            String::from("Dev"),
            addr,
        );

        company.add_employee(emp);
        assert_eq!(company.employee_count(), 1);
    }

    #[test]
    fn test_find_employee() {
        let mut company = Company::new(String::from("Test"));

        let addr = Address::new(
            String::from("1 St"),
            String::from("City"),
            String::from("00000"),
        );

        let emp = Employee::new(
            String::from("Alice"),
            String::from("Dev"),
            addr,
        );

        company.add_employee(emp);

        let found = company.find_employee("Alice");
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Alice");

        let not_found = company.find_employee("Bob");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_list_employees() {
        let mut company = Company::new(String::from("Test"));

        let addr1 = Address::new(
            String::from("1 St"),
            String::from("City"),
            String::from("00000"),
        );

        let addr2 = Address::new(
            String::from("2 St"),
            String::from("City"),
            String::from("00000"),
        );

        company.add_employee(Employee::new(String::from("Alice"), String::from("Dev"), addr1));
        company.add_employee(Employee::new(String::from("Bob"), String::from("Designer"), addr2));

        let names = company.list_employees();
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"Alice"));
        assert!(names.contains(&"Bob"));
    }

    #[test]
    fn test_remove_employee() {
        let mut company = Company::new(String::from("Test"));

        let addr = Address::new(
            String::from("1 St"),
            String::from("City"),
            String::from("00000"),
        );

        company.add_employee(Employee::new(String::from("Alice"), String::from("Dev"), addr));

        assert_eq!(company.employee_count(), 1);

        let result = company.remove_employee("Alice");
        assert!(result.is_ok());
        assert_eq!(company.employee_count(), 0);

        let result2 = company.remove_employee("Alice");
        assert!(result2.is_err());
    }
}
