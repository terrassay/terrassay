# GitHub Setup — Selected

| | |
|---|---|
| **Status** | **Selected / in progress** |
| **Setup** | One human account + one free organisation; Superagent runs under the same account with a repo-scoped Keychain token |
| **Cost** | **$0** on GitHub; no Pro, Team, or Enterprise |
| **Related** | [LEGAL_COMPLIANCE](LEGAL_COMPLIANCE.md) · [DEV_AUTOMATION](DEV_AUTOMATION.md) · [NAMING](NAMING.md) |

> **Known limitation (4 Nov 2026):** the single-account setup authenticates and polls correctly, but Superagent ignores comments authored by its own account. A self-comment cannot trigger a Ticket. The standard fix is the one free machine account `terrassay-superagent` with its own classic PAT and repo access. See [MACHINE_ACCOUNT_SETUP](MACHINE_ACCOUNT_SETUP.md) and [SUPERAGENT_LOCAL](SUPERAGENT_LOCAL.md#51-smoke-test-result-4-november-2026).

---

## 1. The selected setup

| Role | Name | Notes |
|---|---|---|
| Human owner | **`atpl-training`** | Owns the org, reviews, merges, handles billing. Agent commits are authored by this same account; there is no separate bot identity. |
| Organisation | **`terrassay`** | Free GitHub organisation; the public project identity |
| Repository | **`terrassay/terrassay`** | Public; branch-protected `main` |
| Superagent identity | **`atpl-training`** | Same account, using a classic PAT (`public_repo` + `notifications`) stored in macOS Keychain |
| Project path | **`github.com/terrassay/terrassay`** | |

This is compliant with GitHub’s free-plan rule:

- **one free Personal Account** — `atpl-training`;
- **no second human account**;
- **no paid plan** — the org is Free and the repo is public;
- Superagent uses a classic PAT with `public_repo` + `notifications`, stored in macOS Keychain. Fine-grained tokens cannot read notifications.

> If GitHub prompts for a paid plan when creating the org, choose the **Free** plan. GitHub Team is only needed for advanced protections on **private** repositories; this repository is public.

---

## 2. What each thing is

- **Account** — your login/identity. `atpl-training` is the human account.
- **Organisation** — a free container that owns repositories and teams. `terrassay` is the org. It is **not** a repo and **not** a second human account.
- **Repository** — the code project. `terrassay` is the repo; its full path is `terrassay/terrassay`.
- **Machine account** — a separate account used only for automated work. It is not a second human.

```
atpl-training (human owner)
└── terrassay (free organisation)
    └── terrassay (public repository)
        └── github.com/terrassay/terrassay

Superagent (same account, atpl-training)
└── gh-terrassay wrapper → macOS Keychain → repo-scoped PAT
    └── write access to terrassay/terrassay only
```

---

## 3. Confirm the human account

1. Sign in as `atpl-training`.
2. Enable 2FA; store recovery codes in a vault.
3. Use a project-specific email for notifications, not a personal inbox if possible.
4. Do not rename the account to `terrassay`: GitHub usernames and org names share a namespace, so the org handle would be taken.
5. Generate a project-specific SSH key; for Superagent, use the classic PAT described in §6. Do not reuse personal keys.

Git identity for human commits (if any):

```sh
git config --global user.name "Terrassay Maintainers"
git config --global user.email "<id>+atpl-training@users.noreply.github.com"
```

---

## 4. Create the free organisation

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

## 5. Create the public repository

1. Create a **public** repository `terrassay` under the `terrassay` org.
2. Add `LICENSE` (Apache-2.0), `NOTICE`, `README.md`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`, `GOVERNANCE.md`, `TRADEMARKS.md`, and `docs/`.
3. Add `.github/CODEOWNERS`, issue templates, PR template, and `AGENTS.md`.
4. Set the default branch to `main`.
5. Add a ruleset for `main`:
   - Require a pull request.
   - Require one approval. With a single account, you merge via admin bypass; the agent token has no admin permission.
   - Require review from CODEOWNERS.
   - Require checks: `lint`, `test`, `build`, `license`, `security`.
   - Require conversation resolution and linear history.
   - Block force pushes and deletions.
   - Allow admin bypass for the owner. The agent token has no Administration permission, so it cannot bypass.
6. Keep forks enabled for open source; never run secrets on `pull_request_target`.

### CODEOWNERS starting point

```text
*                       @terrassay/maintainers

/docs/legal/            @terrassay/maintainers @terrassay/security
/services/api/src/auth/ @terrassay/maintainers @terrassay/security
/infrastructure/        @terrassay/maintainers
/.github/workflows/     @terrassay/maintainers
/LICENSE                @terrassay/maintainers
/NOTICE                 @terrassay/maintainers
/TRADEMARKS.md          @terrassay/maintainers
```

---

## 6. Superagent token and macOS Keychain

No collaborator invite and no machine account are needed. Superagent runs as `atpl-training`, using a classic PAT with `public_repo` + `notifications`. The token is stored in macOS Keychain, never in the repo or shell history.

### 6.1 Create the token

Superagent reads GitHub notifications to see mentions. **Fine-grained PATs cannot access the notifications API** and fail with `403 Resource not accessible by personal access token`. Use a **classic PAT** with only these scopes:

- `public_repo` — read/write public repositories
- `notifications` — read notifications

**GitHub → Settings → Developer settings → Personal access tokens → Tokens (classic) → Generate new token (classic)**

- Note: `terrassay-superagent`
- Expiration: 90 days
- Scopes: `public_repo`, `notifications`
- Nothing else.

This token can access **public** repositories only, not private ones. It is broader than a fine-grained token across public repos, but it is the narrowest token that lets Superagent read mentions. The account has no other public project repos, so the practical exposure is this repo.

If you later switch to a machine account, use its own classic PAT with the same two scopes; do not share this token.

### 6.2 Store it in Keychain

```sh
security add-generic-password -a "atpl-training" -s "terrassay-github-pat" -w
```

Paste the token at the hidden prompt. To update it later:

```sh
security add-generic-password -U -a "atpl-training" -s "terrassay-github-pat" -w
```

To delete it:

```sh
security delete-generic-password -a "atpl-training" -s "terrassay-github-pat"
```

### 6.3 Create the `gh` wrapper

```sh
mkdir -p ~/.local/bin

cat > ~/.local/bin/gh-terrassay <<'EOF'
#!/bin/sh
export GH_TOKEN="$(security find-generic-password -w -a "atpl-training" -s "terrassay-github-pat")"
exec gh "$@"
EOF

chmod +x ~/.local/bin/gh-terrassay
```

Test:

```sh
~/.local/bin/gh-terrassay api user --jq .login
```

It should print `atpl-training`. The first time, macOS may ask for Keychain access; choose **Always Allow**.

### 6.4 What “full access” means

Superagent can read/write code, create branches, open PRs, and work with issues in `terrassay/terrassay`. It does **not** get org ownership, billing, repository admin, or secrets.

Because it is the same account, a PR it opens cannot be approved by that account. Use this branch-protection pattern:

- require a pull request;
- require CI;
- require one approval;
- allow **admin bypass** (`enforce_admins: false`);
- the token has no Administration permission, so it cannot bypass;
- you merge from the GitHub UI as the owner.

Test this with the first agent PR: confirm the agent cannot merge it.

## 7. Configure Superagent

1. Install Superagent on an isolated VM/OS user (see [DEV_AUTOMATION](DEV_AUTOMATION.md)).
2. Source repository: `terrassay/terrassay`.
3. CLI: `/Users/<you>/.local/bin/gh-terrassay`.
4. Allowed users: `atpl-training` only.
5. Accept mode: `observe` until you have reviewed its behaviour.
6. Prepare command: clone `terrassay/terrassay` and create a worktree per ticket.
7. No Feishu.
8. No auto-merge.
9. Max workers: 2.
10. Test with a docs-only issue; confirm it opens a PR and cannot merge.

---

## 8. Security settings and hygiene

- [ ] 2FA required for the org.
- [ ] Secret scanning + push protection.
- [ ] Dependabot alerts + security updates.
- [ ] Code scanning.
- [ ] Private vulnerability reporting or security contact.
- [ ] Actions restricted; `GITHUB_TOKEN` read-only.
- [ ] `main` ruleset requires PR + CI; owner admin bypass; agent token has no admin.
- [ ] No personal email, SSH keys, or PATs in the repo.
- [ ] `gitleaks detect --source . --no-git` before the first push.
- [ ] Machine account cannot approve or merge its own PR.

---

## 9. Public pre-flight

Because the repository is public from bootstrap:

- [ ] Name/handle/domain reserved; free trademark knock-out search done.
- [ ] Apache-2.0 / NOTICE / third-party notices present.
- [ ] No secrets, personal data, customer data, or vendor data.
- [ ] No copied code; dependency licences reviewed.
- [ ] Synthetic-data banner and disclaimers in README.
- [ ] Security policy and reporting contact.
- [ ] Code of conduct and contribution guide.
- [ ] CI green on a clean clone.
- [ ] `AGENTS.md` and CODEOWNERS in place.

A lawyer is **not** required for this synthetic, non-commercial open-source bootstrap. Counsel review is required before real customer data, a paid service, or a vendor-data integration.

---

## 10. Cost

| Item | Cost |
|---|---:|
| GitHub Free organisation | $0 |
| Public repository | $0 |
| Public-repo Actions | $0 (fair use) |
| Superagent Keychain token | $0 |
| Domain (optional) | ~$10–15/year |
| **Total** | **$0 on GitHub**, plus optional domain |

---

## 11. Day-one commands (human owner)

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
  "enforce_admins": false,
  "required_pull_request_reviews": {
    "required_approving_review_count": 1,
    "dismiss_stale_reviews": true,
    "require_code_owner_reviews": true
  },
  "restrictions": null
}
JSON
```

Use the GitHub UI for rulesets where possible; API shapes change. Verify every setting after creation. With `enforce_admins: false`, the owner can merge via admin bypass; the agent token has no admin permission.

---

## 12. Recovery and bus factor

- One owner is acceptable for a solo project, but document recovery.
- Store 2FA recovery codes and a break-glass PAT in a vault, not in the repo.
- Keep billing/domain accounts independent of a single personal card if possible.
- Export audit logs monthly; retain 12 months.
- Add a trusted co-owner only when you have a real person to share responsibility with.
- If you disappear, the project can be forked; Apache-2.0 guarantees that.

---

## 13. Open decisions

1. Confirm the org handle is `terrassay` and the repo is `terrassay/terrassay`.
2. Confirm the Keychain item service is `terrassay-github-pat` and the wrapper is `~/.local/bin/gh-terrassay`.
3. Register `terrassay.com` / `.dev` now or later.
4. Add a trusted co-owner now, or stay solo with a documented recovery plan.
5. Run the free trademark knock-out search before the first public commit.
