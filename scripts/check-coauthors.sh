#!/usr/bin/env sh
# Fails when a commit carries a Co-authored-by trailer with an email nobody
# issued. Agents and coding CLIs commit with the identity the platform gives
# them - <Name> <noreply@agentes.chat>, <CLI> <noreply@anthropic.com> - so an
# invented address (an agent's own domain, a person's real mail) would credit
# the wrong account. History rewriting is not allowed on main: catch it on the
# PR instead.
#
# Usage: scripts/check-coauthors.sh [base-ref]   (default: origin/main)

set -eu

base="${1:-origin/main}"
allowed='^(noreply@agentes\.chat|noreply@anthropic\.com|noreply@openai\.com|[0-9]+\+capy-ai\[bot\]@users\.noreply\.github\.com)$'
failed=0

for sha in $(git rev-list "$base..HEAD"); do
  while IFS= read -r author; do
    [ -n "$author" ] || continue
    email=$(printf '%s' "$author" | sed -n 's/.*<\(.*\)>.*/\1/p')
    if printf '%s' "$email" | grep -Eq "$allowed"; then
      continue
    fi
    printf 'commit %s has an invented co-author identity: %s\n' \
      "$(git log -1 --format='%h' "$sha")" "$author"
    failed=1
  done <<EOF
$(git log -1 --format='%(trailers:key=Co-authored-by,valueonly)' "$sha")
EOF
done

if [ "$failed" -ne 0 ]; then
  cat <<'MSG'

Use only the identity the platform adds on commit:
  Co-authored-by: <Agent> <noreply@agentes.chat>
  Co-authored-by: <Coding CLI> <noreply@anthropic.com>
Never type a co-author by hand: an address you make up credits a real account.
MSG
  exit 1
fi
