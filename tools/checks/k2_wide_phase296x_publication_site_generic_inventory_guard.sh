#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

CARD="docs/development/current/main/phases/phase-296x/296x-830-PUBLICATION-SITE-INVENTORY-GENERIC-001.md"
PREV_CARD="docs/development/current/main/phases/phase-296x/296x-829-ROUTEPLAN-OBJECTPLAN-HANDOFF-001.md"
TOOL="tools/allocator/hako_publication_site_generic_inventory.py"
SRC="src/object_storage_plan.rs"
INDEX="docs/tools/check-scripts-index.md"
SELF_SCRIPT="tools/checks/k2_wide_phase296x_publication_site_generic_inventory_guard.sh"

[[ -f "$CARD" ]] || { echo "[publication-site-generic-inventory] missing card: $CARD" >&2; exit 1; }
[[ -f "$PREV_CARD" ]] || { echo "[publication-site-generic-inventory] missing previous card: $PREV_CARD" >&2; exit 1; }
[[ -f "$TOOL" ]] || { echo "[publication-site-generic-inventory] missing tool: $TOOL" >&2; exit 1; }
[[ -f "$SRC" ]] || { echo "[publication-site-generic-inventory] missing source: $SRC" >&2; exit 1; }

grep -q '^Status: Landed$' "$CARD" || {
  echo "[publication-site-generic-inventory] card must be Landed" >&2
  exit 1
}
grep -q '^Status: Landed$' "$PREV_CARD" || {
  echo "[publication-site-generic-inventory] previous card must be Landed" >&2
  exit 1
}
grep -q "$SELF_SCRIPT" "$INDEX" || {
  echo "[publication-site-generic-inventory] check index missing guard entry" >&2
  exit 1
}
grep -q "$TOOL" "$INDEX" || {
  echo "[publication-site-generic-inventory] check index missing tool entry" >&2
  exit 1
}

require_line_in_file() {
  local file="$1"
  local expected="$2"
  if ! grep -F -x -q "$expected" "$file"; then
    echo "[publication-site-generic-inventory] missing line in $file: $expected" >&2
    exit 1
  fi
}

tmp_dir="$(mktemp -d /tmp/hakorune_publication_site_generic_inventory.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
report="$tmp_dir/report.out"
python3 "$TOOL" --out "$report"

for expected in \
  "output_contract=hako-publication-site-generic-inventory-v0" \
  "source_evidence=296x-828,296x-829" \
  "source_file=src/object_storage_plan.rs" \
  "inventory_kind=code_vocabulary" \
  "publication_reason_vocabulary_count=8" \
  "publication_reason_expected_count=8" \
  "publication_reason_missing_count=0" \
  "publication_reason_extra_count=0" \
  "publication_reason_plugin_or_extern=1" \
  "publication_reason_host_handle_required=1" \
  "publication_reason_dynamic_array_or_map=1" \
  "publication_reason_dynamic_nyashbox_api=1" \
  "publication_reason_return_as_dynamic_box=1" \
  "publication_reason_task_future_channel_boundary=1" \
  "publication_reason_unknown_fini_or_drop=1" \
  "publication_reason_unknown=1" \
  "unknown_publication_forces_generic_fallback=1" \
  "standalone_publication_plan_enabled=0" \
  "objectplan_execution_enabled=0" \
  "backend_consumes_objectplan=0" \
  "product_default_changed=0" \
  "summary=ok"; do
  require_line_in_file "$CARD" "$expected"
  require_line_in_file "$report" "$expected"
done

for token in \
  "PluginOrExternBoundary" \
  "HostHandleRequired" \
  "DynamicArrayOrMapStorage" \
  "DynamicNyashBoxApi" \
  "ReturnAsDynamicBox" \
  "TaskFutureChannelBoundary" \
  "UnknownFiniOrDrop" \
  "Unknown" \
  "(\"publication_site_generic_inventory_defined\", \"1\")" \
  "(\"publication_reason_vocabulary_count\", \"8\")" \
  "(\"unknown_publication_forces_generic_fallback\", \"1\")"; do
  grep -F -q "$token" "$SRC" || {
    echo "[publication-site-generic-inventory] missing source token: $token" >&2
    exit 1
  }
done

for stop_line in \
  "do not open a new source-front pilot from this row" \
  "do not infer publication sites from helper names" \
  "do not split standalone PublicationPlan from this row" \
  "do not let backend consume ObjectPlan from this row" \
  "do not change product default runtime behavior"; do
  grep -F -q "$stop_line" "$CARD" || {
    echo "[publication-site-generic-inventory] missing stop line: $stop_line" >&2
    exit 1
  }
done

require_line_in_file "$CARD" "selected_next=BACKEND-PLAN-CONSUMER-GUARD-001"

echo "[publication-site-generic-inventory] ok"
