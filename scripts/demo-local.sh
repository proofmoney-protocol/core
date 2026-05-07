#!/usr/bin/env bash
set -euo pipefail

echo "ProofMoney Local MVP Demo"
echo "Safety: local MVP only. No public network, token sale, yield, airdrop, exchange integration, or production wallet."
echo

echo "Resetting local MVP data at ~/.proofmoney ..."
rm -rf "$HOME/.proofmoney"

echo
echo "Building workspace..."
cargo build --workspace --all-targets

echo
echo "Running tests..."
cargo test --workspace --all-targets

echo
echo "Starting State..."
cargo run -p proofmoney-cli -- starting-state

echo
echo "Append release event..."
cargo run -p proofmoney-cli -- simulate-release --interval 1 --append

echo
echo "Ledger status..."
cargo run -p proofmoney-cli -- ledger-status

echo
echo "Verify supply..."
cargo run -p proofmoney-cli -- verify-supply

echo
echo "Create local MVP wallet..."
cargo run -p proofmoney-cli -- create-wallet --force

echo
echo "Show local address..."
cargo run -p proofmoney-cli -- new-address

echo
echo "List release events..."
cargo run -p proofmoney-cli -- list-release-events

echo
echo "List transfer events..."
cargo run -p proofmoney-cli -- list-transfer-events

echo
echo "Export proof snapshot JSON..."
cargo run -p proofmoney-cli -- export-proof-snapshot --json > /tmp/proofmoney-proof-snapshot.json

echo
echo "Prepare local Proof Explorer..."
cargo run -p proofmoney-cli -- prepare-explorer

echo
echo "Demo complete."
echo "Snapshot: /tmp/proofmoney-proof-snapshot.json"
echo "Explorer: $HOME/.proofmoney/explorer/index.html"
echo "Safety: test units have no monetary value."
