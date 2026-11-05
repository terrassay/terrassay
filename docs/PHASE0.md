# Phase 0 — Bootstrap and Validation

| | |
|---|---|
| **Status** | In progress |
| **Goal** | A runnable foundation and a proven map/connector boundary |
| **Related** | [BUILD_PLAN](BUILD_PLAN.md) · [ROADMAP](ROADMAP.md) |

---

## 1. Scope

Phase 0 proves the foundation before the demo features are built:

- Monorepo and CI.
- Rust API with health and read-only project endpoints.
- PostGIS schema and deterministic synthetic seed.
- Mock Revizto connector behind a `BimConnector` trait.
- React shell that reads the API.
- GeoLibre map host via the official image, plus the embed spike checklist.
- Docker Compose stack.
- No authentication, uploads, AI, or writes yet.

## 2. What exists

```text
apps/web/                 React + Vite shell
apps/map-host/            GeoLibre host notes and spike checklist
services/api/             Axum API
services/worker/          Worker skeleton
crates/domain/            Shared API types
crates/db/                Postgres connection + migrations
crates/connectors/        BimConnector trait + MockRevizto
crates/geo/               CRS/georeferencing boundary
fixtures/demo/seed.sql    240 synthetic issues, 3 clashes, 12 sheets
infrastructure/compose/   Docker Compose stack
```

## 3. Acceptance criteria

- [ ] `docker compose -f infrastructure/compose/compose.yaml up --build` starts.
- [ ] `GET /healthz` returns 200.
- [ ] `GET /readyz` returns 200 after Postgres is ready.
- [ ] `GET /api/v1/projects` returns Hafenquartier Zürich.
- [ ] `GET /api/v1/projects/:id/summary` returns 240 issues, 60 critical, 3 clashes, 12 sheets.
- [ ] `GET /api/v1/projects/:id/issues?limit=12` returns critical issues first.
- [ ] Web shell renders the project, stats, issue list, and map iframe.
- [ ] Mock connector tests pass.
- [ ] GeoLibre embed spike checklist is completed or gaps are documented.

## 4. GeoLibre spike

The map is embedded as an iframe in Phase 0. The spike must prove:

1. `@geolibre/embed` connects to the local map host.
2. `setView` changes the camera.
3. `addLayer` accepts a GeoJSON URL.
4. `setFilter` filters an issue layer.
5. `highlight` highlights selected features.
6. `selectionChanged` reaches the shell.

If a command is unavailable, document it and build a first-party plugin inside
the map host. Do not fork GeoLibre for Phase 0.

## 5. Known limitations

- The API is read-only and unauthenticated; this is a local demo only.
- The worker is a heartbeat; the job queue arrives in Phase 1.
- The map host is the official GeoLibre image; `latest` must be pinned before release.
- The seed is synthetic and not a Revizto export.
- No AI assistant yet; the panel is a stub.

## 6. Next

Phase 1 adds the map bridge, localization, mock sync, dataset upload, assistant
tools, and 3D massing on top of this scaffold.
