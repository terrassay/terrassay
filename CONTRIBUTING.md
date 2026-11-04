# Contributing to Terrassay

Thanks for your interest. The project is in planning/bootstrap; contribution guides will expand when code lands.

## Before you contribute

- Read [README.md](README.md), [docs/BUILD_PLAN.md](docs/BUILD_PLAN.md), and the [ADRs](docs/adr/README.md).
- Check the issue tracker; open an issue before a large change.
- Follow the [Code of Conduct](CODE_OF_CONDUCT.md).
- Report security issues privately per [SECURITY.md](SECURITY.md).

## Licence and DCO

Contributions are licensed under **Apache-2.0**. Every commit must be signed off under the Developer Certificate of Origin:

```text
Signed-off-by: Your Name <your-project-email@example.com>
```

Use `git commit -s`. By signing off, you confirm you have the right to submit the contribution under Apache-2.0. There is no CLA.

Do not contribute code you do not have the right to submit, including code from an employer, a proprietary project, or a GPL project that cannot be linked into Apache-2.0 code.

## AI-assisted contributions

AI-assisted work is welcome if:

- A human reviews and understands the change.
- The PR states that it is AI-assisted and names the tool/agent.
- The contribution does not copy unlicensed code.
- The human signs the DCO.

The project uses `dsh-superagent` for scoped tasks. See [docs/DEV_AUTOMATION.md](docs/DEV_AUTOMATION.md).

## Workflow

1. Fork or branch from `main`.
2. Keep changes focused; one concern per PR.
3. Add tests for behaviour changes.
4. Update docs and ADRs when decisions change.
5. Run the checks (once the toolchain exists): `just check`.
6. Open a PR with context, risk, verification, and rollback.
7. Address review comments; a maintainer merges.

## What not to contribute

- Customer data, Revizto project data, personal data, or secrets.
- Code copied from a proprietary or incompatible-licence source.
- New dependencies without a licence/security review.
- Changes to auth, crypto, legal files, or workflows without maintainer approval.
- Marketing claims or benchmarks without reproducible evidence.

## Community

Be respectful, assume good intent, and critique ideas rather than people. Maintainers may moderate to keep the project welcoming.
