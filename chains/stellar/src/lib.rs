#![no_std]
extern crate alloc;

use soroban_sdk::{contract, contractimpl, Env, String, Address};
use orionpay_contract::payments::Payments;
use orionpay_contract::shared::{Payment as SharedPayment, OrionError};

#[contract]
pub struct OrionPayStellar;

#[contractimpl]
impl OrionPayStellar {
    /// Get the contract version
    pub fn version(env: Env) -> String {
        String::from_str(&env, "0.1.0-stellar")
    }

    /// Create a new payment
    pub fn create_payment(
        env: Env,
        sender: Address,
        recipient: Address,
        amount: i128,
        asset: String,
        memo: Option<String>,
    ) -> Result<SharedPayment, OrionError> {
        Payments::create(env, sender, recipient, amount, asset, memo)
    }
}

#[cfg(test)]
mod test;