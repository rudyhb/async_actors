use candid::{CandidType, Nat};
use serde::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
// use std::ops::{Mul, Rem};

#[ic_cdk_macros::update]
fn get_num() -> Nat {
    let my_id = ic_cdk::api::id();
    ic_cdk::println!("start actor2 at {}", my_id);

    // let mut _counter = Nat::from(1u64);
    // let delay = Nat::from(2u64).mul(1_000_000u64);
    // let zero = Nat::from(0u64);
    // // simulate delay
    // while _counter.clone().rem(delay.clone()) != zero {
    //     _counter += 1;
    // }

    let storage: &mut Storage = ic_cdk::storage::get_mut();
    let mut hasher = DefaultHasher::new();
    my_id.to_string().hash(&mut hasher);
    storage.value += Nat::from(hasher.finish() % 100);

    ic_cdk::println!("end actor2 at {}", my_id);
    ic_cdk::storage::get::<Storage>().value.clone()
}

#[derive(CandidType, Deserialize, Default)]
struct Storage {
    value: Nat,
}
