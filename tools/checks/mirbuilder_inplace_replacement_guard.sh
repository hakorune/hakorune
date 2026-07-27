#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mirbuilder-inplace-replacement-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

MANIFEST="$ROOT_DIR/docs/development/current/main/design/fixtures/mirbuilder-inplace-replacement-v1.tsv"
LOWERING="$ROOT_DIR/src/mir/builder/calls/lowering.rs"
PORT_OWNER="$ROOT_DIR/src/mir/builder/port_aware_function_draft_impl.rs"

guard_require_command "$TAG" awk
guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$MANIFEST" "$LOWERING" "$PORT_OWNER"

expected_header=$'record_kind\tid\tpack\tproduction_caller\tnew_owner\tdelete_target\tparity_gate\tdisposition\tstate'
actual_header="$(head -n1 "$MANIFEST")"
if [[ "$actual_header" != "$expected_header" ]]; then
  guard_fail "$TAG" "manifest header drift"
fi

if ! awk -F '\t' '
  NR == 1 { next }
  NF != 9 { exit 1 }
  $1 !~ /^(cell|asset)$/ { exit 1 }
  $9 !~ /^(pending|active|closed|explicit-residual)$/ { exit 1 }
  $1 == "asset" && $8 !~ /^(IntegrateNow|ReuseNeutral|FixtureOnly|Delete)$/ { exit 1 }
  END { if (NR < 2) exit 1 }
' "$MANIFEST"; then
  guard_fail "$TAG" "manifest row contract failed"
fi

current_row_count="$(awk -F '\t' '
  $1 == "cell" &&
  $2 == "CALLABLE-DRAFT-PORT-CUTOVER0" &&
  $9 == "closed" { count += 1 }
  END { print count + 0 }
' "$MANIFEST")"
if [[ "$current_row_count" != "1" ]]; then
  guard_fail "$TAG" "callable draft cutover must have one closed manifest row"
fi

for symbol in \
  build_static_method_draft_v1 \
  build_instance_method_draft_v1 \
  lower_function_body \
  lower_method_body
do
  if rg -n -w "$symbol" "$ROOT_DIR/src" --glob '*.rs' >/dev/null; then
    guard_fail "$TAG" "retired callable body symbol returned: $symbol"
  fi
done

while read -r symbol expected; do
  count="$(rg -o -F "$symbol" "$LOWERING" | wc -l | tr -d '[:space:]')"
  if [[ "$count" != "$expected" ]]; then
    guard_fail "$TAG" "ordinary production caller count drift: $symbol count=$count expected=$expected"
  fi
done <<'EOF'
build_static_method_draft_with_port_v1 1
build_instance_method_draft_with_port_v1 1
finalize_port_aware_draft_for_legacy_v1 2
EOF

for file in "$LOWERING" "$PORT_OWNER" "$ROOT_DIR/tools/checks/mirbuilder_inplace_replacement_guard.sh"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "file exceeds boundary: ${file#"$ROOT_DIR/"} lines=$lines"
  fi
done

echo "[$TAG] ok"
