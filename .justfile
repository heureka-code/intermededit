[private]
default:
    @just --justfile '{{ justfile() }}' --working-directory '{{ invocation_dir() }}' --list

validate:
	cd lists && make lists
	cd crates/duden-reference && cargo run --release --bin duden-validator

generate:
	cd lists && make lists
	cd crates/duden-reference && cargo run --release --bin duden-generator

stats:
	cd lists && make lists
	cd crates/duden-reference && cargo run --release --bin duden-stats

