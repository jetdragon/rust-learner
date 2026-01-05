//! 练习 6: BMI 计算器（综合练习）
//!
//! # 目标
//! 综合运用函数、控制流、数学运算和字符串处理
//!
//! # 任务
//! 创建一个完整的 BMI（身体质量指数）计算器
//!
//! # 难度
//! 中等
//!
//! # 预计时间
//! 30 分钟
//!
//! # 前置知识
//! - 函数定义和调用
//! - 数学运算
//! - if/else 控制流
//! - 字符串返回

/// 计算 BMI（身体质量指数）
///
/// # 参数
/// * `weight_kg` - 体重（公斤）
/// * `height_m` - 身高（米）
///
/// # 返回值
/// BMI 值
///
/// # 公式
/// BMI = weight (kg) / height² (m)
///
/// # 示例
/// ```rust
/// use module_01_basics::exercises::calculate_bmi;
///
/// // 体重 70kg，身高 1.75m 的 BMI 约为 22.86
/// let bmi = calculate_bmi(70.0, 1.75);
/// assert!((bmi - 22.86).abs() < 0.01);
/// ```
pub fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    // TODO: 实现 BMI 计算
    // 提示: BMI = weight / (height * height)
    unimplemented!()
}

/// 根据 BMI 值返回健康分类
///
/// # 参数
/// * `bmi` - BMI 值
///
/// # 返回值
/// 健康分类字符串
///
/// # 分类标准
/// * < 18.5: "体重过轻"
/// * 18.5 - 24.9: "正常"
/// * 25.0 - 29.9: "超重"
/// * >= 30.0: "肥胖"
pub fn bmi_category(bmi: f64) -> &'static str {
    // TODO: 实现 BMI 分类
    // 提示: 使用 if/else 链
    unimplemented!()
}

/// 生成健康报告
///
/// # 参数
/// * `weight_kg` - 体重（公斤）
/// * `height_m` - 身高（米）
///
/// # 返回值
/// 格式化的健康报告字符串
pub fn health_report(weight_kg: f64, height_m: f64) -> String {
    let bmi = calculate_bmi(weight_kg, height_m);
    let category = bmi_category(bmi);

    // TODO: 返回格式化的报告
    // 格式示例:
    // "BMI: 22.9, 分类: 正常"
    unimplemented!()
}

fn main() {
    // 示例使用
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
