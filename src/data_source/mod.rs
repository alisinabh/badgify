mod evm;

use alloy::primitives::U256;

use crate::query::Query;
use serde::{ser::SerializeMap, Serialize};
use std::error::Error;

#[derive(Debug)]
pub enum SourceResponse {
    Decimal { value: U256, decimals: u8 },
    AlphaNumeric { value: String },
}

impl Serialize for SourceResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Decimal { value, decimals } => {
                let formatted = &alloy::primitives::utils::format_units(*value, *decimals)
                    .map_err(|_| serde::ser::Error::custom("Cannot format decimal units"))?;

                let mut map = serializer.serialize_map(Some(4))?;
                map.serialize_entry("type", "decimal")?;
                map.serialize_entry("value", &value.to_string())?;
                map.serialize_entry("decimals", &decimals)?;
                map.serialize_entry("formatted", formatted)?;
                map.serialize_entry(
                    "formatted_tiny",
                    &to_tiny(value, &formatted).map_err(|e| serde::ser::Error::custom(e))?,
                )?;
                map.end()
            }
            Self::AlphaNumeric { value } => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "alphanumeric")?;
                map.serialize_entry("value", &value)?;
                map.end()
            }
        }
    }
}

fn to_tiny(value: &U256, formatted: &str) -> Result<String, String> {
    let mut formatted_iter = formatted.split(".");
    let full = formatted_iter.next().ok_or("Invalid decimal string")?;
    if let Some(partial) = formatted_iter.next() {
        let partial: String = partial.chars().take(4).collect::<String>();

        match partial.trim_end_matches("0") {
            s if s.is_empty() && full == "0" && value.to::<u32>() != 0 => Ok("~0".to_string()),
            s if s.is_empty() => Ok(full.to_string()),
            s => {
                let mut res = full.to_string();
                res.push('.');
                res.push_str(&s);

                Ok(res)
            }
        }
    } else {
        Ok(full.to_string())
    }
}

pub struct DataSource {
    evm_data_source: evm::EVMDataSource,
}

impl Default for DataSource {
    fn default() -> Self {
        Self {
            evm_data_source: evm::EVMDataSource::default(),
        }
    }
}

impl DataSource {
    pub async fn get_data(&self, query: Query) -> Result<SourceResponse, Box<dyn Error>> {
        match query {
            Query::EVM(evm_query) => self.evm_data_source.get_data(evm_query).await,
            Query::Bitcoin(_bitcoin_query) => todo!(),
        }
    }
}
