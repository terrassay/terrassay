#!/bin/sh
# Show the Terrassay-local Superagent status.
set -eu

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SESSION="terrassay-superagent"
PORT=3092

if tmux has-session -t "$SESSION" 2>/dev/null; then
  echo "tmux: running ($SESSION)"
else
  echo "tmux: not running ($SESSION)"
fi

code="$(curl -s -o /dev/null -w '%{http_code}' "http://127.0.0.1:$PORT/" || true)"
echo "panel: http://127.0.0.1:$PORT (HTTP $code)"

if [ -d "$ROOT/.superagent/dsh-superagent/.artifacts" ]; then
  echo "latest logs:"
  ls -1t "$ROOT/.superagent/dsh-superagent/.artifacts" 2>/dev/null | head -3
fi
