use hex::decode;

use crate::symbol::models::*;

pub trait ExtentionHash256 {
    fn from_str(str: &str) -> Result<Self, SymbolError>
    where
        Self: Sized;
}

impl ExtentionHash256 for Hash256 {
    fn from_str(str: &str) -> Result<Self, SymbolError> {
        Ok(Self::new(decode(str).unwrap().try_into().unwrap()))
    }
}
