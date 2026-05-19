use crate::config::load_config;

use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use tracing::{info, error};
use anyhow::{Result};
use futures::{StreamExt, SinkExt};

pub struct WebSocketService;

impl WebSocketService {
    /// 启动 WebSocket 服务，监听链上变动
    pub async fn start(address: &str) -> Result<()> {
        let config = load_config();
        let websocket_url = &config.helius_websocks_url_key;
        
        let (ws_stream, response) = connect_async(websocket_url).await?;
        info!("WebSocket connected: {}", response.status());
        
        let (mut write, mut read) = ws_stream.split();
        
        let subscribe_request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "accountSubscribe",
            "params": {
                "pubkey": address,
                "commitment": "confirmed"
            }
        });
        
        let subscribe_msg = serde_json::to_string(&subscribe_request)?;
        write.send(Message::Text(subscribe_msg)).await?;
        info!("Sent subscription request");
        
        while let Some(msg) = read.next().await{
            match msg{
                Ok(message) => {
                    if let Message::Text(text) = message {
                        info!("Received message: {}", text);
                        // if let Err(e) = Self::handle_message(db, &text).await {
                        //     warn!("Error handling message: {}", e);
                        // }
                    }
                }
                Err(e) => {
                    error!("WebSocket message error: {}", e);
                    break;
                }
            }
        };
        
        Ok(())
    }
    

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::connect::connect;
    use crate::logger::init_logger;

    #[tokio::test]
    async fn test_is_swap_message() {
        
        init_logger();

        let db = connect().await;

        let _ = WebSocketService::start("675kPX9k5ZBu4h3d9MBUDNZA9HPc5xCi21QZ9vZJTkCy").await;
    }
    
}