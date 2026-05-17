use smart_money::config::load_config;
use smart_money::helius::enhanced::EnhancedClient;

#[tokio::main]
async fn main(){
    let config = load_config();
    let client = EnhancedClient::new(config.helius_enhanced_api_url);
    let signatures = vec![
        "62nmEJMBKtsERtVkC9wNwyBnHzPD1yBCfcYRk7KddykDWNKM5tH6vg1SZFV2Nf1ZJASHyucRVCkeofqwtGuFsF83".to_string(),
        "4ZxkJBK1VgvxQ7wUqBW64foqSL5fE9cD67GH6WtG2iuvifrAYSLUs7MYrns7uYpXtYWFxLyQhN2AMXFG1YXZdXH9".to_string(),
    ];

    let transactions = client.get_transactions_for_signatures(signatures).await.unwrap();
    for transaction in transactions.iter().take(10) {
        println!("{:?}", transaction);
    }
}