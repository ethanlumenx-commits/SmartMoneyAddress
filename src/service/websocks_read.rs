use crate::config::load_config;
use crate::dbmodel::helius_json::HeliusJson;
use crate::service::helius_json_seriver::HleiusJsonService;
use crate::service::swaps_seriver::SwapsService;
use sqlx::PgPool;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{info, warn, error, debug};
use anyhow::{Result, anyhow};
use futures::{StreamExt, SinkExt};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct WebSocketTransaction {
    pub description: Option<String>,
    pub r#type: Option<String>,
    pub source: Option<String>,
    pub fee: Option<i64>,
    pub fee_payer: Option<String>,
    pub signature: Option<String>,
    pub slot: Option<u64>,
    pub timestamp: Option<i64>,
    pub native_transfers: Option<Value>,
    pub token_transfers: Option<Value>,
    pub account_data: Option<Value>,
    pub transaction_error: Option<Value>,
    pub instructions: Option<Value>,
    pub events: Option<Value>,
}

#[derive(Debug, Serialize)]
struct SubscribeRequest {
    jsonrpc: String,
    id: u64,
    method: String,
    params: SubscribeParams,
}

#[derive(Debug, Serialize)]
struct SubscribeParams {
    accounts: Vec<String>,
    commitment: String,
}

pub struct WebSocketService;

impl WebSocketService {
    pub async fn start(db: &PgPool) -> Result<()> {
        let config = load_config();
        let websocket_url = &config.helius_websocks_url_key;
        
        info!("Connecting to Helius WebSocket: {}", websocket_url);
        
        let (ws_stream, response) = connect_async(websocket_url).await?;
        info!("WebSocket connected: {}", response.status());
        
        let (mut write, mut read) = ws_stream.split();
        
        let subscribe_request = SubscribeRequest {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method: "accountSubscribe".to_string(),
            params: SubscribeParams {
                accounts: vec!["*".to_string()],
                commitment: "confirmed".to_string(),
            },
        };
        
        let subscribe_msg = serde_json::to_string(&subscribe_request)?;
        write.send(Message::Text(subscribe_msg)).await?;
        info!("Sent subscription request");
        
        while let Some(msg) = read.next().await {
            match msg {
                Ok(message) => {
                    if let Message::Text(text) = message {
                        Self::handle_message(db, &text).await?;
                    }
                }
                Err(e) => {
                    error!("WebSocket message error: {}", e);
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    async fn handle_message(db: &PgPool, message: &str) -> Result<()> {
        let value: Value = serde_json::from_str(message)?;
        
        if let Some(result) = value.get("result") {
            if let Some(value_inner) = result.get("value") {
                if let Some(transaction) = value_inner.get("transaction") {
                    if let Ok(tx) = serde_json::from_value::<WebSocketTransaction>(transaction.clone()) {
                        if tx.r#type.as_deref() == Some("swap") {
                            info!("Received swap transaction: {:?}", tx.signature);
                            Self::process_swap_transaction(db, tx).await?;
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    async fn process_swap_transaction(db: &PgPool, tx: WebSocketTransaction) -> Result<()> {
        let signature = tx.signature.clone().ok_or_else(|| anyhow!("Missing signature"))?;
        let fee_payer = tx.fee_payer.clone().ok_or_else(|| anyhow!("Missing fee_payer"))?;
        let timestamp = tx.timestamp.ok_or_else(|| anyhow!("Missing timestamp"))?;
        
        let helius_json = HeliusJson {
            signature: signature.clone(),
            address: fee_payer,
            timestamp,
            r#type: tx.r#type,
            source: tx.source,
            fee: tx.fee,
            native_transfers: tx.native_transfers,
            token_transfers: tx.token_transfers,
            parsed: Some(false),
            created_at: Some(chrono::Utc::now()),
            events: tx.events,
        };
        
        debug!("Inserting transaction to helius_json: {}", signature);
        HleiusJsonService::insert(db, helius_json.clone()).await?;
        
        if let Some(swap_model) = helius_json.to_swaps_model() {
            debug!("Converting to swap model: {}", signature);
            SwapsService::insert(db, swap_model).await?;
            info!("Successfully processed swap transaction: {}", signature);
        } else {
            warn!("Cannot convert transaction to swap model: {}", signature);
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::connect::connect;
    use crate::logger::init_logger;
    
    #[tokio::test]
    async fn test_websocket_service() {
        init_logger();
        info!("Starting WebSocket service test");
        
        let db = connect().await;
        
        if let Err(e) = WebSocketService::start(&db).await {
            error!("WebSocket service error: {}", e);
        }
    }
}
