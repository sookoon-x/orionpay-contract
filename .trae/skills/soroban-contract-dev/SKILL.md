---
name: "soroban-contract-dev"
description: "Assists with Soroban smart contract development including project scaffolding, module setup, and blockchain integration. Invoke when working on Stellar Soroban projects or smart contract development tasks."
---

# Soroban Contract Development Skill

This skill provides specialized assistance for developing Soroban smart contracts on the Stellar blockchain. It helps with project scaffolding, module management, and cross-chain integration.

## Key Features

- **Project Scaffolding**: Sets up proper directory structure for multi-chain smart contract projects
- **Module Management**: Creates and manages core contract modules (payments, escrow, subscriptions, split payments, bulk payments)
- **Shared Utilities**: Generates shared error handling, type definitions, and utilities
- **Chain-Specific Integration**: Creates Stellar-specific contract implementations with proper Soroban SDK integration
- **Compilation Helpers**: Assists with running cargo checks and resolving common Rust/Soroban compilation issues

## When to Invoke

- When starting a new Soroban smart contract project
- When scaffolding core payment modules for a blockchain application
- When setting up multi-chain support (Stellar, Ethereum, Polygon, Solana)
- When resolving compilation errors in Soroban contracts
- When adding new features to existing Soroban projects

## Usage

The skill automatically handles:
- Proper #![no_std] and alloc crate setup for Soroban compatibility
- Shared error handling with OrionError enum
- Contract type serialization with #[contracttype] macro
- On-chain storage operations using env.storage().persistent()
- Cross-module type consistency across all contract features