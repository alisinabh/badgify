use num::{bigint::BigUint, Num, One};
use once_cell::sync::Lazy;
use std::str::FromStr;

pub static MAX_UINT_256: Lazy<BigUint> = Lazy::new(|| (BigUint::one() << 256) - BigUint::one());

#[derive(Debug)]
pub enum ParseU256Error {
    InvalidDecimal(String),
    InvalidHex,
    OverflowError,
    NoNextItem,
}

pub fn parse_u256(value: &str) -> Result<BigUint, ParseU256Error> {
    if value.starts_with("0x") {
        return parse_u256_hex(value);
    }

    let parsed_value =
        BigUint::from_str(value).map_err(|e| ParseU256Error::InvalidDecimal(e.to_string()))?;

    if parsed_value <= *MAX_UINT_256 {
        Ok(parsed_value)
    } else {
        Err(ParseU256Error::OverflowError)
    }
}

pub fn parse_u256_hex(hex_value: &str) -> Result<BigUint, ParseU256Error> {
    let hex_str = hex_value.trim_start_matches("0x");

    let parsed_value =
        BigUint::from_str_radix(hex_str, 16).map_err(|_| ParseU256Error::InvalidHex)?;

    if parsed_value <= *MAX_UINT_256 {
        Ok(parsed_value)
    } else {
        Err(ParseU256Error::OverflowError)
    }
}

pub trait Uint256IteratorExt {
    /// Returns the next item parsed as a `BigUint`, or an error if parsing fails.
    fn next_uint256(&mut self) -> Result<BigUint, ParseU256Error>;
}

impl<I, S> Uint256IteratorExt for I
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    fn next_uint256(&mut self) -> Result<BigUint, ParseU256Error> {
        self.next()
            .map(|id| parse_u256(id.as_ref()))
            .ok_or(ParseU256Error::NoNextItem)?
    }
}
