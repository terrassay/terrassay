# ADR-0012: 3D massing in the demo; no BIM geometry from Revizto

- **Status:** Accepted
- **Date:** 2026-11-04
- **Deciders:** Maintainer
- **Related:** [REVIZTO_CONNECTOR §8](../REVIZTO_CONNECTOR.md), [GEOSPATIAL §10](../GEOSPATIAL.md)

## Context

Revizto's API/MCP does not expose model import/export, and its geometry/georeference fields are unconfirmed. A demo that promises a 3D BIM model on a globe would depend on an endpoint that may not exist. GeoLibre already provides a Cesium globe, but a compelling 3D demo requires model data we do not have.

## Decision

Make the map **2D-primary**, with an optional **3D massing scene**: synthetic extruded building footprints and terrain in the Cesium globe. Show project data, issues, clashes, sheets, site boundary, and external layers on MapLibre. The 3D scene is explicitly labelled *synthetic massing, no Revizto geometry*.

Full BIM geometry is deferred to Phase 2 and arrives through **IFC/3D Tiles/Speckle**, aligned with the same localization record. Do not build a Revizto geometry importer unless the API documents one.

## Consequences

**Positive**

- Demo does not depend on unconfirmed API capabilities.
- Adds a 3D visual without pretending to be a BIM viewer.
- Focuses on the differentiated workflow: project data in site context.
- Clean path to real 3D via open formats.

**Negative**

- Synthetic massing is not the real model; it must be labelled.
- Some stakeholders may expect a BIM viewer.
- Phase 2 adds a real ingestion path.

**Mitigations**

- Use terrain, hillshade, and a compelling site narrative.
- Show a 3D Tiles teaser only if a licensed sample model is available and clearly labelled as sample data.
- Explain the API limitation and the IFC roadmap.

## Alternatives considered

| Alternative | Why rejected |
|---|---|
| Fake/placeholder model on the map | Misleading; damages trust |
| Scrape Revizto views | Prohibited and unreliable |
| Build a Revizto geometry importer speculatively | May not exist; wasted effort |
| Delay the demo for 3D | Unnecessary risk |

## Review trigger

Revisit when Revizto documents geometry/georeference endpoints or when the IFC/3D Tiles pipeline is ready.
