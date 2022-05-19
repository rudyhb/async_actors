use ic_cdk::export::Principal;
use ic_cdk::api::call;
use candid::Nat;

const ACTOR2_1: &'static str = "ryjl3-tyaaa-aaaaa-aaaba-cai";
const ACTOR2_2: &'static str = "r7inp-6aaaa-aaaaa-aaabq-cai";

#[ic_cdk_macros::update]
async fn run() {
    ic_cdk::println!("start actor1");
    let principal1 = Principal::from_text(ACTOR2_1).unwrap();
    let principal2 = Principal::from_text(ACTOR2_2).unwrap();
    let (_response1, _response2): (Result<(), _>, Result<(),_>) = futures::future::join(
        call::call(principal1, "run1", (Nat::from(3u64),)),
        call::call(principal2, "run1", (Nat::from(1u64),))
    ).await;
    _response1.unwrap();
    _response2.unwrap();
    ic_cdk::println!("end actor1");
}
