# Architecture Decision Records

Each ADR captures a decision that constrains the build. Statuses: **Accepted**, **Proposed**, **Superseded**, **Rejected**.

| ADR | Title | Status |
|---|---|---|
| [0001](0001-reuse-geolibre.md) | Reuse GeoLibre as the map surface; build a separate BIM shell | Accepted |
| [0002](0002-modular-monolith.md) | Rust modular monolith with separate API and worker binaries | Proposed |
| [0003](0003-postgis-system-of-record.md) | PostGIS as system of record; DuckDB for analytics | Proposed |
| [0004](0004-mock-first-connector.md) | Mock-first BIM connector architecture | Accepted |
| [0005](0005-georeferencing.md) | Explicit georeferencing and CRS policy | Proposed |
| [0006](0006-job-queue.md) | Postgres job queue first; Valkey optional | Proposed |
| [0007](0007-ai-orchestration.md) | AI orchestration in Rust with typed tools | Proposed |
| [0008](0008-licensing.md) | Apache-2.0, DCO, and no commercial tier for now | Proposed |
| [0009](0009-new-github-org.md) | Free GitHub organisation with a single human account | Accepted |
| [0011](0011-legal-gate.md) | Two legal gates: synthetic public release and commercial use | Accepted |
| [0012](0012-3d-strategy.md) | 3D massing in the demo; no BIM geometry from Revizto | Accepted |
| [0013](0013-auth-tenancy.md) | Demo-first auth and single-tenant tenancy model | Proposed |

## Conventions

- One decision per ADR.
- Context, decision, consequences, alternatives.
- Link to the relevant plan section.
- Supersede rather than rewrite; keep history.
