use crate::msg::{ExchangeRateResponse, ExchangeRatesQueryResponse, QueryMsg};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError, StdResult,
};

const CONTRACT_ADDRESS: &str = "sei1e3gttzq5e5k49f9f5gzvrl0rltlav65xu6p9xc0aj7e84lantdjqp7cncc";

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new())
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, _msg: Empty) -> StdResult<Binary> {
    // Create the query message for the other contract
    let query_msg: QueryMsg = QueryMsg::ExchangeRates { limit: Some(1) };

    // Perform the query to the other contract
    let response: ExchangeRatesQueryResponse = deps
        .querier
        .query_wasm_smart(CONTRACT_ADDRESS, &query_msg)?;

    // Extract the first item of the exchange_rates array
    if let Some(first_item) = response.exchange_rates.first() {
        let timestamp = first_item.0;
        // Parse the exchange_rate string
        let exchange_rate_str = &first_item.1; 

        // Split the exchange rate on the decimal point
        let parts: Vec<&str> = exchange_rate_str.split('.').collect();

        let (integer_part, fractional_part) = match parts.as_slice() {
            [int_part] => (*int_part, ""), // No decimal point
            [int_part, frac_part] => (*int_part, *frac_part),
            _ => return Err(StdError::generic_err("Invalid exchange rate format")),
        };

        // Determine the number of decimals
        let decimals = fractional_part.len() as u32;

        // Combine integer and fractional parts
        let exchange_rate_combined = format!("{}{}", integer_part, fractional_part);

        // Convert to Uint128
        let exchange_rate_int = exchange_rate_combined
            .parse::<u128>()
            .map_err(|_| StdError::generic_err("Failed to parse exchange rate to integer"))?;

        let exchange_rate_response = ExchangeRateResponse {
            timestamp,
            exchange_rate_value: exchange_rate_int,
            exchange_rate_decimals: decimals,
        };
        return to_json_binary(&exchange_rate_response);
    }

    // If extraction fails, return an error
    Err(StdError::generic_err("No exchange rates available"))
}
