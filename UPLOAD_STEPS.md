# Upload Steps for ProofMoney Core Rust MVP Scaffold v1.0

## Step 1: Create GitHub repository

Create:

```text
https://github.com/organizations/proofmoney-protocol/repositories/new
```

Settings:

```text
Repository name: core
Description: Rust local MVP prototype for ProofMoney.
Visibility: Public
Add README: No
```

## Step 2: Upload files

Extract this ZIP.

Upload all extracted files directly into the repository root.

Do not upload the parent folder itself.

## Step 3: Commit

Commit message:

```text
core: add Rust MVP scaffold v1.0
```

## Step 4: Local verification

If you have Rust installed locally:

```bash
cargo build
cargo test
cargo run -p proofmoney-cli -- starting-state
cargo run -p proofmoney-cli -- simulate-release --interval 1
cargo run -p proofmoney-cli -- verify-supply
cargo run -p proofmoney-cli -- verify-rule
cargo run -p proofmoney-cli -- integrity-status
```

## Important

This is a local MVP scaffold. It is not a public network, sale, airdrop, or yield product.
