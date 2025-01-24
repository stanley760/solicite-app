use chrono::NaiveDateTime;
use serde::Deserialize;
use validator::{Validate, ValidationError};

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExcelParams<'a> {
    #[validate(range(min = 1, max = 2))]
    #[serde(rename = "type")]
    pub stat: u8,
    #[validate(length(min = 1, message = "文件不能为空"))]
    pub path: &'a str,
    #[validate(custom(function = "validate_time_range"))]
    pub time_range: Vec<String>,
    #[validate(length(min = 1, message = "密码不能为空"))]
    pub password: &'a str,
    #[validate(length(min = 1, message = "工作薄必须选中一项"))]
    pub sheet_name: &'a str,
}

fn validate_time_range(time_range: &Vec<String>) -> Result<(), ValidationError> {
    if time_range.is_empty() {
        // the value of the username will automatically be added later
        return Err(ValidationError::new("统计时间范围不能为空"));
    }

    if time_range.len() != 2 {
        return Err(ValidationError::new("请选择统计时间范围区间"));
    }

    let start_time = NaiveDateTime::parse_from_str(&time_range[0], "%Y-%m-%d %H:%M:%S")
        .map_err(|_| ValidationError::new("开始时间格式不正确"))?;
    let end_time = NaiveDateTime::parse_from_str(&time_range[1], "%Y-%m-%d %H:%M:%S")
        .map_err(|_| ValidationError::new("结束时间格式不正确"))?;

    if end_time <= start_time {
        return Err(ValidationError::new("结束时间必须大于开始时间"));
    }
    Ok(())
}
