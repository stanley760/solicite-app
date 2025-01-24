use crate::repository::EsRepositoryService;
use crate::utils::read_file_json;
use crate::FileError;
use crate::{config::CountConfig, error::CommonError};
use anyhow::{anyhow, Result};
use elasticsearch::Elasticsearch;
use hashbrown::HashMap;
use serde_json::{json, Value};
use std::path::Path;
use tokio::sync::Mutex;

async fn count_by_diff_resource(
    cli: Elasticsearch,
    mut conditions: CountConfig,
    resources: Vec<u16>,
) -> Result<i64> {
    let start_time = conditions.start_time.clone().unwrap();
    let end_time = conditions.end_time.clone().unwrap();
    let category_id = conditions.category_id.clone();
    let query = json!({
        "query": {
            "bool": {
                "must": [
                    {
                        "range": {
                            "msgTime": {
                                "gte": start_time,
                                "lt": end_time,
                                "format": "yyyy-MM-dd HH:mm:ss",
                                "time_zone": "+08:00"
                            }
                        }
                    },
                    {
                        "terms": {
                            "resourceType": resources
                        }
                    },
                    {
                        "term": {
                            "categoryId": category_id
                        }
                    }
                ]
            }
        }
    });
    conditions.query = Some(query);
    let config = EsRepositoryService {
        count: conditions,
        cli,
    };
    let count = EsRepositoryService::count_by_conditions(&config).await?;
    Ok(count)
}

async fn count_by_channel(
    cli: Elasticsearch,
    mut conditions: CountConfig,
    content: &str,
) -> Result<i64> {
    let start_time = &conditions.start_time;
    let end_time = &conditions.end_time;

    let query = json!({
        "query": {
            "bool": {
                "must": [
                    {
                        "range": {
                            "msgTime": {
                                "gte": start_time,
                                "lt": end_time,
                                "format": "yyyy-MM-dd HH:mm:ss",
                                "time_zone": "+08:00"
                            }
                        }
                    },
                    {
                        "match_phrase": {
                            "content": format!("（来自：{}入口）", content)
                        }
                    },
                    {
                        "term": {
                            "categoryId": conditions.category_id
                        }
                    }
                ]
            }
        }
    });
    conditions.query = Some(query);
    let config = EsRepositoryService {
        count: conditions,
        cli,
    };
    let count = EsRepositoryService::count_by_conditions(&config).await?;
    Ok(count)
}

fn get_from_path(path: String) -> Result<Value> {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let config_path = Path::new(project_root)
        .join("assets")
        .join("json")
        .join(path);
    let path = config_path.to_str().ok_or(anyhow!("文件不存在"))?;
    let result = read_file_json(path)?;

    Ok(result)
}

pub async fn count_all_channel(
    cli: Elasticsearch,
    conditions: &mut CountConfig,
) -> Result<HashMap<String, i64>> {
    let mut result = HashMap::new();
    let path = conditions
        .clone()
        .path
        .ok_or(anyhow!(FileError::ConfigFileNotFound))?;
    let json_str = get_from_path(path)?;

    let json_clone = json_str.clone();
    let resources = json_clone["resources"]
        .as_array()
        .ok_or_else(|| anyhow!(CommonError::ParseJsonError("resources".to_string())))?;

    for resource in resources {
        let cli = cli.clone();
        conditions.name = Some(resource["code"].to_string());
        let conditions = conditions.clone();
        let values = resource["values"]
            .as_array()
            .ok_or_else(|| anyhow!(CommonError::ParseJsonError("values".to_string())))?;
        let code = String::from(
            resource["code"]
                .as_str()
                .ok_or_else(|| anyhow!(CommonError::ParseJsonError("code".to_string())))?,
        );

        println!("code:{},values:{:?}", code, values);
        let resource_values: Vec<u16> = values
            .iter()
            .filter_map(|value| value.as_u64())
            .filter_map(|num| {
                if num <= u16::MAX as u64 {
                    Some(num as u16)
                } else {
                    None
                }
            })
            .collect();

        if let Ok(count) = count_by_diff_resource(cli, conditions, resource_values).await {
            result.insert(code, count);
        }
    }

    let channels = json_clone["channel"]
        .as_array()
        .expect("解析数组类型channel错误")
        .clone();
    let channel_sum = Mutex::new(0);
    for channel in channels {
        let cli = cli.clone();
        let mut conditions = conditions.clone();
        let name = channel.as_str().unwrap().to_string();
        conditions.name = Some(name.to_string());
        if let Ok(count) = count_by_channel(cli, conditions, channel.as_str().unwrap()).await {
            result.insert(name, count);
            let mut sum = channel_sum.lock().await;
            *sum += count;
        }
    }

    let channel_sum = *channel_sum.lock().await;
    let gov = *result.get("中国政府网").unwrap_or(&0);
    let final_count = gov.clone() - channel_sum.clone();
    result.insert("中国政府网".to_string(), final_count);

    let total = result.clone().iter().map(|(_, &v)| v).sum();

    let total_name = conditions
        .clone()
        .total_name
        .ok_or(anyhow!(CommonError::TotalNameError))?;
    result.insert(total_name, total);
    Ok(result)
}

pub async fn agg_all_categories(
    cli: Elasticsearch,
    mut conditions: CountConfig,
) -> Result<HashMap<String, i64>> {
    let start_time = conditions.start_time.clone().unwrap();
    let end_time = conditions.end_time.clone().unwrap();
    let query = json!({
        "bool": {
            "must": [
                {
                    "range": {
                        "msgTime": {
                            "gte": start_time,
                            "lt": end_time,
                            "format": "yyyy-MM-dd HH:mm:ss",
                            "time_zone": "+08:00"
                        }
                    }
                },
                {
                    "term": {
                        "categoryId": conditions.category_id
                    }
                }
            ]
        }
    });
    conditions.query = Some(query);
    let config = EsRepositoryService {
        count: conditions,
        cli,
    };
    EsRepositoryService::count_by_aggregations(&config).await
}
