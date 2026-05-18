use serde::Deserialize;
use config::{Config,Environment};
use dotenv;

#[derive(Deserialize,Debug)]
pub struct AppConfig{
    pub pg_host: String,
    pub pg_user: String,
    pub pg_password: String,
    pub pg_db: String,
    pub pg_port: u16,
    pub helius_api_url: String,
    pub helius_api_url_beta: String,
    pub helius_api_key: String,
    pub helius_enhanced_api_url: String,
    pub helius_websocks_url_key: String,
}

// 加载配置
pub fn load_config() -> AppConfig {
    dotenv::dotenv().ok();

    let config = Config::builder()
        .add_source(Environment::default())
        .build()
        .unwrap();
        
    config.try_deserialize::<AppConfig>()
        .expect("failed to convet config to setting")
}