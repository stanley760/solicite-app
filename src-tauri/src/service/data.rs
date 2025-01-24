use crate::{parse_date_by_pattern, utils::get_similarity_ratio};
use anyhow::Ok;
use hashbrown::HashMap;

pub fn handle_data(vec: Vec<String>, arr: HashMap<String, i64>) -> Vec<String> {
    let result: Vec<String> = vec
        .iter()
        .flat_map(|item| {
            let item_str = item.as_str();
            arr.iter()
                .filter(|(k, _)| item_str.contains(*k))
                .filter(|(k, _)| get_similarity_ratio(k, item_str) > 45)
                .map(|(_, v)| format!("{}", v))
        })
        .collect();

    result
}

pub fn assemble_column(
    mut data: Vec<String>,
    start_time: Option<String>,
) -> anyhow::Result<Vec<String>> {
    let start_time = start_time.ok_or_else(|| anyhow::anyhow!("开始时间不能为空"))?;
    let time_arr = start_time
        .split(" ")
        .next()
        .ok_or_else(|| anyhow::anyhow!("解析时间错误"))?;
    let date = parse_date_by_pattern(time_arr, "%m月%d日").map_err(|e| anyhow::anyhow!(e))?;
    data.insert(0, date);
    Ok(data)
}
