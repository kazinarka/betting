build:
	cd program; cargo build-bpf

fmt:
	cd program; cargo fmt --all

lint:
	cd program; cargo clippy --all && cargo fix --tests --all-features --allow-dirty

pre-commit: fmt lint
	cd program; cargo build-bpf

init:
	cd client; cargo run -- init -e dev -s /Users/illiafedotov/.config/solana/id.json -m 4kMtMnYWFbsMc7M3jcdnfCceHaiXmrqaMz2QZQAmn88i -t 3e7FKiXHn1kmMSTLDgJkMWxwd2WA6PM9niYcxbfk8EKN

change_close_delay:
	cd client; cargo run -- change_close_delay -e dev -s /Users/illiafedotov/.config/solana/id.json -d 10

lock_bets:
	cd client; cargo run -- lock_bets -e dev -s /Users/illiafedotov/.config/solana/id.json

unlock_bets:
	cd client; cargo run -- unlock_bets -e dev -s /Users/illiafedotov/.config/solana/id.json

new_manager:
	cd client; cargo run -- new_manager -e dev -s /Users/illiafedotov/.config/solana/id.json -m 4mDt5VKSWJbk24HwFD5Na2pqB3WZj7bdrxPwCDT4BAcs

set_global_fee:
	cd client; cargo run -- set_global_fee -e dev -s /Users/illiafedotov/.config/solana/id.json -f 10

set_admin_fee:
	cd client; cargo run -- set_admin_fee -e dev -s /Users/illiafedotov/.config/solana/id.json -f 50

set_winner_fee:
	cd client; cargo run -- set_winner_fee -e dev -s /Users/illiafedotov/.config/solana/id.json -f 50

set_transaction_fee:
	cd client; cargo run -- set_transaction_fee -e dev -s /Users/illiafedotov/.config/solana/id.json -f 0

set_type_price:
	cd client; cargo run -- set_type_price -e dev -s /Users/illiafedotov/.config/solana/id.json -t 1 -p 15

add_supported_token:
	cd client; cargo run -- add_supported_token -e dev -s /Users/illiafedotov/.config/solana/id.json -t GwFvncrafF6zGMSd1UoXdjtTxwPquD7bYoDNzgRahDx7

registration:
	cd client; cargo run -- registration -e dev -s /Users/illiafedotov/.config/solana/id.json -r 6G7Sc3MjR4AZDAgNJZJmSpLuiNUCRksF3bN8opeX2Fuj -p password

add_bot:
	cd client; cargo run -- add_bot -e dev -s /Users/illiafedotov/.config/solana/id.json -b So11111111111111111111111111111111111111112

new_game:
	cd client; cargo run -- new_game -e dev -s /Users/illiafedotov/.config/solana/id.json -v 1

forced_close:
	cd client; cargo run -- forced_close -e dev -s /Users/illiafedotov/.config/solana/id.json -u 4kMtMnYWFbsMc7M3jcdnfCceHaiXmrqaMz2QZQAmn88i

manually_close:
	cd client; cargo run -- manually_close -e dev -s /Users/illiafedotov/.config/solana/user.json

join_game:
	cd client; cargo run -- join_game -e dev -s /Users/illiafedotov/.config/solana/user.json -m 9LZr77sE8J6bHYXcZXM9AeUJEssWZKh3AhmaXj3G7uUn -v 1

close_game:
	cd client; cargo run -- close_game -e dev -s /Users/illiafedotov/.config/solana/id.json -u 4mDt5VKSWJbk24HwFD5Na2pqB3WZj7bdrxPwCDT4BAcs -w 9LZr77sE8J6bHYXcZXM9AeUJEssWZKh3AhmaXj3G7uUn -t 1