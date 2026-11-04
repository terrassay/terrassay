# Licensing and Sustainability

| | |
|---|---|
| **Status** | Proposed |
| **Scope** | Licence strategy, dependency policy, community sustainability |
| **Related** | [LEGAL_COMPLIANCE](LEGAL_COMPLIANCE.md) · [BUILD_PLAN](BUILD_PLAN.md) |

---

## 1. Licence strategy

**Core licence: Apache-2.0.**

Why:

- Permissive enough for contractors, consultancies, and vendors to adopt.
- Includes an express patent grant.
- Compatible with most of the intended ecosystem (MIT/BSD/Apache-2.0).
- Familiar to enterprise legal teams.
- Allows hosting and integration without forcing a dual licence.

**Inbound: Developer Certificate of Origin (DCO), no CLA.**

- Every commit is signed off by a human with the right to contribute.
- Contributors keep their copyright.
- No unilateral relicensing.

**Outbound:**

- `LICENSE`, `NOTICE`, and `THIRD_PARTY_LICENSES` ship with every release and container image.
- Source files carry SPDX headers where practical.
- The project name and logo are governed by [TRADEMARKS](../TRADEMARKS.md), not the code licence.

---

## 2. No commercial tier right now

There is **no paid edition, no open-core paywall, no pricing, and no revenue plan** at this stage.

- The project is free to use, self-host, modify, and redistribute under Apache-2.0.
- There is no hosted service, no customer data, and no support contract.
- The maintainer is not funded; the project is built with time and AI agents.
- Community contributions are welcome and are not asked to sign a CLA.

This avoids the complexity of open-core boundaries, pricing pages, and commercial legal work while the product is unproven.

### If sustainability becomes necessary later

Options, in order of preference:

1. **Donations/sponsorships** (GitHub Sponsors, Open Collective) with transparent use of funds.
2. **Paid support or training**, without closing the core.
3. **Hosted convenience** for teams that do not want to self-host.
4. **Grants** from open-source/geospatial/construction-tech programmes.

Any of these requires a legal/accounting review before accepting money. Do not add a paid tier until users ask for it and the obligations are clear.

---

## 3. Dependency policy

The full matrix is in [LEGAL_COMPLIANCE](LEGAL_COMPLIANCE.md#4-licence-matrix). The rules:

| Category | Rule |
|---|---|
| Permissive (MIT/BSD/Apache/ISC) | Allowed; notices preserved |
| Weak copyleft (LGPL/MPL) | Separate dynamic component; review before distribution |
| Strong copyleft (GPL/AGPL) | Not linked into core; separate process; counsel before a commercial distribution |
| Source-available (SSPL/RSAL/BSL) | Avoid; use a permissive alternative |
| Commercial SDK/API | Terms review; no redistribution assumptions |
| Data | Separate licence per dataset; attribution and redistribution flags required |

Specific decisions:

- **PostGIS** (GPL-2.0-or-later) runs as a separate database service; distribution obligations are documented and reviewed before a commercial appliance.
- **Valkey** (BSD-3-Clause) replaces Redis as the optional cache/rate-limit store.
- **SeaweedFS** (Apache-2.0) is the default self-host object store; MinIO is dev-only pending licence review.
- **pyRevizto** (GPL-3.0) is not used; the Revizto adapter is a clean-room implementation from public docs.
- **GeoLibre** (MIT) is embedded with notices; its bundled engines and basemaps are checked separately.
- **Cesium Ion** is optional; the core works without it.
- **dsh-superagent** (MIT) is a development tool, not distributed with the product.

CI enforces the allowlist with `cargo-deny`, `pnpm licenses`, `pip-licenses`, and an SBOM. A new licence family requires human review.

---

## 4. Community and governance

| Element | Proposal |
|---|---|
| Maintainer | One maintainer; trusted co-maintainers added when earned |
| Decisions | ADRs + public issues; lazy consensus; maintainer decides if needed |
| Roadmap | Public, milestone-driven, with a "not doing" list |
| Contributions | DCO, CoC, good-first-issues, review best-effort |
| Security | Private reporting, 90-day disclosure target, credit |
| Releases | SemVer 0.x; signed images/checksums; changelog |
| Communication | GitHub Discussions/Issues, release notes |
| Funding | None now; transparent if it changes |

A solo project cannot promise enterprise SLAs. Be explicit about that in the README and issue templates.

---

## 5. Revizto relationship

- Apply to the official partner/integration programme when access is needed.
- Comply with API terms, rate limits, and data-handling requirements.
- Use OAuth per user; never a shared service account.
- Do not redistribute project data; do not scrape.
- Use the Revizto name descriptively only; include a disclaimer.
- Seek written permission for co-marketing, logo use, or case studies.
- Keep the adapter open source; credentials and customer data are never bundled.

A partnership is useful but not a dependency. The connector architecture must work with other BIM sources.

---

## 6. Trademarks and naming

- **Terrassay** is the chosen name; see [NAMING](NAMING.md).
- Preliminary availability is not trademark clearance. Run a free knock-out search now; counsel review before a commercial launch.
- Do not use **Revizto**, **GeoLibre**, **DeepSeek**, or their logos in the project name, domain, or icon.
- Publish `TRADEMARKS.md` and apply it consistently.
- The code licence does not grant trademark rights.

---

## 7. Cost of the project

| Item | Cost |
|---|---:|
| Licence | €0 |
| Repository (public) | €0 |
| GitHub Free org/public repo | $0 |
| Domain | ~€10–15/year |
| Hosting | €0 local; optional €5–20/month |
| AI models | €0 local; usage-based if hosted |
| Legal | €0 for Gate A; save for Gate B |
| **Total to demo** | **<€100 first year**, excluding model usage |

There is no paid team, no contractor budget, and no office. The main cost is the maintainer's time.

---

## 8. Sustainability risks

| Risk | Mitigation |
|---|---|
| Maintainer burnout | Narrow scope, milestone-driven, public "not doing" list, breaks |
| No funding | Local-first, free tiers, donations only if needed |
| Bus factor of one | Docs, ADRs, Apache-2.0 forkability, invite co-maintainers |
| AI dependency | Deterministic mocks, local models, no hard vendor lock-in |
| Community expectations exceed capacity | Clear README status, issue templates, no SLA promises |
| Legal review deferred | Stay synthetic/non-commercial until Gate B is funded |
| Vendor API changes | Adapter boundary; mock; multiple sources |

---

## 9. What we will not do

- Close the core or move features behind a paywall after adoption.
- Sell customer data or train on it without explicit permission.
- Claim Revizto endorsement without an agreement.
- Use another company's trademarks as our brand.
- Offer survey-grade or legal-boundary guarantees.
- Accept real customer data before the Gate B legal/security review.
- Promise support that a solo maintainer cannot deliver.
