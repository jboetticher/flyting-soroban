#![no_std]
use soroban_sdk::{
    contractimpl, contracttype, contracterror, symbol, 
    Env, Symbol, Bytes, bytes, Address, AccountId
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

const INSULT: Symbol = symbol!("INSULT");
const FLYT_ID: Symbol = symbol!("FLYT_ID");

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
    SameUser = 1,
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

    pub fn send_flyt(env: Env, recipient: AccountId, content: Bytes, nickname: Option<Symbol>) -> i128 {
        let id: i128 = env.storage()
            .get(FLYT_ID)
            .unwrap_or(Ok(0))
            .unwrap() + 1;
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

    pub fn res_flyt(env: Env, respond_to: i128, content: Bytes, nickname: Option<Symbol>) -> Result<i128, Error> {
        // Get previous flyt
        let prev: Flyt = Self::get_flyt(env.clone(), respond_to);

        // The account can only respond if it was sent to them
        if prev.to != env.source_account() {
            
        }

        let id: i128 = env.storage()
            .get(FLYT_ID)
            .unwrap_or(Ok(0))
            .unwrap() + 1;
        let nick = match nickname {
            None => symbol!(""),
            Some(i) => i
        };

        // Generate Flyt
        let (flyt, stats) = Flyt::respond(prev.to, prev.from, respond_to, content, nick);

        // Store Flyt
        Self::store_new_flyt(env, id, flyt, stats);

        id
    }

    fn store_new_flyt(env: Env, id: i128, flyt: Flyt, stats: FlytStats) {
        // Store flyt at id
        env.storage().set(id, flyt);

        // Store data at -id
        let data_id = id * -1;
        env.storage().set(data_id, stats);
    }

    pub fn get_flyt(env: Env, id: i128) -> Flyt {
        env.storage()
            .get(id)
            .unwrap()
            .unwrap()
    }

    pub fn insult(env: Env, insult: Bytes) {
        env.storage().set(INSULT, insult);
    }

    pub fn get_insult(env: Env) -> Bytes {
        env.storage()
            .get(INSULT)
            .unwrap_or(Ok(bytes!(&env, 0x0)))
            .unwrap()
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

        let insult = client.get_insult();
        assert_eq!(insult, bytes!(&env, 0x0));

        client.insult(&bytes!(&env, 0x123456789));
        let insult = client.get_insult();
        assert_eq!(insult, bytes!(&env, 0x123456789));
    }
}