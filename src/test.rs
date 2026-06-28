#![cfg(test)]
use super::*;
use soroban_sdk::{Env, String};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(OrionPay, ());
    let client = OrionPayClient::new(&env, &contract_id);
    
    // Test version
    let version = client.version();
    assert_eq!(version, String::from_str(&env, "0.1.0"));
    
    // Test hello
    let result = client.hello(&String::from_str(&env, "Orion"));
    assert_eq!(result, String::from_str(&env, "Hello Orion"));
}