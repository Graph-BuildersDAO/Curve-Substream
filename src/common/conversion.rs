use substreams::{scalar::{BigInt, BigDecimal}, errors::Error};


pub fn convert_enum_to_snake_case_prefix(snake_me: &str) -> String {
    snake_me.to_lowercase().replace("_", "-") + "-"
}

// Converts `i64` to `i32`. On overflow, logs the error and returns `0`.
// Note: Use `0` as a fallback only when appropriate for your use case.
pub fn convert_i64_to_i32(value: i64) -> i32 {
    i32::try_from(value).unwrap_or_else(|e| {
        substreams::log::debug!("Warning: Value out of range, error: {}", e);
        0
    })
}

pub fn convert_bigint_to_decimal(value: &BigInt, denominator: u64) -> Result<BigDecimal, Error> {
    if *value == BigInt::from(0) {
        Ok(BigDecimal::from(0))
    } else {
        Ok(BigDecimal::from(value.clone()) / BigDecimal::from(denominator))
    }
}