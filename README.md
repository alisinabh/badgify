# Badgify

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Create beautiful, real-time cryptocurrency badges for your projects. Available at [badgify.io](https://badgify.io)

## Overview

Badgify is a powerful tool that generates dynamic badges displaying real-time cryptocurrency balances for your projects. Whether you want to showcase wallet balances, token holdings, or other blockchain metrics, Badgify makes it easy to create and embed these badges in your documentation, websites, or GitHub repositories.

### Features

- **Dynamic Badges**: Generate real-time badges showing wallet balances, token holdings, and more
- **Multi-Chain Support**: Compatible with Ethereum, Bitcoin, and multiple EVM-compatible networks including testnets
- **Token Tracking**: Track ERC20 token balances with custom thresholds and styling options
- **Customization**: Personalize your badges with different styles, colors, and icons
- **Block Explorer Integration**: Optional linking to blockchain explorers for detailed transaction views

## Getting Started

Visit [badgify.io](https://badgify.io) and follow these simple steps:

1. **Choose Your Network**: Select from supported networks (Ethereum Mainnet, Bitcoin, or any EVM-compatible chain)
2. **Enter Wallet Address**: Input the wallet address you want to track
3. **Customize & Deploy**: Customize your badge with different styles and copy the generated code

### Usage Examples

Vitalk's ETH Balance: [![ethereum Balance](https://badgify.io/badge/evm/1/balance/0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045?v=2)](https://badgify.io/scanner/evm/1/balance/0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045) [![ethereum Balance](https://badgify.io/badge/evm/8453/balance/0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045?v=2)](https://badgify.io/scanner/evm/8453/balance/0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045)

Tether's Treasury USDT Balance: [![ethereum Balance](https://badgify.io/badge/evm/1/erc20_balance/0xdac17f958d2ee523a2206206994597c13d831ec7/0x5754284f345afc66a98fbB0a0Afe71e0F007B949?v=2)](https://badgify.io/scanner/evm/1/erc20_balance/0xdac17f958d2ee523a2206206994597c13d831ec7/0x5754284f345afc66a98fbB0a0Afe71e0F007B949)

Generate yours at [badgify.io](https://badgify.io/generator)

## Development

### Prerequisites

- Node.js (v18 or higher)
- Rust (latest stable)
- pnpm (for frontend dependencies)

### Project Structure

```
badgify/
├── src/             # Rust backend code
├── ui/              # React frontend code
│   ├── src/
│   │   ├── components/
│   │   ├── lib/
│   │   └── ...
│   └── ...
└── ...
```

### Setup

1. Clone the repository:

```bash
git clone https://github.com/yourusername/badgify.git
cd badgify
```

2. Install frontend dependencies:

```bash
cd ui
pnpm install
```

3. Build and run the development server:

```bash
# Terminal 1 - Frontend
cd ui
pnpm dev

# Terminal 2 - Backend
cargo run
```

## Contributing

We welcome contributions to Badgify! Feel free to open Issues and/or Pull Requests if you want to add a feature
or fix a bug.

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/), [React](https://reactjs.org/), and [shadcn/ui](https://ui.shadcn.com/)
- Special thanks to the blockchain community for their continued support
