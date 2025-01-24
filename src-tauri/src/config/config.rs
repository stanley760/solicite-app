use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Clone)]
pub struct EsConfig {
    pub nodes: Vec<String>,
    pub user: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CountConfig {
    pub category_id: u16,
    pub idx: String,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub size_top: u8,
    pub field_agg: String,
    pub query: Option<Value>,
    pub path: Option<String>,
    pub category_path: Option<String>,
    pub name: Option<String>,
    pub total_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ExcelConfig {
    pub channel_insert_start_column: Option<String>,

    pub channel_location: Option<String>,

    pub category_insert_start_column: Option<String>,

    pub category_location: Option<String>,

    pub sheet_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub es: EsConfig,

    pub count: CountConfig,

    pub excel: ExcelConfig,
}
