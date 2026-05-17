use smart_money::config::load_config;
use smart_money::helius::client::HeliusClient;

// get signatures for address
#[tokio::main]
async fn main() {
    let config = load_config();
    let client = HeliusClient::new(config.helius_api_url);
    let signatures = client.get_signatures_for_address("43TMSiFPiw721VP6ue58Kc4cPocFuU1qju1ok9ibDsS6",1000).await.unwrap();
    for signature in signatures.iter().take(10) {
        println!("{}", signature.signature);
    }
    
}