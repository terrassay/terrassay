# API Contract

| | |
|---|---|
| **Status** | Proposed |
| **Base path** | `/api/v1` |
| **Format** | JSON; `application/problem+json` for errors |
| **Related** | [ARCHITECTURE](ARCHITECTURE.md) · [DATA_MODEL](DATA_MODEL.md) · [SECURITY](SECURITY.md) |

---

## 1. Conventions

| Concern | Rule |
|---|---|
| IDs | UUIDv7 strings |
| Field names | `camelCase` in JSON, `snake_case` in the database |
| Timestamps | ISO 8601 UTC, e.g. `2026-11-04T13:21:00Z` |
| Geometry | GeoJSON in API payloads; WGS84 unless a CRS is explicit |
| Pagination | Cursor: `?limit=50&cursor=...`; response `{ "data": [...], "nextCursor": "..." }` |
| Filtering | Explicit query params (`status`, `priority`, `bbox`, `q`), not a query language |
| Sorting | `sort=createdAt:desc`; allowlisted fields only |
| Idempotency | `Idempotency-Key` on job-creating POSTs; 24-hour window |
| Concurrency | `ETag` + `If-Match` on mutable project resources |
| Versioning | `/api/v1`; additive changes preferred; deprecation ≥ 2 minor releases |
| Errors | RFC 9457 problem details with a stable `code` |
| Request ID | `X-Request-Id` echoed; included in logs |
| Rate limits | 429 with `Retry-After` and `X-RateLimit-*` headers |

### Error shape

```json
{
  "type": "https://terrassay.dev/problems/invalid-crs",
  "title": "Unsupported coordinate reference system",
  "status": 422,
  "detail": "EPSG:9999 is not a known CRS.",
  "code": "invalid_crs",
  "requestId": "01J..."
}
```

---

## 2. Authentication and authorization

### Browser sessions

- `POST /auth/login` (demo only) sets an HttpOnly, Secure, SameSite=Lax session cookie.
- Production uses OIDC authorization code + PKCE:
  - `GET /auth/oidc/start`
  - `GET /auth/oidc/callback`
  - `POST /auth/logout`
- CSRF: double-submit token for state-changing requests; `SameSite=Lax` is defence in depth.
- Session lifetime: 8 hours idle, 7 days absolute; refresh rotates the session ID.

### API keys (automation, later)

- `Authorization: Bearer terrassay_...`
- Scoped to an org, hashed at rest, expiry enforced, last-used recorded.
- Not available in the demo.

### Authorization

- Every request resolves `user -> memberships -> org`.
- Roles: `owner`, `admin`, `member`, `viewer`.
- Project access is derived from org membership plus explicit project grants (future).
- The API never trusts `orgId` or `projectId` from the client without a membership check.
- Cross-org access returns **404**, not 403, to avoid resource enumeration.

---

## 3. Endpoint catalog

### 3.1 Auth and session

| Method | Path | Purpose |
|---|---|---|
| POST | `/auth/login` | Demo login (disabled outside `DEMO_MODE`) |
| POST | `/auth/logout` | End session |
| GET | `/auth/me` | Current user, orgs, roles |
| GET | `/auth/oidc/start` | Start OIDC flow |
| GET | `/auth/oidc/callback` | OIDC callback |
| GET | `/auth/csrf` | Issue CSRF token |

### 3.2 Projects

| Method | Path | Purpose |
|---|---|---|
| GET | `/projects` | List projects visible to the user |
| POST | `/projects` | Create project |
| GET | `/projects/{projectId}` | Project detail |
| PATCH | `/projects/{projectId}` | Update metadata |
| DELETE | `/projects/{projectId}` | Archive (soft delete) |
| GET | `/projects/{projectId}/summary` | Counts, freshness, localization status |
| GET | `/projects/{projectId}/saved-views` | List saved views |
| POST | `/projects/{projectId}/saved-views` | Save camera + layer state |

### 3.3 Localization

| Method | Path | Purpose |
|---|---|---|
| GET | `/projects/{projectId}/localization` | Current localization + history summary |
| PUT | `/projects/{projectId}/localization` | Create/replace localization |
| POST | `/projects/{projectId}/localization/solve` | Solve from control points (no save) |
| GET | `/projects/{projectId}/control-points` | List |
| POST | `/projects/{projectId}/control-points` | Add |
| DELETE | `/projects/{projectId}/control-points/{id}` | Remove |
| POST | `/projects/{projectId}/localization/verify` | Mark verified (admin) |

