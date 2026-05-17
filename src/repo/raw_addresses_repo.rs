use sqlx::PgPool;
use chrono::{DateTime, Utc};
use anyhow;
use crate::dbmodel::raw_addresses::RawAddress;

pub struct RawAddressesRepo;

impl RawAddressesRepo {
    pub async fn find_by_address(db: &PgPool, address: &str) -> anyhow::Result<Option<RawAddress>> {
        let result = sqlx::query_as!(
            RawAddress,
            r#"
            SELECT * FROM raw_addresses where address = $1
            "#,
            address
        )
        .fetch_optional(db)
        .await?;
        Ok(result)
    }

    pub async fn insert(db: &PgPool, address: &str,source:&str) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO 
            raw_addresses (address,source,first_seen,checked,remark,checked_at,next_check_at,checked_num,check_status) 
            VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9) 
            "#,
            address,
            source,
            Utc::now(),
            false,
            "",
            None as Option<DateTime<Utc>>,
            None as Option<DateTime<Utc>>,
            0,
            "Pedding"
        )
        .execute(db)
        .await?;

        Ok(())
    }

    pub async fn delete(db: &PgPool, address: &str) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM raw_addresses where address = $1
            "#,
            address
        )
        .execute(db)
        .await?;
        Ok(())
    }

    // 获取待检查的地址
    pub async fn get_wait_check_addresses(db: &PgPool, limit: i64) -> anyhow::Result<Vec<RawAddress>> {
        let result = sqlx::query_as!(
            RawAddress,
            r#"
            SELECT 
            id,
            address,
            source,
            first_seen,
            checked,
            remark,
            checked_at,
            next_check_at,
            checked_num,
            check_status
            FROM raw_addresses where checked = false limit $1
            "#,
            limit
        )
        .fetch_all(db)
        .await?;
        Ok(result)
    }

    // 更新待检查的
    pub async fn update_wait_check_addresses(db: &PgPool, address: Vec<String>) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            UPDATE raw_addresses 
            SET 
            checked = true, checked_num = checked_num + 1
            where address = ANY($1::TEXT[])
            "#,
            &address
        )
        .execute(db)
        .await?;
        Ok(())
    }

}



    

