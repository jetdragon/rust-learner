//! # 练习 6 解答: BMI 计算器（综合练习）
//!
//! **提示**：请先尝试自己实现，再参考此解答！
//!
//! # 实现思路
//!
//! 1. BMI 计算：简单的数学公式，注意浮点数运算
//! 2. BMI 分类：使用 if-else 链按顺序检查边界条件
//! 3. 字符串格式化：使用 format! 宏或 format 方法
//! 4. 组合多个函数完成复杂功能

/// 计算 BMI（身体质量指数）
pub fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}

/// 根据 BMI 值返回健康分类
pub fn bmi_category(bmi: f64) -> &'static str {
    if bmi < 18.5 {
        "体重过轻"
    } else if bmi < 25.0 {
        "正常"
    } else if bmi < 30.0 {
        "超重"
    } else {
        "肥胖"
    }
}

// 使用 match 表达式替代版本：
// pub fn bmi_category(bmi: f64) -> &'static str {
//     match bmi {
//         bmi if bmi < 18.5 => "体重过轻",
//         bmi if bmi < 25.0 => "正常",
//         bmi if bmi < 30.0 => "超重",
//         _ => "肥胖",
//     }
// }

/// 生成健康报告
pub fn health_report(weight_kg: f64, height_m: f64) -> String {
    let bmi = calculate_bmi(weight_kg, height_m);
    let category = bmi_category(bmi);

    format!("BMI: {:.1}, 分类: {}", bmi, category)
}

fn main() {
    let weight = 70.0;
    let height = 1.75;

    println!("=== BMI 计算器 ===");
    println!("体重: {} kg", weight);
    println!("身高: {} m", height);

    let bmi = calculate_bmi(weight, height);
    println!("BMI: {:.1}", bmi);

    let category = bmi_category(bmi);
    println!("分类: {}", category);

    println!("\n{}", health_report(weight, height));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_bmi_normal() {
        let bmi = calculate_bmi(70.0, 1.75);
        assert!((bmi - 22.86).abs() < 0.01);
    }

    #[test]
    fn test_calculate_bmi_underweight() {
        let bmi = calculate_bmi(50.0, 1.75);
        assert!((bmi - 16.33).abs() < 0.01);
    }

    #[test]
    fn test_calculate_bmi_overweight() {
        let bmi = calculate_bmi(85.0, 1.75);
        assert!((bmi - 27.76).abs() < 0.01);
    }

    #[test]
    fn test_bmi_category_underweight() {
        assert_eq!(bmi_category(17.0), "体重过轻");
        assert_eq!(bmi_category(18.0), "体重过轻");
    }

    #[test]
    fn test_bmi_category_normal() {
        assert_eq!(bmi_category(18.5), "正常");
        assert_eq!(bmi_category(22.0), "正常");
        assert_eq!(bmi_category(24.9), "正常");
    }

    #[test]
    fn test_bmi_category_overweight() {
        assert_eq!(bmi_category(25.0), "超重");
        assert_eq!(bmi_category(27.0), "超重");
        assert_eq!(bmi_category(29.9), "超重");
    }

    #[test]
    fn test_bmi_category_obese() {
        assert_eq!(bmi_category(30.0), "肥胖");
        assert_eq!(bmi_category(35.0), "肥胖");
    }

    #[test]
    fn test_health_report() {
        let report = health_report(70.0, 1.75);
        assert!(report.contains("22.9") || report.contains("22.8") || report.contains("22.86"));
        assert!(report.contains("正常"));
    }
}
