
use sqlx::{PgPool};
use bigdecimal::BigDecimal;
use anyhow::{Ok, Result};
use crate::dbmodel::swaps::SwapModel;
use chrono::{ Utc};
pub struct SwapRepo;

impl SwapRepo {
    pub async fn batch_insert(
        db: &PgPool,
        items: Vec<SwapModel>,
    ) -> Result<()> {
        if items.is_empty() {
            return Ok(());
        }

        // 1. 准备基础数据切片
        let signatures: Vec<String> = items.iter().map(|x| x.signature.clone()).collect();
        let addresses: Vec<String> = items.iter().map(|x| x.address.clone()).collect();
        let timestamps: Vec<i64> = items.iter().map(|x| x.timestamp).collect();
        let token_ins: Vec<String> = items.iter().map(|x| x.token_in.clone()).collect();
        let token_outs: Vec<String> = items.iter().map(|x| x.token_out.clone()).collect();
        let amounts_in:Vec<BigDecimal> = items.iter().map(|x| x.amount_in.clone()).collect();
        let amounts_out:Vec<BigDecimal> = items.iter().map(|x| x.amount_out.clone()).collect();
        // 2. 处理 BigDecimal 转为 String 以兼容 text[] 传输
        // 注意：这里假设 amount_in/out 不为 None，若为 Option 需 unwrap_or_default 或过滤
        // let amounts_in_str: Vec<String> = items.iter()
        //     .map(|x| x.amount_in.to_string()) 
        //     .collect();
        // let amounts_out_str: Vec<String> = items.iter()
        //     .map(|x| x.amount_out.to_string()) 
        //     .collect();


        // 4. 执行插入
        // 使用 text[] 传输 numeric 数据，然后在 SQL 中 cast 为 numeric
        sqlx::query!(
            r#"
            INSERT INTO swaps (
                signature, address, timestamp,
                token_in, token_out,
                amount_in, amount_out,
                created_at
            )
            SELECT * FROM UNNEST(
                $1::text[],
                $2::text[],
                $3::bigint[],
                $4::text[],
                $5::text[],
                $6::numeric[],  -- 从 text[] 转换为 numeric[]
                $7::numeric[],  -- 从 text[] 转换为 numeric[]
                $8::timestamptz[]
            )
            ON CONFLICT (signature) DO NOTHING
            "#,
            &signatures,
            &addresses,
            &timestamps,
            &token_ins,
            &token_outs,
            &amounts_in, // 传入 String Vec 的引用，对应 text[]
            &amounts_out, // 传入 String Vec 的引用，对应 text[]
            &vec![Utc::now(); signatures.len()],
        )
        .execute(db)
        .await?;

        Ok(())
    }

    pub async fn insert(db: &PgPool, swap: SwapModel) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO swaps (
                signature, address, timestamp,
                token_in, token_out,
                amount_in, amount_out,
                created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (signature) DO NOTHING
            "#,
            swap.signature,
            swap.address,
            swap.timestamp,
            swap.token_in,
            swap.token_out,
            swap.amount_in,
            swap.amount_out,
            Utc::now(),
        )
        .execute(db)
        .await?;
        Ok(())
    }

    pub async fn find_by_signature(db: &PgPool, signature: &str) -> Result<Option<SwapModel>> {
        let swap = sqlx::query_as!(
            SwapModel,
            r#"
            SELECT 
                signature,
                address,
                timestamp,
                token_in,
                token_out,
                amount_in,
                amount_out,
                created_at
            FROM swaps
            WHERE signature = $1
            "#,
            signature
        )
        .fetch_optional(db)
        .await?;
        Ok(swap)
    }

    pub async fn delete(db: &PgPool, signature: &str) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM swaps where signature = $1
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
    use anyhow::{Ok, Result};
    use crate::database::connect::connect;
    use crate::service::helius_json_seriver::HleiusJsonService;



    #[tokio::test]
    async fn test_batch_insert() -> Result<()> {
        // 创建数据库连接池
        let db = connect().await;

        let items = HleiusJsonService::find_by_signature(&db, "38CiowU9ak7ipyMjAefbS8vFgxnYHhkjYad9afVncE2tFPMYWYLBfwVatFFTCwhFHcUQixMJmbiSv3EmcP189iaT").await?;

        println!("items: {:?}", items);
        if let Some(heliusdata) = items{
            if let Some(swap) = heliusdata.to_swaps_model() {
                SwapRepo::insert(&db, swap).await?;
                println!("insert success");
            }else{
                println!("no swap");
            }
        }else{
            println!("no items");
        }



    Ok(())
    }
}