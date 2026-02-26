#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
APPS_DIR="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps"
OUT_DIR="${SMOKE_INVENTORY_OUT_DIR:-$ROOT_DIR/target/smoke_inventory}"
mkdir -p "$OUT_DIR"

REPORT_TSV="$OUT_DIR/integration_apps_inventory.tsv"
SUMMARY_TXT="$OUT_DIR/integration_apps_summary.txt"

if [[ ! -d "$APPS_DIR" ]]; then
  echo "[FAIL] missing apps directory: $APPS_DIR" >&2
  exit 1
fi

mapfile -t scripts < <(find "$APPS_DIR" -type f -name '*.sh' | sort)

{
  echo "path	family	suffix	fullpath_ref_count	basename_ref_count	class"
  for path in "${scripts[@]}"; do
    base="$(basename "$path")"
    stem="${base%.sh}"
    family="$(printf "%s" "$stem" | sed -E 's/^(phase[0-9]+).*$/\1/')"
    if [[ "$stem" =~ _vm$ ]]; then
      suffix="vm"
    elif [[ "$stem" =~ _llvm_exe$ ]]; then
      suffix="llvm_exe"
    else
      suffix="other"
    fi

    # Exclude inventory script itself and this apps path from self hit inflation.
    fullpath_ref_count="$(
      { rg -nF -- "$path" "$ROOT_DIR/tools/checks" "$ROOT_DIR/tools/smokes" -g '*.sh' -g '!**/smoke_inventory_report.sh' 2>/dev/null || true; } \
        | wc -l | tr -d ' '
    )"
    basename_ref_count="$(
      { rg -nF -- "$base" "$ROOT_DIR/tools/checks" "$ROOT_DIR/tools/smokes" -g '*.sh' -g '!**/smoke_inventory_report.sh' 2>/dev/null || true; } \
        | wc -l | tr -d ' '
    )"

    if [[ "$fullpath_ref_count" -eq 0 && "$basename_ref_count" -eq 0 ]]; then
      class="orphan_candidate"
    else
      class="referenced"
    fi

    printf "%s\t%s\t%s\t%s\t%s\t%s\n" \
      "$path" "$family" "$suffix" "$fullpath_ref_count" "$basename_ref_count" "$class"
  done
} > "$REPORT_TSV"

total="$(wc -l < "$REPORT_TSV")"
data_rows="$(( total - 1 ))"
orphans="$(awk -F'\t' 'NR>1 && $6=="orphan_candidate"{c++} END{print c+0}' "$REPORT_TSV")"
referenced="$(( data_rows - orphans ))"

{
  echo "Smoke Inventory Summary"
  echo "Date: $(date -u '+%Y-%m-%dT%H:%M:%SZ')"
  echo "Dir: $APPS_DIR"
  echo "Total: $data_rows"
  echo "Referenced: $referenced"
  echo "Orphan candidates: $orphans"
  echo
  echo "Suffix breakdown:"
  awk -F'\t' 'NR>1{c[$3]++} END{for (k in c) printf "  %s\t%d\n", k, c[k]}' "$REPORT_TSV" | sort
  echo
  echo "Top families:"
  awk -F'\t' 'NR>1{c[$2]++} END{for (k in c) printf "  %s\t%d\n", k, c[k]}' "$REPORT_TSV" | sort -k2,2nr | head -n 20
  echo
  echo "Top orphan candidate families:"
  awk -F'\t' 'NR>1 && $6=="orphan_candidate"{c[$2]++} END{for (k in c) printf "  %s\t%d\n", k, c[k]}' "$REPORT_TSV" | sort -k2,2nr | head -n 20
} > "$SUMMARY_TXT"

echo "[PASS] smoke inventory written:"
echo "  - $SUMMARY_TXT"
echo "  - $REPORT_TSV"
