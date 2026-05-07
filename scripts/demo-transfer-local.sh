#!/usr/bin/env bash
set -euo pipefail

echo "ProofMoney Local Transfer Demo"
echo "Safety: local MVP only. No public network, token sale, yield, airdrop, exchange integration, or production wallet."
echo

echo "Resetting local MVP ledger..."
cargo run -q -p proofmoney-cli -- reset-ledger --yes

echo
echo "Creating local MVP wallet..."
cargo run -q -p proofmoney-cli -- create-wallet --force

echo
echo "Reading sender address..."
ADDRESS_JSON="$(cargo run -q -p proofmoney-cli -- new-address --json)"
SENDER="$(printf '%s' "$ADDRESS_JSON" | python3 -c 'import json,sys; print(json.load(sys.stdin)["address"])')"
RECEIVER="tprm1reviewreceiver"

echo "Sender: $SENDER"
echo "Receiver: $RECEIVER"

echo
echo "Appending release event to sender..."
cargo run -q -p proofmoney-cli -- simulate-release --interval 1 --recipient "$SENDER" --append

echo
echo "Creating and appending local transfer..."
cargo run -q -p proofmoney-cli -- create-transfer --from "$SENDER" --to "$RECEIVER" --amount 1.25 --append

echo
echo "Validating local state..."
cargo run -q -p proofmoney-cli -- validate-local-state

echo
echo "Detecting tampering..."
cargo run -q -p proofmoney-cli -- detect-tampering

echo
echo "Listing transfer events..."
cargo run -q -p proofmoney-cli -- list-transfer-events

echo
echo "Exporting proof snapshot..."
cargo run -q -p proofmoney-cli -- export-proof-snapshot --output /tmp/proofmoney-transfer-proof-snapshot.json

echo
echo "Preparing local Proof Explorer..."
cargo run -q -p proofmoney-cli -- prepare-explorer

echo
echo "Transfer demo complete."
echo "Snapshot: /tmp/proofmoney-transfer-proof-snapshot.json"
echo "Explorer: $HOME/.proofmoney/explorer/index.html"
echo "Safety: test units have no monetary value."
