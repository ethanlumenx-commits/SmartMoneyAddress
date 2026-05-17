use smart_money::database::{connect};
use sqlx::PgPool;
use dotenv;
use smart_money::service::raw_addresses_seriver;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    
    let db = connect::connect().await;
    println!("✅ Successfully connected to PostgreSQL!");
    example(&db).await;
    
    raw_addresses_seriver::RawAddressesService::insert(&db,"0x1","test").await.unwrap();

    let exists = raw_addresses_seriver::RawAddressesService::find_by_address(&db,"0x1").await.unwrap();
    println!("{:?}",exists);
    

}

async fn example(db: &PgPool) {
    let result:(i32,) = sqlx::query_as("SELECT 1 + 1")
        .fetch_one(db)
        .await
        .expect("failed to fetch wallets");

    println!("{:?}",result);
}
