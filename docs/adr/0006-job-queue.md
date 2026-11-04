# ADR-0006: Postgres job queue first; Valkey optional

- **Status:** Proposed
- **Date:** 2026-11-04
- **Deciders:** Engineering
- **Related:** [ARCHITECTURE §3.5](../ARCHITECTURE.md), [DATA_MODEL §3.6](../DATA_MODEL.md)

## Context

The original stack listed Redis for job processing. The workload is modest: connector syncs, dataset ingestion, analyses, and AI indexing for a single-tenant demo and small self-hosted deployments. Adding Redis increases the self-host dependency surface and the licence review burden.

## Decision

Use a **Postgres-backed job queue**:

- `jobs` table with `FOR UPDATE SKIP LOCKED`, leases, retries, priorities, progress, and idempotency keys.
- `LISTEN/NOTIFY` for wakeups; periodic polling as fallback.
- Workers extend leases and report progress.
- Valkey (BSD-3-Clause) is optional for caching or rate limiting if measurements justify it; Redis is not the default.

## Consequences

**Positive**

- One less service in the self-host stack.
- Job state and domain data share a transaction.
- Easy inspection with SQL; no queue broker to operate.
- Avoids Redis licensing ambiguity.

**Negative**

- Throughput is lower than a dedicated broker at very high scale.
- Long-running jobs need lease management.
- Polling can add latency without `NOTIFY`.

**Mitigations**

- Index `(status, run_at, priority)`; keep jobs short.
- Use `NOTIFY` plus a short poll interval.
- Measure queue depth/latency; add Valkey or a broker only when thresholds are crossed.

## Alternatives considered

| Alternative | Why rejected |
|---|---|
| Redis/Valkey queue | Extra service and licence review for no measured need |
| RabbitMQ | Operational overhead; overkill |
| In-process background tasks | Lose work on restart; no retries |
| Temporal | Powerful but heavy for the demo |

## Review trigger

Revisit if sustained queue latency exceeds 30 s, job throughput exceeds ~100/min, or multiple services need to consume the same events.
