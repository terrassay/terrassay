# Security Policy

## Reporting a vulnerability

Do not open a public issue for a security vulnerability.

Use GitHub's **private vulnerability reporting** for this repository once it is enabled. Until then, contact the maintainers through the private address published in the organisation profile.

Include:

- A description of the issue and its impact.
- Steps to reproduce, or a proof of concept.
- Affected version/commit.
- Any suggested fix or mitigation.
- Whether you intend to disclose publicly and on what timeline.

We aim to acknowledge within **3 business days** and to provide a remediation plan within **10 business days**. We target a **90-day disclosure window**, coordinated with the reporter.

## Scope

In scope:

- Terrassay application code and APIs.
- Authentication, authorization, tenancy, and connector token handling.
- File upload, dataset ingestion, and SSRF protections.
- AI tool execution and prompt-injection boundaries.
- The map bridge and iframe origin/capability controls.
- Container images and build/release configuration.

Out of scope:

- Vulnerabilities in third-party services (report to the vendor).
- Social engineering.
- Denial-of-service against a public demo without prior agreement.
- Testing against a real Revizto account, GeoLibre instance, or customer deployment without written authorization.

## Safe harbour

We will not pursue legal action against researchers who:

- Act in good faith and avoid privacy violations, data destruction, and service disruption.
- Test only against their own deployment or an explicitly authorised environment.
- Report privately and give us reasonable time to fix the issue.
- Do not extort, publicly disclose prematurely, or access data beyond what is needed to demonstrate the issue.

This policy does not authorise testing against third-party systems.

## Supported versions

Until v1.0, only the latest minor release receives security fixes. Security fixes are backported at the maintainers' discretion.

## Disclosure

We publish advisories for fixed vulnerabilities, credit reporters who wish to be named, and document the fix and workaround. See [docs/SECURITY.md](docs/SECURITY.md) for the engineering controls.
