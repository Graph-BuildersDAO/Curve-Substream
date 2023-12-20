use substreams::{scalar::{BigInt, BigDecimal}, errors::Error};


pub fn convert_enum_to_snake_case_prefix(snake_me: &str) -> String {
    snake_me.to_lowercase().replace("_", "-") + "-"
}

pub fn convert_bigint_to_decimal(value: &BigInt, denominator: u64) -> Result<BigDecimal, Error> {
    if *value == BigInt::from(0) {
        Ok(BigDecimal::from(0))
    } else {
        Ok(BigDecimal::from(value.clone()) / BigDecimal::from(denominator))
    }
}