use std::sync::Arc;

use serde::Serialize;

use crate::{evm_chainlist::EvmChain, types::EVMAddress};

#[derive(Serialize)]
pub struct EVMMetadata {
    chain: Arc<EvmChain>,
    source: EVMSource,
}

impl EVMMetadata {
    pub fn new(chain: Arc<EvmChain>, source: EVMSource) -> Self {
        Self { chain, source }
    }
}

#[derive(Serialize)]
#[serde(tag = "type")]
pub enum EVMSource {
    NativeCurrency {
        symbol: String,
    },
    ERC20 {
        symbol: String,
        contract_address: EVMAddress,
    },
}
