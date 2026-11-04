# Geospatial Design

| | |
|---|---|
| **Status** | Proposed |
| **Scope** | CRS policy, georeferencing, formats, tiling, analyses |
| **Related** | [DATA_MODEL](DATA_MODEL.md) · [ARCHITECTURE](ARCHITECTURE.md) · [AI_ASSISTANT](AI_ASSISTANT.md) |

---

## 1. CRS policy

| Layer | Rule |
|---|---|
| Storage | EPSG:4326 (WGS84) for all geometries; Z where available |
| Display | MapLibre/GeoLibre native (Web Mercator); Cesium geographic |
| Analysis | `projects.analysis_crs` — a metric CRS chosen per project |
| Input | CRS is required; never guessed from coordinates |
| Output | GeoJSON in 4326; GeoParquet with explicit CRS metadata |
| Transformation | PROJ, server-side; record source and target |

**Analysis CRS selection:** at project creation, derive a candidate from the site centroid (UTM zone, or a national grid such as EPSG:2056 for Switzerland). An admin can override it. All buffers, distances, areas, and slope calculations run in this CRS.

**Why not store in a projected CRS?** Projects span regions, datasets come from many sources, and web rendering expects 4326. A single storage CRS keeps joins and provenance simple; analysis transforms are explicit and testable.

**Degrees are never used for distance.** `ST_Distance(geography)` is acceptable for small, approximate UI readouts, but analyses use a projected CRS.

---

## 2. Vertical datum and units

- `height` values carry a `vertical_datum` field (`ellipsoidal`, `orthometric`, `local`, `unknown`).
- Do not mix ellipsoidal (GNSS) and orthometric (levelling) heights without a geoid model.
- Record the source units; convert to metres at ingest with PROJ or an explicit factor.
- Display the datum next to elevations; the UI never shows a bare number.
- For BIM, the project base point and true north are stored alongside the localization.

---

## 3. Georeferencing

A BIM model is usually authored in a local coordinate system. Georeferencing establishes the transform to the real world.

### 3.1 Methods, in order of confidence

| Method | Typical accuracy | Confidence | Use |
|---|---|---|---|
| API-provided georeference | Depends on source | `verified` if authoritative | When Revizto/IFC exposes a CRS and transform |
| Surveyed control points | 0.02–0.5 m | `verified` after review | Best practice for construction |
| Map pick + rotation/scale | 1–20 m | `indicative` | Quick demo, early site context |
| Address geocode | 10–100 m | `indicative` | Fallback only; never for analysis |

The UI labels each result and explains what it means. "Verified" requires an admin action and a note; it is not automatic.

### 3.2 2D Helmert (similarity) transform

Given model coordinates `(x, y)` and target coordinates `(E, N)`:

```text
E = tx + s * (x * cosθ - y * sinθ)
N = ty + s * (x * sinθ + y * cosθ)
```

With at least two control points, solve for `tx`, `ty`, `s`, and `θ` by least squares. Three or more points allow residual analysis.

- Store `tx`, `ty`, `rotation_deg`, `scale`, and the full 4×4 matrix.
- Report RMS residual and maximum residual.
- Reject a solution with fewer than 2 non-collinear points.
- Warn when residuals exceed a configurable threshold (default 0.5 m).
- Do not silently drop outliers; flag them and require a decision.

### 3.3 3D transform

For a full 3D alignment, use a 7-parameter Helmert (3 translations, 3 rotations, 1 scale). This is future work; the MVP uses 2D + a vertical offset.

### 3.4 Validation

- Transform the control points back to target coordinates and compare.
- Show a preview of the site boundary and a known landmark.
- Require confirmation before persisting.
- Store the verifier, timestamp, method, and residuals.
- Re-validate when the source CRS or control points change.

### 3.5 What we do not claim

Terrassay does not certify survey accuracy, legal boundaries, or construction tolerances. The accuracy estimate is decision support, not a survey deliverable. The UI says so.

---

## 4. Data ingest

### 4.1 Supported formats

