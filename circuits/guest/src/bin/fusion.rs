#![no_std]

use mugraph_core::{BlindedNote, Fusion, Hash, Join, Result, OUTPUT_SEP};

use risc0_zkvm::guest::env;

#[inline(always)]
fn run() -> Result<()> {
    let request: Join = env::read();

    let [input_a, input_b] = request.inputs;

    assert_eq!(input_a.asset_id, input_b.asset_id);
    assert!(!input_a.nullifier.is_empty());
    assert!(!input_b.nullifier.is_empty());
    assert_ne!(input_a.nullifier, input_b.nullifier);

    let a = Hash::digest(&input_a.as_bytes())?;
    let b = Hash::digest(&input_b.as_bytes())?;

    let total = input_a
        .amount
        .checked_add(input_b.amount)
        .expect("overflow in total amount");

    let output = BlindedNote {
        asset_id: input_a.asset_id,
        amount: total,
        secret: Hash::combine3(OUTPUT_SEP, a, b)?,
    };

    env::write(&output);

    let fusion = Fusion {
        a,
        b,
        c: Hash::digest(&output.as_bytes())?,
    };

    env::commit(&fusion);

    Ok(())
}

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => panic!("{}", e),
    }
}
