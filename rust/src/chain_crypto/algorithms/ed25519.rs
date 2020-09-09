use cryptoxide::ed25519;
use ed25519_bip32::XPub;
use rand_core::{RngCore, CryptoRng};
use crate::chain_crypto::key::{
  AsymmetricKey, AsymmetricPublicKey, PublicKeyError, SecretKeyError
};

/// ED25519 Signing Algorithm
pub struct Ed25519;

#[derive(Clone)]
pub struct Priv([u8; ed25519::SEED_LENGTH]);

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Pub(pub(crate) [u8; ed25519::PUBLIC_KEY_LENGTH]);

#[derive(Clone)]
pub struct Sig(pub(crate) [u8; ed25519::SIGNATURE_LENGTH]);

impl Pub {
    pub fn from_xpub(xpub: &XPub) -> Self {
        let mut buf = [0; ed25519::PUBLIC_KEY_LENGTH];
        xpub.get_without_chaincode(&mut buf);
        Pub(buf)
    }
}

impl AsRef<[u8]> for Priv {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl AsRef<[u8]> for Pub {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8]> for Sig {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsymmetricPublicKey for Ed25519 {
    type Public = Pub;
    /// bech32 human readable part
    const PUBLIC_BECH32_HRP: &'static str = "ed25519_pk";
    const PUBLIC_KEY_SIZE: usize = ed25519::PUBLIC_KEY_LENGTH;

    fn public_from_binary(data: &[u8]) -> Result<Self::Public, PublicKeyError> {
        if data.len() != ed25519::PUBLIC_KEY_LENGTH {
            return Err(PublicKeyError::SizeInvalid);
        }
        let mut buf = [0; ed25519::PUBLIC_KEY_LENGTH];
        buf[0..ed25519::PUBLIC_KEY_LENGTH].clone_from_slice(data);
        Ok(Pub(buf))
    }
}

impl AsymmetricKey for Ed25519 {
    type PubAlg = Ed25519;
    type Secret = Priv;
    /// bech32 human readable part
    const SECRET_BECH32_HRP: &'static str = "ed25519_sk";

    fn generate<T: RngCore + CryptoRng>(mut rng: T) -> Self::Secret {
        let mut priv_bytes = [0u8; ed25519::SEED_LENGTH];
        rng.fill_bytes(&mut priv_bytes);
        Priv(priv_bytes)
    }

    fn compute_public(secret: &Self::Secret) -> <Self::PubAlg as AsymmetricPublicKey>::Public {
        let (_, pk) = ed25519::keypair(&secret.0);
        Pub(pk)
    }

    fn secret_from_binary(data: &[u8]) -> Result<Self::Secret, SecretKeyError> {
        if data.len() != ed25519::SEED_LENGTH {
            return Err(SecretKeyError::SizeInvalid);
        }
        let mut buf = [0; ed25519::SEED_LENGTH];
        buf[0..ed25519::SEED_LENGTH].clone_from_slice(data);
        Ok(Priv(buf))
    }
}
