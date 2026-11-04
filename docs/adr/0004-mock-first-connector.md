# ADR-0004: Mock-first BIM connector architecture

- **Status:** Accepted
- **Date:** 2026-11-04
- **Deciders:** Maintainer
- **Related:** [REVIZTO_CONNECTOR](../REVIZTO_CONNECTOR.md)

## Context

Revizto API access is not available yet. The demo must still prove the connector experience, and the product must not be coupled to one vendor. The API's geometry and georeferencing capabilities are unconfirmed, and no webhooks are documented.

## Decision

Define a provider-agnostic `BimConnector` trait and implement the **mock provider first**. The real Revizto adapter is a second implementation behind the same trait and a feature flag.

- The mock includes a clearly labelled OAuth-shaped consent screen and deterministic fixtures.
- Contract tests run against both adapters.
- The canonical model is ours; vendor JSON is preserved in `raw`.
- Revizto sync is polling-based (no documented webhooks) with cursors/ETags and rate-limit budgets.
- The demo never requires real credentials.

## Consequences

**Positive**

- Demo and CI are independent of vendor access.
- The adapter interface is proven before a real integration.
- Switching/adding vendors is a new adapter, not a rewrite.
- Honest about missing geometry: localization is explicit.

**Negative**

- Fixtures can drift from the real API.
- The mock may hide real-world pagination/rate-limit quirks.
- Extra code to maintain.

**Mitigations**

- Validate fixtures against official docs; replace with recorded responses when access arrives.
- Keep the canonical schema versioned.
- Make the real adapter the default once validated; keep the mock for demos/CI.

## Alternatives considered

| Alternative | Why rejected |
|---|---|
| Wait for API access | Blocks the demo and the architecture |
| Build directly against Revizto | Couples the core to one vendor and unverified endpoints |
| Use pyRevizto | GPL-3.0; cannot link/copy into Apache-2.0 code |
| CSV/IFC import only | Does not prove the connector experience |

## Review trigger

When real access is granted, run the contract tests, record real responses, and decide whether the mock remains the demo default.
