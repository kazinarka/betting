build:
	cd program; cargo build-bpf

fmt:
	cd program; cargo fmt --all

lint:
	cd program; cargo clippy --all && cargo fix --tests --all-features --allow-dirty

pre-commit: fmt lint
	cd program; cargo build-bpf

init:
	cd client; cargo run -- init -e dev -s /home/ideasoft/.config/solana/id.json -m AJuY2ejuYaEu9PJefnLt6bEQW4Z1JVeQTbkw1Zq367YX -t CgTaDeje9owjSxo2et9HS7q7Kyk7hngaPd32HxzGbYLN

change_close_delay:
	cd client; cargo run -- change_close_delay -e dev -s /home/ideasoft/.config/solana/id.json -d 10

lock_bets:
	cd client; cargo run -- lock_bets -e dev -s /home/ideasoft/.config/solana/id1.json

unlock_bets:
	cd client; cargo run -- unlock_bets -e dev -s /home/ideasoft/.config/solana/id1.json

new_manager:
	cd client; cargo run -- new_manager -e dev -s /home/ideasoft/.config/solana/id1.json -m BYX8A4T46wfMbyVKty3z8diuLmJydPDrNzwMwKMFz87P

set_global_fee:
	cd client; cargo run -- set_global_fee -e dev -s /home/ideasoft/.config/solana/id1.json -f 10

set_admin_fee:
	cd client; cargo run -- set_admin_fee -e dev -s /home/ideasoft/.config/solana/id1.json -f 10

set_winner_fee:
	cd client; cargo run -- set_winner_fee -e dev -s /home/ideasoft/.config/solana/id1.json -f 10

set_transaction_fee:
	cd client; cargo run -- set_transaction_fee -e dev -s /home/ideasoft/.config/solana/id1.json -f 10

add_supported_token:
	cd client; cargo run -- add_supported_token -e dev -s /home/ideasoft/.config/solana/id1.json -t 8hp71urEffeQFo49wSbe43rwAnj2Mw5sgCDWhWGTzYH1

registration:
	cd client; cargo run -- registration -e dev -s /home/ideasoft/.config/solana/user.json -r BEDrgp8jU1YnF8BKe15oZ57L2gHEzrEpY4Jds9JMc5Q7 -p password

add_bot:
	cd client; cargo run -- add_bot -e dev -s /home/ideasoft/.config/solana/id1.json -b So11111111111111111111111111111111111111112

new_game:
	cd client; cargo run -- new_game -e dev -s /home/ideasoft/.config/solana/id.json -v 30

forced_close:
	cd client; cargo run -- forced_close -e dev -s /home/ideasoft/.config/solana/id.json -u AJuY2ejuYaEu9PJefnLt6bEQW4Z1JVeQTbkw1Zq367YX

manually_close:
	cd client; cargo run -- manually_close -e dev -s /home/ideasoft/.config/solana/id.json

join_game:
	cd client; cargo run -- join_game -e dev -s /home/ideasoft/.config/solana/user.json -m AJuY2ejuYaEu9PJefnLt6bEQW4Z1JVeQTbkw1Zq367YX -v 30

close_game:
	cd client; cargo run -- close_game -e dev -s /home/ideasoft/.config/solana/id.json -u AJuY2ejuYaEu9PJefnLt6bEQW4Z1JVeQTbkw1Zq367YX -w FUj5oxth59kq6J1V5eKK4pWNLUJ7gMKrdExyxLfqEWAH -t 1