use sqlx::PgPool;
use chrono::{ Utc};
use anyhow;
use crate::dbmodel::helius_json::{HeliusJson};
use serde_json::{Value};

pub struct HeliusJsonRepo;

impl HeliusJsonRepo {
    /// 插入，parsed默认false，无视参数，created默认当前时间，无视参数
    pub async fn insert(db: &PgPool, helius:HeliusJson) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO helius_json 
            (signature, address, timestamp, type, source, fee, native_transfers,token_transfers,parsed,created_at,events)
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11) 
            ON CONFLICT (signature) DO NOTHING
            "#,
            helius.signature,
            helius.address,
            helius.timestamp,
            helius.r#type,
            helius.source,
            helius.fee,
            helius.native_transfers,
            helius.token_transfers,
            false as bool,
            Some(Utc::now()),
            helius.events
        )
        .execute(db)
        .await?;
        Ok(())
    }

    pub async fn batch_insert(db: &PgPool, items:Vec<HeliusJson>)-> anyhow::Result<()>{
        let signatures: Vec<String> = items.iter().map(|x| x.signature.clone()).collect();
        let addresses: Vec<String> = items.iter().map(|x| x.address.clone()).collect();
        let timestamps: Vec<i64> = items.iter().map(|x| x.timestamp).collect();

        let types: Vec<Option<String>> = items.iter().map(|x| x.r#type.clone()).collect();
        let sources: Vec<Option<String>> = items.iter().map(|x| x.source.clone()).collect();
        let fees: Vec<Option<i64>> = items.iter().map(|x| x.fee).collect();

        let native:Vec<Value> = items.iter()
            .filter_map(|x| x.native_transfers.clone())
            .collect();

        let token: Vec<Value> = items.iter()
            .filter_map(|x| x.token_transfers.clone())
            .collect();

        let events: Vec<Value> = items.iter()
            .filter_map(|x| x.events.clone())
            .collect();

        sqlx::query!(
            r#"
            INSERT INTO helius_json
            (signature, address, timestamp, type, source, fee, native_transfers, token_transfers,parsed,created_at,events)
            SELECT *
            FROM UNNEST(
                $1::text[],
                $2::text[],
                $3::bigint[],
                $4::text[],
                $5::text[],
                $6::bigint[],
                $7::jsonb[],
                $8::jsonb[],
                $9::bool[],
                $10::timestamptz[],
                $11::jsonb[]
            )
            ON CONFLICT (signature) DO NOTHING
            "#,
            &signatures,
            &addresses,
            &timestamps,
            &types as &[Option<String>],
            &sources as &[Option<String>],
            &fees as &[Option<i64>],
            &native,
            &token,
            &vec![false; signatures.len()],
            &vec![Utc::now(); signatures.len()],
            &events
        )
        .execute(db)
        .await?;

        Ok(())
    }

    pub async fn find_by_signature(db: &PgPool, signature: &str) -> anyhow::Result<Option<HeliusJson>> {
        let helius = sqlx::query_as!(
            HeliusJson,
            r#"
            SELECT 
                signature,
                address,
                timestamp,
                type as "type?",
                source as "source?",
                fee as "fee?",
                native_transfers as "native_transfers?",
                token_transfers as "token_transfers?",
                parsed as "parsed?",
                created_at,
                events as "events?"
            FROM helius_json
            WHERE signature = $1
            "#,
            signature
        )
        .fetch_optional(db)
        .await?;
        Ok(helius)
    }

    pub async fn delete(db: &PgPool, signature: &str) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM helius_json where signature = $1
            "#,
            signature
        )
        .execute(db)
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests { 
    use super::*;
    use crate::database::connect;
    use serde_json::json;
    use crate::dbmodel::helius_json::{HeliusJson};

    #[tokio::test]
    async fn test_insert() {
        let db = connect::connect().await;
        let helius = HeliusJson {
            signature: "test_signature".to_string(),
            address: "test_address".to_string(),
            timestamp: 1234567890,
            r#type: Some("test_type".to_string()),
            source: Some("test_source".to_string()),
            fee: Some(100),
            native_transfers: Some(json!({"test": "native_transfers"})),
            token_transfers: Some(json!({"test": "token_transfers"})),
            parsed: Some(false),
            created_at: Some(Utc::now()),
            events: Some(json!({"test": "events"})),
        };
        HeliusJsonRepo::insert(&db, helius).await.unwrap();

        let helius_find = HeliusJsonRepo::find_by_signature(&db, "test_signature").await.unwrap();
        println!("helius_find: {:?}",helius_find);   
    }
}