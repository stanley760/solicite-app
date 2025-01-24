use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResultWrapper<T> {
    pub code: u32,
    pub message: String,
    pub data: Option<T>,
}
