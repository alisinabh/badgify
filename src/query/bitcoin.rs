type BitcoinAddress = String;

#[derive(Debug)]
pub enum BitcoinQuery {
    NativeBalance {
        network: BitcoinNetwork,
        address: BitcoinAddress,
    },
}

#[derive(Debug)]
pub enum BitcoinNetwork {
    Mainnet,
    Testnet,
    Signet,
}
