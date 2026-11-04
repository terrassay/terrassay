# Risk Register

| | |
|---|---|
| **Status** | Living document; review at every milestone |
| **Scoring** | Likelihood × Impact: 1–5 each; score 1–25 |
| **Context** | Solo maintainer, no budget, synthetic public demo, no customer data |
| **Related** | [BUILD_PLAN](BUILD_PLAN.md) · [ROADMAP](ROADMAP.md) · [LEGAL_COMPLIANCE](LEGAL_COMPLIANCE.md) |

---

## 1. Top risks

| ID | Risk | L | I | Score | Trigger | Mitigation | Contingency |
|---|---|---:|---:|---:|---|---|---|
| R1 | Revizto API does not expose geometry/georeference | 4 | 5 | 20 | API review | Mock-first; explicit localization; IFC/3D Tiles fallback | Demo is "project data on map", not "model on map" |
| R2 | Revizto access delayed or denied | 4 | 4 | 16 | No test project | Apply later; mock; IFC/Speckle | Keep Revizto as a later adapter |
| R3 | GeoLibre embed lacks a required interaction | 3 | 4 | 12 | Week 1 spike | Typed bridge; capability tests; upstream issue | Custom map-host plugin; fork as last resort |
| R4 | Scope creep into general GIS / BIM viewer | 4 | 5 | 20 | New epic unrelated to six capabilities | Non-goals; demo gate; weekly review | Cut feature; defer |
| R5 | Georeferencing error creates liability | 3 | 5 | 15 | User asks for "exact" placement | Confidence labels, residuals, disclaimers, verification | Disable analysis on indicative locations |
| R6 | AI hallucination or prompt injection | 3 | 5 | 15 | Uncited spatial claim | Typed tools, citations, evals, read-only, injection tests | Disable AI for that project |
| R7 | Licence contamination (GPL/AGPL/copied code) | 3 | 5 | 15 | New dependency without review | Inventory, CI, clean-room, process isolation | Remove/replace before commercial |
| R8 | Superagent causes a bad merge or credential leak | 3 | 5 | 15 | PR touches auth/workflows/secrets | Isolation, allowlist, branch protection, no auto-merge, kill switch | Revert; rotate secrets; pause |
| R9 | GitHub org/token setup violates terms or stalls | 3 | 3 | 9 | No org by day 3 | Free org + single human account + Keychain token; no second human account | Use a personal repo under `atpl-training` |
| R10 | Self-host install too complex | 3 | 3 | 9 | Install > 30 min | Compose profiles, one command, diagnostics, docs | Simplify the demo stack |
| R11 | Performance degrades on large projects | 3 | 3 | 9 | p95 > 1 s at 10k issues | MVT, clustering, COG/PMTiles, load tests | Cap project size; pre-aggregate |
| R12 | Revizto terms restrict caching/AI | 3 | 4 | 12 | Legal/API review | Read terms; ask vendor; data minimization | Local-only; no caching |
| R13 | Basemap/data licence violation | 2 | 4 | 8 | Dataset without licence | Dataset register; attribution; CI check | Remove/replace dataset |
| R14 | Maintainer burnout or bus factor of one | 5 | 4 | 20 | Missed milestone > 4 weeks; health dip | Narrow scope; part-time schedule; public "not doing" list; breaks | Pause; ship smaller demo; invite co-maintainer |
| R15 | AI cost overrun | 3 | 2 | 6 | Monthly cost > €20 | Local models; caps; caching; alerts | Degrade to local/smaller model |
| R16 | Security breach via upload/SSRF | 2 | 5 | 10 | Parser/network finding | Sandboxing, allowlists, limits, self-review | Disable URL import; incident response |
| R17 | Public legal challenge (trademark/API) | 2 | 5 | 10 | Complaint | Free clearance; disclaimers; no vendor data | Rename; remove feature; delay |
| R18 | No contributors or users | 3 | 3 | 9 | 3 months with no external issue/PR | Good docs, demo video, good-first-issues, community posts | Treat as a personal tool; keep it open |
| R19 | No budget for legal/infrastructure | 4 | 3 | 12 | A real project or paid service is proposed | Stay synthetic; local-first; Gate B | Defer commercial step |
| R20 | 3D massing distracts from the core demo | 2 | 3 | 6 | 3D work delays the AI/map slice | Timebox 4 PD; synthetic only; cut if late | Ship 2D, add 3D after |

---

## 2. The risks that matter most

### R1 — Revizto geometry gap

