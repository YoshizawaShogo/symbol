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
    type_dict = get_type_dict(ast_models)
    
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

def parse_test(json_data_of_test, type_dict, ast_models):
    # -> test一つ分丸ごと
    test = json_data_of_test
    struct = find_ast_model(test["schema_name"], ast_models)
    payload = json_data_of_test["payload"]
    payload = parse_struct_rhs(payload, struct, test["descriptor"], type_dict)
    
    # header
    ret = ''
    ret += constant.TEST_FUNC_HEADER
    ret += f'fn {test["test_name"]}() {{'
    
    # exe
    ret += f'let input_payload = decode("{payload}").unwrap();'
    ret += f'let tx = {struct.name}::deserialize(&input_payload).unwrap().0;'
    ret += f'let output_payload = tx.serialize();'
    ret += "assert_eq!(input_payload, output_payload);"
    
    # footer
    ret += '}'
    return ret

def parse_struct_rhs(payload, ast_model_of_struct, json_of_struct: list, type_dict):
    for variable, value in json_of_struct.items():
        if variable == 'type':
            continue
        
        field_ast_model = find_ast_model(variable, ast_model_of_struct.fields)
        field_type = field_ast_model.field_type
        
        if str(field_type) in RUST_PRIMITIVE_INTEGER:
            continue
        elif type(field_type) == catparser.ast.Array: # ast_modelが配列
            payload = parse_vec_rhs(payload, field_type.element_type, value, type_dict)
        else:
            ast_model = type_dict[field_type]
            diplay_type = ast_model.display_type
            if diplay_type == DisplayType.STRUCT:
                payload = parse_struct_rhs(payload, ast_model, value, type_dict)
            elif diplay_type == DisplayType.ENUM:
                continue
            elif diplay_type == DisplayType.BYTE_ARRAY:
                payload = parse_byte_array_rhs(payload, ast_model, value)
            elif diplay_type == DisplayType.INTEGER:
                continue
            else:
                exit("unexpected")
    return payload

def parse_byte_array_rhs(payload, ast_model_of_byte_array, value):
    if "PublicKey" in ast_model_of_byte_array.name:
        old_publickey = str(value)
        new_publickey = generate_publickey(old_publickey)
        payload = payload.replace(old_publickey, new_publickey)
    return payload

def parse_vec_rhs(payload, element_type, value, type_dict):
    if str(element_type) in "uint8":
        return payload

    ast_model = type_dict[element_type]
    diplay_type = ast_model.display_type
    values = value
    for value in values:
        if diplay_type == DisplayType.STRUCT:
            if element_type == "EmbeddedTransaction":
                embedded_element_type = "Embedded" + util.snake_to_camel(value["type"])
                embedded_ast_model = type_dict[embedded_element_type]
                payload = parse_struct_rhs(payload, embedded_ast_model, value, type_dict)
            else:
                payload = parse_struct_rhs(payload, ast_model, value, type_dict)
        elif diplay_type == DisplayType.ENUM:
            continue
        elif diplay_type == DisplayType.BYTE_ARRAY:
            payload = parse_byte_array_rhs(payload, ast_model, value)
        elif diplay_type == DisplayType.INTEGER:
            continue
        else:
            exit("unexpected")
    return payload
    