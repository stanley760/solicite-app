use serde::de::Deserializer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassifyCode {
    pub id: String,
    pub name: String,
    pub code: String,
    #[serde(rename = "parentId")]
    pub parent_id: String,
    pub description: String,
    pub children: Vec<Children>,
    #[serde(rename = "createTime")]
    pub create_time: String,
    #[serde(rename = "updateTime")]
    pub update_time: String,
    #[serde(rename = "sortValue")]
    pub sort_value: String,
    pub status: bool,
    #[serde(rename = "dictionaryId")]
    pub dictionary_id: String,
    #[serde(rename = "hiddenFlag", deserialize_with = "bool_from_int")]
    pub hidden_flag: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Children {
    pub id: String,
    pub name: String,
    pub code: String,
    #[serde(rename = "parentId")]
    pub parent_id: String,
    pub description: String,
    #[serde(rename = "createTime")]
    pub create_time: String,
    #[serde(rename = "updateTime")]
    pub update_time: String,
    #[serde(rename = "sortValue")]
    pub sort_value: String,
    pub status: bool,
    #[serde(rename = "dictionaryId")]
    pub dictionary_id: String,
    #[serde(rename = "hiddenFlag", deserialize_with = "bool_from_int")]
    pub hidden_flag: Option<bool>,
}

fn bool_from_int<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<u8>::deserialize(deserializer)?;
    Ok(opt.map(|v| v != 0))
}
