use std::fs;
use std::io::{BufReader, BufRead};
use smart_money::database::{connect};
// 批量插入
#[tokio::main]
async fn main() {
    let db = connect::connect().await;
    println!("✅ Successfully connected to PostgreSQL!");

    let file = fs::File::open("dune.csv").expect("Failed to open file");
    let reader = BufReader::new(file);

    let array = reader
        .lines()
        .filter_map(|line|{
            let line = line.ok()?;
            let line = line.trim();
            if line.len() > 20{
                Some(line.to_string())
            }else{
                None
            }
        })
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
