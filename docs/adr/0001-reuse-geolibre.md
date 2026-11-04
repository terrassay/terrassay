# ADR-0001: Reuse GeoLibre as the map surface

- **Status:** Accepted
- **Date:** 2026-11-04
- **Deciders:** Maintainer
- **Related:** [BUILD_PLAN §3.1](../BUILD_PLAN.md#31-geolibre-already-covers-most-of-the-generic-gis-surface), [ARCHITECTURE §3.2](../ARCHITECTURE.md)

## Context

Terrassay needs a map, 3D globe, layer management, styling, COG/PMTiles/point-cloud rendering, and geospatial tooling. Building that from scratch would take months and would not differentiate the product.

GeoLibre is an MIT-licensed, React-based GIS workspace with MapLibre, Cesium, deck.gl, DuckDB-WASM, a plugin API, a typed embed client, and a self-hosting story. It already covers most of the generic GIS surface. Its embed API exposes layer add/remove, filters, camera, selection events, screenshots, and feature reads, gated by an origin allowlist and deployment capabilities.

## Decision

Embed GeoLibre as the map and analysis surface. Build a separate **BIM shell** that owns projects, connectors, georeferencing, analyses, AI, and audit. Do not fork GeoLibre for the MVP.

Integration rules:

- Use the official GeoLibre container image and the published `@geolibre/embed` package, pinned to an exact version.
- Serve the map host same-origin under `/map/` to avoid cross-origin cookie/CORS problems.
- Allowlist only the shell origin in `deployment.json`.
- Grant only `data:add` and `export:data`; deny `project:edit` and `processing:run`.
- Keep the shell as the source of truth for layers and view; re-apply state on reconnect.
- If a required command is missing, build a first-party `terrassay-bridge` plugin inside the map host rather than forking core.

## Consequences

**Positive**

- Months of generic GIS work avoided.
- Mature rendering, formats, and self-hosting.
- Clear boundary; either side can be upgraded.
- Upstream contributions possible.

**Negative**

- Version coupling and upgrade testing.
- Embed API gaps may force a plugin or a fork.
- GeoLibre branding/notices must be preserved.
- Same-origin deployment constrains hosting topology.

**Mitigations**

- Three-day spike in week 1 proves add/filter/highlight/selection/signed COG.
- Contract tests around the bridge.
- Fallback: custom map-host plugin; fork only as a last resort.
- Pin the image digest and test upgrades each minor release.

## Alternatives considered

| Alternative | Why rejected |
|---|---|
| Fork GeoLibre | Inherits a large codebase and upstream merge burden before product-market fit |
| Build from scratch | 6–12 months of undifferentiated work |
| Plugin-only | Product UX and project/auth model would be constrained by GeoLibre's shell |
| Another GIS SDK | Would not solve the workspace, formats, or self-hosting problem |

## Review trigger

Revisit if the embed API cannot support two or more required demo interactions after the plugin fallback, or if GeoLibre's licence or maintenance changes.
