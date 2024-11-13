use serde::Serialize;

use crate::query::bitcoin::BitcoinNetwork;

#[derive(Serialize)]
pub struct BitcoinMetadata {
    pub network: BitcoinNetwork,
}

impl BitcoinMetadata {
    pub fn new(network: BitcoinNetwork) -> Self {
        Self { network }
    }

    pub fn symbol(&self) -> String {
        "BTC".to_string()
    }

    pub fn label(&self) -> Option<String> {
        "Bitcoin".to_string().into()
    }

    pub fn logo(&self) -> Option<String> {
        "bitcoin".to_string().into()
    }
}
