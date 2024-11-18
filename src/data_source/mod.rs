mod bitcoin;
mod bitcoing_metadata;
mod evm;
mod evm_metadata;

use alloy::primitives::U256;
use bitcoing_metadata::BitcoinMetadata;
use evm_metadata::EvmMetadata;

use crate::query::Query;
use serde::{ser::SerializeMap, Serialize};
use std::error::Error;

#[derive(Debug)]
pub enum SourceResponse {
    Decimal { value: U256, decimals: u8 },
    AlphaNumeric { value: String },
}

#[derive(Serialize)]
pub struct SourceResponseWithMetadata {
    pub result: SourceResponse,
    pub metadata: SourceMetadata,
}

impl SourceResponseWithMetadata {
    fn new(result: SourceResponse, metadata: SourceMetadata) -> Self {
        Self { result, metadata }
    }
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum SourceMetadata {
    Bitcoin(BitcoinMetadata),
    Evm(EvmMetadata),
}

impl SourceMetadata {
    pub fn symbol(&self) -> String {
        match self {
            Self::Evm(evm_metadata) => evm_metadata.symbol(),
            Self::Bitcoin(bitcoing_metadata) => bitcoing_metadata.symbol(),
        }
    }

    pub fn label(&self) -> Option<String> {
        match self {
            Self::Evm(evm_metadata) => evm_metadata.label(),
            Self::Bitcoin(bitcoing_metadata) => bitcoing_metadata.label(),
        }
    }

    pub fn logo(&self) -> Option<String> {
        match self {
            Self::Evm(evm_metadata) => evm_metadata.logo(),
            Self::Bitcoin(bitcoing_metadata) => bitcoing_metadata.logo(),
        }
    }
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
                    &to_tiny(value, formatted).map_err(serde::ser::Error::custom)?,
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

impl SourceResponse {
    pub fn formatted_tiny(&self) -> String {
        match self {
            Self::Decimal { value, decimals } => {
                let formatted = match alloy::primitives::utils::format_units(*value, *decimals) {
                    Ok(formatted) => formatted,
                    Err(e) => {
                        print!("Failed to format {self:?}: {e:?}");
                        value.to_string()
                    }
                };

                to_tiny(value, &formatted).unwrap_or("-".to_string())
            }
            Self::AlphaNumeric { value } => value.to_string(),
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
            "" => Ok(full.to_string()),
            s => {
                let mut res = full.to_string();
                res.push('.');
                res.push_str(s);

                Ok(res)
            }
        }
    } else {
        Ok(full.to_string())
    }
}

#[derive(Default)]
pub struct DataSource {
    evm_data_source: evm::EvmDataSource,
    bitcoin_data_source: bitcoin::BitcoinDataSource,
}

impl DataSource {
    pub async fn get_data(
        &self,
        query: Query,
    ) -> Result<SourceResponseWithMetadata, Box<dyn Error>> {
        match query {
            Query::Evm(evm_query) => self.evm_data_source.get_data(evm_query).await,
            Query::Bitcoin(bitcoin_query) => self.bitcoin_data_source.get_data(bitcoin_query).await,
        }
    }

    pub async fn get_scanner_link(&self, query: Query) -> Result<String, Box<dyn Error>> {
        match query {
            Query::Evm(evm_query) => self.evm_data_source.get_scanner_link(evm_query).await,
            Query::Bitcoin(bitcoin_query) => {
                self.bitcoin_data_source
                    .get_scanner_link(bitcoin_query)
                    .await
            }
        }
    }
}
