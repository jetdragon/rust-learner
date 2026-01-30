//! # 练习 9 解答: 嵌套结构体
//!
//! **提示**：请先尝试自己实现，再参考此解答！
//!
//! # 实现思路
//!
//! 1. 嵌套结构体：一个结构体可以包含另一个结构体
//! 2. 组合关系：使用结构体组合表达"有一个"关系
//! 3. 访问嵌套字段：使用点语法逐层访问
//! 4. 方法：可以对嵌套结构体进行操作

/// 地址结构体
#[derive(Debug, Clone)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub zip_code: String,
}

impl Address {
    /// 创建新地址
    pub fn new(street: String, city: String, zip_code: String) -> Self {
        Address {
            street,
            city,
            zip_code,
        }
    }

    /// 返回完整地址字符串
    pub fn full_address(&self) -> String {
        format!("{}, {} {}", self.street, self.city, self.zip_code)
    }

    /// 返回城市
    pub fn city(&self) -> &str {
        &self.city
    }
}

/// 员工结构体
#[derive(Debug)]
pub struct Employee {
    pub name: String,
    pub position: String,
    pub address: Address,
}

impl Employee {
    /// 创建新员工
    pub fn new(name: String, position: String, address: Address) -> Self {
        Employee {
            name,
            position,
            address,
        }
    }

    /// 返回员工描述
    pub fn describe(&self) -> String {
        format!(
            "{} - {} - 位于 {}",
            self.name,
            self.position,
            self.address.city
        )
    }

    /// 升职
    pub fn promote(&mut self, new_position: String) {
        self.position = new_position;
    }

    /// 获取员工所在城市
    pub fn get_city(&self) -> &str {
        self.address.city()
    }
}

/// 公司结构体
#[derive(Debug)]
pub struct Company {
    pub name: String,
    pub employees: Vec<Employee>,
}

impl Company {
    /// 创建新公司
    pub fn new(name: String) -> Self {
        Company {
            name,
            employees: Vec::new(),
        }
    }

    /// 添加员工
    pub fn add_employee(&mut self, employee: Employee) {
        self.employees.push(employee);
    }

    /// 返回员工数量
    pub fn employee_count(&self) -> usize {
        self.employees.len()
    }

    /// 根据姓名查找员工
    pub fn find_employee(&self, name: &str) -> Option<&Employee> {
        self.employees.iter().find(|e| e.name == name)
    }

    /// 可变地查找员工
    pub fn find_employee_mut(&mut self, name: &str) -> Option<&mut Employee> {
        self.employees.iter_mut().find(|e| e.name == name)
    }

    /// 列出所有员工姓名
    pub fn list_employees(&self) -> Vec<&str> {
        self.employees.iter().map(|e| e.name.as_str()).collect()
    }

    /// 移除员工
    pub fn remove_employee(&mut self, name: &str) -> Result<(), String> {
        let pos = self
            .employees
            .iter()
            .position(|e| e.name == name)
            .ok_or_else(|| format!("员工 '{}' 不存在", name))?;

        self.employees.remove(pos);
        Ok(())
    }

    /// 获取在特定城市的所有员工
    pub fn employees_in_city(&self, city: &str) -> Vec<&Employee> {
        self.employees
            .iter()
            .filter(|e| e.address.city == city)
            .collect()
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
    if let Some(emp) = company.find_employee_mut("李四") {
        emp.promote(String::from("高级设计师"));
    }

    // 移除员工
    println!("\n--- 移除员工 ---");
    match company.remove_employee("张三") {
        Ok(_) => println!("成功移除员工"),
        Err(e) => println!("移除失败: {}", e),
    }

    println!("剩余员工数: {}", company.employee_count());

    // 查找特定城市的员工
    println!("\n--- 特定城市的员工 ---");
    let boston_employees = company.employees_in_city("Boston");
    for emp in boston_employees {
        println!("波士顿员工: {}", emp.name);
    }
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
            String::from("1 St"),
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

    #[test]
    fn test_employees_in_city() {
        let mut company = Company::new(String::from("Test"));

        let addr1 = Address::new(String::from("1 St"), String::from("NYC"), String::from("10001"));
        let addr2 = Address::new(String::from("2 St"), String::from("Boston"), String::from("02108"));
        let addr3 = Address::new(String::from("3 St"), String::from("Boston"), String::from("02108"));

        company.add_employee(Employee::new(String::from("Alice"), String::from("Dev"), addr1));
        company.add_employee(Employee::new(String::from("Bob"), String::from("Designer"), addr2));
        company.add_employee(Employee::new(String::from("Charlie"), String::from("Manager"), addr3));

        let nyc_employees = company.employees_in_city("NYC");
        assert_eq!(nyc_employees.len(), 1);
        assert_eq!(nyc_employees[0].name, "Alice");

        let boston_employees = company.employees_in_city("Boston");
        assert_eq!(boston_employees.len(), 2);
    }
}
