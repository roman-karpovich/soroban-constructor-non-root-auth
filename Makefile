default:
	cargo clean
	soroban contract build --package smart-contract
	cargo test
