use sqlx::PgPool;
use anyhow;
use crate::dbmodel::helius_json::HeliusJson;
use crate::repo::helius_json_repo::HeliusJsonRepo;

pub struct HleiusJsonService;

impl HleiusJsonService {
    pub async fn find_by_signature(db: &PgPool, signature: &str) -> anyhow::Result<Option<HeliusJson>> {
        HeliusJsonRepo::find_by_signature(db, signature).await
    }

    pub async fn insert(db: &PgPool, helius: HeliusJson) -> anyhow::Result<()> {
        HeliusJsonRepo::insert(db, helius).await
    }

    pub async fn batch_insert(db: &PgPool, items:Vec<HeliusJson>)-> anyhow::Result<()>{
        HeliusJsonRepo::batch_insert(db, items).await
    }
}