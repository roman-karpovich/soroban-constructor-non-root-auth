#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct SmartContract;

#[contractimpl]
impl SmartContract {
    pub fn __constructor(_e: Env, admin: Address) {
        admin.require_auth();
    }
}

mod test;
