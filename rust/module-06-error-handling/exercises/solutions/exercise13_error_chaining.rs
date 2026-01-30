//! # 练习 13: 错误链（Error Chaining）解答
//!
//! **解答要点:**
//! 1. 使用 map_err 添加错误上下文
//! 2. 错误链提供更详细的错误信息
//! 3. 每层都添加有用的上下文，便于调试

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

pub fn parse_port_with_validation(port_str: &str) -> Result<u16, String> {
    let port: u16 = port_str
        .parse()
        .map_err(|e| format!("解析端口失败: {}", e))?;

    if port < 1024 {
        Err(format!("端口 {} 小于 1024（需要 root 权限）", port))
    } else if port == 0 {
        Err("端口不能为 0".to_string())
    } else {
        Ok(port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config_valid() {
        let config = parse_config("localhost", "8080").unwrap();
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 8080);
    }

    #[test]
    fn test_parse_config_empty_host() {
        assert!(parse_config("", "8080").is_err());
    }

    #[test]
    fn test_parse_config_invalid_port() {
        assert!(parse_config("localhost", "abc").is_err());
    }

    #[test]
    fn test_parse_config_with_context_valid() {
        let config = parse_config_with_context("localhost", "8080").unwrap();
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 8080);
    }

    #[test]
    fn test_parse_config_with_context_empty_host() {
        let err = parse_config_with_context("", "8080").unwrap_err();
        assert!(err.contains("host 不能为空"));
        assert!(err.contains("配置解析失败"));
    }

    #[test]
    fn test_parse_port_with_validation_valid() {
        assert_eq!(parse_port_with_validation("8080").unwrap(), 8080);
    }

    #[test]
    fn test_parse_port_with_validation_low() {
        let err = parse_port_with_validation("1023").unwrap_err();
        assert!(err.contains("1023"));
        assert!(err.contains("root 权限"));
    }

    #[test]
    fn test_parse_port_with_validation_zero() {
        let err = parse_port_with_validation("0").unwrap_err();
        assert!(err.contains("端口不能为 0"));
    }
}
