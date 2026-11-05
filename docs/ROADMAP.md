# Roadmap and Delivery Plan

| | |
|---|---|
| **Status** | Proposed |
| **Baseline** | 1 maintainer + the assistant; no paid team; no fixed deadline |
| **Related** | [BUILD_PLAN](BUILD_PLAN.md) · [RISKS](RISKS.md) |

---

## 1. Delivery principles

1. **Demo first, product second.** The demo proves six capabilities end-to-end; everything else waits.
2. **Mock first, vendor second.** The connector contract and demo work without Revizto access.
3. **Reuse the map.** GeoLibre is a dependency, not a workstream.
4. **One vertical slice at a time.** Project → location → data → question → answer.
5. **Public and synthetic by default.** The repository is public, the data is fictional, and no customer data is processed.
6. **Use the assistant for scoped work.** Tests, docs, and refactors are still reviewed and merged by the maintainer.
7. **Milestones over dates.** There is no funded team; a missed week is information, not a failure.
8. **Protect the maintainer.** Scope is cut before health, sleep, or quality.

---

## 2. Phase overview

| Phase | Timing | Effort | Outcome | Gate |
|---|---|---:|---|---|
| **0 — Bootstrap & validation** | Weeks 1–2 | 11 PD | Account/org, licence/naming baseline, GeoLibre spike, fixture, mock contract | Spike go/no-go; fixture ready; public pre-flight v0 |
| **1 — Demo MVP** | Weeks 3–18 (12–16 working weeks) | 100 PD | Six capabilities including 3D massing, synthetic demo, one-command local install | 3 clean rehearsals; scorecard |
| **2 — Real connector & pilot** | After Revizto access | 80 PD | Revizto OAuth/sync, OIDC, IFC/3D Tiles, one partner | Partner project synced; external review |
| **3 — Self-hosted beta** | Ongoing | 82 PD | Connector SDK, collaboration, SSO, docs, release 0.1 | Partners; upgrade/backup test; self-audit |
| **Total** | **Not a deadline** | **273 PD** | Open-source beta | Quality, not calendar |

Capacity assumption: one maintainer, full-time or part-time, with the assistant for scoped work. Phase 1's 100 PD is roughly 12–16 weeks full-time after AI leverage; part-time stretches to 5–8 months. Contingency is held at the milestone level.

---

## 3. Milestones

| ID | Milestone | Timing | Exit criteria |
|---|---|---|---|
| M0 | New account, org, public repo, governance | Week 1 | 2FA, ruleset, Apache-2.0/NOTICE, NAMING, pre-flight v0 |
| M1 | Validation gate | Week 2 | GeoLibre spike passes; fixture spec approved; mock contract v1 |
| M2 | Project on the map | Week 5 | Login, project, localization preview, site boundary, basemap |
| M3 | Mock BIM sync | Week 8 | OAuth-shaped flow, 240 issues, 3 clashes, 12 sheets, filters, detail panel |
| M4 | 3D + grounded assistant | Week 12 | 3D massing, 6 tools, citations, map actions, 10 demo questions pass |
| M5 | Demo ready | Week 16 | 3 rehearsals < 12 min; scorecard green; public pre-flight |
| M6 | Real connector | After access | Revizto OAuth + sync against a partner project; incremental sync |
| M7 | Pilot validated | After M6 | A few users, OIDC, audit, backup restore, feedback |
| M8 | Beta feature complete | Ongoing | Connector SDK, IFC/3D Tiles, collaboration, SSO, docs |
| M9 | Beta release 0.1 | When ready | Upgrade test, public self-audit, release notes |

Dates assume full-time work. If part-time, multiply by roughly 2–2.5.

---

## 4. Phase 0 — Bootstrap and validation (weeks 1–2)

| Workstream | Deliverable | PD | Owner |
|---|---|---:|---|
| Account & org | Free org `terrassay`, human `atpl-training`, public repo, ruleset | 2 | Maintainer |
| Legal/naming baseline | Apache-2.0, NOTICE, CONTRIBUTING, SECURITY, CoC, NAMING, dependency policy | 2 | Maintainer |
| GeoLibre spike | Host map, iframe, addLayer, setFilter, highlight, selection, signed COG | 3 | Maintainer |
| Revizto discovery | Access checklist, API questions, canonical schema v1 | 2 | Maintainer |
| Demo fixture | Narrative, 240 synthetic issues, 3 datasets, control points, massing footprints | 2 | Maintainer |
| **Total** | | **11** | |

