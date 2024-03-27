# How to test

``` bash
cargo test --release
cargo test --release --all-features
```

# Generate catbuffer (src/symbol/models.rs)

```bash
./scripts/run_catbuffer_generator.sh
```

# Generate Test (tests/symbol_models.rs)

``` bash
./scripts/run_testvectors_generator.sh
```

# Abstract graph for design
``` mermaid
    graph LR;
        n0(models_header.rs);
        n1(models.rs);
        n2(address.rs)
        n3(cipher.rs)
        n4(key.rs)
        n5(bip.rs)
        n6(prelude.rs)
        n7(symbol_crypto.rs)
        n8(symbol_models.rs)
        n9(symbol/catbuffer)

        subgraph symbol-repository
            n9 --"generate with Generator.py"--> n1;
            subgraph rust-sdk
                n0 --use--> n1;
                n1 --use--> n2;
                n1 --use--> n3;
                n1 --use--> n4;
                n1 --use--> n5;
                n1 --use--> n6;

                n2 --use--> n6
                n3 --use--> n6
                n4 --use--> n6
                n5 --use--> n6

                n6 --use--> n7;
                n6 --use--> n8;
            end
        end
```

# Prerequisites

``` bash
# setup for Test.
$ sudo apt install curl
$ curl https://sh.rustup.rs -sSf | sh

# setup for Generator.
$ sudo apt install python3 python3-pip
$ pip3 install -r generator/requirements.txt
```