#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DEFAULT_DIR="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps"
TARGET_DIR_INPUT="${SMOKE_INVENTORY_DIR:-$DEFAULT_DIR}"
# normalize to absolute to avoid profile detection failures on relative input
if ! TARGET_DIR="$(realpath "$TARGET_DIR_INPUT" 2>/dev/null)"; then
  echo "[FAIL] invalid inventory directory: $TARGET_DIR_INPUT" >&2
  exit 1
fi
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

# Derive profile name (first path element under profiles) for suite lookup.
PROFILE_NAME=""
TARGET_SCOPE_SUBPATH=""
if [[ "$TARGET_DIR" == "$ROOT_DIR/tools/smokes/v2/profiles/"* ]]; then
  rel_profile="$(realpath --relative-to="$ROOT_DIR/tools/smokes/v2/profiles" "$TARGET_DIR")"
  PROFILE_NAME="${rel_profile%%/*}"
  # Guard against cases like TARGET_DIR exactly profiles/
  if [[ "$PROFILE_NAME" == "$rel_profile" ]]; then
    PROFILE_NAME=""
  else
    TARGET_SCOPE_SUBPATH="${rel_profile#${PROFILE_NAME}/}"
  fi
fi

declare -A SUITE_MEMBERS=()   # key: profile-relative path, val: comma-separated suite names (built later)
declare -A SUITE_TOTAL=()     # key: suite -> total entries
declare -A SUITE_HIT=()       # key: suite -> hits
declare -A DISCOVERED_PATHS=()

suite_entry_in_scope() {
  local entry="$1"
  if [[ -z "$TARGET_SCOPE_SUBPATH" ]]; then
    return 0
  fi
  case "$entry" in
    "$TARGET_SCOPE_SUBPATH"|"$TARGET_SCOPE_SUBPATH"/*)
      return 0
      ;;
  esac
  return 1
}

load_suites() {
  [[ -n "$PROFILE_NAME" ]] || return 0
  local suite_dir="$ROOT_DIR/tools/smokes/v2/suites/$PROFILE_NAME"
  [[ -d "$suite_dir" ]] || return 0

  local suite_file suite_name line rel
  for suite_file in "$suite_dir"/*.txt; do
    [[ -f "$suite_file" ]] || continue
    suite_name="$(basename "$suite_file" .txt)"
    SUITE_TOTAL["$suite_name"]=0
    SUITE_HIT["$suite_name"]=0

    while IFS= read -r line || [[ -n "$line" ]]; do
      # trim
      line="${line#"${line%%[![:space:]]*}"}"
      line="${line%"${line##*[![:space:]]}"}"
      case "$line" in
        ""|\#*) continue ;;
      esac
      rel="$line"
      # forbid absolute paths; fail-fast only for malformed entries
      if [[ "$rel" == /* ]]; then
        echo "[FAIL] suite entry must be profile-relative: $suite_file -> $rel" >&2
        exit 1
      fi
      suite_entry_in_scope "$rel" || continue
      SUITE_TOTAL["$suite_name"]=$(( SUITE_TOTAL["$suite_name"] + 1 ))
      # record membership; we will mark hits after discovery
      if [[ -n "${SUITE_MEMBERS[$rel]+x}" ]]; then
        SUITE_MEMBERS["$rel"]+=",${suite_name}"
      else
        SUITE_MEMBERS["$rel"]="$suite_name"
      fi
    done < "$suite_file"
  done
}

load_suites

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
  echo "path	family	suffix	fullpath_ref_count	basename_ref_count	wrapper_only	suite_hit_count	suite_names	class"
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

    # suite membership
    suites=""
    suite_count=0
    if [[ -n "$PROFILE_NAME" ]]; then
      rel_profile_path="$(realpath --relative-to="$ROOT_DIR/tools/smokes/v2/profiles/$PROFILE_NAME" "$path")"
      DISCOVERED_PATHS["$rel_profile_path"]=1
      if [[ -n "${SUITE_MEMBERS[$rel_profile_path]+x}" ]]; then
        suites="${SUITE_MEMBERS[$rel_profile_path]}"
        # count memberships
        suite_count=$(awk -F',' '{print NF}' <<<"$suites")
        # mark hit for each suite
        IFS=',' read -r -a suite_list <<< "$suites"
        for s in "${suite_list[@]}"; do
          SUITE_HIT["$s"]=$(( SUITE_HIT["$s"] + 1 ))
        done
      fi
    fi

    printf "%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n" \
      "$path" "$family" "$suffix" "$fullpath_ref_count" "$basename_ref_count" "$wrapper_only" "$suite_count" "${suites:-}" "$class"
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
  # `head` intentionally truncates the sorted stream; ignore the expected SIGPIPE under pipefail.
  echo "Top families:"
  awk -F'\t' 'NR>1{c[$2]++} END{for (k in c) printf "  %s\t%d\n", k, c[k]}' "$REPORT_TSV" | sort -k2,2nr | head -n 20 || true
  echo
  echo "Top orphan candidate families:"
  awk -F'\t' 'NR>1 && ($7=="orphan_candidate" || $7=="orphan_wrapper_candidate"){c[$2]++} END{for (k in c) printf "  %s\t%d\n", k, c[k]}' "$REPORT_TSV" | sort -k2,2nr | head -n 20 || true
  echo
  echo "Top orphan wrapper candidate families:"
  awk -F'\t' 'NR>1 && $7=="orphan_wrapper_candidate"{c[$2]++} END{for (k in c) printf "  %s\t%d\n", k, c[k]}' "$REPORT_TSV" | sort -k2,2nr | head -n 20 || true

  if [[ -n "$PROFILE_NAME" ]]; then
  echo
  echo "Suite coverage (profile: $PROFILE_NAME):"
  if compgen -A variable SUITE_TOTAL >/dev/null; then
    echo "  Scope: ${TARGET_SCOPE_SUBPATH:-<profile-root>}"
    for suite in "${!SUITE_TOTAL[@]}"; do
      total_entries="${SUITE_TOTAL[$suite]}"
      hits="${SUITE_HIT[$suite]:-0}"
      missing=$(( total_entries - hits ))
      printf "  %s\t%d/%d (missing %d)\n" "$suite" "$hits" "$total_entries" "$missing"
      done | sort

      # list missing entries for transparency (non-fatal)
      echo
      echo "Suite missing entries (not found after prune/discovery):"
      for suite in "${!SUITE_TOTAL[@]}"; do
        total_entries="${SUITE_TOTAL[$suite]}"
        hits="${SUITE_HIT[$suite]:-0}"
        if (( total_entries == hits )); then
          continue
        fi
        printf "  %s:\n" "$suite"
        suite_dir="$ROOT_DIR/tools/smokes/v2/suites/$PROFILE_NAME/$suite.txt"
        while IFS= read -r line || [[ -n "$line" ]]; do
          line="${line#"${line%%[![:space:]]*}"}"
          line="${line%"${line##*[![:space:]]}"}"
          case "$line" in
            ""|\#*) continue ;;
          esac
          suite_entry_in_scope "$line" || continue
          if [[ -n "${DISCOVERED_PATHS[$line]+x}" ]]; then
            continue
          fi
          echo "    - $line"
        done < "$suite_dir"
      done
    else
      echo "  <no suite manifests found>"
    fi
  fi
} > "$SUMMARY_TXT"

echo "[PASS] smoke inventory written:"
echo "  - $SUMMARY_TXT"
echo "  - $REPORT_TSV"
