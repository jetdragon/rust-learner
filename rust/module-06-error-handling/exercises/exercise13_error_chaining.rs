// exercise13_error_chaining.rs
pub struct Config {
    pub host: String,
    pub port: u16,
}

pub fn parse_config(host_str: &str, port_str: &str) -> Result<Config, String> {
    let host = if host_str.is_empty() {
        return Err("host 不能为空".to_string());
    } else {
        host_str.to_string()
    };

    let port: u16 = port_str
        .parse()
        .map_err(|e| format!("无法解析 port: {}", e))?;

    Ok(Config { host, port })
}

pub fn parse_config_with_context(host_str: &str, port_str: &str) -> Result<Config, String> {
    let host = if host_str.is_empty() {
        return Err("配置解析失败: host 不能为空".to_string());
    } else {
        host_str.to_string()
    };

    let port: u16 = port_str.parse().map_err(|e| {
        format!(
            "解析配置失败: host='{}', port='{}', 错误: {}",
            host_str, port_str, e
        )
    })?;

    Ok(Config { host, port })
}
