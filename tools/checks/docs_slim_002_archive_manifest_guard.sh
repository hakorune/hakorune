#!/usr/bin/env bash
set -euo pipefail

TAG="docs-slim-002-archive-manifest"
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

PHASE="docs/development/current/main/phases/phase-293x"
POLICY="docs/development/current/main/design/current-docs-archive-policy-ssot.md"
LAYOUT="docs/development/current/main/DOCS_LAYOUT.md"
CURRENT_STATE="docs/development/current/main/CURRENT_STATE.toml"
CHECK_INDEX="docs/tools/check-scripts-index.md"
CARD="$PHASE/293x-409-DOCS-SLIM-002-ARCHIVE-MANIFEST-PREP.md"
ARCHIVE_README="$PHASE/archive/README.md"
CARDS_README="$PHASE/archive/cards/README.md"
MANIFEST="$PHASE/archive/cards/phase-293x-card-archive-manifest.md"
DOCS_SLIM_001_GUARD="tools/checks/docs_slim_001_archive_policy_guard.sh"

fail() {
  echo "[$TAG] ERROR: $*" >&2
  exit 1
}

require_file() {
  local path="$1"
  [[ -f "$path" ]] || fail "missing file: $path"
}

require_text() {
  local path="$1"
  local pattern="$2"
  rg -q "$pattern" "$path" || fail "missing pattern '$pattern' in $path"
}

count_root_cards() {
  find "$PHASE" -maxdepth 1 -type f -name '293x-[0-9][0-9][0-9]-*.md' \
    ! -name '*taskboard*' | wc -l | tr -d ' '
}

count_range() {
  local lo="$1"
  local hi="$2"
  find "$PHASE" -maxdepth 1 -type f -name '293x-[0-9][0-9][0-9]-*.md' \
    ! -name '*taskboard*' \
    | sed -E 's#.*/293x-([0-9][0-9][0-9])-.*#\1#' \
    | awk -v lo="$lo" -v hi="$hi" '{ n = $1 + 0; if (n >= lo && n <= hi) c++ } END { print c + 0 }'
}

expect_count() {
  local label="$1"
  local actual="$2"
  local expected="$3"
  [[ "$actual" == "$expected" ]] || fail "$label count expected $expected, got $actual"
}

echo "[$TAG] running DOCS-SLIM-002 archive manifest guard"

for path in "$POLICY" "$LAYOUT" "$CURRENT_STATE" "$CHECK_INDEX" "$CARD" \
  "$ARCHIVE_README" "$CARDS_README" "$MANIFEST" "$DOCS_SLIM_001_GUARD"; do
  require_file "$path"
done

require_text "$POLICY" "DOCS-SLIM-002"
require_text "$LAYOUT" "phase-293x/archive/cards/phase-293x-card-archive-manifest.md"
require_text "$CHECK_INDEX" "docs_slim_002_archive_manifest_guard.sh"
require_text "$CURRENT_STATE" 'latest_card = "293x-409-DOCS-SLIM-002-ARCHIVE-MANIFEST-PREP"'
require_text "$CARD" "Do not move numbered cards in this row"
require_text "$MANIFEST" 'Do not physically move phase cards in `DOCS-SLIM-002`'
require_text "$MANIFEST" "293x-400-499: 10"
require_text "$ARCHIVE_README" "creates the archive entry and manifest only"
require_text "$CARDS_README" "Do not leave both full copies live"

if rg -n 'latest_card = "293x-408-DOCS-SLIM-001-ARCHIVE-POLICY-AND-INVENTORY"' "$DOCS_SLIM_001_GUARD" >/tmp/docs_slim_002_hits.$$ 2>/dev/null; then
  echo "[$TAG] ERROR: DOCS-SLIM-001 guard must not pin CURRENT_STATE latest_card after later docs-slim rows" >&2
  cat /tmp/docs_slim_002_hits.$$ >&2
  rm -f /tmp/docs_slim_002_hits.$$
  exit 1
fi
rm -f /tmp/docs_slim_002_hits.$$

expect_count "root numbered card" "$(count_root_cards)" "409"
expect_count "293x-000-099" "$(count_range 0 99)" "99"
expect_count "293x-100-199" "$(count_range 100 199)" "100"
expect_count "293x-200-299" "$(count_range 200 299)" "100"
expect_count "293x-300-399" "$(count_range 300 399)" "100"
expect_count "293x-400-499" "$(count_range 400 499)" "10"

moved_cards=$(
  find "$PHASE/archive/cards" -type f -name '293x-[0-9][0-9][0-9]-*.md' \
    | wc -l | tr -d ' '
)
expect_count "archived numbered card" "$moved_cards" "0"

direct_guard_refs=$(rg -l 'docs/development/current/main/phases/phase-293x/293x-[0-9][0-9][0-9]-' tools/checks | wc -l | tr -d ' ')
if (( direct_guard_refs < 200 )); then
  fail "expected direct guard reference risk to remain visible; got $direct_guard_refs guard files"
fi

unique_card_refs=$(
  rg -o '293x-[0-9][0-9][0-9]-[A-Za-z0-9_.-]+\.md' \
    tools/checks docs/development/current/main/CURRENT_STATE.toml CURRENT_TASK.md \
    docs/development/current/main/05-Restart-Quick-Resume.md \
    docs/development/current/main/10-Now.md \
    docs/development/current/main/phases/phase-293x/README.md \
    docs/tools/check-scripts-index.md \
    | sed -E 's/.*(293x-[0-9][0-9][0-9]-[^:]+\.md).*/\1/' \
    | sort -u \
    | wc -l \
    | tr -d ' '
)
if (( unique_card_refs < 200 )); then
  fail "expected unique phase-card reference risk to remain visible; got $unique_card_refs references"
fi

echo "[$TAG] ok root_cards=$(count_root_cards) direct_guard_refs=$direct_guard_refs unique_card_refs=$unique_card_refs"
