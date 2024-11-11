use std::sync::Arc;

use serde::Serialize;

use crate::{evm_chainlist::EvmChain, types::EvmAddress};

#[derive(Serialize)]
pub struct EvmMetadata {
    pub chain: Arc<EvmChain>,
    pub source: EvmSource,
}

impl EvmMetadata {
    pub fn new(chain: Arc<EvmChain>, source: EvmSource) -> Self {
        Self { chain, source }
    }

    pub fn symbol(&self) -> String {
        match &self.source {
            EvmSource::NativeCurrency { symbol } => symbol.to_string(),
            EvmSource::ERC20 {
                symbol,
                contract_address: _,
            } => symbol.to_string(),
        }
    }

    pub fn label(&self) -> Option<String> {
        Some(self.chain.name.clone())
    }

    pub fn logo(&self) -> Option<String> {
        match self.source {
            EvmSource::NativeCurrency { symbol: _ } => self.chain.icon.clone(),
            EvmSource::ERC20 {
                symbol: _,
                contract_address: _,
            } => None,
        }
    }
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum EvmSource {
    NativeCurrency {
        symbol: String,
    },
    ERC20 {
        symbol: String,
        contract_address: EvmAddress,
    },
}
