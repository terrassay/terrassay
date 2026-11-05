# GitHub Setup — Selected

| | |
|---|---|
| **Status** | Selected / in progress |
| **Setup** | One human account + one free organisation + one public repo |
| **Cost** | **$0** on GitHub; no Pro, Team, or Enterprise |
| **Related** | [LEGAL_COMPLIANCE](LEGAL_COMPLIANCE.md) · [NAMING](NAMING.md) |

---

## 1. The selected setup

| Role | Name | Notes |
|---|---|---|
| Human owner | **`atpl-training`** | Owns the org, reviews, merges, handles billing |
| Organisation | **`terrassay`** | Free GitHub organisation; the public project identity |
| Repository | **`terrassay/terrassay`** | Public; branch-protected `main` |
| Project path | **`github.com/terrassay/terrassay`** | |

This is compliant with GitHub’s free-plan rule:

- one free Personal Account — `atpl-training`;
- no second human, bot, or machine account;
- no paid plan — the org is Free and the repo is public.

> If GitHub prompts for a paid plan when creating the org, choose the **Free** plan. GitHub Team is only needed for advanced protections on **private** repositories; this repository is public.

---

## 2. What each thing is

- **Account** — your login/identity. `atpl-training` is the human account.
- **Organisation** — a free container that owns repositories and teams. `terrassay` is the org. It is **not** a repo and **not** a second account.
- **Repository** — the code project. `terrassay` is the repo; its full path is `terrassay/terrassay`.

```
atpl-training (human owner)
└── terrassay (free organisation)
    └── terrassay (public repository)
        └── github.com/terrassay/terrassay
```

---

## 3. Create the free organisation

1. Sign in as `atpl-training`.
2. Create an organisation named `terrassay`.
3. Choose the **Free** plan. Do not select Team or Enterprise.
4. Use the project email as billing/security contact.
5. Enable **2FA requirement** for all members.
6. Set **base permissions** to `Read`.
7. Restrict member privileges: no public repo creation, no pages creation, no outside collaborators without owner approval.
8. Enable secret scanning, push protection, Dependabot alerts, and code scanning.
9. Enable private vulnerability reporting if available; otherwise publish a security contact.
10. Set Actions policy: allow selected actions, `GITHUB_TOKEN` read-only by default.

### Free-plan reality

- Public repositories: free, unlimited collaborators, branch protection/rulesets available.
- Private repositories: available on Free, but some advanced protections require Team. This project is public, so $0 is enough.
- Actions: free minutes for public repositories (fair-use limits).

---

## 4. Create the public repository

1. Create a **public** repository `terrassay` under the `terrassay` org.
2. Add `LICENSE` (Apache-2.0), `NOTICE`, `README.md`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`, `GOVERNANCE.md`, `TRADEMARKS.md`, and `docs/`.
3. Add `.github/CODEOWNERS`, issue templates, and PR template.
4. Set the default branch to `main`.
5. Add a ruleset for `main`:
   - Require a pull request.
   - Require checks: `lint`, `test`, `build`, `license`, `security`.
   - Require conversation resolution and linear history.
   - Block force pushes and deletions.
   - Do not allow bypass, including admins.
6. Keep forks enabled for open source; never run secrets on `pull_request_target`.

### CODEOWNERS starting point

```text
*                       @terrassay/maintainers

/docs/legal/            @terrassay/maintainers
/services/api/src/auth/ @terrassay/maintainers
/infrastructure/        @terrassay/maintainers
/.github/workflows/     @terrassay/maintainers
/LICENSE                @terrassay/maintainers
/NOTICE                 @terrassay/maintainers
/TRADEMARKS.md          @terrassay/maintainers
```

---

## 5. Identity and history hygiene

- No personal email, phone, address, or private keys in the repo.
- Start with a clean history; do not import a personal repository.
- Use a project-specific SSH key or a fine-grained PAT for `git push`; do not reuse personal keys.
- Human commits use a project name and GitHub noreply address.
- Do not require signed commits until you have a signing key; if required, configure it and document it.
- Scan before the first push:

```sh
gitleaks detect --source . --no-git
```

- Enable secret scanning and push protection before any code is pushed.

---

## 6. Public pre-flight

- [ ] Name/handle/domain reserved; free trademark knock-out search done.
- [ ] Apache-2.0 / NOTICE / third-party notices present.
- [ ] No secrets, personal data, customer data, or vendor data.
- [ ] No copied code; dependency licences reviewed.
- [ ] Synthetic-data banner and disclaimers in README.
- [ ] Security policy and reporting contact.
- [ ] Code of conduct and contribution guide.
- [ ] CI green on a clean clone.
- [ ] CODEOWNERS in place.

A lawyer is **not** required for this synthetic, non-commercial open-source bootstrap. Counsel review is required before real customer data, a paid service, or a vendor-data integration.

---

## 7. Recovery and bus factor

- One owner is acceptable for a solo project, but document recovery.
- Store 2FA recovery codes and a break-glass credential in a vault, not in the repo.
- Keep billing/domain accounts independent of a single personal card if possible.
- Export audit logs monthly; retain 12 months.
- Add a trusted co-owner only when you have a real person to share responsibility with.
- If you disappear, the project can be forked; Apache-2.0 guarantees that.

---

## 8. Cost

| Item | Cost |
|---|---:|
| GitHub Free organisation | $0 |
| Public repository | $0 |
| Public-repo Actions | $0 (fair use) |
| Domain (optional) | ~$10–15/year |
| **Total** | **$0 on GitHub**, plus optional domain |

---

## 9. Day-one commands (human owner)

```sh
gh auth status
gh api user --jq .login   # should print atpl-training

# Create the public repo (or use the web UI)
gh repo create terrassay/terrassay --public --source=. --remote=origin --push

# Protect main (example; use the UI/rulesets where possible)
gh api -X PUT "repos/terrassay/terrassay/branches/main/protection" --input - <<'JSON'
{
  "required_status_checks": {
    "strict": true,
    "contexts": ["lint", "test", "build", "license", "security"]
  },
  "enforce_admins": true,
  "required_pull_request_reviews": null,
  "restrictions": null
}
JSON
```

Use the GitHub UI for rulesets where possible; API shapes change. Verify every setting after creation.

---

## 10. Open decisions

1. Confirm the org handle is `terrassay` and the repo is `terrassay/terrassay`.
2. Register `terrassay.com` / `.dev` now or later.
3. Add a trusted co-owner now, or stay solo with a documented recovery plan.
4. Run the free trademark knock-out search before the first public commit.
