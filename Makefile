test-generate-vault:
	cd program; cargo test-bpf --test generate_vault

test: test-generate-vault

build:
	cd program; cargo build-bpf

fmt:
	cd program; cargo fmt --all

lint:
	cd program; cargo clippy --all && cargo fix --tests --all-features --allow-dirty

pre-commit: test fmt lint
	cd program; cargo build-bpf