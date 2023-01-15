#![no_std]
use soroban_sdk::{
    contractimpl, contracttype, contracterror, symbol, 
    Env, Symbol, Bytes, bytes, Address, AccountId, Map
};

/* 
Notes about how Soroban works:
- There is no Heap. This means that you have to use the soroban structs to reproduce vec
- Data is retrieved with env.storage().get("KEY");
- Data is stored with env.storage().set("KEY", 420);
*/

/*
Plan for Flyter:
- Flyt insults at a certain address
- People should be able to TIP flyts, and they should be able to LIKE flyts
- Flyts should be queryable
- Send a flyt with an optional nickname
*/

const INITIALIZED: Symbol = symbol!("READY");
const FLYT_ID: Symbol = symbol!("FLYT_ID");
const LIKES: Symbol = symbol!("LIKES_MAP");

#[contracttype]
#[derive(Debug, Eq, PartialEq)]
pub struct Flyt {
    pub from: AccountId,
    pub from_nick: Symbol,
    pub to: AccountId,
    pub content: Bytes,
    pub response: i128
}

#[contracttype]
#[derive(Default, Debug, Eq, PartialEq)]
pub struct FlytStats {
    pub likes: u128,
    pub tips: u128,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    CannotRespondIfNotDirectedTo = 3,
}

impl Flyt {
    pub fn new(from: AccountId, to: AccountId, content: Bytes, from_nick: Symbol) -> (Flyt, FlytStats) {
        (Flyt { from, from_nick, to, content, response: 0 }, FlytStats { likes: 0, tips: 0 })
    }

    pub fn respond(from: AccountId, to: AccountId, response: i128, content: Bytes, from_nick: Symbol) -> (Flyt, FlytStats) {
        (Flyt { from, from_nick, to, content, response }, FlytStats { likes: 0, tips: 0 })
    }
}

pub struct Contract;

#[contractimpl]
impl Contract {
    /// Used to initialize the smart contract
    pub fn initialize(env: Env) -> Result<(), Error> {
        if env.storage().get(INITIALIZED).unwrap_or(Ok(false)).unwrap() {
            return Err(Error::AlreadyInitialized);
        }

        // Creates a new map for likes

        Ok(())
    }

    /// Registers a new flyt
    pub fn send_flyt(env: Env, recipient: AccountId, content: Bytes, nickname: Option<Symbol>) -> i128 {
        let id: i128 = Self::get_count(env.clone()) + 1;
        let nick = match nickname {
            None => symbol!(""),
            Some(i) => i
        };

        // Generate Flyt
        let (flyt, stats) = Flyt::new(env.source_account(), recipient, content, nick);

        // Store Flyt
        Self::store_new_flyt(env, id, flyt, stats);

        id
    } 

    // Registers a new flyt in response to a previous flyt
    pub fn res_flyt(env: Env, respond_to: i128, content: Bytes, nickname: Option<Symbol>) -> Result<i128, Error> {
        // Get previous flyt
        let prev: Flyt = Self::get_flyt(env.clone(), respond_to);

        // The account can only respond if it was sent to them
        if prev.to != env.source_account() {
            return Err(Error::CannotRespondIfNotDirectedTo);
        }

        let id: i128 = Self::get_count(env.clone()) + 1;
        let nick = match nickname {
            None => symbol!(""),
            Some(i) => i
        };

        // Generate Flyt
        let (flyt, stats) = Flyt::respond(prev.to, prev.from, respond_to, content, nick);

        // Store Flyt
        Self::store_new_flyt(env, id, flyt, stats);

        Ok(id)
    }

    // Stores a flyt in its natural habitat
    fn store_new_flyt(env: Env, id: i128, flyt: Flyt, stats: FlytStats) {
        // Store flyt at id
        env.storage().set(id, flyt);

        // Store data at -id
        let data_id = id * -1;
        env.storage().set(data_id, stats);
    }

    /// Returns a flyt by its id, if it exists
    pub fn get_flyt(env: Env, id: i128) -> Flyt {
        env.storage()
            .get(id)
            .unwrap()
            .unwrap()
    }

    /// Returns the number of flyts that exist
    pub fn get_count(env: Env) -> i128 {
        env.storage()
            .get(FLYT_ID)
            .unwrap_or(Ok(0))
            .unwrap()
    }

    pub fn send_like(env: Env, id: i128) {
        
    }

}

#[cfg(test)]
mod test {
    use super::{Contract, ContractClient};
    use soroban_sdk::{Bytes, bytes, Env};

    #[test]
    fn test() {
        let env = Env::default();
        let contract_id = env.register_contract(None, Contract);
        let client = ContractClient::new(&env, &contract_id);

        // let insult = client.get_insult();
        // assert_eq!(insult, bytes!(&env, 0x0));

        // client.insult(&bytes!(&env, 0x123456789));
        // let insult = client.get_insult();
        // assert_eq!(insult, bytes!(&env, 0x123456789));
    }
}