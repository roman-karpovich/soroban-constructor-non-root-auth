#![cfg(test)]
extern crate std;

use crate::{SmartContract, SmartContractArgs, SmartContractClient};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, Env};

pub fn create_contract<'a>(e: &Env, admin: &Address) -> SmartContractClient<'a> {
    SmartContractClient::new(
        e,
        &e.register(SmartContract, SmartContractArgs::__constructor(admin)),
    )
}

#[test]
fn test() {
    let e = Env::default();
    e.mock_all_auths();
    e.budget().reset_unlimited();
    let admin = Address::generate(&e);

    create_contract(&e, &admin);
}
