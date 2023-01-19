#![no_std]
use soroban_sdk::{
    contractimpl, contracttype, contracterror, symbol, 
    Env, Symbol, Bytes, Address
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

const FLYT_ID: Symbol = symbol!("FLYT_ID");

#[contracttype]
#[derive(Debug, Eq, PartialEq)]
pub struct Flyt {
    pub from: Address,
    pub from_nick: Symbol,
    pub to: Address,
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
    CannotRespondIfNotDirectedTo = 3
}

impl Flyt {
    pub fn new(from: Address, to: Address, content: Bytes, from_nick: Symbol) -> (Flyt, FlytStats) {
        (Flyt { from, from_nick, to, content, response: 0 }, FlytStats { likes: 0, tips: 0 })
    }

    pub fn respond(from: Address, to: Address, response: i128, content: Bytes, from_nick: Symbol) -> (Flyt, FlytStats) {
        (Flyt { from, from_nick, to, content, response }, FlytStats { likes: 0, tips: 0 })
    }
}

pub struct Contract;

#[contractimpl]
impl Contract {
    /// Registers a new flyt
    pub fn send_flyt(env: Env, recipient: Address, content: Bytes, nickname: Option<Symbol>) -> i128 {
        let id: i128 = Self::get_count(env.clone()) + 1;
        let nick = match nickname {
            None => symbol!(""),
            Some(i) => i
        };

        // Generate Flyt
        let (flyt, stats) = Flyt::new(env.invoker(), recipient, content, nick);

        // Store Flyt
        Self::store_new_flyt(env, id, flyt, stats);

        id
    } 

    // Registers a new flyt in response to a previous flyt
    pub fn res_flyt(env: Env, respond_to: i128, content: Bytes, nickname: Option<Symbol>) -> Result<i128, Error> {
        // Get previous flyt
        let prev: Flyt = Self::get_flyt(env.clone(), respond_to);

        // The account can only respond if it was sent to them
        if prev.to != env.invoker() {
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

    /// Adds a like to a flyt
    pub fn send_like(env: Env, id: i128) -> Result<(), Error> {
        let stats_id = id * -1;
        let mut stats: FlytStats = env.storage()
            .get(stats_id)
            .unwrap()
            .unwrap();

        stats.likes += 1;

        env.storage().set(stats_id, stats);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::{Contract, ContractClient};
    use soroban_sdk::{bytes, Env, Address, testutils::AccountId, symbol};

    #[test]
    fn test_send_flyt() {
        let env = Env::default();
        let contract_id = env.register_contract(None, Contract);
        let client = ContractClient::new(&env, &contract_id);

        let recipient = <soroban_sdk::AccountId as AccountId>::random(&env);
        let recipient = Address::Account(recipient);

        let id = client.send_flyt(&recipient, &bytes!(&env, 0x0123456789), &None);
        let flyt= client.get_flyt(&id);

        assert_eq!(flyt.from, Address::Account(env.source_account()));
        assert_eq!(flyt.from_nick, symbol!(""));
        assert_eq!(flyt.to, recipient);
        assert_eq!(flyt.content, bytes!(&env, 0x0123456789));
        assert_eq!(flyt.response, 0);
    }

    #[test]
    fn test_response_flyt() {
        let env = Env::default();
        let contract_id = env.register_contract(None, Contract);
        let client = ContractClient::new(&env, &contract_id);

        let recipient_id = <soroban_sdk::AccountId as AccountId>::random(&env);
        let recipient = Address::Account(recipient_id.clone());
        let id = client.send_flyt(&recipient, &bytes!(&env, 0x0123456789), &None);

        env.set_source_account(&recipient_id);
        let res_id = client.res_flyt(&id, &bytes!(&env, 0x0123), &None);

        let flyt= client.get_flyt(&res_id);

        // assert_eq!(flyt.from, Address::Account(env.source_account()));
        assert_eq!(flyt.from_nick, symbol!(""));
        assert_eq!(flyt.from, recipient);
        // assert_eq!(flyt.to, Address::Account(original_source_account));
        assert_eq!(flyt.content, bytes!(&env, 0x0123));
        assert_eq!(flyt.response, id);
    }
}