use futures::stream::{self, StreamExt};

use reqwest::Client;
use serde_json::{json, Value};
use serde::{Deserialize};
use anyhow;

use crate::dbmodel::helius_json::HeliusJson;

// 👇 这就是【你现在返回的真实结构】
#[derive(Debug, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransactionSignature {
    pub description:Option<String>,       
    pub r#type: Option<String>, 
    pub source: Option<String>,
    pub fee: Option<i64>,
    pub fee_payer: Option<String>,
    pub signature: Option<String>,
    pub slot: Option<u64>,
    pub timestamp: Option<i64>,

    // 不需要解析的复杂结构，直接用 Value 存
    pub native_transfers: Option<Value>,
    pub token_transfers: Option<Value>,
    pub account_data: Option<Value>,
    pub transaction_error: Option<Value>,
    pub instructions: Option<Value>,
    pub events: Option<Value>,
}

pub struct EnhancedClient  {
    api_url: String,
    client: Client,
}

impl EnhancedClient {
    pub fn new(enhanced_client_api_url: String) -> Self {
        let client = Client::new();
        Self { api_url:enhanced_client_api_url, client }
    }

    /// 根据签名获取交易信息，每100个签名一组请求
    pub async fn get_transactions_for_signatures(
        &self,
        signatures: Vec<String>,
    ) -> anyhow::Result<Vec<TransactionSignature>> {
        let chunks = signatures
            .chunks(100) // 100个一组切片，引用
            .map(|chunk| chunk.to_vec())// clone
            .collect::<Vec<_>>(); // 转换为 Vec<Vec<String>>
        let result = stream::iter(chunks)
            .map(|chunk| async move {
                let body = json!({
                    "transactions": chunk,
                });
                let resp = self
                    .client
                    .post(&self.api_url)
                    .json(&body)
                    .send()
                    .await?
                    .json::<Vec<TransactionSignature>>()
                    .await?;
                Ok(resp)
            })
            .buffer_unordered(5)
            .collect::<Vec<_>>()
            .await;

        let mut all_transactions = Vec::new();
        for res in result{
            match res {
                Ok(mut txs) => all_transactions.append(&mut txs),
                Err(e) => return Err(e),
            }
            
        }
        Ok(all_transactions)

    }
}

impl TransactionSignature {
    // 返回数据库类型
    pub fn to_db_model(&self) -> Option<HeliusJson> {
        Some(HeliusJson {
            signature: self.signature.clone()?,
            address: self.fee_payer.clone()?,
            timestamp: self.timestamp?,
            r#type: self.r#type.clone(),
            source: self.source.clone(),
            fee: self.fee.clone(),
            native_transfers: self.native_transfers.clone(),
            token_transfers: self.token_transfers.clone(),
            parsed: Some(false),
            created_at: Some(chrono::Utc::now()),
            events: self.events.clone(),
        })
    }
}