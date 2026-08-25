#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mir-metadata-consumer-manifest-guard"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CHECKER="tools/checks/mir_metadata_consumer_manifest.py"
MANIFEST="tools/checks/manifests/mir_function_metadata_consumer_manifest_v1.json"
SOURCE="src/mir/function/metadata.rs"
CHECKED_ACCESS="src/mir/function/metadata/checked_callout_access.rs"
LINEAR_ACCESS="src/mir/function/metadata/linear_slot_access.rs"
INDEX="docs/tools/check-scripts-index.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$CHECKER" "$MANIFEST" "$SOURCE" "$CHECKED_ACCESS" "$LINEAR_ACCESS" "$INDEX"
guard_require_exec_files "$TAG" "$CHECKER"
guard_expect_in_file "$TAG" "mir_metadata_consumer_manifest_guard.sh" "$INDEX" \
  "check index must list the metadata consumer manifest guard"

for source_file in "$SOURCE" "$CHECKED_ACCESS" "$LINEAR_ACCESS"; do
  source_lines="$(wc -l < "$source_file" | tr -d '[:space:]')"
  if (( source_lines >= 760 )); then
    guard_fail "$TAG" "$source_file exceeds the 760-line split threshold: $source_lines"
  fi
done
guard_expect_in_file "$TAG" "mod checked_callout_access;" "$SOURCE" \
  "metadata owner must retain the checked-callout nested module"
guard_expect_in_file "$TAG" "mod linear_slot_access;" "$SOURCE" \
  "metadata owner must retain the linear-slot nested module"
if rg -n -F '#[path]' "$SOURCE" "$CHECKED_ACCESS" "$LINEAR_ACCESS" >/dev/null; then
  guard_fail "$TAG" "metadata owner split must not add #[path] module glue"
fi

python3 "$CHECKER" --root "$ROOT_DIR"

tmp_dir="$(mktemp -d "/tmp/${TAG}.XXXXXX")"
trap 'rm -f "$tmp_dir/missing.json" "$tmp_dir/duplicate.json" "$tmp_dir/out"; rmdir "$tmp_dir" 2>/dev/null || true' EXIT

python3 - "$MANIFEST" "$tmp_dir/missing.json" "$tmp_dir/duplicate.json" <<'PY'
import json
import sys

source, missing, duplicate = sys.argv[1:]
data = json.load(open(source, encoding="utf-8"))
data["rows"] = data["rows"][:-1]
json.dump(data, open(missing, "w", encoding="utf-8"), indent=2)
data = json.load(open(source, encoding="utf-8"))
data["rows"][-1]["field"] = data["rows"][0]["field"]
json.dump(data, open(duplicate, "w", encoding="utf-8"), indent=2)
PY

if python3 "$CHECKER" --root "$ROOT_DIR" --manifest "$tmp_dir/missing.json" >"$tmp_dir/out" 2>&1; then
  echo "[$TAG] ERROR: missing-field manifest unexpectedly passed" >&2
  exit 1
fi
grep -Fq "row count drift" "$tmp_dir/out" || {
  echo "[$TAG] ERROR: missing-field negative did not report row count drift" >&2
  cat "$tmp_dir/out" >&2
  exit 1
}

if python3 "$CHECKER" --root "$ROOT_DIR" --manifest "$tmp_dir/duplicate.json" >"$tmp_dir/out" 2>&1; then
  echo "[$TAG] ERROR: duplicate-field manifest unexpectedly passed" >&2
  exit 1
fi
grep -Fq "duplicate family/field row" "$tmp_dir/out" || {
  echo "[$TAG] ERROR: duplicate-field negative did not report duplicate row" >&2
  cat "$tmp_dir/out" >&2
  exit 1
}

echo "[$TAG] ok fields=127 missing=reject duplicate=reject"
