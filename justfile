#!/usr/bin/env just --justfile

set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

release:
    cargo build --release    

lint:
    cargo clippy

fmt:
    cargo fmt -- --check

check-deadcode:
    rg '#\[allow\(dead_code\)\]'

check-todos:
    rg -i '// TODO:'

run-default:
    cargo run --bin default

run-data-gen:
    cargo run --bin data_gen

run-debug:
    cargo run --bin debug
