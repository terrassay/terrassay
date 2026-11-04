# Governance

## Current stage

Terrassay is in bootstrap. A small maintainer group makes decisions in public where practical, and in private for security, legal, and personnel matters.

## Roles

| Role | Responsibility |
|---|---|
| Maintainers | Merge rights, releases, security response, roadmap, legal gates |
| Contributors | Issues, PRs, docs, reviews, community support |
| Security lead | Vulnerability response, threat model, release sign-off |
| Legal/compliance owner | Licence matrix, launch gate, vendor terms |

Maintainers are listed in the organisation profile. New maintainers are added by consensus of existing maintainers after a sustained contribution record.

## Decision making

- **Routine changes:** PR review + one maintainer approval.
- **Architecture decisions:** ADR in `docs/adr/`, reviewed by maintainers.
- **Large/breaking changes:** public RFC issue, at least 7 days for comment, then a maintainer decision.
- **Security/legal:** security lead or counsel sign-off; not decided by popularity.
- **Deadlock:** lazy consensus; if unresolved, a maintainer vote; ties defer to the project lead for the current stage.

## Roadmap

The public roadmap is a direction, not a contract. Items may move for security, legal, or sustainability reasons. A "not doing" list is maintained to keep scope honest.

## Releases

- SemVer while pre-1.0; breaking changes noted in the changelog.
- Signed tags/checksums; SBOM and third-party notices per release.
- Security fixes may be released out of band.
- No release without the legal/security checklist for the scope.

## Relicensing

The core is Apache-2.0. Relicensing requires contributor permission and a public RFC; it will not be done unilaterally.

## Funding and commercial activity

Commercial services (hosted, support, enterprise) are allowed and disclosed. The open-source core remains useful without payment. Sponsorships and donations are disclosed. No pay-for-review or hidden influence.

## Changes to governance

Proposed by a maintainer, discussed publicly for at least 14 days, and adopted by consensus. Emergency changes (security/legal) may be made immediately and explained afterwards.
