use chrono::{DateTime, Duration, NaiveDate, NaiveDateTime, Utc};

pub fn parse_date_by_pattern(item: &str, pattern: &str) -> Result<String, String> {
    if pattern.is_empty() {
        return Err("时间格式不能为空".to_string());
    }

    // Try to parse as full datetime first
    if let Ok(datetime) = item.parse::<DateTime<Utc>>() {
        let naive_datetime: NaiveDateTime = datetime.naive_utc();
        return Ok(naive_datetime.format(pattern).to_string());
    }

    if let Ok(naive_datetime) = NaiveDateTime::parse_from_str(item, "%Y-%m-%d %H:%M:%S") {
        return Ok(naive_datetime.format(pattern).to_string());
    }

    // If full datetime parsing fails, try to parse as date only
    if let Ok(date) = NaiveDate::parse_from_str(item, "%Y-%m-%d") {
        let naive_datetime = date.and_hms_opt(0, 0, 0).unwrap();
        return Ok(naive_datetime.format(pattern).to_string());
    }

    Err(format!("无法解析项: {}", item))
}

pub fn calculated_days(start: Option<String>, end: Option<String>) -> Result<i64, String> {
    if start.is_none() || end.is_none() {
        return Err("开始时间和结束时间不能为空".to_string());
    }
    let start = start.clone().unwrap();
    let end = end.clone().unwrap();
    let start = start.as_str();
    let end = end.as_str();
    let start = NaiveDate::parse_from_str(start, "%Y-%m-%d %H:%M:%S")
        .map_err(|_| "开始时间格式错误".to_string())?;
    let end = NaiveDate::parse_from_str(end, "%Y-%m-%d %H:%M:%S")
        .map_err(|_| "结束时间格式错误".to_string())?;

    let days = end.signed_duration_since(start).num_days();
    Ok(days)
}

pub fn calculated_range_time(start: &str, end: &str, i: i64) -> anyhow::Result<(String, String)> {
    let start_datetime = NaiveDateTime::parse_from_str(start, "%Y-%m-%d %H:%M:%S")
        .map(|date| {
            let date = date + Duration::days(i - 1);
            date
        })
        .map_err(|_| anyhow::anyhow!("开始时间格式错误".to_string()))?;
    let end_time = NaiveDateTime::parse_from_str(end, "%Y-%m-%d %H:%M:%S").unwrap();
    let pattern = format!(
        "{} {}",
        start_datetime.format("%Y-%m-%d"),
        end_time.format("%H:%M:%S")
    );
    let end_datetime = NaiveDateTime::parse_from_str(pattern.as_str(), "%Y-%m-%d %H:%M:%S")
        .map(|date| {
            let date = date + Duration::days(1);
            date
        })
        .map_err(|_| anyhow::anyhow!("结束时间格式错误".to_string()))?;
    Ok((start_datetime.to_string(), end_datetime.to_string()))
}
