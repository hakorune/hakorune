#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$root"

tag="routing-classification-completeness-guard"
state_doc="docs/development/current/main/CURRENT_STATE.toml"
source "$root/tools/checks/lib/guard_common.sh"

guard_require_command "$tag" rg
guard_require_command "$tag" sed
guard_require_command "$tag" awk
guard_require_command "$tag" wc

toml_scalar() {
  local key="$1"
  sed -n 's/^[[:space:]]*'"$key"'[[:space:]]*=[[:space:]]*"\(.*\)"[[:space:]]*$/\1/p' \
    "$state_doc" | head -n1
}

card_from_current_state() {
  local path
  path="$(toml_scalar latest_card_path)"
  [[ -n "$path" ]] || guard_fail "$tag" "latest_card_path is empty"
  [[ "$path" != /* ]] || guard_fail "$tag" "latest_card_path must be repo-relative"
  [[ "$path" != docs/development/archive/* ]] || \
    guard_fail "$tag" "latest card must not resolve to historical archive"
  printf '%s/%s' "$root" "$path"
}

check_card() {
  local card="$1"
  guard_require_files "$tag" "$card"

  local line_count
  line_count="$(wc -l < "$card" | tr -d '[:space:]')"
  if (( line_count >= 1000 )); then
    guard_fail "$tag" "active card exceeds 999 lines: $card has $line_count"
  fi

  local header_line
  header_line="$(rg -n -i '^[[:space:]]*\|[[:space:]]*(state|outcome)[[:space:]]*\|' \
    "$card" | head -n1 | cut -d: -f1 || true)"
  [[ -n "$header_line" ]] || guard_fail "$tag" "finite state/outcome table header is missing"

  local header
  header="$(sed -n "${header_line}p" "$card")"
  [[ "$header" =~ authority|issuer ]] || \
    guard_fail "$tag" "finite table is missing authority/issuer column"
  [[ "$header" =~ before[[:space:]]+effects|pre-effect ]] || \
    guard_fail "$tag" "finite table is missing pre-effect column"
  [[ "$header" =~ terminal|continuation ]] || \
    guard_fail "$tag" "finite table is missing terminal/continuation column"
  [[ "$header" =~ fallback ]] || \
    guard_fail "$tag" "finite table is missing fallback column"

  local table_rows
  table_rows="$(awk -v start="$header_line" '
    NR > start && /^\|/ {
      if ($0 !~ /^\|[[:space:]:|-]+[[:space:]]*$/) print
    }
    NR > start && !/^\|/ && seen == 0 { next }
    NR > start && !/^\|/ && seen > 0 { exit }
    /^\|/ && NR > start && $0 !~ /^\|[[:space:]:|-]+[[:space:]]*$/ { seen = 1 }
  ' "$card")"
  local data_rows
  data_rows="$(printf '%s\n' "$table_rows" | awk 'NF { count += 1 } END { print count + 0 }')"
  if (( data_rows < 1 )); then
    guard_fail "$tag" "finite state/outcome table has no data row"
  fi

  if ! rg -q -i '(Unavailable|Absent|Unresolved|Neither|NoCandidate)' <<<"$table_rows"; then
    guard_fail "$tag" "finite table has no explicit neutral neither-selected-nor-rejected state"
  fi
  rg -q 'NoSafeSlice' "$card" || \
    guard_fail "$tag" "explicit NoSafeSlice stop line is missing"
  rg -q -i 'negative|reject|rejected' "$card" || \
    guard_fail "$tag" "negative/rejection evidence is missing"
}

if [[ "$#" -eq 0 ]]; then
  card="$(card_from_current_state)"
elif [[ "$#" -eq 2 && "$1" == "--card" ]]; then
  card="$2"
else
  guard_fail "$tag" "usage: $0 [--card CARD_PATH]"
fi

check_card "$card"
echo "$tag: PASS"
