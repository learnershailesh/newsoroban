#![cfg(test)]

extern crate std;

use soroban_sdk::{
    testutils::{Env as TestEnv, Address as TestAddress},
    Address, Env, Symbol, String,
};
use crate::{NFTContract, NFTContractClient};

#[test]
fn test_mint() {
    let env = Env::default();
    let contract_address = env.register_contract(None, NFTContract);

    let client = NFTContractClient::new(&env, &contract_address);

    let owner = Address::from_id(&[1; 32]);
    let metadata = String::from("First NFT Metadata");

    // Mint an NFT
    let token_id = client.mint(&owner, &metadata);

    // Assert the token ID starts at 0
    assert_eq!(token_id, 0);

    // Assert the owner and metadata were stored correctly
    assert_eq!(client.get_owner(&token_id), owner);
    assert_eq!(client.get_metadata(&token_id), metadata);
}

#[test]
fn test_transfer() {
    let env = Env::default();
    let contract_address = env.register_contract(None, NFTContract);

    let client = NFTContractClient::new(&env, &contract_address);

    let owner = Address::from_id(&[1; 32]);
    let new_owner = Address::from_id(&[2; 32]);
    let metadata = String::from("First NFT Metadata");

    // Mint an NFT
    let token_id = client.mint(&owner, &metadata);

    // Attempt transfer from a non-owner (should fail)
    env.set_source_account(&new_owner);
    assert_panics!(
        client.transfer(&token_id, &new_owner),
        "Caller is not the owner of the token"
    );

    // Transfer the NFT from the owner
    env.set_source_account(&owner);
    client.transfer(&token_id, &new_owner);

    // Assert the ownership was updated
    assert_eq!(client.get_owner(&token_id), new_owner);
}

#[test]
fn test_multiple_mints() {
    let env = Env::default();
    let contract_address = env.register_contract(None, NFTContract);

    let client = NFTContractClient::new(&env, &contract_address);

    let owner1 = Address::from_id(&[1; 32]);
    let metadata1 = String::from("First NFT Metadata");
    let owner2 = Address::from_id(&[2; 32]);
    let metadata2 = String::from("Second NFT Metadata");

    // Mint two NFTs
    let token_id1 = client.mint(&owner1, &metadata1);
    let token_id2 = client.mint(&owner2, &metadata2);

    // Assert the token IDs increment
    assert_eq!(token_id1, 0);
    assert_eq!(token_id2, 1);

    // Assert owners and metadata
    assert_eq!(client.get_owner(&token_id1), owner1);
    assert_eq!(client.get_metadata(&token_id1), metadata1);

    assert_eq!(client.get_owner(&token_id2), owner2);
    assert_eq!(client.get_metadata(&token_id2), metadata2);
}

