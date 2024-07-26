#[cfg(test)]
use crate::testing::*;
#[cfg(test)]
use proptest::{collection::vec, prelude::*};
use serde::{Serialize, Serializer};
#[cfg(test)]
use test_strategy::Arbitrary;

use plonky2::{
    field::goldilocks_field::GoldilocksField,
    plonk::{
        circuit_data::VerifierCircuitData, config::PoseidonGoldilocksConfig,
        proof::ProofWithPublicInputs,
    },
};

pub mod delegate;

pub use curve25519_dalek::traits::*;

pub type Hash = [u8; 32];
pub type Point = curve25519_dalek::ristretto::RistrettoPoint;
pub type Scalar = curve25519_dalek::scalar::Scalar;
pub type PublicKey = Point;
pub type SecretKey = Scalar;
pub type CompressedPoint = curve25519_dalek::ristretto::CompressedRistretto;

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Signature {
    #[cfg_attr(test, strategy(point()))]
    pub r: Point,
    #[cfg_attr(test, strategy(scalar()))]
    pub s: Scalar,
}

#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Note {
    /// The ID for the asset in the Cardano blockchain
    pub asset_id: Hash,

    /// The amount included in this note
    pub amount: u64,

    /// Unblinded signature from the server from this note creation
    ///
    /// Equivalent to C in the protocol, returned by the server after minting or swapping
    /// assets.
    pub signature: Signature,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct UnblindedNote {
    pub asset_id: Hash,
    pub amount: u64,
    pub nonce: Hash,
}

#[derive(Debug, Clone)]
pub struct Proof {
    pub proof: ProofWithPublicInputs<GoldilocksField, PoseidonGoldilocksConfig, 2>,
    pub data: VerifierCircuitData<GoldilocksField, PoseidonGoldilocksConfig, 2>,
}

impl Serialize for Proof {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Compress the proof
        let common = self.data.common.clone();
        let digest = self.data.verifier_only.circuit_digest.clone();

        let compressed_proof = self.proof.clone().compress(&digest, &common).unwrap();

        // Serialize the compressed proof with public inputs
        let bytes = compressed_proof.to_bytes();
        serializer.serialize_str(&hex::encode(bytes))
    }
}

#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Proof {
    type Parameters = ();

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        todo!()
    }

    type Strategy = BoxedStrategy<Self>;
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Commit {}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct Swap {
    #[cfg_attr(test, strategy(vec(any::<Signature>(), 0..=16)))]
    pub inputs: Vec<Signature>,

    /// The blinded secrets to be signed by the delegate.
    ///
    /// Corresponds to B' in the protocol.
    #[cfg_attr(test, strategy(vec(point(), 0..=16)))]
    pub outputs: Vec<Point>,

    pub proof: Proof,
}
