# ADR-0003: PostGIS as system of record; DuckDB for analytics

- **Status:** Proposed
- **Date:** 2026-11-04
- **Deciders:** Engineering
- **Related:** [DATA_MODEL](../DATA_MODEL.md), [GEOSPATIAL](../GEOSPATIAL.md)

## Context

Terrassay needs transactional project data, spatial indexes, joins between BIM records and GIS layers, and analytical scans over large GeoParquet files. A single database cannot be optimal for both.

## Decision

- **PostgreSQL + PostGIS** is the system of record for tenants, projects, connections, localization, issues, clashes, sheets, datasets, layers, jobs, conversations, and audit.
- **DuckDB Spatial** runs inside workers for large GeoParquet scans and analytical joins; results are written back as assets or small tables.
- **Object storage** holds originals and derived COG/PMTiles/3D Tiles; the database stores metadata and signed-URL references.
- All geometries are stored in EPSG:4326; analyses transform to the project's metric CRS.

## Consequences

**Positive**

- Transactions, constraints, and spatial indexes in one place.
- Mature ecosystem and operational tooling.
- DuckDB handles columnar analytics without loading Postgres.
- Clear provenance: metadata in Postgres, bytes in object storage.

**Negative**

- PostGIS is GPL-2.0-or-later; distribution needs counsel review.
- Two query engines increase cognitive load.
- Large raster/point-cloud data must not go through the database.

**Mitigations**

- Keep PostGIS as a separate service; do not link it into the application.
- Document when to use DuckDB vs PostGIS.
- Enforce asset-size limits and object-storage-first design.
- Legal review before distributing an appliance.

## Alternatives considered

| Alternative | Why rejected |
|---|---|
| DuckDB-only | Weak concurrent writes/transactions; not a multi-user system of record |
| Postgres without PostGIS | Loses spatial indexes/operators |
| SpatiaLite | Single-writer, not a server |
| MongoDB/Elasticsearch | Weaker spatial integrity and relational joins |

## Review trigger

Revisit if Postgres becomes the analytical bottleneck, or if a managed spatial warehouse changes the cost/performance balance.
