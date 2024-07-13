//! A light client state transition program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    let n = sp1_zkvm::io::read::<u32>();
    let is_valid = n % 2 == 0;

    assert!(is_valid);

    sp1_zkvm::io::commit(&is_valid);
}