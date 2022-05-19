use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Mul, Rem};
use once_cell::sync::Lazy;
use parking_lot::{RwLock};
use candid::{Nat,CandidType};
use serde::*;

#[ic_cdk_macros::update]
fn get_num() -> Nat {
    let my_id = ic_cdk::api::id();
    ic_cdk::println!("start actor2 at {}", my_id);
    let mut _counter = Nat::from(1u64);
    let delay = Nat::from(3u64).mul(1_000_000u64);
    let zero = Nat::from(0u64);

    // simulate delay
    while _counter.clone().rem(delay.clone()) != zero {
        _counter += 1;
    }
    let old = STORAGE.read().value.clone();

    let mut hasher = DefaultHasher::new();
    my_id.to_string().hash(&mut hasher);
    STORAGE.write().value = old.add(Nat::from(hasher.finish() % 100));

    let ret = STORAGE.read().value.clone();
    ic_cdk::println!("end actor2: {}", ret);
    ret
}

#[derive(CandidType, Deserialize, Default)]
struct Storage {
    value: Nat,
}

static STORAGE: Lazy<RwLock<Storage>> = Lazy::new(|| RwLock::new(Storage::default()));