### 3.4 Connections and connectors

| Method | Path | Purpose |
|---|---|---|
| GET | `/connections` | List connections (no secrets) |
| GET | `/connections/{id}` | Connection detail + health |
| DELETE | `/connections/{id}` | Disconnect, revoke, schedule purge |
| POST | `/connections/{id}/sync` | Trigger sync |
| GET | `/connections/{id}/runs` | Sync history |
| POST | `/connectors/revizto/oauth/start` | Build authorize URL + state |
| GET | `/connectors/revizto/oauth/callback` | Exchange code, create connection |
| GET | `/connectors/revizto/regions` | Supported regions |
| GET | `/connections/{id}/external-projects` | Remote project list |
| POST | `/projects/{projectId}/link` | Link project to external project |
| DELETE | `/projects/{projectId}/link` | Unlink |

### 3.5 Datasets and layers

| Method | Path | Purpose |
|---|---|---|
| GET | `/projects/{projectId}/datasets` | List |
| POST | `/projects/{projectId}/datasets/upload-url` | Presigned upload |
| POST | `/projects/{projectId}/datasets/{datasetId}/finalize` | Validate + enqueue ingest |
| GET | `/datasets/{datasetId}` | Metadata, assets, provenance |
| DELETE | `/datasets/{datasetId}` | Delete dataset + assets |
| GET | `/projects/{projectId}/layers` | List |
| POST | `/projects/{projectId}/layers` | Create layer |
| PATCH | `/layers/{layerId}` | Style, visibility, order, filter |
| DELETE | `/layers/{layerId}` | Delete |
| GET | `/layers/{layerId}/tiles/{z}/{x}/{y}.mvt` | Vector tile |
| GET | `/projects/{projectId}/features` | GeoJSON features (bounded) |

### 3.6 BIM records

| Method | Path | Purpose |
|---|---|---|
| GET | `/projects/{projectId}/issues` | Filter by status, priority, discipline, bbox, q |
| GET | `/projects/{projectId}/issues/{issueId}` | Detail |
| GET | `/projects/{projectId}/issues/aggregate` | Counts by group |
| GET | `/projects/{projectId}/clashes` | List |
| GET | `/projects/{projectId}/sheets` | List |
| GET | `/projects/{projectId}/sheets/{sheetId}` | Detail |

### 3.7 Jobs and events

| Method | Path | Purpose |
|---|---|---|
| GET | `/jobs/{jobId}` | Status, progress, error |
| GET | `/projects/{projectId}/jobs` | Recent jobs |
| POST | `/jobs/{jobId}/cancel` | Request cancellation |
| GET | `/projects/{projectId}/events` | SSE: job + sync events |

### 3.8 Analyses

| Method | Path | Purpose |
|---|---|---|
| POST | `/projects/{projectId}/analyses` | Run an allowlisted analysis |
| GET | `/projects/{projectId}/analyses` | History |
| GET | `/analyses/{analysisId}` | Result summary + asset |
| GET | `/analyses/{analysisId}/result` | GeoJSON / signed asset URL |

### 3.9 Assistant

| Method | Path | Purpose |
|---|---|---|
| POST | `/projects/{projectId}/assistant/conversations` | Create conversation |
| GET | `/projects/{projectId}/assistant/conversations` | List |
| GET | `/assistant/conversations/{id}` | Messages + citations |
| POST | `/assistant/conversations/{id}/messages` | Send message; SSE response |
| DELETE | `/assistant/conversations/{id}` | Delete |
| GET | `/projects/{projectId}/assistant/suggestions` | Suggested questions |

### 3.10 Admin and ops

| Method | Path | Purpose |
|---|---|---|
| GET | `/orgs/{orgId}/members` | Members and roles |
| PATCH | `/orgs/{orgId}/members/{userId}` | Change role |
| GET | `/orgs/{orgId}/audit-events` | Audit log (paginated) |
| GET | `/healthz` | Liveness |
| GET | `/readyz` | DB + object store readiness |
| GET | `/metrics` | Prometheus metrics (internal) |
| GET | `/openapi.json` | OpenAPI 3.1 |
| POST | `/mcp` | MCP endpoint (later) |

