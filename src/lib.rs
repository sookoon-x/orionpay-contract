#![no_std]
extern crate alloc;
use soroban_sdk::{contract, contractimpl, Env, String};

// Shared modules that contain common types and errors used across all features
pub mod shared {
    pub use crate::shared::errors::OrionError;
    pub use crate::shared::types::{Payment, Recipient, Status};
    
    mod errors;
    mod types;
}

// Core feature modules
pub mod payments;
pub mod bulk_payments;
pub mod split_payments;

#[contract]
pub struct OrionPay;

#[contractimpl]
impl OrionPay {
    /// Version of the OrionPay contract
    pub fn version(env: Env) -> String {
        String::from_str(&env, "0.1.0")
    }
    
    /// Simple hello function to verify the contract is working
    pub fn hello(_env: Env, to: String) -> String {
        // Simplified to avoid string concatenation issues
        to
    }
}

#[cfg(test)]
mod test;