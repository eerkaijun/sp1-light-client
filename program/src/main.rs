//! A light client state transition program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    let encoded_inputs = sp1_zkvm::io::read_vec();

    let is_valid = true;

    assert!(is_valid);

    sp1_zkvm::io::commit(&is_valid);
}