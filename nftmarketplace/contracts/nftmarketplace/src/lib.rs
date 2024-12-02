#![no_std]



use soroban_sdk::{ symbol_short, Address, Env, String, Symbol};

pub struct NFTContract;

#[derive(Clone)]
pub struct NFT {
    pub owner: Address,
    pub metadata: String,
}



impl NFTContract {
    // Storage Keys
    const OWNER: Symbol = symbol_short!("owner");
    const META: Symbol = symbol_short!("meta");
    const NEXT_ID: Symbol = symbol_short!("next_id");

    /// Mint a new NFT
    pub fn mint(env: Env, owner: Address, metadata: String) -> u32 {
        let token_id = Self::next_token_id(&env);

        // Store the owner and metadata
        env.storage().instance().set(&(Self::OWNER, token_id), &owner);
        env.storage().instance().set(&(Self::META, token_id), &metadata);

        env.events().publish(
            (symbol_short!("Minted"), &owner),
            token_id,
        );

        token_id
    }

    /// Get NFT owner by token ID
    pub fn get_owner(env: Env, token_id: u32) -> Address {
        env.storage()
            .instance()
            .get(&(Self::OWNER, token_id))
            .expect("Token ID not found")
    }

    /// Get NFT metadata by token ID
    pub fn get_metadata(env: Env, token_id: u32) -> String {
        env.storage()
            .instance()
            .get(&(Self::META, token_id))
            .expect("Token ID not found")
    }

    /// Transfer ownership of the NFT
    pub fn transfer(env: Env, token_id: u32, new_owner: Address) {
        let current_owner: Address = Self::get_owner(env.clone(), token_id);

        // Ensure the caller is the current owner
        let caller = env.current_contract_address();
        if caller != current_owner {
            panic!("Caller is not the owner of the token");
        }

        // Update the owner
        env.storage()
            .instance()
            .set(&(Self::OWNER, token_id), &new_owner);

        env.events().publish(
            (symbol_short!("Transfer"), &current_owner),
            &new_owner,
        );
    }

    /// Get the next token ID
    fn next_token_id(env: &Env) -> u32 {
        let id: u32 = env
            .storage()
            .instance()
            .get(&Self::NEXT_ID)
            .unwrap_or(0);
        env.storage().instance().set(&Self::NEXT_ID, &(id + 1));
        id
    }
}


/*stellar contract invoke `
  --id CCRDWI2C2IQAFCKUPVV3GRMBFP74PYO5UCMAX5H2MA77MQKNNLX2E6IX `
  --source alice `
  --network testnet `
  -- `
  hello `
  --to RPC */