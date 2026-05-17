use reqwest::Client;
use serde_json::{json};
use serde::{Deserialize};
use anyhow;

#[derive(Debug, Deserialize)]
pub struct TransactionSignature {
    pub signature: String,
}

#[derive(Debug, Deserialize)]
pub struct RpcResponse<T> {
    pub result: T,
}

pub struct HeliusClient {
    api_url: String,
    client: Client,
}

impl HeliusClient {
    pub fn new(api_url: String) -> Self {
        let client = Client::new();
        Self { api_url, client }
    }

    /// 获取地址最近的1000个交易签名
    pub async fn get_signatures_for_address(
        &self,
        address: &str,
        limit: u64,
    ) -> anyhow::Result<Vec<TransactionSignature>> {
        let body = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getSignaturesForAddress",
            "params": [
                address,
                {
                    "limit": limit
                }
            ]
        });

        let resp = self
            .client
            .post(&self.api_url)
            .json(&body)
            .send()
            .await?
            .json::<RpcResponse<Vec<TransactionSignature>>>()
            .await?;

        Ok(resp.result)
    }
}