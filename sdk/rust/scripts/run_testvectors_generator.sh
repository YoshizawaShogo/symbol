#!/bin/bash

set -ex

function generate_model_vectors() {
	# $1 blockchain
	local git_root
	git_root="$(git rev-parse --show-toplevel)"
	rust_sdk="${git_root}/sdk/rust"

	python3 "${rust_sdk}/generator/models_test_vector/Generator.py" "${rust_sdk}/tests/${1}_models.rs"
}

generate_model_vectors "symbol"
cargo fmt
cargo check