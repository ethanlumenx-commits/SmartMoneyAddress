use anyhow::Ok;

use sqlx::PgPool;
use anyhow;
use crate::repo::raw_addresses_repo::RawAddressesRepo;
use crate::dbmodel::raw_addresses::RawAddress;

pub struct RawAddressesService;

impl RawAddressesService {
    pub async fn find_by_address(db: &PgPool, address: &str) -> anyhow::Result<Option<RawAddress>> {
        RawAddressesRepo::find_by_address(db, address).await
    }

    pub async fn insert(db: &PgPool, address: &str,source:&str) -> anyhow::Result<()> {
        let exists  = RawAddressesRepo::find_by_address(db, address).await?;
        if exists .is_none() {
            RawAddressesRepo::insert(db, address,source).await?;
        }
        Ok(())
    }

    pub async fn delete(db: &PgPool, address: &str) -> anyhow::Result<()> {
        let exists  = RawAddressesRepo::find_by_address(db, address).await?;
        if exists.is_some() {
            RawAddressesRepo::delete(db, address).await?;
            Ok(())
        }else{
            Err(anyhow::anyhow!("address is none"))
        }
    }
}
