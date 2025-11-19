use axum::http::StatusCode;
use serde::{Deserialize, Deserializer, Serializer};

/// 自定义序列化：将 StatusCode 序列化为 u16
pub fn serialize_status_code<S>(status: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u16(status.as_u16())
}

/// 自定义反序列化：从 u16 反序列化为 StatusCode
pub fn deserialize_status_code<'de, D>(deserializer: D) -> Result<StatusCode, D::Error>
where
    D: Deserializer<'de>,
{
    let code = u16::deserialize(deserializer)?;
    StatusCode::from_u16(code).map_err(serde::de::Error::custom)
}
