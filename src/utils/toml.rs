use anyhow::{anyhow, Result};
use serde_json::Value;
use toml::Table;

/// serde_json::Value를 toml::Value로 변환합니다.
pub fn json_to_toml(value: &Value) -> Result<toml::Value> {
    match value {
        Value::Null => Ok(toml::Value::Table(Table::new())),
        Value::Bool(b) => Ok(toml::Value::Boolean(*b)),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(toml::Value::Integer(i))
            } else if let Some(f) = n.as_f64() {
                Ok(toml::Value::Float(f))
            } else {
                Err(anyhow!("Invalid number in metadata"))
            }
        }
        Value::String(s) => Ok(toml::Value::String(s.clone())),
        Value::Array(arr) => {
            let mut toml_arr = Vec::new();
            for v in arr {
                toml_arr.push(json_to_toml(v)?);
            }
            Ok(toml::Value::Array(toml_arr))
        }
        Value::Object(obj) => {
            let mut table = Table::new();
            for (k, v) in obj {
                table.insert(k.clone(), json_to_toml(v)?);
            }
            Ok(toml::Value::Table(table))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_json_to_toml_basic() {
        let json = json!({
            "string": "hello",
            "number": 123,
            "bool": true,
            "array": [1, 2, 3],
            "nested": {
                "key": "value"
            }
        });

        let toml = json_to_toml(&json).unwrap();
        let toml_str = toml::to_string(&toml).unwrap();

        assert!(toml_str.contains("string = \"hello\""));
        assert!(toml_str.contains("number = 123"));
        assert!(toml_str.contains("bool = true"));
        assert!(toml_str.contains("array = [1, 2, 3]"));
        assert!(toml_str.contains("[nested]\nkey = \"value\""));
    }
}
