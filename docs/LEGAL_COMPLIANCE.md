# Legal and Compliance Plan

| | |
|---|---|
| **Purpose** | Keep the project legally clean while it is open source, testing-only, and pre-revenue |
| **Status** | Proposed — self-audit for the synthetic public repo; counsel before customer data or commercial service |
| **Owner** | Maintainer |
| **Related** | [LICENSING_BUSINESS](LICENSING_BUSINESS.md) · [GITHUB_ORG_SETUP](GITHUB_ORG_SETUP.md) · [SECURITY](SECURITY.md) |

> **This is not legal advice.** It is an engineering compliance plan. The synthetic, non-commercial public repository uses the self-audit in §12. A qualified lawyer reviews the position before any real customer data, paid service, or vendor-data integration. If you cannot afford counsel yet, stay in the synthetic/testing posture described here.

---

## 1. Scope and stance

The project is currently:

- **Open source** (Apache-2.0 for original code, with third-party licences respected).
- **Testing-only**: synthetic data, no customers, no production service, no revenue.
- **Self-hosted by design**: no central processing of customer data in the demo.
- **Independent**: not affiliated with, endorsed by, or sponsored by Revizto, GeoLibre, or DeepSeek.

The legal strategy is simple: **write original code, use official interfaces, take only data we are allowed to take, and document every third-party dependency.** When in doubt, do not ship it; stay synthetic and defer the commercial step.

---

## 2. Non-negotiable rules

1. **No copied code.** Do not copy from proprietary tools, decompile binaries, paste unlicensed snippets, or reproduce code from GPL projects into Apache-2.0 files.
2. **No scraped data.** Do not scrape Revizto, GeoLibre, map providers, or competitor sites. Use official APIs, open datasets, or synthetic data.
3. **No customer or vendor data.** The demo uses fictional projects and records. No Revizto project exports, screenshots, models, or issue text.
4. **No credential sharing.** OAuth tokens belong to the authorising user and are encrypted at rest. Never put a vendor secret in git, chat, or a demo fixture.
5. **No trademark misuse.** Do not use another company's name or logo as our brand, or imply endorsement.
6. **No unlicensed dependencies.** Every dependency is inventoried, licence-checked, and approved before merge.
7. **No anonymous evasion.** The GitHub bot account is disclosed as automation. We do not create fake identities or evade platform limits.
8. **No customer data or paid service without the commercial gate.** The public synthetic repo uses the self-audit in §12; the commercial/customer-data gate requires counsel.

---

## 3. Clean-room development

To reduce the risk of copying protected code or data:

- Contributors work from **public documentation, public APIs, and observed behaviour** — not from proprietary source.
- The Revizto adapter is implemented from the official API documentation and our own contract tests. `pyRevizto` (GPL-3.0) is **not used in the build**; it is listed only to flag the licence risk. No code is copied, linked, or vendored.
- The GeoLibre integration uses the published `@geolibre/embed` package and official container image. We do not copy GeoLibre internals into our code.
- AI coding agents are instructed not to reproduce code from training data or external repositories; AI-assisted PRs are scanned and reviewed.
- Design documents record the source of any non-obvious behaviour (a link to public docs, a test, or a spec).

If a contributor has seen proprietary source at a previous employer, they must not contribute equivalent code. Pause and seek advice before accepting the contribution.

---

## 4. Licence matrix

This is a starting inventory. It is verified by automated tooling and updated as dependencies are added.

