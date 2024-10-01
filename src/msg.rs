use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct ExchangeRatesQueryResponse {
    pub exchange_rates: Vec<ExchangeRateItem>,
}

#[cw_serde]
pub struct ExchangeRateItem(
    pub u64,    // timestamp
    pub String, // exchange_rate
);

#[cw_serde]
pub struct ExchangeRateResponse {
    pub timestamp: u64,
    pub exchange_rate_value: u128,
    pub exchange_rate_decimals: u32,
}

#[cw_serde]
pub enum QueryMsg {
    ExchangeRates {
        limit: Option<u32>
    },
}