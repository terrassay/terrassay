#!/bin/sh
# Store the Terrassay fine-grained GitHub PAT in macOS Keychain.
# The token is entered at a hidden prompt; it is never written to disk or shell history.
set -eu

SERVICE="terrassay-github-pat"
ACCOUNT="atpl-training"

if ! command -v security >/dev/null 2>&1; then
  echo "error: macOS 'security' command not found" >&2
  exit 1
fi

if security find-generic-password -a "$ACCOUNT" -s "$SERVICE" >/dev/null 2>&1; then
  echo "Use a classic PAT with scopes: public_repo, notifications."
echo "Updating existing Keychain item '$SERVICE'..."
  security add-generic-password -U -a "$ACCOUNT" -s "$SERVICE" -w
else
  echo "Use a classic PAT with scopes: public_repo, notifications."
echo "Creating Keychain item '$SERVICE'..."
  security add-generic-password -a "$ACCOUNT" -s "$SERVICE" -w
fi

echo
echo "Stored. Test with:"
echo "  scripts/gh-terrassay api user --jq .login"
echo
echo "Expected output: atpl-training"
