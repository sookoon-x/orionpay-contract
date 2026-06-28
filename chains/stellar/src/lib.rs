#![no_std]
extern crate alloc;

use soroban_sdk::{contract, contractimpl, Env, String, Address, Vec};
use orionpay_contract::payments::Payments;
use orionpay_contract::split_payments::SplitPayments;
use orionpay_contract::bulk_payments::BulkPayments;
use orionpay_contract::shared::{Payment as SharedPayment, OrionError, Recipient, BulkPaymentRecipient};

#[contract]
pub struct OrionPayStellar;

#[contractimpl]
impl OrionPayStellar {
    /// Get the contract version
    pub fn version(env: Env) -> String {
        String::from_str(&env, "0.1.0-stellar")
    }

    // ============ Single Payments ============
    
    /// Create a new single payment
    pub fn create_payment(
        env: Env,
        sender: Address,
        recipient: Address,
        amount: i128,
        asset: String,
        _memo: Option<String>,
    ) -> Result<SharedPayment, OrionError> {
        Payments::create(env, sender, recipient, amount, asset, _memo)
    }

    /// Get a payment by ID
    pub fn get_payment(env: Env, payment_id: String) -> Result<SharedPayment, OrionError> {
        Payments::get(env, payment_id)
    }

    // ============ Split Payments ============
    
    /// Create a new split payment
    pub fn create_split_payment(
        env: Env,
        sender: Address,
        recipients: Vec<Recipient>,
        total_amount: i128,
        asset: String,
    ) -> Result<String, OrionError> {
        SplitPayments::create(env, sender, recipients, total_amount, asset)
    }

    /// Process a recipient in a split payment
    pub fn process_split_recipient(
        env: Env,
        split_id: String,
        recipient_index: u32,
    ) -> Result<(), OrionError> {
        SplitPayments::process_recipient(env, split_id, recipient_index)
    }

    // ============ Bulk Payments ============
    
    /// Create a new bulk payment
    pub fn create_bulk_payment(
        env: Env,
        sender: Address,
        recipients: Vec<BulkPaymentRecipient>,
    ) -> Result<String, OrionError> {
        BulkPayments::create(env, sender, recipients)
    }

    /// Process a recipient in a bulk payment
    pub fn process_bulk_recipient(
        env: Env,
        bulk_id: String,
        recipient_index: u32,
    ) -> Result<(), OrionError> {
        BulkPayments::process_recipient(env, bulk_id, recipient_index)
    }
}

#[cfg(test)]
mod test;