use anyhow::{anyhow, Ok, Result};
use serde_json::Value;
use serde_yml;
use std::fs::{self};
use std::path::Path;

use crate::error::FileError;

pub fn read_file_yml<T: serde::de::DeserializeOwned>(path: &str) -> Result<T> {
    // 将路径字符串转换为 Path 结构体
    let path_obj = Path::new(path);
    // 获取文件
    let file = read_file_content(path_obj)?;

    let obj = serde_yml::from_str(&file)
        .map_err(|err| FileError::YamlDeserializeError(err.to_string()))?;
    Ok(obj)
}

pub fn read_file_json(path: &str) -> Result<Value> {
    let path_obj = Path::new(path);
    let content = read_file_content(path_obj)?;

    let obj: Value = serde_json::from_str(content.as_str())
        .map_err(|e| FileError::JsonDeserializeError(e.to_string()))?;

    Ok(obj)
}

pub fn read_file_content(path: &Path) -> Result<String> {
    if !path.exists() {
        return Err(anyhow!(FileError::FileNotFound));
    }
    if !path.is_file() {
        return Err(anyhow!(FileError::NotAFile));
    }

    let content = fs::read_to_string(path).map_err(|_err| FileError::ReadError)?;

    Ok(content)
}

pub fn update_file_by_content(path: &str, content: &str) -> Result<String> {
    if path.trim().is_empty() {
        return Err(FileError::FileNotFound.into());
    }
    if content.trim().is_empty() {
        return Err(FileError::SaveContentNotNull.into());
    }

    fs::write(path, content).map_err(|_e| FileError::WriteError)?;

    Ok(String::from("更新成功"))
}
