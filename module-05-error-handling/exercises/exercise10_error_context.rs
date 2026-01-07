//! # 练习 10: 错误上下文
//!
//! **难度**: 困难
//! **预计时间**: 25 分钟

/// 模拟读取配置文件
fn read_file(path: &str) -> Result<String, String> {
    if path.contains("notfound") {
        Err(format!("文件不存在: {}", path))
    } else if path.contains("permission") {
        Err(format!("权限不足: {}", path))
    } else {
        Ok("{ \"key\": \"value\" }".to_string())
    }
}

/// 模拟解析 JSON
fn parse_json(json: &str) -> Result<String, String> {
    if json.contains("invalid") {
        Err("JSON 格式错误".to_string())
    } else {
        Ok("value".to_string())
    }
}

/// 读取配置值并添加错误上下文
pub fn read_config_value(path: &str, key: &str) -> Result<String, String> {
    // TODO: 在每个步骤添加错误上下文
    // 例如：文件读取失败 -> "无法读取配置文件 {path}: 原错误"
    todo!()
}

fn main() {
    println!("=== 读取配置 ===");
    match read_config_value("config.json", "key") {
        Ok(value) => println!("配置值: {}", value),
        Err(e) => println!("错误: {}", e),
    }

    match read_config_value("notfound.json", "key") {
        Ok(value) => println!("配置值: {}", value),
        Err(e) => println!("错误: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_config_value() {
        assert!(read_config_value("config.json", "key").is_ok());
        assert!(read_config_value("notfound.json", "key").is_err());
    }

    #[test]
    fn test_error_context() {
        let err = read_config_value("notfound.json", "key").unwrap_err();
        assert!(err.contains("无法读取配置文件"));
    }
}
