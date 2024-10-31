# -include .env

# .PHONY: build deploy init add_account claim_tokens reclaim_tokens create_pool

# build:
# 	cargo build --target wasm32-unknown-unknown --release

# deploy:
# 	near deploy walnet.openedu-vbi.testnet target/wasm32-unknown-unknown/release/walnet.wasm

# init:
# near call token.openedu-vbi.testnet new_default_meta '{"owner_id": "token.openedu-vbi.testnet", "total_supply": "1000000000000000000000"}' --accountId token.openedu-vbi.testnet
# add_account:
# 	near call 117airdrop.testnet add_account_amount '{"pool_id": 0,"accounts_and_amounts": [["116airdrop.testnet", "1000000"]]}' --accountId 117airdrop.testnet

# claim_tokens:
# 	near call 117airdrop.testnet claim_tokens '{"pool_id": 0}' --accountId 116airdrop.testnet

# reclaim_tokens:
# 	near call 117airdrop.testnet reclaim_unclaimed_tokens '{"pool_id": 0}' --accountId 117airdrop.testnet

# create_pool:
# 	near call 117airdrop.testnet create_pool '{"end_date": 1, "token_amount": "1000000000000", "description": "Openedu12"}' --accountId 117airdrop.testnet

# add_account_to_pool:
# 	near call 117airdrop.testnet add_account_amount '{"pool_id": 1,"accounts_and_amounts": [["112airdrop.testnet", "1000000"]]}' --accountId 117airdrop.testnet