| Component | Licence | How we use it | Obligations / risk | Action |
|---|---|---|---|---|
| **Terrassay original code** | Apache-2.0 | Core | Include licence, NOTICE, patent grant; no trademark grant | Approved |
| **GeoLibre** | MIT | Embedded via official image/embed API | Preserve copyright + permission notice; no endorsement | Approved with notice |
| **@geolibre/embed** | MIT | npm dependency | Preserve notice | Approved |
| **MapLibre GL JS** | BSD-3-Clause | Rendering | Attribution in distributions | Approved |
| **deck.gl** | MIT | Overlays | Preserve notice | Approved |
| **CesiumJS** | Apache-2.0 | 3D globe | NOTICE; attribution | Approved |
| **Cesium Ion** | Commercial terms | Optional terrain/imagery | Token, attribution, usage limits; not required | Optional; review terms |
| **PostgreSQL** | PostgreSQL Licence | Database | Permissive | Approved |
| **PostGIS** | GPL-2.0-or-later | Separate database service | GPL obligations may attach to distribution of the database; keep separate process; counsel review | Review before distribution |
| **pgvector** | PostgreSQL Licence | Embeddings (optional) | Permissive | Approved |
| **DuckDB** | MIT | Analytics | Preserve notice | Approved |
| **GDAL** | MIT/X11 | Raster/vector I/O | Preserve notice | Approved |
| **PROJ** | MIT | CRS transforms | Preserve notice | Approved |
| **PDAL** | BSD-3-Clause | LiDAR | Preserve notice | Approved |
| **GEOS** | LGPL-2.1 | Geometry engine (via PostGIS/Shapely) | Dynamic linking; notice | Review distribution |
| **Arrow / Parquet** | Apache-2.0 | Columnar data | NOTICE | Approved |
| **Rust crates** | Mostly MIT/Apache-2.0 | API/worker | Per-crate notices; `cargo-deny` allowlist | Automated |
| **FastAPI** | MIT | Internal Python service | Preserve notice | Approved |
| **PyTorch** | BSD-3-Clause | ML inference | Preserve notice | Approved |
| **GeoPandas / Shapely / rasterio** | BSD-3-Clause | Geo processing | Preserve notices | Approved |
| **Valkey** | BSD-3-Clause | Optional cache/rate limit | Preserve notice | Approved |
| **Redis** | RSALv2 / SSPLv1 / AGPLv3 (varies by version) | Not default | Copyleft/service-use risk | Avoid; use Valkey |
| **SeaweedFS** | Apache-2.0 | Default self-host object store | NOTICE | Approved |
| **MinIO** | AGPL-3.0 (verify current) | Local development only | Network-copyleft risk if distributed/modified | Dev only; review |
| **pyRevizto** | GPL-3.0 | Not used; listed for licence risk | Cannot copy/link into Apache-2.0 code | Prohibited in product |
| **dsh-superagent** | MIT | Development tool only | Preserve notice; research prototype | Approved for dev |
| **OpenStreetMap data** | ODbL | Basemap | Attribution + share-alike for derived DB | Attribution; review |
| **Protomaps / OpenFreeMap** | Check current terms | Basemap | Attribution/usage limits | Review |
| **Swisstopo / cantonal data** | Open government terms | Demo layers | Attribution; some datasets non-commercial | Verify per dataset |
| **Copernicus / ESA** | Open terms | Imagery | Attribution | Verify |
| **Fonts / icons** | OFL / MIT / CC0 | UI | Preserve notices | Inventory |

### Compatibility rules of thumb

- **Apache-2.0 inbound:** contributions are accepted under Apache-2.0 via DCO.
- **MIT/BSD:** compatible; preserve notices.
- **GPL/AGPL:** do not link into core code. Keep the component in a separate process and document the boundary. Counsel confirms distribution and network-use obligations.
- **SSPL/RSAL:** avoid in the default distribution; use permissively licensed alternatives.
- **Commercial APIs/SDKs:** check terms for caching, derived data, attribution, and redistribution; do not assume "API access" means "redistribution rights".
- **Data licences:** a dataset's licence is separate from the code licence. Record it per dataset and display attribution.

Automated checks:

```sh
cargo deny check licenses
cargo about generate about.hbs > THIRD_PARTY_LICENSES.html
pnpm licenses list
pip-licenses --format=json
syft dir:. -o cyclonedx-json > sbom.json
reuse lint   # once SPDX headers are adopted
```

A dependency without a recognised licence fails CI until reviewed.

---

## 5. Revizto-specific compliance

Revizto is a third-party platform. We do not own its API, data, or marks.

### Access and authorisation

- API access must be enabled at the account level by the account owner/admin.
- Use the official **OAuth 2.0 Developer Portal** flow; register exact redirect URIs; use HTTPS outside localhost.
- Tokens are per user. The integration may only access projects that the authorising user can access.
- Do not share tokens between users, tenants, or environments.
- Store client secrets in a secrets manager, never in the repository.
- Revoke tokens on disconnect and rotate secrets on a schedule.

### Data handling

- Do not redistribute Revizto project data or expose it to other customers.
- Do not use Revizto data to train models without explicit written permission.
- Cache only what is necessary for the feature; document retention and deletion.
- Honour deletion requests: disconnect and purge cached project data.
- Do not attempt to access projects the user cannot access, bypass rate limits, or reverse engineer the API.
- Confirm whether the API terms permit caching, derived analyses, and AI processing; record the answer.

### Trademarks and partnership

