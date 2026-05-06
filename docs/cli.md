# ProofMoney CLI

## Commands

```bash
proofmoney starting-state
proofmoney starting-state --json
proofmoney simulate-release --interval 1
proofmoney simulate-release --interval 1 --json
proofmoney verify-supply
proofmoney verify-supply --json
proofmoney verify-rule
proofmoney verify-rule --json
proofmoney integrity-status
proofmoney integrity-status --json
proofmoney create-wallet
proofmoney create-wallet --json
```

Later MVP steps should add:

```bash
proofmoney new-address
proofmoney sign-message --message "verify ownership"
proofmoney verify-ownership --address <address> --message <message> --signature <signature> --public-key <public_key>
proofmoney create-transfer --from <address> --to <address> --amount 1.25
proofmoney verify-flow --tx <transaction_id>
```
