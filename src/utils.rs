use alloy::{
    hex::FromHex,
    primitives::{Address, U256},
};
use std::str::FromStr;

#[derive(Debug)]
pub enum ParseU256Error {
    InvalidU256String,
    InvalidU256Hex,
    NoNextItem,
}

#[derive(Debug)]
pub struct ParseAddressError;

pub fn parse_u256(value: &str) -> Result<U256, ParseU256Error> {
    if value.starts_with("0x") {
        return parse_u256_hex(value);
    }

    if value.trim().is_empty() {
        return Err(ParseU256Error::InvalidU256String);
    }

    U256::from_str(value).map_err(|_| ParseU256Error::InvalidU256String)
}

pub fn parse_u256_hex(hex_value: &str) -> Result<U256, ParseU256Error> {
    let hex_str = hex_value.trim_start_matches("0x");

    if hex_str.trim().is_empty() {
        return Err(ParseU256Error::InvalidU256Hex);
    }

    U256::from_str_radix(hex_str, 16).map_err(|_| ParseU256Error::InvalidU256Hex)
}

pub trait Uint256IteratorExt {
    /// Returns the next item parsed as a `BigUint`, or an error if parsing fails.
    fn next_uint256(&mut self) -> Result<U256, ParseU256Error>;
}

impl<I, S> Uint256IteratorExt for I
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    fn next_uint256(&mut self) -> Result<U256, ParseU256Error> {
        self.next()
            .map(|id| parse_u256(id.as_ref()))
            .ok_or(ParseU256Error::NoNextItem)?
    }
}

pub trait EVMAddressInteratorExt {
    fn next_evm_address(&mut self) -> Result<Address, ParseAddressError>;
}

impl<I, S> EVMAddressInteratorExt for I
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    fn next_evm_address(&mut self) -> Result<Address, ParseAddressError> {
        self.next()
            .map(|address| Address::from_hex(address.as_ref()))
            .ok_or(ParseAddressError)?
            .map_err(|_| ParseAddressError)
    }
}
