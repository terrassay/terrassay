# Phase 1 — Map Bridge and Project Intelligence

| | |
|---|---|
| **Status** | In progress — first vertical slice |
| **Goal** | Connect the shell to GeoLibre, place the project, sync mock data, and answer a grounded spatial question |
| **Related** | [PHASE0](PHASE0.md) · [BUILD_PLAN](BUILD_PLAN.md) |

---

## 1. What landed in this increment

- **GeoLibre map bridge** using `@geolibre/embed`:
  - connects to the local map host;
  - fits the project bbox;
  - adds site, flood, buildings, and issue layers;
  - layer visibility toggles;
  - map selection → selected issue;
  - issue click → fly to the issue and highlight it.
- **Localization record** exposed by the API and displayed in the shell:
  - method, source/target CRS, transform, accuracy, confidence, verified.
- **Mock sync job**:
  - POST `/api/v1/projects/:id/sync` queues a `connector.sync`;
  - the worker claims it, records counts, updates `last_synced_at`, and completes.
- **Project data layers**:
  - site boundary, flood hazard, six synthetic buildings;
  - 240 issues as GeoJSON;
  - layers list from the database.
- **Deterministic assistant**:
  - answers summaries and flood-zone critical issue questions with PostGIS;
  - returns citations and a map highlight action;
  - explicitly labelled `deterministic-tools` — no external LLM yet.

## 2. API surface added

```text
GET  /api/v1/projects/:id/layers
GET  /api/v1/projects/:id/localization
GET  /api/v1/projects/:id/features/:layerKey
GET  /api/v1/projects/:id/issues.geojson
POST /api/v1/projects/:id/sync
POST /api/v1/projects/:id/assistant/ask
```

## 3. Verified

- Localization returns a verified control-point record with 0.18 m RMS.
- Layer list returns six layers.
- Site/flood/buildings features return valid GeoJSON.
- Issues GeoJSON returns 240 features.
- Assistant returns **47 critical open issues** intersecting the seeded flood zone, with citations.
- Sync job moves `queued → running → succeeded`, records `240 issues / 3 clashes / 12 sheets`, and updates the project timestamp.
- Map bridge connects and registers the four layers in GeoLibre.
- Frontend builds and typechecks; Rust tests pass.

The exact assistant count is data-dependent; the demo must not hardcode “7”. Use the returned count and citations.

## 4. Known limitations

- The assistant is deterministic, not an LLM. It handles summaries and flood-zone questions only.
- Map host is the official GeoLibre image at `latest`; pin it before release.
- The default GeoLibre globe can take several seconds to show imagery; it is blank while loading.
- No uploads, analyses, 3D massing, or write actions yet.
- Local verification used debug images; release Dockerfiles remain the source of truth and build in CI.

## 5. Next increment

1. Real map style/basemap configuration and 3D massing.
2. Project localization edit workflow (map pick + control points).
3. Dataset upload and layer ingestion.
4. Typed assistant tool registry with an LLM provider and evals.
5. Replace the debug local images with release images.
