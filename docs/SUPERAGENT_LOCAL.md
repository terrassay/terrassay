# Local Superagent Setup

| | |
|---|---|
| **Status** | Installed and running locally |
| **Location** | `$HOME/GeoAI/.superagent/dsh-superagent` |
| **DSH home** | `$HOME/GeoAI/.dsh` |
| **State** | `$HOME/GeoAI/.superagent/state` |
| **Panel** | <http://127.0.0.1:3092> |
| **tmux session** | `terrassay-superagent` |
| **GitHub repo** | `terrassay/terrassay` |
| **Agent identity** | `atpl-training` (same human account) |

Everything for this project lives under `$HOME/GeoAI`. The only thing outside the folder is the macOS Keychain item holding the PAT, which is intentional.

---

## 1. What was set up

- Superagent cloned into `.superagent/dsh-superagent` and built.
- A dedicated DSH profile at `.dsh/profiles/superagent`, with:
  - `stateDir = $HOME/GeoAI/.superagent/state`
  - panel port `3092`
- The local plugin linked into that profile.
- GitHub Source configured for `terrassay/terrassay`.
- GitHub CLI configured to `$HOME/GeoAI/scripts/gh-terrassay`.
- Policy allowlist: `atpl-training` only.
- Executor: 2 workers; work dir `$HOME/GeoAI/.superagent/work`.
- Blocked commands: cloud CLIs, SSH, SCP, rsync, Terraform, Vault.
- No Feishu.
- No auto-merge.

The older instance on port `3082` is separate and was not touched.

---

## 2. Start, status, stop

```sh
./scripts/superagent-start.sh
./scripts/superagent-status.sh
```

Stop:

```sh
tmux kill-session -t terrassay-superagent
```

Panel:

```text
http://127.0.0.1:3092
```

The panel is bound to localhost. Do not expose it to the internet.

---

## 3. GitHub token

The token is **not** in a file in the repo. It is in macOS Keychain:

- Service: `terrassay-github-pat`
- Account: `atpl-training`

Set or rotate it:

```sh
./scripts/setup-keychain.sh
```

The wrapper `scripts/gh-terrassay` reads the token and injects it only into the `gh` process.

Test:

```sh
./scripts/gh-terrassay api user --jq .login
```

Expected: `atpl-training`.

Never paste the token into chat, an issue, a commit, or a file.

---

## 4. GitHub source settings

| Setting | Value |
|---|---|
| Repository | `terrassay/terrassay` |
| CLI | `$HOME/GeoAI/scripts/gh-terrassay` |
| Allowed user | `atpl-training` only |
| Accept mode | `observe` |
| Observe comments | on (needed because a self-mention may not notify) |
| Feishu | off |
| Auto-merge | off |
| Max workers | 2 |
| Work dir | `$HOME/GeoAI/.superagent/work` |

---

## 5. First run

The repository is currently empty. Before Superagent can prepare a worktree, the initial project commit must be pushed.

After the first push:

1. Create a clone for worktrees:

```sh
mkdir -p .superagent/repo
git clone https://github.com/terrassay/terrassay.git .superagent/repo
```

2. Set the executor prepare command in the panel (or via the local API):

```sh
git -C $HOME/GeoAI/.superagent/repo fetch --no-tags \
  "https://github.com/$DSH_SUPERAGENT_GITHUB_REPOSITORY.git" \
  "${DSH_SUPERAGENT_GITHUB_PR_HEAD:-HEAD}:refs/dsh-superagent/$DSH_SUPERAGENT_TICKET" &&
git -C $HOME/GeoAI/.superagent/repo worktree add \
  -b "agent/$DSH_SUPERAGENT_NAME" \
  "$DSH_SUPERAGENT_DIR" \
  "refs/dsh-superagent/$DSH_SUPERAGENT_TICKET"
```

3. Open a test issue, for example:

> Add a `docs/hello.md` with one sentence. No code changes.

4. Comment from `atpl-training`:

```text
@atpl-training implement this. Acceptance: docs/hello.md exists with one sentence.
```

5. Confirm:
   - a Ticket appears in the panel;
   - a PR opens;
   - CI runs;
   - the agent cannot merge;
   - you merge via admin bypass.

If the self-mention does not arrive, the source already has `observeComments` on; confirm it is reading comments. If it still does not trigger, the fallback is a dedicated machine account, but that is not required to start.

---

## 6. Same-account caveats

- Commits and PRs are authored by `atpl-training`; there is no separate bot identity.
- A PR cannot be approved by its own author. Branch protection should require PR + CI + 1 approval, with admin bypass for the owner; the token has no Administration permission.
- The agent can see the account’s notifications. Treat the runner as a trusted-but-limited environment.
- Keep the PAT scoped to one repo; rotate it every 90 days.
- Disclose AI-assisted PRs in the PR body.

---

## 7. Repair / upgrade

If Superagent stops:

```sh
./scripts/superagent-status.sh
tmux capture-pane -pt terrassay-superagent | tail -50
```

To upgrade:

```sh
cd .superagent/dsh-superagent
git pull
pnpm install
pnpm build
cd ../..
./scripts/superagent-start.sh
```

Back up `.superagent/state` before upgrades. The state database may not be readable by an older version.
