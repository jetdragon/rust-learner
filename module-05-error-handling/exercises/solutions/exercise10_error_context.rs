//! # 练习 10: 错误上下文解答
//!
//! **解答要点:**
//! 1. 使用 map_err() 为错误添加上下文
//! 2. 使用 ? 传播错误
//! 3. 在每一步添加有意义的错误信息

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
    // 在每个步骤添加错误上下文
    let json = read_file(path)
        .map_err(|e| format!("无法读取配置文件 '{}': {}", path, e))?;

    let value = parse_json(&json)
        .map_err(|e| format!("无法解析配置文件 '{}': {}", path, e))?;

    Ok(value)
}

// /// 使用 ? 操作符的替代实现
// pub fn read_config_value_alt(path: &str, key: &str) -> Result<String, String> {
//     let json = read_file(path)
//         .map_err(|e| format!("读取文件失败 '{}': {}", path, e))?;
//
//     let value = parse_json(&json)
//         .map_err(|e| format!("解析 JSON 失败: {}", e))?;
//
//     Ok(value)
// }

/// 获取配置项（带完整上下文）
pub fn get_config_item(path: &str, key: &str) -> Result<String, String> {
    read_file(path)
        .map_err(|e| format!("步骤1: 读取文件 '{}' 失败 - {}", path, e))
        .and_then(|json| {
            parse_json(&json)
                .map_err(|e| format!("步骤2: 解析 JSON 失败 - {}", e))
        })
        .map(|value| format!("配置项 [{}]: {}", key, value))
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

    match read_config_value("permission.json", "key") {
        Ok(value) => println!("配置值: {}", value),
        Err(e) => println!("错误: {}", e),
    }

    match read_config_value("config.json", "key") {
        Ok(value) => println!("配置值: {}", value),
        Err(e) => println!("错误: {}", e),
    }

    println!("\n=== 完整上下文示例 ===");
    match get_config_item("notfound.json", "database_url") {
        Ok(msg) => println!("{}", msg),
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
        assert!(err.contains("notfound.json"));
    }

    #[test]
    fn test_permission_error() {
        let err = read_config_value("permission.json", "key").unwrap_err();
        assert!(err.contains("无法读取配置文件"));
        assert!(err.contains("权限不足"));
    }
}
