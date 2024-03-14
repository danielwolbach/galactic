use serde::{Deserialize, Deserializer};
use toml::Value;

pub fn deserialize_vec_from_value_toml<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Value = Deserialize::deserialize(deserializer)?;

    match value {
        Value::String(s) => Ok(vec![s]),
        Value::Array(arr) => {
            let mut vec = Vec::new();

            for v in arr {
                if let Value::String(s) = v {
                    vec.push(s);
                } else {
                    return Err(serde::de::Error::custom("e"));
                }
            }

            Ok(vec)
        }
        _ => Err(serde::de::Error::custom("e")),
    }
}
