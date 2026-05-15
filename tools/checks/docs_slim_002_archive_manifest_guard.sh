#!/usr/bin/env bash
set -euo pipefail

TAG="docs-slim-002-archive-manifest"
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

PHASE="docs/development/current/main/phases/phase-293x"
POLICY="docs/development/current/main/design/current-docs-archive-policy-ssot.md"
LAYOUT="docs/development/current/main/DOCS_LAYOUT.md"
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

echo "[$TAG] running DOCS-SLIM-002 archive manifest guard"

for path in "$POLICY" "$LAYOUT" "$CHECK_INDEX" "$CARD" \
  "$ARCHIVE_README" "$CARDS_README" "$MANIFEST" "$DOCS_SLIM_001_GUARD"; do
  require_file "$path"
done

require_text "$POLICY" "DOCS-SLIM-002"
require_text "$LAYOUT" "phase-293x/archive/cards/phase-293x-card-archive-manifest.md"
require_text "$CHECK_INDEX" "docs_slim_002_archive_manifest_guard.sh"
require_text "$CARD" "Do not move numbered cards in this row"
require_text "$MANIFEST" 'Do not physically move phase cards in `DOCS-SLIM-002`'
require_text "$MANIFEST" "293x-400-499: 10"
require_text "$MANIFEST" "Reference Risk"
require_text "$ARCHIVE_README" "creates the archive entry and manifest only"
require_text "$CARDS_README" "Do not leave both full copies live"

pin_pattern='latest_card = "'
old_docs_slim_001_card='293x-408-DOCS-SLIM-001-ARCHIVE-POLICY-AND-INVENTORY'
if rg -n "${pin_pattern}${old_docs_slim_001_card}" "$DOCS_SLIM_001_GUARD" >/tmp/docs_slim_002_hits.$$ 2>/dev/null; then
  echo "[$TAG] ERROR: DOCS-SLIM-001 guard must not pin CURRENT_STATE latest_card after later docs-slim rows" >&2
  cat /tmp/docs_slim_002_hits.$$ >&2
  rm -f /tmp/docs_slim_002_hits.$$
  exit 1
fi
rm -f /tmp/docs_slim_002_hits.$$

echo "[$TAG] ok"
