# ADR-0009: Free GitHub organisation with a single human account

- **Status:** Accepted
- **Date:** 2026-11-04
- **Deciders:** Maintainer
- **Related:** [GITHUB_ORG_SETUP](../GITHUB_ORG_SETUP.md)

## Context

The project needs a clean public identity and a path to open source without a paid GitHub plan. A GitHub organisation must be owned by a Personal Account. Creating a free organisation does not require a paid plan.

The project is built by one maintainer working interactively with a coding assistant. It does not use an autonomous coding agent, a bot account, or a machine account.

## Decision

Use a **free GitHub organisation** with a **single human account**:

- **Human owner:** `atpl-training` owns the free organisation `terrassay`; reviews, merges, and handles billing.
- **Organisation:** `terrassay` on the **Free** plan; the project’s public identity.
- **Repository:** `terrassay/terrassay`, public from bootstrap, with branch protection, CODEOWNERS, required CI, secret scanning, and Dependabot.
- **No machine account, bot account, or paid plan** is used.
- Branch protection requires a pull request and CI. Because there is one human, no separate approval is required; the maintainer reviews and merges.

## Consequences

**Positive**

- $0 on GitHub for the organisation and public repo.
- Clean public identity with a separate org namespace.
- No second account, token, or automation identity to maintain.
- Human review and merge remain with the maintainer.

**Negative**

- No autonomous agent; scoped work must be done interactively with the assistant.
- One owner is a bus-factor risk.
- A public repo means the code and history are visible; secrets and data hygiene are mandatory.

**Mitigations**

- Keep secrets out of the repo; use environment-scoped deployment secrets only if needed.
- Protect `main`; no force pushes or deletions.
- Document recovery and add a trusted co-owner only when a real person is available.
- Use the assistant for scoped work, with human review and tests.

## Alternatives considered

| Alternative | Why rejected |
|---|---|
| Create a second free human account | Violates GitHub’s one-free-account rule; risk of account closure |
| Add a machine/bot account | More moving parts and a second identity; not needed for an interactive build |
| Paid GitHub Pro human account | Not needed for a free public org |
| Use a personal repo without an org | Works, but loses the project namespace and future team structure |
| Self-hosted Git | Loses network effects and integrations; can be a mirror later |

## Review trigger

Revisit if the project adds automated CI agents, needs private-repo protections that require Team, joins a foundation, or needs an organisation-level legal entity.