The API is a coordination-data API, not a model-exchange API. If the demo promises a 3D model on a map and the API cannot deliver it, the project loses credibility. The plan avoids this by promising **project data in geographic context** and by making localization a first-class workflow. The 3D massing is synthetic and labelled; the real model arrives later through IFC/3D Tiles.

**Watch:** any request that assumes model geometry from Revizto. Answer with the capability matrix and the IFC path.

### R8 — Superagent with full repo access

Superagent is a research prototype with full OS access and will have full repository access. The biggest risk is not a malicious agent; it is a plausible but wrong change that a tired maintainer merges. The second risk is credential exposure.

**Controls:** isolated VM/OS user, only the repo-scoped Keychain token, no org admin, no secrets, protected `main`, required CI, human review, no auto-merge, kill switch. If a PR touches auth, workflows, or secrets, close it and redo with a human.

### R14 — Solo burnout

This is the highest-likelihood high-impact risk. A solo, unfunded project can fail because the maintainer runs out of energy, not because the code is wrong.

**Controls:** milestone-driven plan, a public "not doing" list, small PRs, AI for boring work, breaks, and permission to ship a smaller demo. Do not trade health for a self-imposed deadline.

### R19 — No legal budget

The project stays in the synthetic, no-customer-data, no-paid-service posture until Gate B. This is a deliberate constraint, not a gap. If a real project or payment appears, pause and fund the review.

---

## 3. Decision gates

| Gate | When | Question | If no |
|---|---|---|---|
| G0 | Day 3 | Free org/repo ready and Keychain token works? | Use a personal repo under `atpl-training`; same token |
| G1 | Week 2 | GeoLibre embed supports add/filter/highlight/selection? | Switch to custom map-host plugin |
| G2 | Week 4 | Revizto access path clear? | Continue mock; defer connector |
| G3 | Week 8 | Mock sync + issues + map working? | Cut dataset breadth; focus on core narrative |
| G4 | Week 12 | AI answers grounded with citations + 3D massing works? | Cut 3D; reduce tools; use deterministic responses |
| G5 | Week 16 | Demo passes 3 rehearsals? | Delay; fix reliability, not features |
| G6 | After access | Real connector validated? | Keep mock; try IFC/Speckle |
| G7 | Before customer data | Gate B legal/security review funded? | Stay synthetic; no customers |

---

## 4. Kill criteria and pivots

**Kill / rethink if:**

- Revizto access is denied **and** no one will share an IFC/Speckle workflow.
- GeoLibre cannot support the map interactions after a plugin attempt, and forking is unmaintainable.
- The demo cannot answer a spatial question reliably after two attempts.
- The maintainer cannot sustain the project without harm.
- A legal challenge cannot be resolved without excluding the core use case.

**Pivots:**

- **IFC-first:** connect directly to IFC/Speckle; Revizto becomes optional.
- **Analysis-only:** a geospatial analysis plugin/service for existing BIM tools.
- **Personal tool:** keep it open source but stop chasing adoption.
- **Pause:** archive with a clear README; forks remain possible under Apache-2.0.

---

## 5. Assumptions to validate

| Assumption | How to validate | By |
|---|---|---|
| The demo is understandable without a BIM expert | Show it to 3 people; watch where they get lost | Week 16 |
| Contractors feel the export/screenshot pain | 3–5 conversations; no paid research | Ongoing |
| Self-hosting matters to security-conscious firms | 3–5 conversations with IT/security | After demo |
| A 10-minute demo is convincing | 3 rehearsals + feedback | Week 16 |
| GeoLibre is stable enough to embed | Spike + one upgrade cycle | Week 8 |
| Superagent saves time on this codebase | Measure 10 tasks | Week 8 |
| A solo maintainer can deliver the demo | Weekly burn-up; cut scope if behind | Weekly |

---

## 6. Watch list

- Revizto API versioning/deprecation policy.
- Model provider data-retention changes.
- GitHub policy changes affecting fine-grained PATs.
- Cesium Ion pricing/terms.
- Open-data licence changes (cantonal/OSM).
- Browser changes to iframe storage/cookies.
- Competitor bundling (Autodesk, Bentley, Esri).
- Maintainer availability and health.
- Domain/trademark availability.

---

## 7. Review cadence

- **Weekly:** top 5 risks, triggers, and actions.
- **At each milestone:** full register review; rescore.
- **After any incident:** add a risk if missing; update controls.
- **Before any customer data/payment:** Gate B review.
