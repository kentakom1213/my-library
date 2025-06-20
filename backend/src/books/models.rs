use serde::{Deserialize, Serialize};

use crate::utility::deserialize_published_date_from_any;

/// APIからの全体レスポンス
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")] // JSONのcamelCaseをRustのsnake_caseにマッピング
pub struct BooksApiResponse {
    pub kind: String,
    pub total_items: u32,
    pub items: Option<Vec<Volume>>, // 書籍が見つからない場合もあるのでOption<Vec>
}

/// items配列内の各書籍のデータ
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Volume {
    pub kind: String,
    pub id: String,
    pub etag: String,
    pub self_link: String,
    pub volume_info: VolumeInfo,
}

/// volumeInfo 内の書籍情報
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeInfo {
    pub title: String,
    pub authors: Option<Vec<String>>,
    #[serde(deserialize_with = "deserialize_published_date_from_any", default)]
    pub published_date: String,
    #[serde(default)]
    pub description: String,
    pub industry_identifiers: Option<Vec<IndustryIdentifier>>,
    pub image_links: Option<ImageLinks>,
    pub language: Option<String>,
    pub preview_link: Option<String>,
    pub info_link: Option<String>,
    pub canonical_volume_link: Option<String>,
}

/// industryIdentifiers 内の ISBN 情報
#[derive(Debug, Deserialize, Serialize)]
pub struct IndustryIdentifier {
    #[serde(rename = "type")] // "type" はRustのキーワードなのでrenameする
    pub identifier_type: String,
    pub identifier: String,
}

/// imageLinks 内の画像URL
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageLinks {
    pub small_thumbnail: String,
    pub thumbnail: String,
}
