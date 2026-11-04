# ADR-0013: Demo-first auth and single-tenant tenancy model

- **Status:** Proposed
- **Date:** 2026-11-04
- **Deciders:** Engineering
- **Related:** [SECURITY](../SECURITY.md), [API](../API.md)

## Context

The first release is an internal demo, not a public SaaS. Building SSO, SCIM, billing, and multi-tenant isolation now would delay the demo. But the data model must not need a rewrite when the product becomes multi-tenant.

## Decision

- **Demo:** local seeded accounts, session cookies, `DEMO_MODE=1`, no public sign-up.
- **Model:** every tenant-owned row carries `org_id`; roles are `owner/admin/member/viewer`; repository-level scoping with IDOR tests.
- **Production path:** OIDC authorization code + PKCE; SSO via the IdP; RLS as defence in depth before hosted multi-tenancy.
- **Do not** build billing, SCIM, or per-tenant schemas for the demo.

## Consequences

**Positive**

- Fast demo without weakening the domain model.
- Clear upgrade path to OIDC and multi-tenancy.
- Security tests can start early.

**Negative**

- Demo auth is not production-grade.
- RLS is deferred; application scoping must be rigorous.
- Some enterprise features wait.

**Mitigations**

- Hard-block demo auth outside `DEMO_MODE`.
- Automated cross-org tests in CI.
- Add RLS and SSO before any pilot with real data.
- Document the production checklist.

## Alternatives considered

| Alternative | Why rejected |
|---|---|
| Build full multi-tenancy now | Delays the demo; no validated demand |
| Shared single user | No audit/roles; unsafe |
| Auth0/Clerk from day one | Vendor dependency and cost before validation |
| No org model | Guarantees a painful migration |

## Review trigger

Before the first design-partner pilot with real data, or when a second organisation needs access.
