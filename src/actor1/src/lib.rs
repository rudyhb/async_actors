use candid::Nat;
use ic_cdk::api::call;
use ic_cdk::export::Principal;

const ACTOR2_1: &'static str = "ryjl3-tyaaa-aaaaa-aaaba-cai";
const ACTOR2_2: &'static str = "r7inp-6aaaa-aaaaa-aaabq-cai";
const ACTOR2_3: &'static str = "rdmx6-jaaaa-aaaaa-aaadq-cai";
const ACTOR2_4: &'static str = "qoctq-giaaa-aaaaa-aaaea-cai";
const ACTOR2_5: &'static str = "qjdve-lqaaa-aaaaa-aaaeq-cai";
const ACTOR2_6: &'static str = "qaa6y-5yaaa-aaaaa-aaafa-cai";
const ACTOR2_7: &'static str = "qhbym-qaaaa-aaaaa-aaafq-cai";
const ACTOR2_8: &'static str = "qsgjb-riaaa-aaaaa-aaaga-cai";
const ACTOR2_9: &'static str = "qvhpv-4qaaa-aaaaa-aaagq-cai";
const ACTOR2_10: &'static str = "renrk-eyaaa-aaaaa-aaada-cai";

#[ic_cdk_macros::update]
async fn run() {
    ic_cdk::println!("start actor1");
    let principals: Vec<_> = [
        ACTOR2_1, ACTOR2_2, ACTOR2_3, ACTOR2_4, ACTOR2_5, ACTOR2_6, ACTOR2_7, ACTOR2_8, ACTOR2_9,
        ACTOR2_10,
    ]
    .iter()
    .map(|p| Principal::from_text(p).unwrap())
    .collect();
    let mut responses: Vec<(String, Nat)> =
        futures::future::join_all(principals.into_iter().map(|p| async move {
            let result: Result<(Nat,), _> = call::call(p, "get_num", ()).await;
            (p.to_string(), result.unwrap().0.clone())
        }))
        .await
        .into_iter()
        .collect();
    responses.sort_by(|a, b| a.0.cmp(&b.0));
    ic_cdk::println!(
        "responses:\n{}",
        responses
            .into_iter()
            .map(|r| format!("{}: {}", r.0, r.1.to_string()))
            .collect::<Vec<String>>()
            .join("\n")
    );
    ic_cdk::println!("end actor1");
}
