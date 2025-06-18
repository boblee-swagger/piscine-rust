#!/bin/bash
#clone the rust-tests repo on github.com/01-edu/rust-tests and place into piscine-rust
#running this file into the root project on each exo for testing.

exo_name="$1"
cp -r "../rust-tests/tests/${exo_name}_test/" .

cat <<EOF >> Cargo.toml
[[test]]
name = "01_tests" # Name your test target
path = "${exo_name}_test/src/main.rs" # Point to the main file for this test target
EOF

# Corrected: Running the test target
cargo test --test 01_tests