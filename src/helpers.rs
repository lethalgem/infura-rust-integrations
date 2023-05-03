use clap::ArgMatches;

use crate::errors::EssenError;

pub fn get_block_number(sub_m: &ArgMatches) -> Result<u64, EssenError> {
    return sub_m
        .get_one::<String>("BLOCK")
        .ok_or_else(|| EssenError::Arg("Invalid block argument".to_string()))?
        .parse::<u64>()
        .map_err(|e| EssenError::ParseInt(e.to_string(), e));
}
