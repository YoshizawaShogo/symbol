#!/usr/bin/python

from pathlib import Path
import json

import catparser
from catparser.DisplayType import DisplayType

from generator import util
from generator import constant

RUST_PRIMITIVE_INTEGER = ("uint8", "uint16", "uint32", "uint64", "int8", "int16", "int32", "int64")

class Generator:
    @staticmethod
    def generate(ast_models, output):
        print(f'python catbuffer generator called with output: {output}')
        generate_files(ast_models, Path(output))

def generate_files(ast_models, output_path: Path):
    # util.update_int_type_of_struct(ast_models)
    
    type_dict = get_type_dict(ast_models)
    # ast_model_dict = get_ast_model_dict(ast_models)
    
    output = '#[cfg(not(feature = "private_network"))] #[allow(unused)] mod symbol_models_test {use hex::decode; use std::str::FromStr; use symbol::symbol::prelude::*;'
    
    # transaction
    with open("../../tests/vectors/symbol/models/transactions.json") as f:
        json_data =  json.load(f)
        
    for test in json_data:
        output += parse_test(test, type_dict, ast_models)

    output += '}'
    with open(output_path, 'w', encoding='utf8', newline='') as output_file:
        output_file.write(output)
        
def get_publickey_member_name_and_type_list(ast_models):
    return [(f.name, f.field_type) for f in ast_models if "PublicKey" in str(f.field_type)]

def get_type_dict(ast_models):
    type_dict = {} # key: type_name, value: ast
    for ast_model in ast_models:
        name = ast_model.name
        type_dict[name] = ast_model
    
    return type_dict

def find_ast_model(ast_model_name, ast_models):
    index = [f.name for f in ast_models].index(ast_model_name)
    astmodel = ast_models[index]
    return astmodel

def generate_publickey(privatekey):
    from nacl.signing import SigningKey
    import nacl.utils
    private_key = SigningKey(bytes.fromhex(privatekey))
    return private_key.verify_key.encode(encoder=nacl.encoding.HexEncoder).decode('utf-8').upper()

payload = ''
def parse_test(json_data_of_test, type_dict, ast_models):
    # -> test一つ分丸ごと
    test = json_data_of_test
    global payload
    payload = json_data_of_test["payload"]
    
    # header
    ret = ''
    ret += constant.TEST_FUNC_HEADER
    ret += f'fn {test["test_name"]}() {{'
    
    # exe
    struct = find_ast_model(test["schema_name"], ast_models)
    ret += '\n\n// Serialize Test\n'
    ret += "let tx = "
    ret += parse_struct_rhs(struct, test["descriptor"], type_dict)
    ret += ";"
    ret += f'let payload = decode("{payload}").unwrap();'
    ret += "assert_eq!((payload.len(), payload.clone()), (tx.serialize().len(), tx.serialize()));"
    
    # footer
    ret += '\n\n// Deserialize Test\n'
    ret += f'let _ = {test["schema_name"]}::deserialize(&payload).unwrap().0;'
    ret += '}'
    return ret

def parse_struct_rhs(ast_model_of_struct, json_of_struct: list, type_dict):
    ret = '{'
    ret += f'let mut tmp_struct = {ast_model_of_struct.name}::default();'
    if 'network' in [f.name for f in ast_model_of_struct.fields]:
        ret += f'tmp_struct.network = NetworkType::TESTNET;'
    publickey_member_list = get_publickey_member_name_and_type_list(ast_model_of_struct.fields)
    for pubkey_name, pubkey_type in publickey_member_list:
        ret += f'tmp_struct.{pubkey_name} = {pubkey_type}::from_bytes(&[0; 32]).unwrap();'
    
    for variable, value in json_of_struct.items():
        if variable == 'type':
            continue
        
        field_ast_model = find_ast_model(variable, ast_model_of_struct.fields)
        field_type = field_ast_model.field_type
        # if field_type == "PublicKey" or field_type == "Signature":
        
        ret += f"tmp_struct.{variable} = "
        if str(field_type) in RUST_PRIMITIVE_INTEGER:
            ret += parse_primitive_integer_rhs(value)
        elif type(field_type) == catparser.ast.Array: # ast_modelが配列
            ret += parse_vec_rhs(field_type.element_type, value, type_dict)
        else:
            ast_model = type_dict[field_type]
            diplay_type = ast_model.display_type
            if diplay_type == DisplayType.STRUCT:
                ret += parse_struct_rhs(ast_model, value, type_dict)
            elif diplay_type == DisplayType.ENUM:
                ret += parse_enum_rhs(ast_model, value)
            elif diplay_type == DisplayType.BYTE_ARRAY:
                ret += parse_byte_array_rhs(ast_model, value)
            elif diplay_type == DisplayType.INTEGER:
                ret += parse_integer_rhs(ast_model, value)
            else:
                exit("unexpected")
        ret += ";"
    ret += 'tmp_struct }'
    return ret

def parse_primitive_integer_rhs(value):
    return f'{value}'
def parse_integer_rhs(ast_model_of_integer, value):
    return f'{ast_model_of_integer.name}({value})'
def parse_enum_rhs(ast_model_of_enum, value):
    enum_type = ast_model_of_enum.name
    value = value.upper()
    ret = ''
    if ' ' in value:
        values = value.split(" ")
        for value in values:
            ret += f'{enum_type}::{value} |'
        ret = ret[:-1]
    else:
        ret += f'{enum_type}::{value}'
    return ret
def parse_byte_array_rhs(ast_model_of_byte_array, value):
    if "PublicKey" in ast_model_of_byte_array.name:
        old_publickey = str(value)
        new_publickey = generate_publickey(old_publickey)
        global payload
        payload = payload.replace(old_publickey, new_publickey)
        value = new_publickey
    return f'{ast_model_of_byte_array.name}::from_str("{value}").unwrap()'
def parse_vec_rhs(element_type, value, type_dict):
    if str(element_type) in "uint8":
        return f'"{value}".as_bytes().to_vec()'

    ast_model = type_dict[element_type]
    diplay_type = ast_model.display_type
    ret = '{'
    ret += 'let mut tmp_vec = Vec::new();'
    values = value
    for value in values:
        ret += 'tmp_vec.push('
        if diplay_type == DisplayType.STRUCT:
            if element_type == "EmbeddedTransaction":
                embedded_element_type = "Embedded" + util.snake_to_camel(value["type"])
                embedded_ast_model = type_dict[embedded_element_type]
                ret += parse_struct_rhs(embedded_ast_model, value, type_dict)
                ret += ".into()"
            else:
                ret += parse_struct_rhs(ast_model, value, type_dict)
        elif diplay_type == DisplayType.ENUM:
            ret += parse_enum_rhs(ast_model, value)
        elif diplay_type == DisplayType.BYTE_ARRAY:
            ret += parse_byte_array_rhs(ast_model, value)
        elif diplay_type == DisplayType.INTEGER:
            ret += parse_integer_rhs(ast_model, value)
        else:
            exit("unexpected")
        ret += ');'
    if str(ast_model.name).endswith("Mosaic"):
        ret += f'tmp_vec.sort_unstable();'
    ret += 'tmp_vec}'
    return ret
    