use std::fs;
use std::io::{BufReader, BufRead};
use smart_money::database::{connect};

#[tokio::main]
async fn main() {
    let db = connect::connect().await;
    println!("✅ Successfully connected to PostgreSQL!");

    let file = fs::File::open("dune_long.csv").expect("Failed to open file");
    let reader = BufReader::new(file);

    let array = reader
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect::<Vec<_>>();
    
    sqlx::query!(
        r#"
        INSERT INTO raw_addresses (address,source,first_seen,checked) 
        SELECT address,'Dune',NOW(),false
        FROM UNNEST($1::TEXT[]) AS address
        ON conflict (address) DO NOTHING  
        "#,
        &array
    ).execute(&db).await.expect("Failed to insert data");
}
