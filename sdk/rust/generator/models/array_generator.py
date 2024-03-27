def generate_bytearray(ast_model):
    name = ast_model.name
    size = ast_model.size
    
    ret = f'''
        // generated from generate_bytearray()
        pub struct {name}(pub [u8; {size}]);
        impl {name} {{
            pub const SIZE: usize = {size};
            pub fn new({name.lower()}: [u8; {size}]) -> Self {{
                Self({name.lower()})
            }}
            pub fn default() -> Self {{
                Self([0; {size}])
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
                Ok((Self(bytes.try_into()?), rest))
            }}
            pub fn serialize(&self) -> Vec<u8> {{
                self.0.to_vec()
            }}
        }}
    '''
    
    return ret