---

## 4. Examples

### 4.1 Create a project

```http
POST /api/v1/projects
Content-Type: application/json
Idempotency-Key: 9f2c...

{
  "name": "Hafenquartier Zürich",
  "code": "HQZ-2026",
  "country": "CH",
  "timezone": "Europe/Zurich",
  "address": "Zürich West, Switzerland",
  "analysisCrs": "EPSG:2056"
}
```

Response `201 Created`:

```json
{
  "id": "018f...",
  "orgId": "018e...",
  "name": "Hafenquartier Zürich",
  "code": "HQZ-2026",
  "status": "active",
  "analysisCrs": "EPSG:2056",
  "localization": null,
  "createdAt": "2026-11-04T13:21:00Z"
}
```

### 4.2 Set localization from control points

```http
PUT /api/v1/projects/{projectId}/localization
Content-Type: application/json

{
  "method": "control_points",
  "sourceCrs": "LOCAL:PROJECT",
  "targetCrs": "EPSG:2056",
  "controlPoints": [
    { "label": "CP-01", "modelX": 0, "modelY": 0, "modelZ": 412.2, "lon": 8.51701, "lat": 47.38452, "height": 412.1 },
    { "label": "CP-02", "modelX": 182.4, "modelY": 0, "modelZ": 411.8, "lon": 8.51921, "lat": 47.38450, "height": 411.9 },
    { "label": "CP-03", "modelX": 0, "modelY": 121.7, "modelZ": 412.5, "lon": 8.51703, "lat": 47.38561, "height": 412.4 }
  ]
}
```

Response:

```json
{
  "method": "control_points",
  "sourceCrs": "LOCAL:PROJECT",
  "targetCrs": "EPSG:2056",
  "transform": {
    "type": "helmert2d",
    "tx": 2683120.41,
    "ty": 1247310.87,
    "rotationDeg": 12.4,
    "scale": 1.000021,
    "rms": 0.18
  },
  "accuracyM": 0.18,
  "confidence": "calculated",
  "verified": false
}
```

A separate `POST /verify` with an admin role promotes it to `verified` and writes an audit event.

### 4.3 Upload and finalize a dataset

```http
POST /api/v1/projects/{projectId}/datasets/upload-url
Content-Type: application/json

{
  "filename": "flood-hazard.geojson",
  "contentType": "application/geo+json",
  "sizeBytes": 1843200
}
```

Response:

```json
{
  "datasetId": "018f...",
  "uploadUrl": "https://storage.example/...",
  "expiresAt": "2026-11-04T13:36:00Z",
  "method": "PUT",
  "headers": { "Content-Type": "application/geo+json" }
}
```

After the upload:

```http
POST /api/v1/projects/{projectId}/datasets/{datasetId}/finalize
Content-Type: application/json

{
  "name": "Flood hazard zone",
  "licence": "CC-BY-4.0",
  "attribution": "Kanton Zürich, fictionalized for demo"
}
```

Response `202 Accepted`:

```json
{
  "jobId": "018f...",
  "status": "queued",
  "links": { "job": "/api/v1/jobs/018f..." }
}
```

### 4.4 Trigger a sync

```http
POST /api/v1/connections/{id}/sync
Idempotency-Key: sync-2026-11-04
```

```json
{
  "jobId": "018f...",
  "status": "queued"
}
```

### 4.5 Ask the assistant (SSE)

```http
POST /api/v1/assistant/conversations/{id}/messages
Accept: text/event-stream
Content-Type: application/json

{
  "content": "Which critical issues are inside the flood zone?"
}
```

Event stream:

```text
event: message.start
data: {"messageId":"018f..."}

event: tool.call
data: {"name":"spatial_query","arguments":{"layer":"flood-hazard","predicate":"intersects","filters":{"priority":"critical"}}}

event: tool.result
data: {"count":7,"featureIds":["...","..."],"citations":[{"type":"dataset","id":"..."}]}

event: message.delta
data: {"text":"Seven critical issues fall inside the flood zone."}

event: citation
data: {"type":"issue","id":"...","label":"ISSUE-142"}

event: map.action
data: {"action":"highlight","layerId":"issues","featureIds":["..."]}

event: message.done
data: {"messageId":"018f...","tokensIn":1200,"tokensOut":180}
```

Errors are sent as `event: error` with a problem-details payload, followed by `message.done`.

