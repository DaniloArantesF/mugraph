use core::ops::Range;

use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "std", derive(test_strategy::Arbitrary))]
pub struct Input {
    pub inputs: [Note; 2],
}

impl SerializeBytes for Input {
    const SIZE: usize = 2 * Note::SIZE;

    #[inline]
    fn to_slice(&self, out: &mut [u8]) {
        self.inputs[0].to_slice(&mut out[..Note::SIZE]);
        self.inputs[1].to_slice(&mut out[Note::SIZE..]);
    }

    #[inline]
    fn from_slice(input: &[u8]) -> Result<Self> {
        Ok(Self {
            inputs: [
                Note::from_slice(&input[..Note::SIZE])?,
                Note::from_slice(&input[Note::SIZE..Self::SIZE])?,
            ],
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "std", derive(test_strategy::Arbitrary))]
pub struct Output {
    pub a: Hash,
    pub b: Hash,
    pub c: Hash,
}

impl SerializeBytes for Output {
    const SIZE: usize = 3 * 32;

    #[inline]
    fn to_slice(&self, out: &mut [u8]) {
        self.a.to_slice(&mut out[..Hash::SIZE]);
        self.b.to_slice(&mut out[Hash::SIZE..Hash::SIZE * 2]);
        self.c.to_slice(&mut out[Hash::SIZE * 2..Hash::SIZE * 3]);
    }

    #[inline]
    fn from_slice(input: &[u8]) -> Result<Self> {
        Ok(Self {
            a: Hash::from_slice(&input[..Hash::SIZE])?,
            b: Hash::from_slice(&input[Hash::SIZE..Hash::SIZE * 2])?,
            c: Hash::from_slice(&input[Hash::SIZE * 2..Hash::SIZE * 3])?,
        })
    }
}

pub const FUSION_TOTAL_SIZE: usize = Input::SIZE + BlindedNote::SIZE + Output::SIZE;
pub const FUSION_STDIN_RANGE: Range<usize> = 0..Input::SIZE;
pub const FUSION_STDOUT_RANGE: Range<usize> = Input::SIZE..Input::SIZE + BlindedNote::SIZE;
pub const FUSION_JOURNAL_RANGE: Range<usize> = Input::SIZE + BlindedNote::SIZE..FUSION_TOTAL_SIZE;

#[inline]
pub fn fusion(hasher: &mut Sha256, memory: &mut [u8; FUSION_TOTAL_SIZE]) -> Result<()> {
    let input = Input::from_slice(&mut memory[FUSION_STDIN_RANGE])?;
    let [ia, ib] = input.inputs;
    let (a, b) = (ia.digest(hasher), ib.digest(hasher));

    assert_eq!(ia.asset_id, ib.asset_id);
    assert!(!ia.nullifier.is_empty());
    assert!(!ib.nullifier.is_empty());
    assert_ne!(ia.nullifier, ib.nullifier);

    let total = ia
        .amount
        .checked_add(ib.amount)
        .expect("overflow in total amount");

    let output = BlindedNote {
        asset_id: ia.asset_id,
        amount: total,
        secret: Hash::combine3(hasher, OUTPUT_SEP, a, b)?,
    };

    output.to_slice(&mut memory[FUSION_STDOUT_RANGE]);

    let fusion = Output {
        a,
        b,
        c: output.digest(hasher),
    };

    fusion.to_slice(&mut memory[FUSION_JOURNAL_RANGE]);

    Ok(())
}

#[cfg(all(feature = "std", test))]
mod tests {
    use super::*;

    use proptest::prelude::*;
    use sha2::Digest;
    use test_strategy::proptest;

    #[proptest]
    fn test_fusion_success(
        a: Note,
        #[strategy(any::<Note>().prop_map(move |mut b| { b.asset_id = #a.asset_id; b }))] b: Note,
    ) {
        prop_assume!(a.amount.checked_add(b.amount).is_some());

        let mut hasher = Sha256::new();
        let mut memory = [0u8; FUSION_TOTAL_SIZE];

        Input {
            inputs: [a.clone(), b.clone()],
        }
        .to_slice(&mut memory[FUSION_STDIN_RANGE]);

        fusion(&mut hasher, &mut memory)?;
        let result = BlindedNote::from_slice(&memory[FUSION_STDOUT_RANGE])?;

        assert_eq!(a.asset_id, result.asset_id);
        assert_eq!(a.amount + b.amount, result.amount);
    }

    #[proptest]
    #[should_panic]
    fn test_fusion_overflow(
        #[strategy((u64::MAX / 2) + 1..u64::MAX)] _amount_a: u64,
        #[strategy((u64::MAX / 2) + 1..u64::MAX)] _amount_b: u64,
        #[strategy(any::<Note>().prop_map(move |mut a| { a.amount = #_amount_a; a }))] a: Note,
        #[strategy(any::<Note>().prop_map(move |mut b| {
            b.asset_id = #a.asset_id;
            b.amount = #_amount_b;
            b
        }))]
        b: Note,
    ) {
        let mut hasher = Sha256::new();
        let mut memory = [0u8; FUSION_TOTAL_SIZE];

        Input {
            inputs: [a.clone(), b.clone()],
        }
        .to_slice(&mut memory[FUSION_STDIN_RANGE]);

        fusion(&mut hasher, &mut memory)?;
    }

    #[proptest]
    #[should_panic]
    fn test_fusion_asset_id_mismatch(a: Note, b: Note) {
        prop_assume!(a.asset_id != b.asset_id);

        let mut hasher = Sha256::new();
        let mut memory = [0u8; FUSION_TOTAL_SIZE];

        Input { inputs: [a, b] }.to_slice(&mut memory[FUSION_STDIN_RANGE]);

        fusion(&mut hasher, &mut memory)?;
    }
}
