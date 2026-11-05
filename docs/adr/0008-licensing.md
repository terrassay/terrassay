# ADR-0008: Apache-2.0, DCO, and no commercial tier for now

- **Status:** Proposed
- **Date:** 2026-11-04
- **Deciders:** Maintainer
- **Related:** [LICENSING_BUSINESS](../LICENSING_BUSINESS.md), [LEGAL_COMPLIANCE](../LEGAL_COMPLIANCE.md)

## Context

The project is a solo, unfunded, open-source effort. It should be adoptable by contractors and vendors, compatible with the geospatial ecosystem, and free to self-host. There is no revenue plan, no hosted service, and no customer data. The dependency graph includes permissive, weak-copyleft, and GPL components.

## Decision

- Licence original code under **Apache-2.0**.
- Accept contributions under the **DCO**, with no CLA.
- **No commercial tier, open-core paywall, or hosted service** at this stage.
- Keep the entire product open source, including the Revizto adapter and connector SDK.
- Audit every dependency; isolate GPL/AGPL components behind process boundaries.
- Revisit sustainability (donations, support, hosting) only if users ask and the legal/accounting position is clear.

## Consequences

**Positive**

- Patent grant and trademark carve-out.
- Enterprise-friendly; compatible with most of the ecosystem.
- Community trust from a permissive licence and no CLA.
- No open-core boundary to explain or defend.
- Anyone can fork, test, and self-host.

**Negative**

- No revenue to fund development.
- Competitors can host the core.
- Strong-copyleft dependencies need care.
- No unilateral relicensing without contributor permission.

**Mitigations**

- Keep costs near zero: local-first, free tiers, the assistant.
- Accept donations/sponsorships only with transparency.
- CI licence checks; Gate B review before any commercial step.
- Publish `NOTICE` and third-party licences.
- Invite co-maintainers if the project grows.

## Alternatives considered

| Alternative | Why rejected |
|---|---|
| AGPL | Deters adoption; complicates self-host |
| MIT | No explicit patent grant; weaker for enterprise |
| BSL/SSPL | Not OSI-approved; conflicts with "open source" promise |
| Open-core now | Premature; adds legal/pricing complexity before users exist |
| Dual licence | Adds contributor/legal complexity before traction |

## Review trigger

Revisit only if a sustainable funding path emerges and users ask for it; do not close the core retroactively.
