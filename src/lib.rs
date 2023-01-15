#![no_std]
use soroban_sdk::{contractimpl, symbol, Env, Symbol, Bytes, bytes};

/* 
Notes about how Soroban works:
- There is no Heap. This means that you have to use the soroban structs to reproduce vec
- Data is retrieved with env.storage().get("KEY");
- Data is stored with env.storage().set("KEY", 420);
*/



const INSULT: Symbol = symbol!("INSULT");

pub struct Contract;

#[contractimpl]
impl Contract {

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