---

## 5. Job model

```json
{
  "id": "018f...",
  "type": "connector.sync",
  "status": "running",
  "progress": 42,
  "attempts": 1,
  "maxAttempts": 3,
  "createdAt": "2026-11-04T13:21:00Z",
  "startedAt": "2026-11-04T13:21:02Z",
  "finishedAt": null,
  "error": null
}
```

States: `queued → running → succeeded | failed | cancelled`. A failed job with attempts remaining returns to `queued` with `runAt` set by exponential backoff.

SSE event types:

| Event | Payload |
|---|---|
| `job.queued` | `{ jobId, type }` |
| `job.started` | `{ jobId, type }` |
| `job.progress` | `{ jobId, progress, message? }` |
| `job.succeeded` | `{ jobId, result? }` |
| `job.failed` | `{ jobId, error }` |
| `sync.completed` | `{ connectionId, counts }` |

Clients reconnect with `Last-Event-ID`; the server replays from a short in-memory buffer and otherwise sends a snapshot.

---

## 6. Map bridge protocol

The shell embeds GeoLibre under `/map/` and communicates with `@geolibre/embed`. The public protocol is GeoLibre's; our wrapper adds:

### Host → map

| Command | Payload | Notes |
|---|---|---|
| `setView` | `{center, zoom, pitch?, bearing?}` | Animated; resolves when settled |
| `fitBounds` | `[west, south, east, north]` | Padding option |
| `addLayer` | `{id, name, type, source, style, metadata}` | Source is a signed URL or inline GeoJSON |
| `removeLayer` | `{id}` | Idempotent |
| `setVisibility` | `{id, visible}` | |
| `setFilter` | `{id, expression}` | MapLibre expression, validated |
| `highlight` | `{layerId, featureIds}` | Clears previous highlight |
| `clearHighlight` | `{}` | |
| `getViewport` | `{}` | Returns bbox + zoom |
| `exportImage` | `{width?, height?}` | Returns a data URL |

### Map → host

| Event | Payload |
|---|---|
| `ready` | `{version}` |
| `selectionChanged` | `{layerId, featureIds}` |
| `viewChanged` | `{center, zoom, pitch, bearing, bbox}` |
| `projectLoaded` | `{projectId}` |
| `error` | `{code, message}` |

Security:

- The iframe origin is allowlisted in GeoLibre's `deployment.json`.
- The host validates `event.origin` and ignores unknown message shapes.
- Capabilities are restricted to `data:add` and `export:data`; `project:edit` and `processing:run` are denied.
- Asset URLs are short-lived and scoped; no vendor tokens reach the iframe.

---

## 7. Rate limits and quotas

| Scope | Limit (demo) | Behaviour |
|---|---|---|
| Authenticated API | 120 requests/min/user | 429 + `Retry-After` |
| Login | 10 attempts/15 min/IP | Lockout + audit |
| Upload URL | 20/hour/project | 429 |
| Sync trigger | 1 per connection per 5 min | 409 with last run |
| Assistant | 20 messages/hour/user | 429; token budget |
| Tile | 600 requests/min/user | 429; CDN cache in production |
| Export | 10/hour/user | 429 |

Limits are configurable per deployment. Self-hosted defaults are generous; hosted limits are plan-based.

---

## 8. OpenAPI and generated clients

- The Rust API derives OpenAPI 3.1 with `utoipa`.
- CI publishes `openapi.json` as an artefact and fails if it changes without a corresponding client update.
- The TypeScript client is generated with `openapi-typescript`; runtime validation uses Zod at the boundary.
- Breaking changes require a new path version or a deprecation window.

---

## 9. Webhooks (future)

No Revizto webhooks are documented. If a future connector supports them:

- Verify signatures with a per-connection secret.
- Store the raw event before processing.
- Process idempotently by event ID.
- Return 2xx quickly; enqueue the work.
- Provide a replay endpoint for admins.

---

## 10. MCP (future)

A `/mcp` endpoint will expose a read-only subset of tools to external assistants:

- `list_projects`, `get_project_summary`, `search_issues`, `spatial_query`, `list_layers`.
- OAuth or API-key auth; scoped to one org.
- No writes in v1.
- Audited like any other tool call.

This is separate from the Revizto MCP server; Terrassay does not proxy or impersonate it.
