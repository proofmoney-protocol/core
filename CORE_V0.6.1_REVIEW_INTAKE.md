# ProofMoney Core v0.6.1 Review Intake Pack

## Purpose

This patch prepares the repository to receive external review feedback in a structured way.

It does not change protocol logic.

It does not add monetary functionality.

## Added

```text
.github/ISSUE_TEMPLATE/review_finding.yml
.github/ISSUE_TEMPLATE/documentation_feedback.yml
.github/ISSUE_TEMPLATE/config.yml
.github/pull_request_template.md
SECURITY.md
docs/review-intake-process.md
docs/reviewer-outreach-message.md
```

## Why

After v0.6.0 external review preparation, reviewers need a clear way to:

- report findings;
- classify severity;
- identify affected files;
- provide reproduction steps;
- submit documentation feedback;
- open PRs safely;
- avoid forbidden token-sale / airdrop / yield / exchange scope.

## Safety Boundary

This pack only improves review intake and project governance.

It does not create PRM with monetary value, a public network, a token sale, airdrop, exchange integration, hosted API, or production wallet.
