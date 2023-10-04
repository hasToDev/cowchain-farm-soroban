#![cfg(test)]

use super::*;
use soroban_sdk::{arbitrary::std, Env};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CowContract);
    let _client = CowContractClient::new(&env, &contract_id);

    env.as_contract(&contract_id, || {
        let mut gender = CowGender::Male;

        let value = env.prng().u64_in_range(1..=6);
        if value % 2 == 0 {
            gender = CowGender::Female;
        }

        std::println!("gender: {:#?}", &gender);
    })
}
