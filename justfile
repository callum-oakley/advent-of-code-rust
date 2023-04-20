run year="" day="":
    YEAR={{ year }} DAY={{ day }} cargo run -r -F allow_dead_code
    cargo clippy

log:
    just run | tee results.log

init year day:
    #! /usr/bin/env bash
    set -euxo pipefail
    dir="src/solutions/year{{ year }}"
    mkdir -p "${dir}"
    path="${dir}/day$(printf %02s {{ day }}).rs"
    touch "${path}"
    cargo build
    code "${path}"
