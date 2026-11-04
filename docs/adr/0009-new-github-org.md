# ADR-0009: Free GitHub organisation with a single human account

- **Status:** Accepted
- **Date:** 2026-11-04
- **Deciders:** Maintainer
- **Related:** [GITHUB_ORG_SETUP](../GITHUB_ORG_SETUP.md) · [DEV_AUTOMATION](../DEV_AUTOMATION.md)

## Context

The project needs a clean public identity, least-privilege automation, and a path to open source without a paid GitHub plan. A GitHub organisation must be owned by a Personal Account. GitHub's Terms permit one free Personal Account and one free machine account; a second free human account is not permitted. Creating a free organisation does not require a paid plan.

The maintainer wants Superagent to operate under the human account rather than a separate machine account, and to store the token in macOS Keychain.

## Decision

Use a **free GitHub organisation** and a **single human account**:

- **Human owner:** `atpl-training` owns the free organisation `terrassay`; reviews, merges, and handles billing.
- **Organisation:** `terrassay` on the **Free** plan; the project’s public identity.
- **Repository:** `terrassay/terrassay`, public from bootstrap, with branch protection, CODEOWNERS, required CI, secret scanning, and Dependabot.
- **Superagent:** runs as `atpl-training` using a fine-grained PAT scoped only to `terrassay/terrassay`. The token is stored in macOS Keychain and injected only into the `gh` process by the `gh-terrassay` wrapper.
- **No machine account, collaborator invite, or paid plan** is used.
- Because the PR author and reviewer are the same account, branch protection requires a PR and CI, allows owner admin bypass, and the agent token has no Administration permission. The owner merges.

## Consequences

**Positive**

- $0 on GitHub for the organisation, public repo, and token.
- Clean public identity with a separate org namespace.
- No second account to create or maintain.
- Token is scoped to one repo and stored encrypted in Keychain.
- Human review/merge remains with the owner.

**Negative**

- Commits and PRs are authored by `atpl-training`; there is no separate bot identity.
- A same-account PR cannot be approved by that account; the owner merges via admin bypass.
- The agent can see the account’s notifications; isolation is weaker than a machine account.
- A compromised agent acts as the human within token scope.

**Mitigations**

- Scope the PAT to one repo and Write permissions; no admin, secrets, or org access.
- Keep `main` protected; the token cannot bypass.
- Run Superagent on an isolated OS user/VM where possible.
- Rotate the token every 90 days; delete it on compromise.
- Disclose AI-assisted PRs.

## Alternatives considered

| Alternative | Why rejected |
|---|---|
| Create a second free human account | Violates GitHub’s one-free-account rule; risk of account closure |
| Separate free machine account | Stronger isolation, but the maintainer wants the agent on the human account and the collaborator flow was blocked |
| Paid GitHub Pro human account | Not needed for a free public org |
| Use a personal repo without an org | Works, but loses the project namespace and future team structure |
| Self-hosted Git | Loses network effects and integrations; can be a mirror later |

## Review trigger

Revisit if audit separation becomes necessary, the project needs private-repo protections that require Team, or the project joins a foundation.