| Kind | Formats | Conversion |
|---|---|---|
| Vector | GeoJSON, GeoParquet, FlatGeobuf, Shapefile (zip), GeoPackage, CSV (lon/lat), KML | GeoParquet + PMTiles (or MVT from PostGIS) |
| Raster | GeoTIFF, COG | COG (cloud-optimized, overviews) |
| Point cloud | LAS, LAZ, COPC | COPC (streaming) |
| Tiles | PMTiles, MBTiles, 3D Tiles | Serve as-is or repackage |
| Tables | CSV/Parquet with lon/lat | Vector points |
| Services | XYZ, WMS, WMTS, WFS | Reference, not copied |

### 4.2 Pipeline

```text
upload → validate → inspect (GDAL/PDAL) → reproject/convert → optimize → register → render
```

Validation:

- File signature and content type.
- Size and geometry count limits.
- CRS present and known; reject if missing.
- Geometry validity; bounds sanity.
- No executable content; no path traversal in archives.
- Virus scan optional in hosted deployments.

Optimization:

- Vector: GeoParquet with sorted spatial index; PMTiles for large layers.
- Raster: COG with internal overviews, compression, and nodata.
- Point cloud: COPC with an octree; reject whole-file LAS over the streaming limit.
- 3D Tiles: validate tileset.json and content; cap total size.

Naming and provenance:

- Content-addressed object keys: `sha256/ab/cd/<hash>`.
- Dataset record stores original filename, source, licence, attribution, and retrieval time.
- Derived assets link to the original.

### 4.3 Limits (MVP)

| Resource | Limit |
|---|---|
| Single upload | 2 GB |
| Vector features | 5 million |
| Raster size | 20 GB uncompressed |
| Point cloud | 50 million points (COPC) |
| 3D Tiles | 10 GB |
| Concurrent ingests | 2 per worker |

Limits are configurable; the API returns a clear error with the limit and the next step.

---

## 5. Tiling and serving

### 5.1 Vector tiles

- Generated by the API from PostGIS with `ST_AsMVT` and `ST_AsMVTGeom`.
- Tile extent 4096; buffer 256; simplify by zoom.
- Feature cap per tile (e.g., 50,000) with a warning layer/aggregate beyond it.
- Cache headers: `public, max-age=300` for project data; ETag.
- Authentication: session cookie or signed URL; no public bucket.

```sql
WITH mvtgeom AS (
  SELECT
    ST_AsMVTGeom(
      ST_Transform(i.location, 3857),
      ST_TileEnvelope($1, $2, $3),
      4096, 256, true
    ) AS geom,
    i.id, i.title, i.priority, i.status
  FROM core.issues i
  WHERE i.project_id = $4
    AND i.location && ST_Transform(ST_TileEnvelope($1, $2, $3), 4326)
)
SELECT ST_AsMVT(mvtgeom.*, 'issues', 4096, 'geom') FROM mvtgeom;
```

### 5.2 Raster tiles

- Prefer COG served directly from object storage with range requests.
- GeoLibre renders COG client-side; no tile server needed for the MVP.
- For very large rasters or WMS-only sources, use a tile proxy with caching.
- Band math and colormaps are client-side style, not baked tiles.

### 5.3 3D Tiles

- Served as static tilesets from object storage with signed URLs.
- The tileset must be self-contained or reference signed content.
- Cesium loads it via GeoLibre; no application-server proxying of tile content.
- Generate 3D Tiles from IFC/glTF in Phase 2, not in the MVP.

### 5.4 Basemaps

| Use | Source | Licence |
|---|---|---|
| Demo | OpenFreeMap / Protomaps / OSM | Attribution; check terms |
| Self-host | PMTiles archive | ODbL attribution for OSM-derived data |
| Air-gapped | Bundled PMTiles + local glyphs/sprites | Verify data licences |
| 3D terrain | Cesium Ion (optional) or generated quantized-mesh | Ion terms; local generation for self-host |

No Google/Bing basemaps without a commercial agreement. No keyless scraping of tile services.

---

## 6. Analysis catalog

### 6.1 MVP (demo)

| Analysis | Inputs | Output |
|---|---|---|
| `buffer_count` | Layer, distance, target layer | Count + buffer polygon |
| `intersect` | Layer A, Layer B | Intersecting features + area |
| `within` | Issues, site boundary | Issues inside |
| `nearest` | Source features, target layer, N | Nearest N + distance |
| `aggregate` | Issues, groupBy | Counts by group |
| `elevation_sample` | Points, DEM COG | Elevation per point |
| `slope` | DEM, AOI | Slope raster/statistics (Phase 2) |

