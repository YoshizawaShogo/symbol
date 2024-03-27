
def generate_enum(astmodel):
    name = astmodel.name
    bits_size = astmodel.size * 8
    type = ('u' if astmodel.is_unsigned else 'i') + str(bits_size)
    values = astmodel.values
    
    # common (i.e. prepare)
    ret = '// generated from generate_enum()\n'
    ret += '#[derive(PartialOrd, Ord)]'
    ret += '#[derive(Copy)]'

    # structure
    ret += '#[allow(non_camel_case_types)]'
    ret += f'#[repr({type})]'
    if name == 'NetworkType':
        ret += '#[cfg(not(feature = "private_network"))]'
    ret += f'pub enum {name} {{'
    ret += ''.join(
        list(
            map(
                lambda e: f'{e.name} = {e.value},',
                values,
            )
        )
    )
    if astmodel.is_bitwise:
        ret += f'X({type}),'
    ret += '}'

    # implement
    if name == 'NetworkType':
        ret += '#[cfg(not(feature = "private_network"))]'
    ret += 'impl ' + name + ' {'
    
    ## SIZE
    ret += f'pub const SIZE: usize = {astmodel.size};'

    ## constructor
    ret += 'pub fn default() -> Self {'
    ret += f'Self::{values[0].name}'
    ret += '}'

    ## size
    ret += 'pub fn size(&self) -> usize {'
    ret += f'Self::SIZE'
    ret += '}'
    
    ## to_num
    if astmodel.is_bitwise:
        ret += f'pub fn to_num(&self) -> {type} {{'
        ret += 'match self {'
        ret += ''.join(
            list(
                map(
                    lambda e: f'Self::{e.name} => {e.value},',
                    values,
                )
            )
        )
        ret += 'Self::X(x) => *x'
        ret += '}'
        ret += '}'

    ## deserialize
    ret += f'pub fn deserialize(payload: &[u8]) -> Result<(Self, &[u8]), SymbolError> {{'
    ret += 'if payload.len() < Self::SIZE { return Err(SymbolError::SizeError{expect: vec![Self::SIZE], real: payload.len()}) }'
    ret += 'let (bytes, rest) = payload.split_at(Self::SIZE);'
    ret += f'match {type}::from_le_bytes(bytes.try_into()?) {{'
    ret += ''.join(
        list(
            map(
                lambda e: f'{e.value} => Ok(({name}::{e.name}, rest)),',
                values,
            )
        )
    )
    if astmodel.is_bitwise:
        ret += 'x if ('
        ret += ''.join(
            list(
                map(
                    lambda e: f'!{e.value} & ',
                    values,
                )
            )
        )
        ret += 'x) == 0 => Ok((Self::X(x), rest)),'
    ret += f'other => Err(SymbolError::MismatchError{{ pattern: vec![ other as u32], place: "{name}".to_string() }}),'
    ret += '}'
    ret += '}'


    ## serialize
    if astmodel.is_bitwise:
        ret += f'pub fn serialize(&self) -> Vec<u8> {{'
        ret += 'match self {'
        ret += ''.join(
            list(
                map(
                    lambda e: f'Self::{e.name} => {e.value}{type}.to_le_bytes().to_vec(),',
                    values,
                )
            )
        )
        ret += 'Self::X(x) => x.to_le_bytes().to_vec(),'
        ret += '}'
        ret += '}'
    else:
        ret += f'pub fn serialize(&self) -> Vec<u8> {{'
        ret += f'(self.clone() as {type}).to_le_bytes().to_vec()'
        ret += '}'

    # end
    ret += '}'
    
    # Bit OP
    if astmodel.is_bitwise:
        ret += f'''
            impl BitOr for {name} {{
                type Output = Self;
                fn bitor(self, rhs: Self) -> Self::Output {{
                    Self::X(self.to_num() | rhs.to_num())
                }}
            }}
            impl BitOrAssign for {name} {{
                fn bitor_assign(&mut self, rhs: Self) {{
                    *self = *self | rhs;
                }}
            }}
            impl BitAnd for {name} {{
                type Output = Self;
                fn bitand(self, rhs: Self) -> Self::Output {{
                    Self::X(self.to_num() & rhs.to_num())
                }}
            }}
            impl BitAndAssign for {name} {{
                fn bitand_assign(&mut self, rhs: Self) {{
                    *self = *self & rhs;
                }}
            }}
            impl BitXor for {name} {{
                type Output = Self;
                fn bitxor(self, rhs: Self) -> Self::Output {{
                    Self::X(self.to_num() ^ rhs.to_num())
                }}
            }}
            impl BitXorAssign for {name} {{
                fn bitxor_assign(&mut self, rhs: Self) {{
                    *self = *self ^ rhs;
                }}
            }}
            impl Not for {name} {{
                type Output = Self;
                fn not(self) -> Self::Output {{
                    Self::X(!self.to_num())
                }}
            }}
        '''

    return ret
