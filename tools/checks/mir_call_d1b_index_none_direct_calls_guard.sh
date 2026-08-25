#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mir-call-d1b-index-none-direct-calls"
RESOLVER="$ROOT_DIR/src/mir/resolved_semantics/resolver.rs"
TESTS="$ROOT_DIR/src/mir/resolved_semantics/resolver_tests.rs"
MANIFEST="$ROOT_DIR/tools/checks/guard_rows.toml"

fail() {
  echo "[$TAG] $*" >&2
  exit 1
}

for file in "$RESOLVER" "$TESTS" "$MANIFEST"; do
  [[ -f "$file" ]] || fail "missing owner ${file#$ROOT_DIR/}"
done

python3 - "$RESOLVER" "$TESTS" "$MANIFEST" <<'PY'
from pathlib import Path
import sys

resolver, tests, manifest = map(Path, sys.argv[1:])
resolver_text = resolver.read_text()
tests_text = tests.read_text()
manifest_text = manifest.read_text()

if "if callable_index.is_none() && !draft.direct_calls.is_empty()" not in resolver_text:
    raise SystemExit("index-none direct-call rejection is missing")
if '"direct calls require a callable index"' not in resolver_text:
    raise SystemExit("stable DraftInvariant message is missing")
if "None => BTreeMap::new()," not in resolver_text:
    raise SystemExit("empty no-call path was not retained")
if "canonical_resolver_rejects_unindexed_direct_call_draft" not in tests_text:
    raise SystemExit("focused negative witness is missing")
if 'id = "mir-call-d1b-index-none-direct-calls"' not in manifest_text:
    raise SystemExit("guard row is not registered")
PY

echo "[$TAG] ok"
