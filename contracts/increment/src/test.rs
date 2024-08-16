#![cfg(test)]

use super::{IncrementContract, IncrementContractClient};
use soroban_sdk::{testutils::Logs, Env};

extern crate std;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    //test increment function
    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);

    //test decrement function
    assert_eq!(client.decrement(), 2);

    //test get_current_value function
    assert_eq!(client.get_current_value(), 2);

    //test reset function
    assert_eq!(client.reset(), 0);

    std::println!("{}", env.logs().all().join("\n"));
}
