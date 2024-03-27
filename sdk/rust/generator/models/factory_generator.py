import catparser
from generator import util

def generate_factory(factory, products):
    factory_name = factory.name
    ret = '// generated from generate_factory()\n'
    
    # define enum
    ret += f'pub enum {factory_name} {{'
    for p in products:
        ret += f'{p.name}({p.name}),'
    ret += '}'
    
    # implement
    ret += '#[allow(unreachable_patterns)]'
    ret += f'impl {factory_name} {{'
    
    ## size
    ret += 'pub fn size(&self) -> usize {'
    ret += 'match self {'
    for p in products:
        ret += f'Self::{p.name}(x) => x.size(),'
    ret += '}}'
    
    ## deserialize
    ret += 'pub fn deserialize(payload: &[u8]) -> Result<(Self, &[u8]), SymbolError> {'
    ret += 'let mut _tmp_payload = payload;'
    for f in factory.fields:
        if f.is_const:
            continue
        fn = f.name
        if util.constantized_by(fn, factory):
            if fn not in factory.discriminator:
                fn = '_' + fn
        
        ft = f.field_type
        fs = f.size
        if f.name == 'size':
            ret += f'if _tmp_payload.len() < {fs} {{ return Err(SymbolError::SizeError{{expect: vec![{fs}], real: _tmp_payload.len()}}) }}'
        if type(ft) == catparser.ast.FixedSizeInteger:
            ret += f'let _{fn} = {ft}::from_le_bytes(_tmp_payload[..{fs}].try_into()?);'
            ret += f'_tmp_payload = &_tmp_payload[{fs}..];'
        elif type(ft) == catparser.ast.Array:
            ret += f'let mut _{fn} = Vec::new();'
            ret += f'for _ in 0..{fs} {{'
            
            fte = ft.element_type
            if type(fte) == catparser.ast.FixedSizeInteger:
                ftes = fte.size
                # ften = fte.name
                ret += f'let mut bytes = [0u8; {ftes}];'
                ret += f'bytes.copy_from_slice(&_tmp_payload[..{ftes}]);'
                ret += f'let element = {fte}::from_le_bytes(bytes);'
                ret += f'_tmp_payload = &_tmp_payload[{ftes}..];'
                ret += f'_{fn}.push(element);'
            elif type(fte) == str:
                ret += f'let element;'
                ret += f'(element, _tmp_payload) = {fte}::deserialize(_tmp_payload)?;'
                ret += f'_{fn}.push(element);'
            else:
                raise "unexpected"
            ret += '}'
        else:
            ret += f'let _{fn};'
            ret += f'(_{fn}, _tmp_payload) = {ft}::deserialize(_tmp_payload)?;'
        
        if f.name == 'size':
            ret += f'if _size as usize > _tmp_payload.len() + {fs} {{ return Err(SymbolError::SizeError{{expect: vec![_size as usize], real: _tmp_payload.len() + {fs} }}); }}'
        if f.is_reserved:
            ret += f'if _{fn} != 0 {{ return Err(SymbolError::ReservedIsNotZeroError(_{fn} as u32)); }}'
        if util.constantized_by(f.name, factory):
            pass
        
    ret += 'match ('
    for d in factory.discriminator:
        ret += f'_{d}, '
    ret += ') {'
    for p in products:
        ret += '('
        for d in factory.discriminator:
            ret += f'{p.name}::{util.constantized_by(d, p)}, '
        ret += ') => {'
        
        ret += f'let (product, payload) = {p.name}::deserialize(payload)?;'
        ret += f'Ok((Self::{p.name}(product), payload))'
        
        ret += '},'
    ret += '('
    for d in factory.discriminator:
        ret += f'other_{d}, '
    ret += ')'
    ret += ' => Err(SymbolError::MismatchError{ pattern: vec!['
    for d in factory.discriminator:
        ret += f'other_{d} as u32, '
    ret += '], '
    ret += f'place: "{factory_name}".to_string()'
    ret += '}),'
    ret += '}}' 
    
    ## serialize
    ret += 'pub fn serialize(&self) -> Vec<u8> {'
    ret += 'match self {'
    for p in products:
        ret += f'Self::{p.name}(x) => x.serialize(),'
    ret += '}}'

    ret += '}'
    
    for p in products:
        ret += f'impl From<{p.name}> for {factory_name} {{ fn from(value: {p.name}) -> Self {{ Self::{p.name}(value) }} }}'
    
    return ret.replace('type', 'type_').replace('_type_', '_type')
    
    