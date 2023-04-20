solution_path := "src/solutions/year%s/day%02s.rs"

run year="" day="":
    YEAR={{ year }} DAY={{ day }} cargo run -r -F allow_dead_code
    cargo clippy

log:
    just run | tee results.log

init year day:
    touch $(printf {{ solution_path }} {{ year }} {{ day }})
    cargo build
    code $(printf {{ solution_path }} {{ year }} {{ day }})
