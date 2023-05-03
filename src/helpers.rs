use clap::ArgMatches;
use web3::types::U256;

use crate::errors::EssenError;

pub fn get_block_number(sub_m: &ArgMatches) -> Result<u64, EssenError> {
    return sub_m
        .get_one::<String>("BLOCK")
        .ok_or_else(|| EssenError::Arg("Invalid block argument".to_string()))?
        .parse::<u64>()
        .map_err(|e| EssenError::ParseInt(e.to_string(), e));
}

pub fn to_f64(value: U256) -> f64 {
    let (quotient, remainder) = value.div_mod(U256::from(10u64.pow(18)));
    let quotient_f64 = quotient.low_u64() as f64;
    let remainder_f64 = remainder.low_u64() as f64 / 1e18;
    quotient_f64 + remainder_f64
}
