# ADR-0011: Two legal gates: synthetic public release and commercial use

- **Status:** Accepted
- **Date:** 2026-11-04
- **Deciders:** Maintainer
- **Related:** [LEGAL_COMPLIANCE](../LEGAL_COMPLIANCE.md)

## Context

The project will be open source and integrates with third-party platforms. Licence, trademark, data, and API-term mistakes are cheaper to prevent than to fix after a public launch. The project is testing-only and uses synthetic data today, but that will change.

## Decision

Use **two gates**, because the synthetic public repo and a commercial service carry different risk.

### Gate A — public synthetic repository (self-audit)

Before the first public push:

- Name/handle/domain reserved; free trademark knock-out search.
- Apache-2.0/NOTICE/third-party notices.
- Dependency licence audit and SBOM.
- No GPL/AGPL code linked into core; PostGIS documented as a separate service.
- No secrets, personal data, customer data, or vendor data in history.
- Synthetic fixtures labelled; GeoLibre/MapLibre/Cesium attributions.
- Security policy, CoC, CONTRIBUTING, GOVERNANCE, TRADEMARKS.
- AI-assisted code policy and disclosure.

A lawyer is not required for this synthetic, non-commercial bootstrap. A second person or a fresh self-review checks the list.

### Gate B — customer data, paid service, or vendor integration (counsel)

Do not accept real project data, payment, or a vendor-data integration until a qualified lawyer reviews:

- Entity/IP ownership.
- Revizto API terms and data handling.
- Privacy/DPA/SCC/DPIA.
- Trademark clearance/registration.
- Terms of service, liability, insurance.
- AI provider DPA and retention terms.
- Security review/pen test.
- Export/sanctions if applicable.

**No commercial step before Gate B.** This is the constraint that keeps an unfunded project out of trouble.

## Consequences

**Positive**

- Ships the synthetic open-source repo without a legal budget.
- Avoids a public relaunch/rename after a complaint.
- Forces dependency and data hygiene early.
- Creates a reusable compliance process for the commercial step.

**Negative**

- Commercial work is blocked until Gate B is funded.
- Gate A still takes time and discipline.
- Some real-project opportunities may be deferred.

**Mitigations**

- Run Gate A before the first public push; keep the evidence in the repo or vault.
- Automate licence/SBOM checks so the gate is mostly evidence collection.
- Defer Gate B until a real customer, payment, or vendor integration appears; reserve €5k–€15k then.
- Keep the demo synthetic and local; no NDA or customer data is needed.

## Alternatives considered

| Alternative | Why rejected |
|---|---|
| Publish first, fix later | Irreversible disclosure; trademark/licence risk |
| Rely on "it's open source" | Open source does not cure API/trademark/data violations |
| Skip counsel | Acceptable for the synthetic Gate A; unacceptable for Gate B |

## Review trigger

Revisit when a commercial step is considered; do not lower Gate B under schedule pressure.
