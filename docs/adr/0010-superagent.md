# ADR-0010: Superagent for scoped development automation

- **Status:** Accepted
- **Date:** 2026-11-04
- **Deciders:** Maintainer
- **Related:** [DEV_AUTOMATION](../DEV_AUTOMATION.md)

## Context

DeepSeek's `dsh-superagent` is an MIT-licensed research prototype that can take GitHub requests, work in isolated sessions, and open pull requests. It requires a dedicated GitHub user account and runs with full OS user access. The project wants to integrate it early without compromising security or code quality.

## Decision

Use Superagent from week 1 for **scoped, reviewable work** only:

- Tests, docs, fixtures, small refactors, dependency-free utilities.
- No auth/crypto, legal/licence files, workflows, release engineering, or customer data.
- Dedicated isolated VM/OS user; only the bot's GitHub credentials.
- Allowlisted maintainers; observe mode first; no Feishu.
- Branch protection, CODEOWNERS, required CI, human approval, no auto-merge.
- AI-assisted PRs are disclosed; a human signs the DCO and owns the change.
- Kill switch tested in week 1.

## Consequences

**Positive**

- Parallel progress on low-risk work.
- Forces clear acceptance criteria and `AGENTS.md`.
- First-hand experience with agent permissions that informs the product.
- Measurable productivity data.

**Negative**

- Research-prototype stability and security risk.
- Review overhead; plausible-but-wrong changes.
- Model-provider data handling and cost.
- Potential for licence contamination if the agent copies code.

**Mitigations**

- Isolation and least privilege.
- Narrow tasks and small PRs.
- Automated licence/security scans.
- Monthly value review; pause if rework exceeds benefit.

## Alternatives considered

| Alternative | Why rejected |
|---|---|
| No agent | Leaves parallel low-risk work on the critical path |
| General-purpose coding agent with repo admin | Too much access; no mandate model |
| Wait until the project is mature | Loses the bootstrap benefit and the learning |
| Build our own agent | Distraction from the product |

## Review trigger

Pause or narrow if any control is bypassed, rework exceeds human PRs, or the prototype becomes unmaintained.
