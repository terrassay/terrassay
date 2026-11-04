# ADR-0005: Explicit georeferencing and CRS policy

- **Status:** Proposed
- **Date:** 2026-11-04
- **Deciders:** Engineering, domain
- **Related:** [GEOSPATIAL](../GEOSPATIAL.md), [DATA_MODEL](../DATA_MODEL.md)

## Context

BIM models are authored in local coordinates. Revizto may not expose a georeference or model geometry. Placing project data on a map without an explicit transform is error-prone and can create liability. Construction users need to know how a location was derived and how accurate it is.

## Decision

Make georeferencing an explicit, reviewable record on every project.

- Store geometries in EPSG:4326; use a project-specific metric CRS for analysis.
- Record method, source CRS, target CRS, transform parameters, control points, residuals, accuracy, confidence, verifier, and timestamp.
- Methods in confidence order: API metadata → surveyed control points → map pick → address geocode.
- Label results **Verified**, **Calculated**, or **Indicative**.
- Require confirmation before persisting; require an admin to mark verified.
- Never claim survey-grade accuracy; show disclaimers where location affects a decision.

## Consequences

**Positive**

- Auditable, reproducible placement.
- Clear UX for uncertainty.
- Same transform can align later 3D Tiles/IFC geometry.
- Reduces liability from silent assumptions.

**Negative**

- More UI work than a single "set location" pin.
- Users may skip verification and trust an indicative result.
- Control-point solving adds complexity.

**Mitigations**

- Make the confidence badge prominent and persistent.
- Disable or caveat analyses on indicative locations.
- Provide a guided control-point workflow and residual report.
- Store a verification checklist in the audit log.

## Alternatives considered

| Alternative | Why rejected |
|---|---|
| Auto-geocode only | Too inaccurate for construction; hides uncertainty |
| Assume Revizto georeference | Unconfirmed and unavailable for geometry |
| Store a single lat/lon | Loses rotation, scale, and residuals |
| Store projected coordinates only | Breaks portability and web rendering |

## Review trigger

Revisit when the Revizto API exposes a documented georeference or when IFC `IfcMapConversion` support lands.
