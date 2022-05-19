use candid::Nat;
use ic_cdk::api::call;
use ic_cdk::export::Principal;

const ACTOR2S: [&'static str; 10] = [
    "ryjl3-tyaaa-aaaaa-aaaba-cai",
    "r7inp-6aaaa-aaaaa-aaabq-cai",
    "rkp4c-7iaaa-aaaaa-aaaca-cai",
    "rno2w-sqaaa-aaaaa-aaacq-cai",
    "renrk-eyaaa-aaaaa-aaada-cai",
    "rdmx6-jaaaa-aaaaa-aaadq-cai",
    "qoctq-giaaa-aaaaa-aaaea-cai",
    "qjdve-lqaaa-aaaaa-aaaeq-cai",
    "qaa6y-5yaaa-aaaaa-aaafa-cai",
    "qhbym-qaaaa-aaaaa-aaafq-cai",
];

#[ic_cdk_macros::update]
async fn run() {
    ic_cdk::println!("start actor1");
    let principals: Vec<_> = ACTOR2S
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
