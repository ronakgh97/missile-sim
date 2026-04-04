#!/usr/bin/env just --justfile

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