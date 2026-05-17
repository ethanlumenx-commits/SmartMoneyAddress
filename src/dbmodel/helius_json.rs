use std::str::FromStr;
use serde_json::Value;
use bigdecimal::BigDecimal;
use crate::dbmodel::swaps::SwapModel;

#[derive(Debug, sqlx::FromRow,Clone)]
/// helius_json 根据交易签名获取交易信息
pub struct HeliusJson {

    pub signature: String,
    pub address: String,

    pub timestamp: i64, // BIGINT

    pub r#type: Option<String>,
    pub source: Option<String>,

    pub fee: Option<i64>,

    pub native_transfers: Option<Value>,
    pub token_transfers: Option<Value>,

    pub parsed: Option<bool>,

    pub created_at: Option<chrono::DateTime<chrono::Utc>>,

    pub events: Option<Value>,
}

impl HeliusJson {

    pub fn to_swaps_model(&self) -> Option<SwapModel> {
        let events = self.events.as_ref()?;
        let swap = events.get("swap")?;

        // token in / amount in 
        let (token_in, amount_in);
        if let Some(_native_input) = swap.get("nativeInput").filter(|v| !v.is_null()) {
            return None;
        } else {
            let input = swap.get("tokenInputs")?.as_array()?.first()?;
            let mint = input.get("mint")?.as_str()?.to_string();
            let raw_amount = input.get("rawTokenAmount")?.get("tokenAmount")?.as_str()?;
            let decimals = input.get("rawTokenAmount")?.get("decimals")?.as_u64()? as u32;
            let amount = BigDecimal::from_str(raw_amount).ok()? / BigDecimal::from(10u64.pow(decimals));
            token_in = mint;
            amount_in = amount;
        }

        // token out / amount out
        let (token_out, amount_out);
        // nativeOutput 有值
        if let Some(native_output) = swap.get("nativeOutput").filter(|v| !v.is_null()) {
            let raw_amount = native_output.get("amount")?.as_str()?;
            let amount = BigDecimal::from_str(raw_amount).ok()? / BigDecimal::from(1_000_000_000u64);
            token_out = "SOL".to_string();
            amount_out = amount;
        } else {
            return None;
        }

        Some(SwapModel {
            signature: self.signature.clone(),
            address: self.address.clone(),
            timestamp: self.timestamp,
            token_in,
            token_out,
            amount_in,
            amount_out,
            created_at: Some(chrono::Utc::now()),
        })
    }
}