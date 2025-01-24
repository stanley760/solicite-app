use crate::config::Config;
use crate::error::CommonError;
use anyhow::{anyhow, Result};
use elasticsearch::auth::Credentials;
use elasticsearch::http::transport::MultiNodeConnectionPool;
use elasticsearch::{http::transport::TransportBuilder, Elasticsearch};
use tracing::info;
use url::Url;

pub async fn create_client(config: &Config) -> Result<Elasticsearch> {
    let es_config = &config.es;
    let credentials = Credentials::Basic(
        es_config
            .user
            .as_ref()
            .ok_or_else(|| anyhow!(CommonError::EsUserNotEmpty))?
            .clone(),
        es_config
            .password
            .as_ref()
            .ok_or_else(|| anyhow!(CommonError::EsPwdNotEmpty))?
            .clone(),
    );
    let urls = es_config
        .nodes
        .clone()
        .iter()
        .map(|node| {
            Url::parse(node).map_err(|e| anyhow!(CommonError::EsUrlConfigError(e.to_string())))
        })
        .collect::<Result<Vec<Url>>>()?;

    let connection_pool = MultiNodeConnectionPool::round_robin(urls, None);
    let transport = TransportBuilder::new(connection_pool)
        .auth(credentials)
        .build()
        .map_err(|e| anyhow!(CommonError::EstransportError(e.to_string())))?;

    let client = Elasticsearch::new(transport);
    let response = client.info().send().await?;
    info!("elasticsearch 自检程序开始......");
    if response.status_code() == 200 {
        // 解析响应体
        let body: serde_json::Value = response.json().await?;
        if let Some(version) = body.get("version").and_then(|v| v.get("number")) {
            info!("Elasticsearch 版本: {} 连接成功", version);
        }
    } else {
        return Err(anyhow!("连接失败，状态码: {}", response.status_code()));
    }
    Ok(client)
}
