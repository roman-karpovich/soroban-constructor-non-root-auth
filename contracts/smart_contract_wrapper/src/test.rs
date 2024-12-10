#![cfg(test)]
extern crate std;

use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, Env};

mod smart_contract {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/smart_contract.wasm"
    );
}

pub fn create_contract<'a>(e: &Env, admin: &Address) -> smart_contract::Client<'a> {
    smart_contract::Client::new(
        e,
        &e.register(
            smart_contract::WASM,
            smart_contract::Args::__constructor(admin),
        ),
    )
}

#[test]
fn test() {
    let e = Env::default();
    e.mock_all_auths();
    e.budget().reset_unlimited();
    let admin = Address::generate(&e);

    e.mock_all_auths_allowing_non_root_auth();
    create_contract(&e, &admin);
}