### 6.2 Later

- Cut/fill volumes from design surface vs existing terrain.
- Viewshed / line-of-sight for site logistics.
- Flood extent overlay and depth sampling.
- Change detection from imagery pairs.
- Proximity to utilities, protected areas, transport.
- 4D schedule-linked progress (Phase 4).
- Clash-to-issue geolocation and density.

### 6.3 Contract

Every analysis:

- Has a typed parameter schema (JSON Schema).
- Runs as a job with progress and cancellation.
- Writes a result dataset (GeoJSON/GeoParquet) and a summary JSON.
- Records inputs, CRS, parameters, algorithm version, and operator.
- Is reproducible from the record.
- Never runs arbitrary SQL from a user or an LLM.

---

## 7. Performance

| Metric | Target |
|---|---|
| Vector tile p95 | ≤ 500 ms uncached; ≤ 50 ms cached |
| COG first view | ≤ 2 s for a 1 GB raster on a warm cache |
| Point cloud first view | ≤ 3 s for a 10M-point COPC |
| Issue layer render | 10k points at 60 fps (clustered/MVT) |
| Spatial query | ≤ 300 ms for 100k issues with GiST |
| Ingest throughput | ≥ 10 MB/s vector, ≥ 5 MB/s raster |

Techniques:

- MVT for large vector layers; clustering at low zoom.
- COG overviews; range requests; client-side rendering.
- COPC streaming; point budget.
- Materialized views for heavy aggregates.
- Cache project summaries and tile responses.
- Load tests with 10k/100k/1M features before the beta.

---

## 8. Data licensing and attribution

Every dataset record carries:

- `licence` (SPDX identifier where possible).
- `attribution` text.
- `source_url`.
- `retrieved_at`.
- `redistribution` flag (`allowed`, `internal_only`, `unknown`).

Rules:

- A dataset with `unknown` redistribution cannot be published in a shared project or exported.
- The map shows attribution in the layer details and the map corner.
- OSM-derived data requires "© OpenStreetMap contributors" and ODbL compliance for derived databases.
- Government data (Swisstopo, cantons) may have non-commercial or attribution clauses; verify per dataset.
- Revizto data is never treated as a public dataset; it is project-scoped and access-controlled.

---

## 9. Quality assurance

- CRS round-trip tests: known coordinates transform to expected values within tolerance.
- Georeferencing tests: synthetic control points with known transform and noise; residuals within bounds.
- Geometry validity tests: invalid polygons rejected or repaired explicitly.
- Tile tests: feature counts and bounds at z/x/y.
- Visual regression: Playwright screenshots of the demo map at fixed views.
- Data licence tests: every fixture has a licence and attribution field.
- Performance tests: seeded 100k issues; p95 thresholds in CI (nightly, not every PR).

---

## 10. 3D strategy

- **MVP:** 2D-primary, with a **synthetic 3D massing scene** (extruded building footprints + terrain) in the Cesium globe. Label it clearly as synthetic; no Revizto geometry.
- **Phase 2:** import real 3D Tiles (from IFC/glTF), align with the project localization, and show the model in the globe.
- **Terrain:** optional Cesium Ion for the demo; self-hosted terrain generation (quantized-mesh) for air-gapped deployments.
- **Point clouds:** COPC streaming in GeoLibre for site scan context; no processing in the MVP.
- **Performance:** lazy-load Cesium; do not put the 3D engine on the critical path.

The 3D model is a **context layer**, not a replacement for the BIM review tool. Terrassay does not compete with Revizto's model navigation.

---

## 11. References

- GeoLibre architecture and rendering — <https://geolibre.app/architecture/>
- GeoLibre embedding (COG, PMTiles, LiDAR) — <https://geolibre.app/user-guide/embedding/>
- PostGIS `ST_AsMVT` — <https://postgis.net/docs/ST_AsMVT.html>
- PROJ — <https://proj.org/>
- COG — <https://www.cogeo.org/>
- COPC — <https://copc.io/>
- 3D Tiles — <https://github.com/CesiumGS/3d-tiles>
- OSM licence — <https://www.openstreetmap.org/copyright>
