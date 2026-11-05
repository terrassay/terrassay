# Security

| | |
|---|---|
| **Status** | Proposed |
| **Scope** | Demo, self-hosted beta, development automation |
| **Related** | [ARCHITECTURE](ARCHITECTURE.md) · [AI_ASSISTANT](AI_ASSISTANT.md) · [LEGAL_COMPLIANCE](LEGAL_COMPLIANCE.md) |

---

## 1. Security goals

- Project data, vendor tokens, and credentials stay inside the deployment.
- A user can only see projects in their organisation.
- A compromised map iframe cannot read API credentials or call arbitrary backend functions.
- An AI prompt cannot escalate permissions or exfiltrate data.
- A malicious upload cannot execute code, reach internal networks, or exhaust the service.
- A supply-chain compromise is detectable and contained.
- Every privileged action is auditable.

---

## 2. Threat model (summary)

| Threat | Vector | Impact | Controls |
|---|---|---|---|
| Token theft | Logs, browser storage, misconfigured CORS | Account/project access | Server-side tokens, encryption, no tokens in JS, CSP, short sessions |
| IDOR / cross-tenant access | Guessing IDs, missing checks | Data breach | `org_id` scoping, repository tests, 404 on mismatch, RLS later |
| SSRF | URL-based dataset import, webhooks | Internal network access | URL allowlist, block private/link-local/metadata IPs, no redirects to private |
| Malicious upload | Crafted GeoJSON/GeoTIFF/zip | RCE, DoS | Type/size limits, parser isolation, no shell, sandboxed worker, ClamAV optional |
| Prompt injection | Issue text, dataset metadata | Data exfiltration, wrong actions | Typed tools, no arbitrary HTTP/SQL, scope injection, confirmation for writes |
| XSS | GeoJSON properties, comments, names | Session theft | Output encoding, sanitization, CSP, no `dangerouslySetInnerHTML` |
| CSRF | State-changing requests | Unauthorized actions | SameSite cookies, CSRF token, origin checks |
| Map iframe abuse | Untrusted host origin | Command injection | Origin allowlist, capability gating, narrow bridge |
| Supply chain | npm/crates/PyPI compromise | RCE, data theft | Lockfiles, pinned versions, SCA, SBOM, provenance, minimal dependencies |
| Secret leakage | Git, logs, errors | Credential compromise | Secret scanning, redaction, vault, no secrets in env examples |
| DoS | Large files, expensive queries, AI loops | Outage/cost | Limits, quotas, timeouts, job budgets, rate limits |
| Insider misuse | A maintainer account or local machine is compromised | Repo compromise | Least privilege, branch protection, no auto-merge, audit |
| Backup theft | Unencrypted backups | Data breach | Encryption at rest, access control, restore tests |

---

## 3. Authentication and sessions

- Production: OIDC authorization code + PKCE with a trusted IdP (Authentik/Keycloak/Entra/Okta).
- Demo: local login only when `DEMO_MODE=1`; seeded accounts; clearly labelled.
- Passwords (if used): Argon2id, minimum length 12, breach-list check, no rotation mandate.
- Sessions: opaque random ID, HttpOnly, Secure, SameSite=Lax, 8-hour idle / 7-day absolute, rotated on privilege change.
- CSRF: double-submit token; origin/referer checks for state-changing requests.
- Logout invalidates the server-side session; "log out everywhere" is available.
- Failed logins are rate-limited and audited; no user enumeration.
- 2FA is required for admins in hosted deployments; OIDC MFA is delegated to the IdP.

---

## 4. Authorization and tenancy

- Every tenant table has `org_id`; every query is scoped by the authenticated membership.
- Project access is checked in the service layer, not the handler only.
- Cross-org requests return 404 to avoid enumeration.
- Roles: `owner`, `admin`, `member`, `viewer`; destructive actions require `admin`+.
- Connector tokens are bound to the connection and org; a token cannot be used by another org.
- Integration tests cover IDOR for projects, datasets, layers, jobs, conversations, and audit.
- Before hosted multi-tenancy: PostgreSQL RLS as defence in depth; per-tenant encryption keys for object storage (future).

---

## 5. Secrets and token handling

| Secret | Storage | Rotation |
|---|---|---|
| OAuth client secret | Environment/KMS | On incident; yearly |
| Connection access/refresh tokens | Encrypted in `connection_secrets` | Refresh flow; revoke on disconnect |
| Session signing key | Environment/KMS | Quarterly |
| Encryption key (KEK) | KMS/environment, not in DB | Yearly with key versioning |
| AI provider key | Environment/KMS | Quarterly |
| SMTP/webhook secrets | Environment/KMS | Quarterly |
| Backup encryption key | KMS, separate from DB | Yearly |

Rules:

- AES-256-GCM or XChaCha20-Poly1305 with a unique nonce; associated data includes connection ID and key version.
- No tokens in logs, traces, error messages, or browser storage.
- Redact `authorization`, `cookie`, `set-cookie`, `token`, `secret`, `password` fields.
- Debug mode never dumps full prompts or headers by default.
- `.env.example` contains placeholders only; CI blocks real-looking secrets.

---

## 6. Network and SSRF

The API may fetch user-supplied URLs for dataset import. This is a classic SSRF risk.

Controls:

- Allowlist schemes: `https` only (and `http` only for localhost in dev).
- Resolve DNS and block private, loopback, link-local, multicast, and metadata ranges (169.254.169.254, fd00::/8, etc.).
- Re-check after redirects; limit redirects to 2 and re-validate each hop.
- Timeout 30 s; max download size; content-type allowlist.
- Do not forward cookies or authorization headers to user URLs.
- Run URL fetches in a worker with egress restrictions.
- Log the URL, resolved IP, and outcome (not the content).