**Decision gate (end week 2):**

- If the embed spike fails, switch to the custom map-host plugin path before building the shell.
- If the org cannot be created, use a personal repo under `atpl-training`.
- If the AI provider is not settled, use local models or a deterministic mock.

---

## 5. Phase 1 — Demo MVP (weeks 3–18)

### 5.1 Work breakdown

| Epic | Deliverable | PD | Depends |
|---|---|---:|---|
| E0 | Repo, CI, Compose skeleton, `just` tasks, coding standards | 4 | M0 |
| E1 | Demo fixtures, seed/reset, synthetic-data banner | 6 | M1 |
| E2 | Core model + projects API + migrations | 9 | E0 |
| E3 | Dev auth/session/CSRF + seeded users | 4 | E2 |
| E4 | Map host + bridge + layer specs | 10 | M1 |
| E5 | Georeferencing workflow (pick + control points + confidence) | 7 | E2, E4 |
| E6 | Mock connector + OAuth screen + sync | 9 | E2 |
| E7 | Issues/clashes layers, filters, detail panel | 9 | E6, E4 |
| E8 | Dataset upload/finalize + GeoJSON/COG/XYZ layers | 8 | E2, E4 |
| E9 | AI assistant: tools, SSE, citations, map actions, evals | 12 | E7, E8 |
| E10 | Spatial analyses: buffer/intersect/within/nearest | 6 | E7, E8 |
| E11 | Jobs, progress, SSE, audit | 5 | E2 |
| E12 | Deployment, observability, docs, demo reset | 4 | E0 |
| E13 | QA, rehearsal, demo packaging | 3 | All |
| E14 | 3D massing + terrain scene | 4 | E4, E7 |
| | **Total** | **100** | |

### 5.2 Fast path (14 working weeks)

| Week | Focus | Demoable at end of week |
|---|---|---|
| 1–2 | Account, org, governance, spike, fixtures | M1 gate |
| 3 | Repo, CI, Compose, migrations | Shell + API health |
| 4 | Projects API, dev auth, seed | Login, project list, project detail |
| 5 | Map host, bridge, site boundary, basemap | Project at the right location |
| 6 | Localization UI, control points, confidence | Location can be set and verified |
| 7 | Mock OAuth, connection, project picker | Connect and select a remote project |
| 8 | Sync issues/clashes/sheets, layers, detail panel | Project data on the map |
| 9 | Dataset upload, GeoJSON/COG, styling, legend | External layers on the map |
| 10 | Assistant tools 1–3, SSE, citations | First grounded answers |
| 11 | Assistant tools 4–6, map actions, evals | Spatial question end-to-end |
| 12 | 3D massing, analyses, job UI, audit | 3D context + analysis result |
| 13 | Polish, performance, accessibility, docs | Demo run-through |
| 14 | Rehearsals, fixes, packaging, go/no-go | Three clean rehearsals |

If the pace is slower, extend to 16–18 weeks; do not cut the rehearsal or security steps.

### 5.3 Exit criteria

- [ ] Six capabilities demonstrated end-to-end, including 3D massing.
- [ ] Three consecutive rehearsals under 12 minutes, no manual fixes.
- [ ] Fresh bootstrap < 10 minutes; reset < 60 seconds.
- [ ] p95 API < 300 ms; first map < 5 s; AI first token < 3 s.
- [ ] 10 demo questions pass; 25 eval questions recorded.
- [ ] No critical/high findings in the demo security self-review.
- [ ] Public pre-flight checklist green; all data synthetic and labelled.
- [ ] Demo script and fallback plan documented.

---

## 6. Phase 2 — Real connector and pilot (after access)

| Workstream | Deliverable | PD |
|---|---|---:|
| Revizto OAuth + sync | Real authorize/token/refresh, projects, issues, clashes, sheets, polling, rate limits | 16 |
| Product auth | OIDC, roles, session hardening, audit export | 10 |
| 3D context | IFC/3D Tiles ingest, align with localization, Cesium globe | 10 |
| Georeferencing | Control-point solve hardening, residuals, verification workflow | 8 |
| Analyses | Slope, elevation sampling, buffer/intersect at scale | 12 |
| Hardening | Backups, restore, upgrade path, diagnostics | 8 |
| Pilot | Partner onboarding, docs, feedback, fixes | 8 |
| Security | Threat model review, dependency audit, external review | 5 |
| Contingency | | 3 |
| **Total** | | **80** |