- "Revizto" is used only to describe the integration (nominative fair use).
- Do not use the Revizto logo in the product, org avatar, or marketing without permission.
- Include a clear disclaimer: "Terrassay is an independent project and is not affiliated with or endorsed by Revizto."
- Apply to the official partner/integration programme before public marketing.
- Do not claim certification or partnership until a signed agreement exists.

### Testing

- Until access is granted, use only the mock connector and synthetic fixtures.
- Do not test against a real Revizto account without the account owner's written permission.
- When a partner project is used, minimise data, restrict access, and delete it after validation.

---

## 6. GeoLibre compliance

- GeoLibre is MIT-licensed. Include its copyright and permission notice in `NOTICE` and any distributed image.
- Use the official package/image; do not strip notices or re-brand it as our own.
- Do not use the GeoLibre name or logo as the Terrassay brand. Credit it as the map engine.
- Contribute improvements upstream where practical; this reduces divergence and supports the community.
- Check the licences of GeoLibre's bundled engines and data sources (Cesium, basemaps, fonts).
- If we build a custom map host, keep the GeoLibre notice and make the modifications available under MIT where required.

---

## 7. AI and development automation

### Product AI

- **Model licences:** if we self-host a model, its licence (Apache-2.0, Llama Community Licence, etc.) governs use, attribution, and acceptable-use restrictions. Record the model, version, and licence.
- **Provider terms:** hosted providers must offer a data-processing agreement, no-training-on-customer-data terms, and a retention policy acceptable to the customer. Prefer zero-retention endpoints.
- **No customer data in prompts without a DPA.** The demo uses synthetic data only.
- **Generated content:** answers are not legal, engineering, or survey advice. The UI carries a disclaimer.
- **Copyright:** do not ask the model to reproduce copyrighted text or code; citations are to project records, not third-party content.
- **Prompt injection:** project text is untrusted; tools are typed and permission-scoped.

### Development automation (Superagent)

