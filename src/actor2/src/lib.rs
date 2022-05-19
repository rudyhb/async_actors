use std::ops::{Mul, Rem};
use candid::Nat;

#[ic_cdk_macros::update]
fn run1(delay: Nat) {
    ic_cdk::println!("start actor2");
    let mut _counter = Nat::from(1u64);
    let delay = delay.mul(1_000_000u64);
    let zero = Nat::from(0u64);

    // simulate delay
    while _counter.clone().rem(delay.clone()) != zero {
        _counter += 1;
    }
    ic_cdk::println!("end actor2");
}
