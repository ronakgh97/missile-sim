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

run-plot:
    cargo build --bin plot
    cargo run --bin plot

run-data-gen:
    cargo build --bin data
    cargo run --bin data

run-debug:
    cargo build --bin debug
    cargo run --bin debug
