def generate_integer(astmodel):
    name = astmodel.name
    bytes_size  = astmodel.size
    bits_size = bytes_size * 8
    type = ('u' if astmodel.is_unsigned else 'i') + str(bits_size)
    
    ret = f'''
        // generated from generate_integer()
        #[derive(PartialOrd, Ord, Copy)]
        pub struct {name}(pub {type});
        impl {name} {{
            pub const SIZE: usize = {bytes_size};
            pub fn new({name.lower()}: {type}) -> Self {{
                Self({name.lower()})
            }}
            pub fn default() -> Self {{
                Self(0)
            }}
            pub fn size(&self) -> usize {{
                Self::SIZE
            }}
            pub fn deserialize(payload: &[u8]) -> Result<(Self, &[u8]), SymbolError> {{
                if payload.len() < Self::SIZE {{
                    return Err(SymbolError::SizeError {{
                        expect: vec![Self::SIZE],
                        real: payload.len(),
                    }});
                }}
                let (bytes, rest) = payload.split_at(Self::SIZE);
                let value = {type}::from_le_bytes(bytes.try_into()?);
                Ok((Self(value), rest))
            }}
            pub fn serialize(&self) -> Vec<u8> {{
                self.0.to_le_bytes().to_vec()
            }}
        }}
    '''
    
    return ret