In self-hosted/air-gapped mode, outbound fetches can be disabled entirely.

---

## 7. Upload and parser security

- Presigned uploads are scoped to one object and one content type; size enforced by the storage policy.
- Validate magic bytes, not just `Content-Type`/extension.
- Reject archives with path traversal, symlinks, or excessive file counts.
- Parse in a worker with CPU/memory/time limits, not in the API process.
- GDAL/PDAL run with restricted options; no `/vsicurl/`, `/vsizip/` recursion, or arbitrary drivers unless allowlisted.
- Do not execute uploaded content; no macros, no scripts, no HTML rendering.
- Store originals in a quarantine prefix until validation passes.
- Optional antivirus scan in hosted deployments.
- Delete rejected uploads and audit the rejection.

---

## 8. AI security

- **Scope injection:** `org_id`/`project_id` come from the session, never the model or tool args.
- **Typed tools only:** no raw SQL, shell, file read, or arbitrary HTTP.
- **Allowlisted predicates:** spatial query builders use fixed operators and parameterized values.
- **Result budgets:** row/byte limits, timeout, and truncation.
- **Write actions:** disabled by default; explicit confirmation with a diff; audit.
- **Prompt injection:** project text is untrusted; system prompt says so; tools are the only side effects.
- **Data minimization:** send only the context needed; redact PII where possible.
- **Provider controls:** DPA, no-training, retention policy, region; local model option.
- **Audit:** tool name, args hash, row count, model, tokens, latency; full prompts only in a short-lived debug mode with consent.
- **Cost controls:** per-user and per-org budgets; circuit breaker on anomalies.
- **Evals:** injection and exfiltration cases in the golden set.

---

## 9. Frontend and iframe security

- CSP: `default-src 'self'`; explicit `connect-src` for API, storage, basemap, and model streaming; `frame-src 'self'`; `frame-ancestors 'self'`; no `unsafe-eval` unless a tested dependency requires it (documented).
- GeoLibre map host is same-origin under `/map/`; embed origins are exact.
- The bridge validates `event.origin`, message shape, and request IDs; unknown messages are ignored.
- Deployment capabilities limit the iframe to `data:add` and `export:data`; no project editing, no processing.
- No tokens in `localStorage`; session cookie only.
- All user-controlled text is rendered as text; markdown is sanitized; links use `rel="noopener noreferrer"`.
- File uploads are not previewed as HTML/SVG.
- Third-party scripts are avoided; if added, pinned with SRI and reviewed.

---

## 11. Supply chain

- Lockfiles committed; CI uses `--frozen-lockfile` / `--locked`.
- Pinned container images by digest; Dependabot/Renovate with review.
- `cargo-deny` (licences, advisories, bans, sources).
- `npm audit`/`pnpm audit`, `pip-audit`.
- SBOM (CycloneDX) per release.
- Secret scanning + push protection.
- Signed releases/checksums; provenance attestations where practical.
- Minimal base images; non-root containers; read-only root filesystem where possible.
- No `curl | sh` in build scripts; no unreviewed postinstall scripts.

---

## 12. Audit and monitoring

Audit events (append-only):

- Login/logout, failed login, role change.
- Connection create/revoke, sync start/finish/failure, token refresh failure.
- Project create/archive, localization change/verify, control point change.
- Dataset upload/delete, layer change, analysis run.
- Assistant tool call, write confirmation, conversation delete.
- Audit export, settings change, API key create/revoke.

Logs:

- Structured JSON, request ID, org/project/user IDs, route, status, latency.
- No tokens, passwords, full prompts, or file contents.
- Retention 30 days hot, 12 months cold (configurable).
- Alerts: 5xx spike, auth failure spike, job failure rate, queue age, storage errors, AI cost anomaly.

Metrics:

- `http_request_duration_seconds`, `jobs_total{status}`, `job_duration_seconds`, `sync_records_total`, `tile_cache_hits_total`, `ai_tokens_total`, `ai_tool_calls_total`.

---

## 13. Backups and recovery

- Nightly `pg_dump` + WAL archiving for production; daily object-store snapshot.
- Encrypted at rest; separate credentials from the application.
- Restore tested monthly; document RPO/RTO:
  - Demo: RPO 24 h, RTO 4 h.
  - Beta: RPO 1 h, RTO 2 h.
- Backups stored in a different failure domain.
- Deletion requests propagate to backups within the retention window.

---

## 14. Incident response

1. **Detect:** alert, report, or audit anomaly.
2. **Contain:** revoke tokens, disable the connection, pause the worker, isolate the host.
3. **Assess:** scope, data affected, legal/notification obligations.
4. **Eradicate:** patch, rotate secrets, rebuild from a clean image.
5. **Recover:** restore, verify, monitor.
6. **Notify:** affected users/customers and regulators if required; counsel involved.
7. **Learn:** blameless post-mortem, action items with owners and dates.

Contacts and severity levels are maintained in the private runbook. A public `SECURITY.md` explains private reporting and a 90-day disclosure target.

---

## 15. Pre-pilot security checklist

- [ ] Threat model reviewed with a second engineer.
- [ ] IDOR tests for every tenant resource.
- [ ] SSRF tests for URL import (private IPs, redirects, metadata).
- [ ] Upload fuzzing for GeoJSON/GeoTIFF/zip.
- [ ] CSP and iframe origin checks verified in a browser.
- [ ] Token encryption and rotation tested.
- [ ] Dependency audit and SBOM generated.
- [ ] Secret scan of full history.
- [ ] Backup/restore tested.
- [ ] Pen test (external) with critical/high findings fixed.
- [ ] Incident runbook and contacts current.
- [ ] AI injection/exfiltration evals pass.

No pilot with real data until this checklist is green.