**Exit criteria:** real OAuth works in the partner's region; a partner project syncs incrementally; a few users can log in through OIDC; a 3D Tiles model aligns with the site; backup/restore is tested; critical findings are fixed.

---

## 7. Phase 3 — Self-hosted beta (ongoing)

| Workstream | Deliverable | PD |
|---|---|---:|
| Connector SDK | Trait docs, test harness, example adapter; IFC/Speckle adapters | 14 |
| Collaboration | Comments, shares, saved views, activity feed | 12 |
| Admin & SSO | Org admin, roles, OIDC/SSO, basic SCIM | 10 |
| Observability | Metrics, tracing, dashboards, alerting, log export | 8 |
| Performance | 100k issues, large COG, load tests, query tuning | 7 |
| Docs & community | Self-host guide, API docs, tutorials, contributing, website | 8 |
| Beta hardening | Upgrade paths, migrations, security fixes, accessibility | 9 |
| Partners | 3 partners, onboarding, feedback | 8 |
| Release | Versioning, changelog, signed images, checklist | 6 |
| **Total** | | **82** |

This phase is intentionally unscheduled. It depends on community interest, Revizto access, and available time.

---

## 8. Critical path

```text
Account/repo → GeoLibre spike → map host + bridge → localization → mock sync
→ issue layers → datasets → assistant tools → 3D massing → rehearsal
```

Parallel tracks:

- Naming/trademark and public pre-flight.
- Revizto access (starts week 1; does not block the demo).
- Fixtures/design (week 1–2, then continuous).

Schedule risks:

| Risk | Impact | Mitigation |
|---|---|---|
| GeoLibre embed gap | 1–3 weeks | Spike week 1; plugin fallback |
| Revizto access delay | Phase 2 slip | Mock-first; start partner conversations now |
| AI quality/tooling | 1–2 weeks | Deterministic mock; narrow tools; evals |
| Solo capacity/burnout | 1–2 months | Cut scope; part-time schedule; public "not doing" list |
| Legal review absent | Commercial launch slip | Stay synthetic; defer customer data |

---

## 9. Solo modes

| Mode | Demo | Beta | Changes |
|---|---:|---:|---|
| Solo, full-time + AI | 12–16 weeks | 6–9 months | Baseline |
| Solo, part-time (~20 h/week) | 5–8 months | 12–18 months | Most likely for an unfunded project |
| Solo + regular contributors | 10–14 weeks | 5–8 months | Community review helps; coordination costs |
| Paid team | 8–12 weeks | 4–6 months | Not funded; comparison only |

AI leverage is assumed for tests, docs, fixtures, and small features. It does not remove human review, integration, or demo polish.

---

## 10. First 90 days

| Days | Focus | Output |
|---|---|---|
| 1–14 | Account, org, governance, spike, fixture | M1 gate |
| 15–28 | Foundation, projects, auth, map, localization | Project on the map |
| 29–42 | Mock sync, issues, datasets | Project data + external layers |
| 43–56 | Assistant, 3D massing, analyses | Grounded answers + 3D context |
| 57–70 | Polish, performance, accessibility | Demo candidate |
| 71–84 | Rehearsals, fixes, public pre-flight | M5 demo ready |
| 85–90 | Demo delivery + Phase 2 planning | Go/no-go for real connector |

At 20 hours/week, treat each 14-day block as roughly 4–5 weeks.

---

## 11. Definition of done (all phases)

- Code reviewed by a human; CI green.
- Tests cover new behaviour; no skipped tests without an issue.
- Docs updated (README, API, ADR if a decision changed).
- No new dependency without licence/security review.
- No secrets, customer data, or personal data.
- Performance budget respected.
- Accessibility checked for UI changes.
- Audit events added for new mutations.
- Rollback or disable path documented.
- AI-assisted PRs disclosed and reviewed.

---

## 12. Release criteria for v0.1 (beta)

- [ ] Self-host install < 30 minutes on a clean Linux host.
- [ ] Upgrade from previous minor with backup/restore tested.
- [ ] 3 active pilot users active.
- [ ] Revizto connector validated on at least one real project.
- [ ] OIDC/SSO and org roles work.
- [ ] Audit export and data deletion work.
- [ ] No open critical/high security findings.
- [ ] Public self-audit signed.
- [ ] Docs, API reference, and support runbook published.
