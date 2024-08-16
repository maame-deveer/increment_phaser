#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Increment increments an internal counter, and returns the value.
    pub fn increment(env: Env) -> u32 {
        // Get the current count.
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0); // If no value set, assume 0.
        log!(&env, "count: {}", count);

        // Increment the count.
        count += 1;

        // Save the count.
        env.storage().instance().set(&COUNTER, &count);

        env.storage().instance().extend_ttl(50, 100);

        // Return the count to the caller.
        count
    }

    //decrement counter
    pub fn decrement(env: Env) -> u32 {
        // Get the current count.
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0); // If no value set, assume 0.

        //decrease value
        count -= 1;
        log!(&env, "count: {}", count);

        // Save the count.
        env.storage().instance().set(&COUNTER, &count);

        // Return the count to the caller.
        count
    }

    //get current counter value
    pub fn get_current_value(env: Env) -> u32 {
        // Get the current count.
        let count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0); // If no value set, assume 0.
        log!(&env, "count: {}", count);

        // Return the count to the caller.
        count
    }

    //reset counter
    pub fn reset(env: Env) -> u32 {
        // Get the current count.
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0); // If no value set, assume 0.

        //set count to zero
        count -= count ;
        log!(&env, "count: {}", count);

        // Save the count.
        env.storage().instance().set(&COUNTER, &count);

        // Return the count to the caller.
        count
    }
}

mod test;
