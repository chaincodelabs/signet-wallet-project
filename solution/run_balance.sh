# PYTHON
pip install ecdsa
python ./solution/python/balance.py

# RUST
# Running on CI server
# cargo run --manifest-path ./solution/rust/Cargo.toml -p balance -- ~/.bitcoin/signet/.cookie

# Testing locally
# cargo run --manifest-path ~/chaincodelabs/signet-wallet-project-sb1752/solution/rust/Cargo.toml -p balance -- "/Users/shaanbatra/Library/Application Support/Bitcoin/signet/.cookie"
# Note: You can also just cd into your rust directory and run `cargo run -p balance` or `cargo run -p spend` depending on which program you want to run