# ADR-0002: Rust modular monolith with separate API and worker binaries

- **Status:** Proposed
- **Date:** 2026-11-04
- **Deciders:** Engineering
- **Related:** [ARCHITECTURE §3](../ARCHITECTURE.md)

## Context

The platform needs a high-throughput API, background processing, spatial queries, and AI orchestration. The original concept listed several services (API, geospatial, AI). Operating many services for a single-tenant demo would add deployment complexity without a proven scaling need.

## Decision

Build a **modular monolith** in Rust:

- `services/api`: Axum HTTP API.
- `services/worker`: background jobs, same domain crates.
- Shared crates: `domain`, `db`, `connectors`, `geo`, `ai`, `objectstore`, `telemetry`.
- Python only for inference and heavy geospatial processing, behind a service boundary.

Modules are separated by domain boundaries (auth, projects, connectors, datasets, jobs, assistant, audit), with service interfaces and no cross-module table access. The API and worker are separate processes for independent scaling and failure isolation.

## Consequences

**Positive**

- One deployable domain model; transactional consistency.
- Simple self-host topology (two binaries + Postgres + storage).
- Easy local development and debugging.
- Can split services later along existing module seams.

**Negative**

- A single codebase can become entangled without discipline.
- Rust compile times and team ramp-up.
- The worker and API share a release cadence.

**Mitigations**

- Enforce module boundaries in CI (import rules, crate boundaries).
- Keep domain logic out of HTTP handlers.
- Extract a service only when a measured boundary (scaling, failure, team ownership) justifies it.

## Alternatives considered

| Alternative | Why rejected |
|---|---|
| Microservices from day one | Operational cost, distributed transactions, no scale need |
| Python/FastAPI monolith | Weaker performance/safety for the core; Python still used for geo/AI |
| Serverless | Cold starts, long-running jobs, self-host complexity |

## Review trigger

Revisit when a module needs independent scaling (for example tile serving), a separate team owns it, or deployment frequency diverges.
