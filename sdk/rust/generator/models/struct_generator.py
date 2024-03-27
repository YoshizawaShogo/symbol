import catparser
from catparser.DisplayType import DisplayType
from generator import constant
from generator import util

def generate_struct(astmodel):
    struct_name = astmodel.name
    ret = '// generated from generate_struct()\n'
    if struct_name.endswith('Mosaic'):
        ret += '#[derive(PartialOrd, Ord)]'
    
    # struct
    ret += f'pub struct {astmodel.name} {{'
    for f in astmodel.fields:
        if not util.field_is_member_of_astmodel(f, astmodel):
            continue
        if type(f.field_type) == catparser.ast.Array:
            ret += f'pub {f.name}: Vec<{f.field_type.element_type}>,'
        else:
            ret += f'pub {f.name}: {f.field_type},'
    ret += '}'
    
    # implement
    ## const
    ret += 'impl ' + struct_name + ' {'
    for f in astmodel.fields:
        if not f.is_const:
            continue
        
        if type(f.value) == str:
            ret += f'pub const {f.name}: {f.field_type} = {f.field_type}::{f.value};'
        elif type(f.value) == int:
            ret += f'pub const {f.name}: {f.field_type} = {f.value};'
        else:
            raise "unexpected"
        
    for f in astmodel.fields:
        const = util.constantized_by(f.name, astmodel)
        if not const:
            continue
        
        if const in [f.name for f in astmodel.fields]:
            ret += f'pub fn {f.name}(&self) -> {f.field_type} {{ Self::{const} }}'
        else:
            ret += f'pub fn {f.name}(&self) -> {f.field_type} {{ {f.field_type}::default() }}'
        
    ## constructor
    ret += 'pub fn new('
    for f in astmodel.fields:
        if not util.field_is_member_of_astmodel(f, astmodel):
            continue
        if util.skip_in_constructor(f):
            continue
        
        if type(f.field_type) == catparser.ast.Array:
            ret += f'{f.name}: Vec<{f.field_type.element_type}>,'
        else:
            ret += f'{f.name}: {f.field_type},'
    ret += ') -> Self {'
    ret += 'Self {'
    for f in astmodel.fields:
        if not util.field_is_member_of_astmodel(f, astmodel):
            continue
        if util.skip_in_constructor(f):
            ret += f'{f.name}: {f.field_type}::default(),'
            continue
        
        ret += f'{f.name},'
    ret += '}'
    ret += '}'

    ret += 'pub fn default() -> Self {'
    ret += 'Self {'
    for f in astmodel.fields:
        if not util.field_is_member_of_astmodel(f, astmodel):
            continue
        
        if type(f.field_type) == catparser.ast.Array:
            ret += f'{f.name}: Vec::new(),'
        elif type(f.value) == catparser.ast.Conditional or f.value is None:
            ret += f'{f.name}: {f.field_type}::default(),'
        else:
            ret += f'{f.name}: {f.value},'
    ret += '}'
    ret += '}'
    
    ## size
    ret += 'pub fn size(&self) -> usize {'
    ret += 'let mut size = 0;'
    for f in astmodel.fields:
        if f.is_const:
            continue
        if f.is_conditional:
            linked_field_name = f.value.linked_field_name
            linked_field_astmodel = [f for f in astmodel.fields if f.name == linked_field_name][0]
            linked_field_type = linked_field_astmodel.field_type
            linked_field_value = f.value.value
            ret += f'if self.{linked_field_name} == {linked_field_type}::{linked_field_value} {{'
    
        if f.size is None:
            if util.is_method(f, astmodel):
                ret += f'size += self.{f.name}().size();'
            else:
                ret += f'size += self.{f.name}.size();'
        elif type(f.field_type) == catparser.ast.Array:
            ft = f.field_type
            fte = ft.element_type
            alignment = ft.alignment
            if alignment is None:
                if ft.is_expandable or type(fte) == str:
                    ret += f'size += self.{f.name}.iter().map(|x| x.size()).sum::<usize>();'
                elif type(fte) == catparser.ast.FixedSizeInteger:
                    ret += f'size += {fte.size} * self.{f.name}.len();'
                else:
                    raise "unexpected"
            else:
                if ft.is_expandable or type(fte) == str:
                    ret += f'size += (self.{f.name}.iter().map(|x| x.size()).sum::<usize>() + {alignment-1}) & !{alignment-1};'
                else:
                    raise "unexpected"
        elif type(f.size) == int:
            ret += f'size += {f.size};'
        else:
            raise "unexpected"
        
        if f.is_conditional:
            ret += '}'
    ret += 'size'
    ret += '}'

    ## deserialize
    was_conditional_list = []
    ret += 'pub fn deserialize(mut payload: &[u8]) -> Result<(Self, &[u8]), SymbolError> {'
    ret += '#[allow(unused)] let initial_payload_len = payload.len();'
    for f in astmodel.fields:
        if f.is_const:
            continue
        fn = f.name
        if util.constantized_by(f.name, astmodel):
            fn = '_' + fn
        
        ft = f.field_type
        fs = f.size
        if f.name == 'size':
            ret += f'if payload.len() < {fs} {{ return Err(SymbolError::SizeError{{expect: vec![{fs}], real: payload.len()}}) }}'
        
        if type(ft) == catparser.ast.FixedSizeInteger:
            ret += f'let {fn} = {ft}::from_le_bytes(payload[..{fs}].try_into()?);'
            ret += f'payload = &payload[{fs}..];'
        elif type(ft) == catparser.ast.Array:
            ret += f'let mut {fn} = Vec::new();'
            alignment = ft.alignment
            ret += f'#[allow(unused)] let tmp_payload_len = payload.len();'
            fte = ft.element_type
            
            if alignment is not None:
                if fs == 0:
                    if ft.is_expandable:
                        ret += f'while initial_payload_len - payload.len() < size as usize {{'
                    else:
                        ret += f'while tmp_payload_len - payload.len() < size as usize {{'
                else:
                    ret += f'while tmp_payload_len - payload.len() < {fs} as usize {{'
            elif ft.is_expandable:
                ret += f'while initial_payload_len - payload.len() < size as usize {{'
            else:
                ret += f'for _ in 0..{fs} {{'
            
            if type(fte) == catparser.ast.FixedSizeInteger:
                ftes = fte.size
                ret += f'let mut bytes = [0u8; {ftes}];'
                ret += f'bytes.copy_from_slice(&payload[..{ftes}]);'
                ret += f'let element = {fte}::from_le_bytes(bytes);'
                ret += f'payload = &payload[{ftes}..];'
                ret += f'{fn}.push(element);'
            elif type(fte) == str:
                ret += f'let element;'
                ret += f'(element, payload) = {fte}::deserialize(payload)?;'
                ret += f'{fn}.push(element);'
                if alignment is not None:
                    ret += f'payload = &payload[(({alignment} - (tmp_payload_len - payload.len()) % {alignment}) % {alignment})..];'
            else:
                raise "unexpected"
            ret += '}'
            
        else:
            ret += f'let {fn};'
            if f.is_conditional:
                linked_field_name = f.value.linked_field_name
                if linked_field_name not in was_conditional_list:
                    ret += f'let payload_for_conditional = &payload[..{ft}::SIZE];'
                    ret += f'payload = &payload[{ft}::SIZE..];'
                    was_conditional_list.append(linked_field_name)
                ret += f'({fn}, _) = {ft}::deserialize(payload_for_conditional)?;'
            else:
                ret += f'({fn}, payload) = {ft}::deserialize(payload)?;'
        
        if f.name == 'size':
            ret += f'if size as usize > payload.len() + {fs} {{ return Err(SymbolError::SizeError{{expect: vec![size as usize], real: payload.len() + {fs} }}); }}'
        if f.is_reserved:
            ret += f'if {f.name} != 0 {{ return Err(SymbolError::ReservedIsNotZeroError({f.name} as u32)); }}'
        if util.constantized_by(f.name, astmodel):
            pass
    
    ret += f'let self_ = Self {{'
    for f in astmodel.fields:
        if not util.field_is_member_of_astmodel(f, astmodel):
            continue
        ret += f'{f.name},'
    ret += '};'
        
    ret += 'Ok((self_, payload))'
        
    ret += '}'
    
    ## serialize
    ret += 'pub fn serialize(&self) -> Vec<u8> {'
    for f in astmodel.fields:
        if f.is_const:
            continue
        fn = f.name
        ft = f.field_type
        fs = f.size
        
        if fn == 'size' or util.constantized_by(f.name, astmodel):
            if type(ft) == catparser.ast.FixedSizeInteger:
                ret += f"let {fn} = (self.{fn}() as {ft}).to_le_bytes();"
            else:
                ret += f"let {fn} = self.{fn}().serialize();"
        elif f.is_reserved:
            ret += f'let {fn} = 0{ft.short_name}.to_le_bytes();'
        elif util.is_size_of_other(f, astmodel):
            other_field = util.is_size_of_other(f, astmodel)
            ofn = other_field.name
            alignment = other_field.field_type.alignment
            if alignment is None:
                ret += f'let {fn} = (self.{ofn}.len() as {ft}).to_le_bytes();'
            else:
                ret += f'let {fn} = (((self.{ofn}.iter().map(|x| x.size()).sum::<usize>() + {alignment-1}) & !{alignment-1}) as {ft}).to_le_bytes();'
        elif type(ft) == catparser.ast.Array:
            fte = ft.element_type
            alignment = ft.alignment
            if alignment is None:
                if type(fte) == catparser.ast.FixedSizeInteger:
                    ret += f'let {fn}: Vec<u8> = self.{fn}.iter().flat_map(|x| x.to_le_bytes()).collect();'
                else:
                    ret += f'let {fn}: Vec<u8> = self.{fn}.iter().flat_map(|x| x.serialize()).collect();'
            else:
                if ft.is_expandable or type(fte) == str:
                    ret += f'''
                        let mut {fn} = Vec::new();
                        for x in &self.{fn} {{
                            {fn}.extend_from_slice(&x.serialize());
                            let len = {fn}.len();
                            {fn}.resize((len + {alignment-1}) & !{alignment-1}, 0);
                        }}
                    '''
                else:
                    raise "unexpected"
        elif type(ft) == catparser.ast.FixedSizeInteger:
            ret += f'let {fn} = self.{fn}.to_le_bytes();'
        else:
            if f.is_conditional:
                linked_field_name = f.value.linked_field_name
                linked_field_astmodel = [f for f in astmodel.fields if f.name == linked_field_name][0]
                linked_field_type = linked_field_astmodel.field_type
                linked_field_value = f.value.value
                ret += f'''
                    let {fn} = if self.{linked_field_name} == {linked_field_type}::{linked_field_value} {{
                        self.{fn}.serialize()
                    }} else {{
                        Vec::new()
                    }};
                '''
            else:
                ret += f'let {fn} = self.{fn}.serialize();'
            
    ret += '['
    for f in astmodel.fields:
        if f.is_const:
            continue
        fn = f.name
        ret += f'{fn}.iter(),'
    ret += '].into_iter().flat_map(|a| a).map(|x| *x).collect()'
        
    ret += '}'
    ret += '}'
    
    ## trait for SIGN
    for f in astmodel.fields:
        fn = f.name
        if fn not in constant.TRAITS_FOR_SIGN:
            continue
        if type(f.field_type) == catparser.ast.Array:
            ft = f'Vec<{f.field_type.element_type}>'
        else:
            ft = f.field_type
        
        ret += f'''
            impl Trait{util.snake_to_camel(fn)} for {struct_name} {{
            fn get_{fn}(&self) -> &{ft} {{ &self.{fn} }}
            fn set_{fn}(&mut self, {fn}: {ft}) {{ self.{fn} = {fn}; }}
            }}
        '''
        
    return ret.replace('type', 'type_').replace('_type_', '_type')