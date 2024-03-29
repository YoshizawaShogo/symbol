use crate::symbol::models::*;

pub trait ExtentionPublicKey
where
    Self: Sized,
{
    fn from_str(str: &str) -> Result<Self, SymbolError>;
    fn address(&self, network: NetworkType) -> UnresolvedAddress;
    fn verify(&self, message: &[u8], signature: &Signature) -> Result<(), SymbolError>;
    fn verify_transaction<T: TraitMessage + TraitSignature>(
        &self,
        transaction: &T,
    ) -> Result<(), SymbolError>;
    fn to_bytes(&self) -> [u8; PublicKey::SIZE];
    fn as_bytes(&self) -> &[u8; PublicKey::SIZE];
}

impl ExtentionPublicKey for PublicKey {
    fn from_str(str: &str) -> Result<Self, SymbolError> {
        Ok(Self(hex::decode(str).unwrap().try_into().unwrap()))
    }
    fn address(&self, network_type: NetworkType) -> UnresolvedAddress {
        use ripemd::Ripemd160;
        use sha3::Sha3_256;

        let part_one_hash = get_hash::<Sha3_256>(&self.0);
        let part_two_hash = get_hash::<Ripemd160>(part_one_hash);

        let mut version = network_type.serialize();
        version.extend_from_slice(&part_two_hash);

        let mut checksum = get_hash::<Sha3_256>(&version)[0..3].to_vec();

        let mut address = version;
        address.append(&mut checksum);

        UnresolvedAddress::new(address.try_into().unwrap())
    }
    fn verify(&self, msg: &[u8], signature: &Signature) -> Result<(), SymbolError> {
        use ed25519_dalek::{Verifier, VerifyingKey};
        let verifying_key = VerifyingKey::from_bytes(&self.0)?;
        let signature = ed25519_dalek::Signature::from_bytes(&signature.0);
        Ok(verifying_key.verify(msg, &signature)?)
    }
    fn verify_transaction<T: TraitMessage + TraitSignature>(
        &self,
        transaction: &T,
    ) -> Result<(), SymbolError> {
        use ed25519_dalek::{Verifier, VerifyingKey};
        let verifying_key = VerifyingKey::from_bytes(&self.0)?;
        let message = transaction.get_message();
        let signature = transaction.get_signature().0;
        let signature = ed25519_dalek::Signature::from_bytes(&signature);
        Ok(verifying_key.verify(&message, &signature)?)
    }
    #[inline]
    fn to_bytes(&self) -> [u8; Self::SIZE] {
        self.0
    }
    #[inline]
    fn as_bytes(&self) -> &[u8; Self::SIZE] {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PrivateKey([u8; Self::SIZE]);

impl PrivateKey {
    pub const SIZE: usize = 32;
    pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> Self {
        Self(*bytes)
    }
    #[inline]
    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        self.0
    }
    #[inline]
    pub fn as_bytes(&self) -> &[u8; Self::SIZE] {
        &self.0
    }
    pub fn from_str(str: &str) -> Result<Self, SymbolError> {
        Ok(Self::from_bytes(hex::decode(str)?.as_slice().try_into()?))
    }
    pub fn sign(&self, msg: &[u8]) -> Signature {
        use ed25519_dalek::{Signer, SigningKey};
        let signing_key = SigningKey::from_bytes(&self.to_bytes());
        Signature(signing_key.sign(msg).to_bytes())
    }
    pub fn sign_transaction<T: TraitMessage + TraitSignature>(&self, mut transaction: T) -> T {
        transaction.set_signature(self.sign(transaction.get_message()));
        transaction
    }
    pub fn verify_transaction<T: TraitMessage + TraitSignature>(
        &self,
        transaction: &T,
    ) -> Result<(), SymbolError> {
        self.pubilc_key().verify_transaction(transaction)
    }
    pub fn pubilc_key(&self) -> PublicKey {
        PublicKey(
            ed25519_dalek::SigningKey::from_bytes(&self.0)
                .verifying_key()
                .to_bytes(),
        )
    }
    pub fn shared_key(&self, other_public_key: PublicKey) -> SharedKey {
        use curve25519_dalek::{edwards::CompressedEdwardsY, scalar::Scalar};
        use hkdf::Hkdf;
        use sha2::{Sha256, Sha512};

        let private_key = self;

        let unpacked_public_key = CompressedEdwardsY(other_public_key.to_bytes())
            .decompress()
            .unwrap();

        let mut scalar = get_hash::<Sha512>(private_key.as_bytes())[..Self::SIZE].to_vec();
        scalar[0] &= 248;
        scalar[31] &= 127;
        scalar[31] |= 64;

        #[allow(deprecated)]
        // This scalar is used only for multiplication with EdwardsPoint.
        let scalar = Scalar::from_bits(scalar.as_slice().try_into().unwrap());

        let shared_secret = (scalar * unpacked_public_key).compress();

        let hkdf = Hkdf::<Sha256>::new(None, &shared_secret.to_bytes());
        let mut shared_key = [0u8; Self::SIZE];
        hkdf.expand(b"catapult", &mut shared_key).unwrap();

        SharedKey(shared_key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SharedKey(pub [u8; Self::SIZE]);

impl SharedKey {
    pub const SIZE: usize = PublicKey::SIZE;
    #[inline]
    pub fn to_bytes(&self) -> [u8; Self::SIZE] {
        self.0
    }
    #[inline]
    pub fn as_bytes(&self) -> &[u8; Self::SIZE] {
        &self.0
    }
}
