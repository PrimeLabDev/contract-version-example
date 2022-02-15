#![allow(clippy::ref_in_deref)]

use contract_version::Version;
pub use near_sdk::json_types::{Base64VecU8, U64};
use near_sdk_sim::{self as sim, call, view};

pub mod utils;

#[test]
fn test_counter() {
    let root = sim::init_simulator(None);
    let counter = utils::setup_counter(&root);

    let res = call!(root, counter.increment());
    let val: u32 = res.unwrap_json();
    assert_eq!(val, 1);

    let res = view!(counter.version());
    let version: Version = res.unwrap_json();
    assert_eq!(version.name, "example-counter".to_string());
    assert_eq!(version.semver, "0.0.2".to_string());
}
