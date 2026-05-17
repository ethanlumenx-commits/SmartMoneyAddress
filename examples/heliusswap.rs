use reqwest::Client;
use serde_json::{json,Value};
use smart_money::config::load_config;

// get signatures for address
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let config = load_config();
    let url = config.helius_api_key;

    

    let body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getSignaturesForAddress",
        "params": [
            "43TMSiFPiw721VP6ue58Kc4cPocFuU1qju1ok9ibDsS6",
            {
                "limit": 1000
            }
        ]
    });

    let resp:Value = client
        .post(url)
        .json(&body)
        .send()
        .await?
        .json()
        .await?;

    let signature:Vec<String> = resp["result"]
        .as_array()
        .unwrap()
        .iter()
        .map(|x|x["signature"].as_str().unwrap().to_string())
        .collect();

    println!("{}",signature.len());
    for i in signature.iter().take(10){
        println!("{}",i);
    }

    Ok(())
}