use crate::symbol::models::*;

pub trait ExtentionSignature
where
    Self: Sized,
{
    fn to_bytes(&self) -> [u8; Signature::SIZE];
    fn as_bytes(&self) -> &[u8; Signature::SIZE];
    fn from_bytes(bytes: &[u8; Signature::SIZE]) -> Self;
    fn from_str(str: &str) -> Result<Self, SymbolError>;
}

impl ExtentionSignature for Signature {
    #[inline]
    fn to_bytes(&self) -> [u8; Self::SIZE] {
        self.0
    }
    #[inline]
    fn as_bytes(&self) -> &[u8; Self::SIZE] {
        &self.0
    }
    fn from_bytes(bytes: &[u8; Signature::SIZE]) -> Self {
        Self(*bytes)
    }
    fn from_str(str: &str) -> Result<Self, SymbolError> {
        Ok(Self(hex::decode(str).unwrap().try_into().unwrap()))
    }
}
