mod state_machine;
use candid::{candid_method, export_service};
use ic_cdk::api::call::ManualReply;
use ic_cdk_macros::{query, pre_upgrade, post_upgrade, update};
use state_machine::State;
use std::cell::{RefCell};
use candid::types::number::Nat;
thread_local! {
    static STATE: RefCell<State> =RefCell::new(State::default())
}

#[candid_method(update)]
#[update]
fn increment() {
    STATE.with(|state: &RefCell<State> | state.borrow_mut().increment());
}