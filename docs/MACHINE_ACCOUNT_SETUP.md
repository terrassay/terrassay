# Machine Account Setup — `terrassay-superagent`

| | |
|---|---|
| **Purpose** | Create the one free machine account Superagent runs as |
| **Account** | `terrassay-superagent` |
| **Repo access** | Write to `terrassay/terrassay` only |
| **Token** | Classic PAT, `public_repo` + `notifications` |
| **Cost** | $0 |
| **Related** | [GITHUB_ORG_SETUP](GITHUB_ORG_SETUP.md) · [SUPERAGENT_LOCAL](SUPERAGENT_LOCAL.md) |

---

## 1. Why a machine account

Superagent ignores comments authored by the account it runs as, to prevent loops. If it runs as `atpl-training`, your own comments cannot trigger it.

The machine account is a separate GitHub user used only for automated work. GitHub permits **one free machine account** in addition to your free Personal Account. This is not a second human account.

---

## 2. Create the account

Use a private browser window so you do not accidentally create it inside your existing session.

1. Go to <https://github.com/signup>.
2. Use a **dedicated email** that is not already attached to a GitHub account. A masked email alias (iCloud Hide My Email, Proton alias, Fastmail masked email, or a dedicated mailbox) is ideal.
3. Choose the username **`terrassay-superagent`**. If it is taken, use `terrassay-agent` or `terrassay-automation`.
4. Use a long unique password and store it in your password manager.
5. Complete email verification and any CAPTCHA/phone verification GitHub requires. Do not use a fake identity or a phone number you do not control.
6. Do not follow, star, or comment on unrelated repositories. This account exists only for automation.

---

## 3. Profile and security

1. Name: `Terrassay Superagent`
2. Bio: `Automated account. Not a human. Operated by the Terrassay maintainer.`
3. Website: `https://github.com/terrassay/terrassay`
4. Enable **2FA** (Settings → Password and authentication). Use TOTP, store the secret and recovery codes in your vault.
5. Keep the email private (Settings → Emails → Keep my email addresses private).
6. Do not add payment methods, billing, or org ownership.

---

## 4. Give it access to the repo

As `atpl-training`:

1. Open `https://github.com/terrassay/terrassay`.
2. Go to **Settings → Collaborators and teams**.
3. Click **Add people**.
4. Enter `terrassay-superagent`.
5. Choose role **Write**.
6. Send the invitation.

If GitHub blocks outside collaborators:

- Org → **People → Invite member**, role **Member** (free), then add the account to the repo with Write; or
- Org → **Settings → Member privileges**, allow outside collaborators if your policy permits.

As `terrassay-superagent`, accept the invitation from the email or GitHub notifications.

Verify: the machine account can open the repo, but cannot access org settings, billing, or secrets.

---

## 5. Create its classic PAT

Sign in as `terrassay-superagent`.

**GitHub → Settings → Developer settings → Personal access tokens → Tokens (classic) → Generate new token (classic)**

- Note: `terrassay-superagent`
- Expiration: 90 days
- Scopes: **`public_repo`**, **`notifications`**
- Nothing else.

Do not use a fine-grained token: it cannot read notifications.

---

## 6. Store it in Keychain

On this machine:

```sh
./scripts/setup-agent-keychain.sh
```

This stores the token under:

- Service: `terrassay-agent-pat`
- Account: `terrassay-superagent`

The wrapper is:

```text
/Users/karim/GeoAI/scripts/gh-terrassay-agent
```

Test:

```sh
./scripts/gh-terrassay-agent api user --jq .login
```

Expected: `terrassay-superagent`.

---

## 7. Point Superagent at the machine account

Once the account has repo access and the token works, update the Superagent GitHub source:

- CLI: `/Users/karim/GeoAI/scripts/gh-terrassay-agent`
- Repository: `terrassay/terrassay`
- Allowed user: `atpl-training`
- Accept mode: `observe`
- No auto-merge
- Restart the local instance:

```sh
tmux kill-session -t terrassay-superagent
./scripts/superagent-start.sh
```

Then comment as `atpl-training`:

```text
@terrassay-superagent please implement this.
```

The machine account sees the mention, creates a Ticket, and opens a PR. You review and merge as `atpl-training`; the machine account cannot approve or merge its own PR.

---

## 8. Security rules

- Use this account only for automation.
- Never use it for human discussion, billing, or another project.
- Do not share its password or PAT.
- Keep the PAT scoped to `public_repo` + `notifications`; no `repo`, `admin`, or `workflow`.
- Rotate the PAT every 90 days.
- If GitHub flags or limits the account, stop and contact Support; do not create another account.
