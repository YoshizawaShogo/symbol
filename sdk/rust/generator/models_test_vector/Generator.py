#!/usr/bin/python

import sys
from pathlib import Path
import json

SYMBOL_MODELS_TESTVECTORS_PATH = "../../tests/vectors/symbol/models"

def main():
    args = sys.argv
    output_path = Path(args[1])
    generate_file(output_path=output_path)

def generate_file(output_path: Path):
    output = 'use hex::decode;use symbol::symbol::prelude::*;'
    
    for json_name in ("transactions.json", "blocks.json", "receipts.json", "other.json"):
        with open(SYMBOL_MODELS_TESTVECTORS_PATH + "/" + json_name) as f:
            json_data =  json.load(f)
        for test in json_data:
            output += parse_test(test)
            
    with open(output_path, 'w', encoding='utf8', newline='') as output_file:
        output_file.write(output)

def parse_test(json_data_of_test):
    test = json_data_of_test
    payload = json_data_of_test["payload"]
    
    ret = f'''
        #[test]
        #[allow(non_snake_case)]
        fn {test["test_name"]}() {{
            let input_payload = decode("{payload}").unwrap();
            let tx = {test["schema_name"]}::deserialize(&input_payload).unwrap().0;
            let output_payload = tx.serialize();
            assert_eq!(input_payload, output_payload);
        }}
    '''
    return ret

if __name__ == "__main__":
    main()