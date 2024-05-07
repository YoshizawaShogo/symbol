// use data_encoding::DecodeError;
use ed25519_dalek::ed25519;
use hex::FromHexError;
use std::array::TryFromSliceError;

#[derive(Debug)]
pub enum SymbolError {
    FromHexError(FromHexError),
    Base32DecodeError(String),
    TryFromSliceError(TryFromSliceError),
    SizeError { expect: Vec<usize>, real: usize },
    InvalidLengthError(crypto_common::InvalidLength),
    ReservedIsNotZeroError(u32),
    MismatchError { pattern: Vec<u32>, place: String },
    Ed25519Error(ed25519::Error),
    AesGcmError(aes_gcm::Error),
    OverflowError(String),
    IoError(std::io::Error),
}

impl From<FromHexError> for SymbolError {
    fn from(err: FromHexError) -> Self {
        SymbolError::FromHexError(err)
    }
}
impl From<TryFromSliceError> for SymbolError {
    fn from(err: TryFromSliceError) -> SymbolError {
        SymbolError::TryFromSliceError(err)
    }
}
impl From<ed25519::Error> for SymbolError {
    fn from(err: ed25519::Error) -> SymbolError {
        SymbolError::Ed25519Error(err)
    }
}
impl From<aes_gcm::Error> for SymbolError {
    fn from(err: aes_gcm::Error) -> SymbolError {
        SymbolError::AesGcmError(err)
    }
}
impl From<std::io::Error> for SymbolError {
    fn from(err: std::io::Error) -> SymbolError {
        SymbolError::IoError(err)
    }
}
impl From<crypto_common::InvalidLength> for SymbolError {
    fn from(err: crypto_common::InvalidLength) -> SymbolError {
        SymbolError::InvalidLengthError(err)
    }
}

#[cfg(feature = "private_network")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkType(pub u8);

#[cfg(feature = "private_network")]
impl NetworkType {
    pub const SIZE: usize = 1;
    pub fn default() -> Self {
        Self(u8::default())
    }
    pub fn size(&self) -> usize {
        Self::SIZE
    }
    pub fn deserialize(payload: &[u8]) -> Result<(Self, &[u8]), SymbolError> {
        if payload.len() < Self::SIZE {
            return Err(SymbolError::SizeError {
                expect: vec![Self::SIZE],
                real: payload.len(),
            });
        }
        let (bytes, rest) = payload.split_at(Self::SIZE);
        Ok((NetworkType(u8::from_le_bytes(bytes.try_into()?)), rest))
    }
    pub fn serialize(&self) -> Vec<u8> {
        (self.0 as u8).to_le_bytes().to_vec()
    }
}

pub(crate) fn get_hash<Hasher: digest::Digest>(data: impl AsRef<[u8]>) -> Vec<u8> {
    Hasher::new().chain_update(data).finalize().to_vec()
}