use crate::config::CountConfig;
use anyhow::anyhow;
use elasticsearch::{CountParts, Elasticsearch, SearchParts};
use serde_json::{json, Value};
use tracing::info;

#[derive(Debug, Clone)]
pub struct EsRepositoryService {
    pub count: CountConfig,

    pub cli: Elasticsearch,
}

impl EsRepositoryService {
    pub async fn count_by_conditions(&self) -> anyhow::Result<i64> {
        let index = self.count.idx.as_str();
        let query = self.count.clone().query.unwrap();
        let cli = self.cli.clone();

        let query_clone = query.clone();
        let name = self.count.clone().name;

        let response = cli
            .count(CountParts::Index(&[index]))
            .body(query)
            .send()
            .await
            .expect("elasticsearch query count with conditions");

        let response_body = response.json::<Value>().await?;
        let count = response_body["count"]
            .as_i64()
            .ok_or(anyhow!("Failed to parse count from elasticsearch"))?;
        let count_clone = count.clone();

        info!(
            ">>>>>>>>>>>>>>>>>查询名称:{}>>>>>>>>>>>>>>>>>",
            name.unwrap_or("未命名查询名称".to_string())
        );
        info!("查询条件:{}, 总数:{}", query_clone, count_clone);

        Ok(count)
    }

    pub async fn count_by_aggregations(&self) -> anyhow::Result<hashbrown::HashMap<String, i64>> {
        let idx = self.count.idx.as_str();
        let size = self.count.clone().size_top;
        let field_name = self.count.clone().field_agg;
        let query = self.count.clone().query.unwrap();

        let query_body = json!({
            "query": query,
            "aggs": {
                "top_subcategories": {
                    "terms": {
                        "field": field_name,
                        "size": size,
                        "order": {
                            "_count": "desc"
                        }
                    }
                }
            }
        });
        let cli = self.cli.clone();

        let response = cli
            .search(SearchParts::Index(&[idx]))
            .body(query_body)
            .send()
            .await
            .expect("elasticsearch query count with conditions: ");
        let body = response.json::<Value>().await?;
        // 处理响应中的聚合结果
        let mut result: hashbrown::HashMap<String, i64> = hashbrown::HashMap::new();
        if let Some(terms_agg) = body
            .get("aggregations")
            .and_then(|agg| agg.get("top_subcategories"))
        {
            if let Some(buckets) = terms_agg.get("buckets") {
                match buckets.as_array() {
                    Some(bucket_array) => {
                        for bucket in bucket_array {
                            if let (Some(key), Some(doc_count)) =
                                (bucket.get("key"), bucket.get("doc_count"))
                            {
                                if let (Some(key_str), Some(doc_count_u64)) =
                                    (key.as_str(), doc_count.as_i64())
                                {
                                    result.insert(key_str.to_string(), doc_count_u64);
                                } else {
                                    return Err(anyhow!(
                                        "Invalid key or doc_count in bucket: {:?}",
                                        bucket
                                    ));
                                }
                            }
                        }
                    }
                    None => {
                        return Err(anyhow!(
                            "Expected 'buckets' to be an array but found: {:?}",
                            buckets
                        ))
                    }
                }
            } else {
                return Err(anyhow!("No 'buckets' found in top_subcategories"));
            }
        } else {
            return Err(anyhow!("No 'top_subcategories' aggregation found"));
        }

        Ok(result)
    }
}
