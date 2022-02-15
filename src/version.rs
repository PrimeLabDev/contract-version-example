use crate::Counter;
use contract_version::{version_from_env, IVersion, Version};
use near_sdk::near_bindgen;

#[cfg(not(target_arch = "wasm32"))]
use crate::CounterContract;

#[near_bindgen]
impl IVersion for Counter {
    fn version(&self) -> Version {
        version_from_env!()
    }
}
