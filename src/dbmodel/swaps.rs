use bigdecimal::BigDecimal;

#[derive(Debug, sqlx::FromRow)]
/// 根据签名获取到的swap交易数据
pub struct SwapModel {

    pub signature: String,
    pub address: String,

    pub timestamp: i64,

    pub token_in: String,
    pub token_out: String,

    pub amount_in: BigDecimal,   // NUMERIC 
    pub amount_out: BigDecimal,

    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

