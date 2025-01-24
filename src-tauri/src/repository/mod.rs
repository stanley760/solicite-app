mod es_repository;

pub use es_repository::*;

#[cfg(test)]
mod test {
    use crate::config;
    use crate::config::Config;
    use crate::repository::EsRepositoryService;
    use crate::utils::read_file_yml;
    use serde_json::json;
    use std::path::Path;

    #[tokio::test]
    async fn test_count_by_conditions() {
        let query = json!({
            "query": {
                "bool": {
                    "must": [
                        {
                            "range": {
                                "msgTime": {
                                    "gte": "2024-12-25 09:00:01",
                                    "lt": "2024-12-26 09:00:00",
                                    "format": "yyyy-MM-dd HH:mm:ss",
                                    "time_zone": "+08:00"
                                }
                            }
                        },
                        {
                            "terms": {
                                "resourceType": [4,5]
                            }
                        },
                        {
                            "term": {
                                "categoryId": 58
                            }
                        }
                    ]
                }
            }
        });
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("assets")
            .join("yml")
            .join("config.yaml");
        let path_str = path.to_str().expect("Failed to convert path to &str");
        let config: Config = read_file_yml(path_str).unwrap();
        let mut count_config = config.clone().count;
        let cli = config::create_client(&config).await.unwrap();
        count_config.query = Some(query);
        let config = EsRepositoryService {
            count: count_config,
            cli,
        };
        let count = EsRepositoryService::count_by_conditions(&config)
            .await
            .unwrap();
        println!("count: {}", count);
    }

    #[tokio::test]
    async fn test_agg_by_categories() {
        let query = json!({
            "bool": {
                "must": [
                    {
                        "range": {
                            "msgTime": {
                                "gte": "2025-01-22 09:00:01",
                                "lt": "2025-01-23 09:00:00",
                                "format": "yyyy-MM-dd HH:mm:ss",
                                "time_zone": "+08:00"
                            }
                        }
                    },
                    {
                        "term": {
                            "categoryId": 58
                        }
                    }
                ]
            }
        });

        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("assets")
            .join("yml")
            .join("config.yaml");
        let path_str = path.to_str().expect("Failed to convert path to &str");
        let config: Config = read_file_yml(path_str).unwrap();
        let mut count_config = config.clone().count;
        let cli = config::create_client(&config).await.unwrap();
        count_config.query = Some(query);
        let config = EsRepositoryService {
            count: count_config,
            cli,
        };
        let agg = EsRepositoryService::count_by_aggregations(&config)
            .await
            .unwrap();

        println!("agg:{:?}", agg);
    }
}
