use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
/// 存量地址，未筛选地址
pub struct RawAddress {
    pub id: i32,
    pub address: String,
    pub source: Option<String>,
    pub first_seen: Option<DateTime<Utc>>,
    pub checked: Option<bool>,
    pub remark: Option<String>,
    pub checked_at: Option<DateTime<Utc>>,
    pub next_check_at: Option<DateTime<Utc>>,
    pub checked_num: Option<i32>,
    pub check_status: Option<String>
}