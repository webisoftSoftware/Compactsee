# CompactSee

A real-time contract event monitoring tool for the Midnight blockchain network. Built with Rust and Leptos, CompactSee provides a clean, responsive interface for developers to monitor smart contract events and state changes as they happen.

🌐 **Live Demo**: [https://compactsee.fly.dev](https://compactsee.fly.dev)

## What is CompactSee?

CompactSee is a web application that connects to Midnight's testnet indexer to provide real-time monitoring of smart contract events. Simply enter a contract address, and CompactSee will:

- **Monitor contract events** in real-time via WebSocket connection
- **Display contract state changes** with detailed, readable formatting
- **Track deployment, calls, and updates** to your contracts
- **Provide a clean interface** to browse through event history
- **Auto-disconnect after 5 minutes** to prevent resource leaks

## Why is CompactSee Useful?

### For Developers
- **Real-time debugging**: See contract events as they happen during development and testing
- **State inspection**: View contract state changes in a human-readable format
- **Event tracking**: Monitor contract deployments, calls, and updates without manual blockchain exploration
- **Testing workflow**: Quickly verify contract behavior during development cycles

### For Midnight Ecosystem
- **Developer tooling**: Provides essential monitoring capabilities for Midnight smart contract development
- **Network transparency**: Makes contract activity visible and accessible
- **Educational resource**: Helps developers understand how Midnight contracts behave in practice

## Features

- 🔗 **Real-time WebSocket connection** to Midnight testnet indexer
- 📊 **Event visualization** with clean, organized interface
- 🔍 **State inspection** with formatted contract state display
- ⏱️ **Automatic timeout** (5 minutes) to prevent resource leaks
- ⚡ **Fast and efficient** Rust-based backend with Leptos frontend

## Technology Stack

- **Backend**: Rust with Axum web server
- **Frontend**: Leptos (Rust-based reactive framework)
- **Styling**: Tailwind CSS with DaisyUI components
- **Blockchain**: Midnight network integration
- **Real-time**: WebSocket connections for live event streaming

## Getting Started

CompactSee is designed to work with Midnight testnet contracts. Simply:

1. Enter a valid Midnight testnet contract address
2. Click "Connect" to start monitoring
3. View real-time events and state changes
4. The connection automatically times out after 5 minutes

### Test Contract Address

To quickly test CompactSee, you can use this sample contract address:
```
0200cc2f4e37bb554c344c04aff7ad746d8df129a4985d3908b509712b4cd721f163
```

Perfect for developers building on Midnight who need to monitor their smart contracts during development and testing phases.
