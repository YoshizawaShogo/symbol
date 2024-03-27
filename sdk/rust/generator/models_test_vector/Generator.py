#!/usr/bin/python

from pathlib import Path
import json

import catparser
from catparser.DisplayType import DisplayType

from generator import util

RUST_PRIMITIVE_INTEGER = ("uint8", "uint16", "uint32", "uint64", "int8", "int16", "int32", "int64")

class Generator:
    @staticmethod
    def generate(astmodels, output):
        print(f'python catbuffer generator called with output: {output}')
        generate_files(astmodels, Path(output))

def generate_files(astmodels, output_path: Path):
    type_dict = get_type_dict(astmodels)
    
    output = 'mod symbol_models_test {use hex::decode;use symbol::symbol::prelude::*;'
    
    # transaction
    with open("../../tests/vectors/symbol/models/transactions.json") as f:
        json_data =  json.load(f)
        
    for test in json_data:
        output += parse_test(test, type_dict, astmodels)

    output += '}'
    with open(output_path, 'w', encoding='utf8', newline='') as output_file:
        output_file.write(output)

def parse_test(json_data_of_test, type_dict, astmodels):
    test = json_data_of_test
    struct = find_astmodel(test["schema_name"], astmodels)
    payload = json_data_of_test["payload"]
    payload = modify_public_key_of_sturct(payload, struct, test["descriptor"], type_dict)
    
    ret = f'''
        #[test]
        #[allow(non_snake_case)]
        fn {test["test_name"]}() {{
            let input_payload = decode("{payload}").unwrap();
            let tx = {struct.name}::deserialize(&input_payload).unwrap().0;
            let output_payload = tx.serialize();
            assert_eq!(input_payload, output_payload);
        }}
    '''
    return ret

def get_type_dict(astmodels):
    type_dict = {} # key: type_name, value: ast
    for astmodel in astmodels:
        name = astmodel.name
        type_dict[name] = astmodel
    
    return type_dict

def find_astmodel(astmodel_name, astmodels):
    index = [f.name for f in astmodels].index(astmodel_name)
    astmodel = astmodels[index]
    return astmodel
    
def modify_public_key_of_sturct(payload, astmodel_of_struct, json_of_struct: list, type_dict):
    def generate_publickey(privatekey):
        from nacl.signing import SigningKey
        import nacl.utils
        private_key = SigningKey(bytes.fromhex(privatekey))
        return private_key.verify_key.encode(encoder=nacl.encoding.HexEncoder).decode('utf-8').upper()

    def modify_public_key_of_byte_array(payload, astmodel_of_byte_array, value):
        if astmodel_of_byte_array.name.endswith("PublicKey"):
            old_publickey = str(value).upper()
            new_publickey = generate_publickey(old_publickey)
            payload = payload.replace(old_publickey, new_publickey)
        return payload

    def modify_public_key_of_vec(payload, element_type, value, type_dict):
        if str(element_type) in "uint8":
            return payload

        astmodel = type_dict[element_type]
        diplay_type = astmodel.display_type
        values = value
        for value in values:
            if diplay_type == DisplayType.STRUCT:
                if element_type == "EmbeddedTransaction":
                    embedded_element_type = "Embedded" + util.snake_to_camel(value["type"])
                    embedded_astmodel = type_dict[embedded_element_type]
                    payload = modify_public_key_of_sturct(payload, embedded_astmodel, value, type_dict)
                else:
                    payload = modify_public_key_of_sturct(payload, astmodel, value, type_dict)
            elif diplay_type == DisplayType.ENUM:
                continue
            elif diplay_type == DisplayType.BYTE_ARRAY:
                payload = modify_public_key_of_byte_array(payload, astmodel, value)
            elif diplay_type == DisplayType.INTEGER:
                continue
            else:
                exit("unexpected")
        return payload
    for variable, value in json_of_struct.items():
        if variable == 'type':
            continue
        
        field_astmodel = find_astmodel(variable, astmodel_of_struct.fields)
        field_type = field_astmodel.field_type
        
        if str(field_type) in RUST_PRIMITIVE_INTEGER:
            continue
        elif type(field_type) == catparser.ast.Array:
            payload = modify_public_key_of_vec(payload, field_type.element_type, value, type_dict)
        else:
            astmodel = type_dict[field_type]
            diplay_type = astmodel.display_type
            if diplay_type == DisplayType.STRUCT:
                payload = modify_public_key_of_sturct(payload, astmodel, value, type_dict)
            elif diplay_type == DisplayType.ENUM:
                continue
            elif diplay_type == DisplayType.BYTE_ARRAY:
                payload = modify_public_key_of_byte_array(payload, astmodel, value)
            elif diplay_type == DisplayType.INTEGER:
                continue
            else:
                exit("unexpected")
    return payload