- `dsh-superagent` is MIT-licensed and is a **research prototype**, not a production product ([repository](https://github.com/deepseek-ai/dsh-superagent-prototype)).
- It is used only on original project code, with synthetic fixtures, in an isolated environment.
- No customer data, vendor credentials, production secrets, or personal data in the agent's workspace.
- AI-assisted pull requests are reviewed by a human, scanned for copied code, and disclosed.
- The agent's GitHub account is labelled as automation; it does not approve or merge its own work.
- The model provider's terms apply to prompts and outputs. Use a provider/configuration approved for source code; do not send third-party confidential code.
- **Ownership:** human contributors remain responsible for the contribution. The DCO is signed by a human who has the right to submit the code. AI-generated code may have uncertain copyright status in some jurisdictions; the policy is documented now and counsel confirms it before a commercial launch.

---

## 8. Data protection and privacy

### Testing phase

- All demo projects, people, issues, and coordinates are **fictional**.
- No personal data is collected from users of the demo.
- If analytics are enabled, they are privacy-preserving, documented, and off by default in self-hosted builds.
- No cookies beyond a session cookie; no third-party tracking in the app.

### Future customer deployments

- A Data Processing Agreement (DPA) is required before processing customer data.
- Data residency follows the customer's Revizto region and hosting choice.
- Collect the minimum: account identity, project metadata, issue data needed for the feature.
- Retention and deletion are configurable; disconnect purges cached data.
- Subprocessors (hosting, model provider, email) are listed and approved.
- Support access is time-bound, logged, and requires customer approval.
- A privacy policy and security page are published before hosted service.

### GDPR/UK GDPR readiness

- Lawful basis, records of processing, DPIA for AI features, data subject request process.
- Standard contractual clauses for transfers outside the EEA/UK.
- Breach notification process and 72-hour assessment.
- Counsel reviews the DPA and privacy policy.

---

## 9. Trademarks and branding

- The project name must be cleared before the org is created or the repo is public. "Terrassay" is a working label and likely too generic to protect.
- Do not use "Revizto", "GeoLibre", "DeepSeek", or their logos in the product name, domain, org handle, or app icon.
- Nominative references are allowed to describe integrations, with a non-affiliation disclaimer.
- Do not imitate another product's trade dress or UI in a confusing way.
- Run a free knock-out search now; register the name/logo only after counsel clears it before a commercial launch. Domain and handle are not trademarks.
- Publish a `TRADEMARKS.md` explaining permitted use of the project's name and logo.

---

## 10. Open-source governance and contributions

- **Licence:** Apache-2.0 for original code; includes a patent grant and trademark carve-out.
- **Inbound:** Developer Certificate of Origin (DCO), no CLA. Every commit is signed off by a human with the right to contribute.
- **Outbound:** `NOTICE` and `THIRD_PARTY_LICENSES` are generated and shipped with releases.
- **Contributor conduct:** Contributor Covenant code of conduct.
- **Security:** private vulnerability reporting, 90-day disclosure target, no bounty until funded.
- **Governance:** maintainers, public roadmap, RFC/ADR process, no unilateral relicensing without community review.
- **AI contributions:** disclose, review, and scan; the human signatory owns the DCO.
- **Employer IP:** contributors confirm they have permission; a lightweight contribution policy explains this.

---

## 11. Platform and security-testing compliance

- Comply with the GitHub Acceptable Use Policies and Terms of Service.
- GitHub permits one free Personal Account and one free **machine account**, but this project uses only the human account: `atpl-training` owns the free `terrassay` organisation and runs Superagent under a repo-scoped token stored in macOS Keychain. There is no second human account.
- Superagent activity under `atpl-training` is disclosed in PRs as AI-assisted; no spam, scraping, fake engagement, or limit evasion.
- Do not perform security testing against Revizto, GeoLibre, or any third party without written authorisation.
- Report vulnerabilities in third-party software through their security channels.
- Do not use the project to bypass access controls, licences, or paywalls.
- Sanctions/export: standard open-source encryption and geospatial software; screen contributors and customers only as required by law. Counsel confirms.

---

## 12. Launch gates

There are two gates, because the public synthetic repo and a commercial service carry different risk.

### Gate A — public synthetic repository (self-audit)

This is the gate for making the repository public and inviting contributors. It does **not** require a lawyer, but it does require evidence.

| # | Item | Evidence | Owner |
|---|---|---|---|
| 1 | Name/handle/domain reserved; free trademark knock-out search done | Search notes | Maintainer |
| 2 | Apache-2.0 + NOTICE + third-party notices complete | Files in repo | Maintainer |
| 3 | Dependency licence audit clean/documented | CI report + SBOM | Maintainer |
| 4 | No GPL/AGPL code linked into core; PostGIS documented as a separate service | Architecture note | Maintainer |
| 5 | No secrets, personal data, customer data, or vendor data in history | gitleaks + manual review | Maintainer |
| 6 | All demo data synthetic and labelled | Fixture audit | Maintainer |
| 7 | GeoLibre/MapLibre/Cesium attributions present | NOTICE + UI credit | Maintainer |
| 8 | Security policy and private reporting contact | GitHub settings + SECURITY.md | Maintainer |
| 9 | CoC, CONTRIBUTING, GOVERNANCE, TRADEMARKS published | Files | Maintainer |
| 10 | AI-assisted code policy and disclosure live | PR template + policy | Maintainer |
| 11 | Dataset licences and attribution verified | Dataset register | Maintainer |
| 12 | README carries status, synthetic-data, and non-affiliation disclaimers | README | Maintainer |
| 13 | Clean-clone build and CI green | CI run | Maintainer |
| 14 | A second person (or a fresh self-review) checks the list | Review note | Maintainer |

**Gate A is enough for the synthetic, non-commercial public repo.** If any item is uncertain, keep the repo private and resolve it first.

### Gate B — customer data, paid service, or vendor integration (counsel)

Do not cross this line until a qualified lawyer reviews the position.

| # | Item | Evidence |
|---|---|---|
| 1 | Legal entity and IP ownership confirmed | Entity docs / assignment |
| 2 | Revizto API terms reviewed; data handling, caching, and AI processing permitted | Counsel memo |
| 3 | Privacy policy, DPA, SCCs, subprocessor list, DPIA | Documents |
| 4 | Trademark clearance and registration decision | Search report + counsel |
| 5 | Terms of service, liability limitation, insurance | Counsel/broker |
| 6 | AI provider DPA and no-training/retention terms | Signed terms |
| 7 | Security review or pen test for the deployed service | Report |
| 8 | Export/sanctions position if applicable | Counsel note |
| 9 | Counsel sign-off | Signed memo |

**Do not accept a real project, customer data, or payment before Gate B.** This is the line that keeps the project out of trouble while there is no legal budget.

---

## 14. Ongoing compliance

- **Every dependency PR:** automated licence + vulnerability check; human review if a new licence family appears.
- **Every dataset:** licence, attribution, and redistribution rights recorded in the dataset table.
- **Every release:** regenerate SBOM and third-party notices; archive the evidence.
- **Every quarter:** review the licence matrix, Revizto terms, model provider terms, and privacy policy.
- **Every incident:** log, assess, notify if required, and update the runbook.

Compliance is a build artefact, not a one-time document.
