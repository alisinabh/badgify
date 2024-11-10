use std::sync::Arc;

use serde::Serialize;

use crate::{evm_chainlist::EvmChain, types::EvmAddress};

#[derive(Serialize)]
pub struct EvmMetadata {
    chain: Arc<EvmChain>,
    source: EvmSource,
}

impl EvmMetadata {
    pub fn new(chain: Arc<EvmChain>, source: EvmSource) -> Self {
        Self { chain, source }
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
