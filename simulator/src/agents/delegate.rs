use color_eyre::eyre::Result;
use mugraph_core::{crypto, error::Error, types::*};
use mugraph_node::{context::Context, v0::transaction_v0};
use rand::prelude::*;
use tracing::info;

#[derive(Debug, Clone)]
pub struct Delegate {
    pub context: Context,
}

impl Delegate {
    pub fn new<R: Rng + CryptoRng>(rng: &mut R, context: Context) -> Result<Self, Error> {
        let failure_rate = rng.gen_range(0.01f64..0.8f64);

        info!(
            "Starting delegate with failure rate {:.2}%",
            failure_rate * 100.0
        );

        Ok(Self { context })
    }

    pub fn public_key(&self) -> PublicKey {
        self.context.keypair.public_key
    }

    pub fn secret_key(&self) -> SecretKey {
        self.context.keypair.secret_key
    }

    pub fn emit(&mut self, asset_id: Hash, amount: u64) -> Result<Note> {
        let mut note = Note {
            delegate: self.public_key(),
            asset_id,
            nonce: Hash::random(&mut self.context.rng),
            amount,
            signature: Signature::default(),
        };

        let blind = crypto::blind_note(&mut self.context.rng, &note);
        let signed = crypto::sign_blinded(&self.secret_key(), &blind.point);
        note.signature = crypto::unblind_signature(&signed, &blind.factor, &self.public_key())?;

        Ok(note)
    }

    #[inline(always)]
    pub fn recv_transaction_v0(&mut self, tx: Transaction) -> Result<V0Response, Error> {
        transaction_v0(tx, &mut self.context)
    }
}
