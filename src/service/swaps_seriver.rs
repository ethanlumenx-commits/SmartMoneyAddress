use sqlx::PgPool;
use anyhow;
use crate::{dbmodel::swaps::SwapModel, repo::swaps_repo::SwapRepo};
use crate::dbmodel::helius_json::HeliusJson;

pub struct SwapsService;

impl SwapsService {
    pub async fn find_by_signature(db: &PgPool, signature: &str) -> anyhow::Result<Option<SwapModel>> {
        SwapRepo::find_by_signature(db, signature).await
    }

    pub async fn insert(db: &PgPool, swap: SwapModel) -> anyhow::Result<()> {
        SwapRepo::insert(db, swap).await
    }

    pub async fn batch_insert(db: &PgPool, items:Vec<SwapModel>)-> anyhow::Result<()>{
        SwapRepo::batch_insert(db, items).await
    }

    /// 过滤出swap交易并原数组不可用
    pub fn filter_swaps(items: Vec<HeliusJson>) -> Vec<HeliusJson> {
        items.into_iter()
            .filter(|item| {
                item.events.as_ref()
                .and_then(|x| x.get("swap"))
                .is_some()
            })
            .collect()
    }

    /// 过滤出swap交易并原数组可用
    pub fn filter_swaps_2(items: Vec<&HeliusJson>) -> Vec<HeliusJson> 
    where HeliusJson: Clone{
        items.into_iter()
            .filter(|x| x.r#type.as_deref() == Some("swap"))
            .cloned()
            .collect()
    }

}