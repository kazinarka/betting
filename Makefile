build:
	cd program; cargo build-bpf

fmt:
	cd program; cargo fmt --all

lint:
	cd program; cargo clippy --all && cargo fix --tests --all-features --allow-dirty

pre-commit: fmt lint
	cd program; cargo build-bpf

init:
	cd client; cargo run -- init -e dev -s /home/ideasoft/.config/solana/id.json -m E5L2TjtD8nVjNxoEwgizoM4wsdrAtXg52VCnFF4BG2gg -t HgTtcbcmp5BeThax5AU8vg4VwK79qAvAKKFMs8txMLW6

change_close_delay:
	cd client; cargo run -- change_close_delay -e dev -s /home/ideasoft/.config/solana/id.json -d 10

lock_bets:
	cd client; cargo run -- lock_bets -e dev -s /home/ideasoft/.config/solana/id.json

unlock_bets:
	cd client; cargo run -- unlock_bets -e dev -s /home/ideasoft/.config/solana/id.json

new_manager:
	cd client; cargo run -- new_manager -e dev -s /home/ideasoft/.config/solana/id.json -m E5L2TjtD8nVjNxoEwgizoM4wsdrAtXg52VCnFF4BG2gg

set_global_fee:
	cd client; cargo run -- set_global_fee -e dev -s /home/ideasoft/.config/solana/id.json -f 10

set_admin_fee:
	cd client; cargo run -- set_admin_fee -e dev -s /home/ideasoft/.config/solana/id.json -f 10

set_winner_fee:
	cd client; cargo run -- set_winner_fee -e dev -s /home/ideasoft/.config/solana/id.json -f 10

set_transaction_fee:
	cd client; cargo run -- set_transaction_fee -e dev -s /home/ideasoft/.config/solana/id.json -f 10

add_supported_token:
	cd client; cargo run -- add_supported_token -e dev -s /home/ideasoft/.config/solana/id.json

registration:
	cd client; cargo run -- registration -e dev -s /home/ideasoft/.config/solana/id.json

add_bot:
	cd client; cargo run -- add_bot -e dev -s /home/ideasoft/.config/solana/id.json