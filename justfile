debug := "false"

run year="" day="":
    YEAR={{ year }} DAY={{ day }} cargo run -F allow_dead_code \
        {{ if debug == "true" { "" } else { "-r" } }}
    cargo clippy

log:
    just run | tee results.log

init year day:
    #! /bin/sh
    set -eux
    dir="src/solutions/year{{ year }}"
    mkdir -p "${dir}"
    path="${dir}/day$(printf %02s {{ day }}).rs"
    touch "${path}"
    cargo build
    code "${path}"
