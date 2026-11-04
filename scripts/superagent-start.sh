#!/bin/sh
# Start the Terrassay-local dsh-superagent profile.
# Uses its own DSH_HOME, tmux session, and port so it cannot collide with other profiles.
set -eu

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
export DSH_HOME="$ROOT/.dsh"
SESSION="terrassay-superagent"
PORT=3092

if [ ! -d "$ROOT/.superagent/dsh-superagent" ]; then
  echo "error: Superagent is not installed at $ROOT/.superagent/dsh-superagent" >&2
  echo "See docs/SUPERAGENT_LOCAL.md" >&2
  exit 1
fi

if tmux has-session -t "$SESSION" 2>/dev/null; then
  echo "Superagent is already running in tmux session '$SESSION'."
  echo "Panel: http://127.0.0.1:$PORT"
  exit 0
fi

tmux new-session -d -s "$SESSION" \
  "cd '$ROOT/.superagent/dsh-superagent' && DSH_HOME='$DSH_HOME' dsh --profile superagent"

echo "Started tmux session '$SESSION'."
echo "Panel: http://127.0.0.1:$PORT"
echo "Logs:  $ROOT/.superagent/dsh-superagent/.artifacts/"
