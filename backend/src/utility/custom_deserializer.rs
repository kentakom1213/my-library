use serde::{Deserialize, Deserializer};

/// published_dateフィールド用のカスタムデシリアライザ
/// 数値と文字列の両方を受け入れ、文字列に変換
pub fn deserialize_published_date_from_any<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    // 文字列でなければ、serde_json::Valueとしてデシリアライズを試みる
    // これは数値、ブール値、配列、オブジェクトなど何でも受け入れられる
    let val = serde_json::Value::deserialize(deserializer)?;

    match val {
        serde_json::Value::String(s) => Ok(s),
        serde_json::Value::Number(num) => {
            // 数値であれば文字列に変換
            Ok(num.to_string())
        }
        // 他の型の場合はエラーとする（必要に応じて他の型もハンドリング可能）
        _ => Err(serde::de::Error::custom(
            "Invalid type for published_date, expected string or number",
        )),
    }
}
