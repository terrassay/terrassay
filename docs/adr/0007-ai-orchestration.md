# ADR-0007: AI orchestration in Rust with typed tools

- **Status:** Proposed
- **Date:** 2026-11-04
- **Deciders:** Engineering
- **Related:** [AI_ASSISTANT](../AI_ASSISTANT.md), [SECURITY](../SECURITY.md)

## Context

The assistant must answer questions using project data and spatial queries. Putting the agent loop in Python would create a second authorization surface and duplicate the domain model. Letting the model write SQL or call arbitrary HTTP would be unsafe.

## Decision

Implement the agent loop in Rust:

- Context builder, prompt builder, tool registry, permission checks, tool executor, SSE streaming, audit.
- Tools are typed with JSON Schema and executed by the API/worker, not the model.
- No raw SQL, shell, file access, or arbitrary HTTP tools.
- Project/org scope is injected from the session, never from tool arguments.
- Writes are disabled by default; a write requires UI confirmation and audit.
- Python is used only for inference services that need its ecosystem (vision, embeddings), behind an internal boundary.

## Consequences

**Positive**

- One authorization and audit path.
- Tools reuse the same repositories and spatial query builders as the API.
- Easier to test and reason about.
- Provider-agnostic; local models are a configuration.

**Negative**

- Rust LLM/tooling ecosystem is less mature than Python's.
- More upfront work for streaming and provider adapters.
- Python ML features need a clean service contract.

**Mitigations**

- Keep the provider trait narrow (chat, tools, streaming).
- Start with one hosted provider and one local provider.
- Extensive tool-level tests with a mock model.
- Version the tool schema.

## Alternatives considered

| Alternative | Why rejected |
|---|---|
| Python agent (LangChain/LlamaIndex) | Duplicate auth/domain; harder to audit |
| Model-generated SQL | Injection, correctness, and permission risk |
| Vendor-hosted agent only | Lock-in; no local/self-host option |
| No tools (RAG only) | Cannot answer spatial questions or act on the map |

## Review trigger

Revisit if the Rust provider ecosystem cannot support required tool-calling/streaming features, or if a Python-only model integration becomes essential.
