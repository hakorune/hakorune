#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DEFAULT_DIR="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps"
TARGET_DIR="${SMOKE_INVENTORY_DIR:-$DEFAULT_DIR}"
OUT_DIR="${SMOKE_INVENTORY_OUT_DIR:-$ROOT_DIR/target/smoke_inventory}"
INCLUDE_ARCHIVE="${SMOKE_INVENTORY_INCLUDE_ARCHIVE:-0}"
INCLUDE_PRUNED="${SMOKE_INVENTORY_INCLUDE_PRUNED:-$INCLUDE_ARCHIVE}"
PRUNE_DIRS="${SMOKE_INVENTORY_PRUNE_DIRS:-archive:lib:tmp:fixtures}"
mkdir -p "$OUT_DIR"

if rel_path="$(realpath --relative-to="$ROOT_DIR/tools/smokes/v2/profiles" "$TARGET_DIR" 2>/dev/null)"; then
  LABEL_DEFAULT="$(printf '%s' "$rel_path" | tr '/-' '__')"
else
  LABEL_DEFAULT="$(basename "$TARGET_DIR" | tr '-' '_')"
fi
LABEL="${SMOKE_INVENTORY_LABEL:-$LABEL_DEFAULT}"

REPORT_TSV="$OUT_DIR/${LABEL}_inventory.tsv"
SUMMARY_TXT="$OUT_DIR/${LABEL}_summary.txt"

if [[ ! -d "$TARGET_DIR" ]]; then
  echo "[FAIL] missing inventory directory: $TARGET_DIR" >&2
  exit 1
fi

if [[ "$INCLUDE_PRUNED" == "1" ]]; then
  mapfile -t scripts < <(find "$TARGET_DIR" -type f -name '*.sh' | sort)
else
  find_args=()
  IFS=':' read -r -a prune_names <<< "$PRUNE_DIRS"
  prune_added=0
  for prune_name in "${prune_names[@]}"; do
    [[ -n "$prune_name" ]] || continue
    if [[ "$prune_added" -eq 0 ]]; then
      find_args+=( "(" -type d "(" -name "$prune_name" )
    else
      find_args+=( -o -name "$prune_name" )
    fi
    prune_added=1
  done
  if [[ "$prune_added" -eq 1 ]]; then
    find_args+=( ")" -prune ")" -o "(" -type f -name '*.sh' -print ")" )
  else
    find_args=( -type f -name '*.sh' -print )
  fi
  mapfile -t scripts < <(find "$TARGET_DIR" "${find_args[@]}" | sort)
fi

{
  echo "path	family	suffix	fullpath_ref_count	basename_ref_count	wrapper_only	class"
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

    if rg -q '^[[:space:]]*exec[[:space:]]+".*\.sh"[[:space:]]*' "$path" \
      && ! rg -q 'test_runner\.sh|require_env|test_pass|test_fail' "$path"; then
      wrapper_only="1"
    else
      wrapper_only="0"
    fi

    if [[ "$fullpath_ref_count" -eq 0 && "$basename_ref_count" -eq 0 ]]; then
      if [[ "$wrapper_only" -eq 1 ]]; then
        class="orphan_wrapper_candidate"
      else
        class="orphan_candidate"
      fi
    else
      class="referenced"
    fi

    printf "%s\t%s\t%s\t%s\t%s\t%s\t%s\n" \
      "$path" "$family" "$suffix" "$fullpath_ref_count" "$basename_ref_count" "$wrapper_only" "$class"
  done
} > "$REPORT_TSV"

total="$(wc -l < "$REPORT_TSV")"
data_rows="$(( total - 1 ))"
orphans="$(awk -F'\t' 'NR>1 && ($7=="orphan_candidate" || $7=="orphan_wrapper_candidate"){c++} END{print c+0}' "$REPORT_TSV")"
orphan_wrappers="$(awk -F'\t' 'NR>1 && $7=="orphan_wrapper_candidate"{c++} END{print c+0}' "$REPORT_TSV")"
referenced="$(( data_rows - orphans ))"

{
  echo "Smoke Inventory Summary"
  echo "Date: $(date -u '+%Y-%m-%dT%H:%M:%SZ')"
  echo "Dir: $TARGET_DIR"
  echo "Label: $LABEL"
  echo "Include pruned buckets: $INCLUDE_PRUNED"
  echo "Pruned dir names: $PRUNE_DIRS"
  echo "Total: $data_rows"
  echo "Referenced: $referenced"
  echo "Orphan candidates: $orphans"
  echo "  - Wrapper-only orphan candidates: $orphan_wrappers"
  echo
  echo "Suffix breakdown:"
  awk -F'\t' 'NR>1{c[$3]++} END{for (k in c) printf "  %s\t%d\n", k, c[k]}' "$REPORT_TSV" | sort
  echo
  echo "Top families:"
  awk -F'\t' 'NR>1{c[$2]++} END{for (k in c) printf "  %s\t%d\n", k, c[k]}' "$REPORT_TSV" | sort -k2,2nr | head -n 20
  echo
  echo "Top orphan candidate families:"
  awk -F'\t' 'NR>1 && ($7=="orphan_candidate" || $7=="orphan_wrapper_candidate"){c[$2]++} END{for (k in c) printf "  %s\t%d\n", k, c[k]}' "$REPORT_TSV" | sort -k2,2nr | head -n 20
  echo
  echo "Top orphan wrapper candidate families:"
  awk -F'\t' 'NR>1 && $7=="orphan_wrapper_candidate"{c[$2]++} END{for (k in c) printf "  %s\t%d\n", k, c[k]}' "$REPORT_TSV" | sort -k2,2nr | head -n 20
} > "$SUMMARY_TXT"

echo "[PASS] smoke inventory written:"
echo "  - $SUMMARY_TXT"
echo "  - $REPORT_TSV"
