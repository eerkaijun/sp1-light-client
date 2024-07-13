//! A light client state transition program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

use std::time::SystemTime;
use ssz_rs::prelude::*;

use common::consensus::verify_update;
use common::config::types::Forks;
use common::consensus::types::{LightClientStore, Update};
pub use ssz_rs::prelude::{Bitvector, Vector};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ProofInputs {
    pub update: Update,
    pub now: SystemTime,
    pub genesis_time: u64,
    pub store: LightClientStore,
    pub genesis_root: Vec<u8>,
    pub forks: Forks,
}

pub fn main() {
    let encoded_inputs = sp1_zkvm::io::read_vec();

    let ProofInputs {
        update,
        now,
        genesis_time,
        store,
        genesis_root,
        forks,
    } = serde_cbor::from_slice(&encoded_inputs).unwrap();

    let is_valid = verify_update(&update, now, genesis_time, &store, genesis_root, &forks).is_ok();

    assert!(is_valid);

    sp1_zkvm::io::commit(&is_valid);
}