# Upload Steps for Core CI and Alpha Pack

## Target Repository

```text
https://github.com/proofmoney-protocol/core
```

## Files to Upload

Upload all files from this ZIP into the repository root.

This pack adds or updates:

```text
.github/workflows/rust-ci.yml
crates/proofmoney-crypto/Cargo.toml
CONTRIBUTING.md
SECURITY.md
LOCAL_VERIFICATION.md
REPO_SETTINGS_CHECKLIST.md
RELEASE_NOTES_v0.1.0-alpha.md
CORE_NEXT_STEPS_v1.1.md
UPLOAD_STEPS.md
```

## Commit Message

```text
core: add CI and alpha release docs
```

## After Upload

Open the repository Actions tab.

Expected workflow:

```text
Rust CI
```

If it fails, copy the error log and send it for debugging.
