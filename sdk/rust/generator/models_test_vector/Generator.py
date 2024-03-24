#!/usr/bin/python

from pathlib import Path
import json
import copy

import catparser
from catparser.DisplayType import DisplayType

from generator import util
from generator import constant

class Generator:
    @staticmethod
    def generate(ast_models, output):
        print(f'python catbuffer generator called with output: {output}')
        generate_files(ast_models, Path(output))

def generate_files(ast_models, output_path: Path):
    util.update_int_type_of_struct(ast_models)
    
    type_dict = {} # key: type_name, value: DisplayType of type_name
    ast_model_dict = {} # key: ast_model_name, value: ast_model

    for ast_model in ast_models:
        ast_model_dict[ast_model.name] = ast_model
        name = ast_model.name
        display_type = ast_model.display_type
        # if display_type == DisplayType.STRUCT:
        #     continue
        type_dict[name] = display_type

    PRIMITIVE_INTEGER = "Primitive Integer"
    type_dict["u8"] = PRIMITIVE_INTEGER
    type_dict["u16"] = PRIMITIVE_INTEGER
    type_dict["u32"] = PRIMITIVE_INTEGER
    type_dict["u64"] = PRIMITIVE_INTEGER
    type_dict["i8"] = PRIMITIVE_INTEGER
    type_dict["i16"] = PRIMITIVE_INTEGER
    type_dict["i32"] = PRIMITIVE_INTEGER
    type_dict["i64"] = PRIMITIVE_INTEGER
    
    output = '#[cfg(not(feature = "private_network"))] mod symbol_models_test {use hex::decode; use symbol::symbol::prelude::*;'
    
    # transaction
    with open("../../tests/vectors/symbol/models/transactions.json") as f:
        json_data =  json.load(f)
        
    for test in json_data:
        output += constant.TEST_FUNC_HEADER
        output += f'fn {test["test_name"]}() {{'
        output += f'let mut tx = {test["schema_name"]}::default();'
        output += 'tx.network = NetworkType::TESTNET;'
        
        target = test["schema_name"]
        ast_model_fields = ast_model_dict[target].fields
        for key, value in test["descriptor"].items():
            if key == "type":
                continue
            name_list = [f.name for f in ast_model_fields]
            if key not in name_list:
                continue
                        
            index = [f.name for f in ast_model_fields].index(key)
            f = ast_model_fields[index]
            print("### ", test["test_name"])
            print(key)
            if type(f.field_type) == catparser.ast.Array:
                print(f.field_type.element_type)
                dis_type = type_dict[str(f.field_type.element_type)]
                if dis_type == PRIMITIVE_INTEGER:
                    output += f'tx.{key}.extend_from_slice("{value}".as_bytes());'
                for value in value:
                    if dis_type == DisplayType.STRUCT:
                        print("out")
                    elif dis_type == DisplayType.ENUM:
                        value = value.upper()
                        output += f'tx.{key}.push({f.field_type.element_type}::{value.upper()});'
                    elif dis_type == DisplayType.BYTE_ARRAY:
                        output += f'tx.{key}.push({f.field_type.element_type}::from_str("{value}").unwrap());'
                    elif dis_type == DisplayType.INTEGER:
                        pass
                    elif dis_type == PRIMITIVE_INTEGER:
                        pass
                    else:
                        exit('??')
                continue
            dis_type = type_dict[str(f.field_type)]
            
            if dis_type == DisplayType.STRUCT:
                base = key
                for key, value in value.items():
                    if key == "type":
                        continue
                    output += f"tx.{base}.{key} = {f.field_type}({value});"
            elif dis_type == DisplayType.ENUM:
                value = value.upper()
                if ' ' in value:
                    values = value.split(" ")
                    output += f'tx.{key} = '
                    for value in values:
                        output += f'{f.field_type}::{value.upper()}|'
                    output = output[:-1] + ';'
                else:
                    output += f'tx.{key} = {f.field_type}::{value.upper()};'
                continue
            elif dis_type == DisplayType.BYTE_ARRAY:
                continue
            elif dis_type == DisplayType.INTEGER:
                output += f'tx.{key} = {f.field_type}({value});'
            elif dis_type == PRIMITIVE_INTEGER:
                output += f'tx.{key} = {value};'
                
        output += '}'
    output += '}'
    with open(output_path, 'w', encoding='utf8', newline='') as output_file:
        output_file.write(output)
    