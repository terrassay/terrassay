# ADR-0014: Evaluate Expo Exact for the client runtime

- **Status:** Rejected for the core workspace; proposed for a future native companion
- **Date:** 2026-11-09
- **Deciders:** Maintainer
- **Related:** [ARCHITECTURE](../ARCHITECTURE.md), [ADR-0001](0001-reuse-geolibre.md)

## Context

[Expo Exact](https://github.com/expo/exact) is an MIT-licensed, pre-1.0
cross-platform application runtime. It describes UI in **Contract**, a small
declarative language, and data logic in TypeScript or Rust. It renders natively on
web, macOS, iOS, and Linux without a webview in the UI, and it exposes a ten-operation
agent driver for building, testing, and screenshotting apps on every host.

Terrassay's current client is a React/Vite web shell that embeds **GeoLibre** (a
React/MapLibre/Cesium application) through an iframe and the `@geolibre/embed` API.
The shell also uses React state, MapLibre/Cesium rendering, and the broader npm
ecosystem.

Exact is attractive because it is agent-native, fast, Rust-friendly, and could one
day give Terrassay a native macOS/iOS field client from one codebase.

## Decision

**Do not adopt Exact for the core Terrassay workspace now.**

Keep the React + Vite shell and the GeoLibre iframe. Revisit Exact only for a
**separate native companion** after the self-hosted beta, and only if there is real
demand for offline site inspection on macOS/iOS.

A future evaluation must prove, in a time-boxed spike:

1. How a map is hosted: an iframe/webview with a working bridge, or a native map
   module (MapKit) that replaces GeoLibre.
2. Whether the map interaction model (layer add, filter, highlight, selection) can be
   expressed through Contract actions and native modules.
3. Whether the data layer can call the existing Rust API without duplicating domain
   logic.
4. That Exact is stable enough at the chosen version; the project explicitly has no
   API stability, no backwards compatibility, and no migration guide before 1.0.

## Why not for the core

- **No React/JSX tier.** The existing shell cannot be reused; adopting Exact means a
  UI rewrite in Contract.
- **No npm code in TypeScript sources.** Exact's TypeScript sources can import only
  types from packages. MapLibre, deck.gl, Cesium, and the GeoLibre embed client cannot
  be used as they are today.
- **The map is the product surface.** GeoLibre is a React/web application; on native
  Exact hosts it would need an iframe/webview bridge or a separate native map. That is
  the highest-risk part of the product, not a detail to port late.
- **Pre-1.0 instability.** Exact changes daily, has no API stability, and no migration
  guide. A solo, unfunded project cannot absorb that churn while also building the
  domain.
- **Toolchain cost.** Exact needs Bun, Rust, Xcode, Hermes, and Chrome; native builds
  take 10–15 minutes the first time. That is a large addition for a web-first demo.
- **No packaged CLI.** Working from a source checkout is another operational burden.

## Why it may fit later

- A native **field app** for iOS/macOS: offline issue capture, GPS, camera, site
  photos, and a map. That is a different product surface from the desktop analysis
  workspace.
- Exact's Rust data sources align with Terrassay's Rust API and domain types.
- Its agent driver and Contract tests fit the "human + assistant" workflow.
- One Contract source could cover web, macOS, and iOS if the map problem is solved.

## Consequences

**Positive**

- No rewrite; the GeoLibre embed and React ecosystem remain.
- The core stays web-first and testable with the current stack.
- Exact remains an option for a native companion without contaminating the core.

**Negative**

- No native desktop/mobile client from Exact in the near term.
- A future native client may duplicate some UI and map logic.
- The team forgoes Exact's performance and agent-native testing for now.

## Alternatives considered

| Alternative | Why not now |
|---|---|
| Rewrite the shell in Exact | Huge rewrite; map/GeoLibre integration unproven; pre-1.0 |
| Use Exact only for a settings/admin tool | Not worth a second runtime for a small surface |
| Tauri + React | Keeps React/GeoLibre; a later native-shell option |
| React Native | Map/3D stack and GeoLibre reuse are poor fits |
| PWA / installed web app | Keeps the current stack; sufficient for field-lite use |

## Review trigger

Revisit when Exact reaches 1.0 or a stable API, when an iframe/native-map bridge is
demonstrated, or when a design partner asks for an offline native field client.
