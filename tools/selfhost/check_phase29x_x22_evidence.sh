#!/bin/bash
# check_phase29x_x22_evidence.sh
# Validate Phase 29x X22 three-day evidence table quality.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC_PATH="$ROOT_DIR/docs/development/current/main/phases/phase-29x/29x-44-vm-route-three-day-gate-evidence.md"
STRICT=0

usage() {
  cat <<'USAGE' >&2
Usage:
  check_phase29x_x22_evidence.sh [--strict]

Options:
  --strict   exit non-zero unless Day1-3 are all PASS with unique ascending dates
USAGE
}

while [ $# -gt 0 ]; do
  case "$1" in
    --strict)
      STRICT=1
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "unknown arg: $1" >&2
      usage
      exit 2
      ;;
  esac
done

if [ ! -f "$DOC_PATH" ]; then
  echo "[x22-check] missing doc: $DOC_PATH" >&2
  exit 2
fi

trim() {
  local s="$1"
  s="${s#"${s%%[![:space:]]*}"}"
  s="${s%"${s##*[![:space:]]}"}"
  printf "%s" "$s"
}

declare -A day_date
declare -A day_result
declare -A day_notes

while IFS= read -r line; do
  [[ "$line" =~ ^\|[[:space:]]*[123][[:space:]]*\| ]] || continue
  IFS='|' read -r _ col_day col_date col_result col_notes _ <<< "$line"
  day="$(trim "$col_day")"
  date="$(trim "$col_date")"
  result="$(trim "$col_result")"
  notes="$(trim "$col_notes")"
  day_date["$day"]="$date"
  day_result["$day"]="$result"
  day_notes["$day"]="$notes"
done < "$DOC_PATH"

pass_count=0
filled_count=0
for d in 1 2 3; do
  date="${day_date[$d]:-missing}"
  result="${day_result[$d]:-missing}"
  if [ "$date" != "missing" ] && [ "$date" != "pending" ]; then
    filled_count=$((filled_count + 1))
  fi
  if [ "$result" = "PASS" ]; then
    pass_count=$((pass_count + 1))
  fi
done

echo "[x22-check] file=$DOC_PATH"
echo "[x22-check] day1 date=${day_date[1]:-missing} result=${day_result[1]:-missing}"
echo "[x22-check] day2 date=${day_date[2]:-missing} result=${day_result[2]:-missing}"
echo "[x22-check] day3 date=${day_date[3]:-missing} result=${day_result[3]:-missing}"
echo "[x22-check] progress filled_days=$filled_count/3 pass_days=$pass_count/3"

unique_dates=1
ascending=1
if [ "${day_date[1]:-pending}" != "pending" ] && [ "${day_date[2]:-pending}" != "pending" ] && [ "${day_date[3]:-pending}" != "pending" ]; then
  if [ "${day_date[1]}" = "${day_date[2]}" ] || [ "${day_date[2]}" = "${day_date[3]}" ] || [ "${day_date[1]}" = "${day_date[3]}" ]; then
    unique_dates=0
  fi
  if [[ "${day_date[1]}" > "${day_date[2]}" || "${day_date[2]}" > "${day_date[3]}" ]]; then
    ascending=0
  fi
fi

if [ "$STRICT" -eq 1 ]; then
  if [ "$pass_count" -ne 3 ]; then
    echo "[x22-check] strict=FAIL reason=pass_days_not_3" >&2
    exit 1
  fi
  if [ "$unique_dates" -ne 1 ]; then
    echo "[x22-check] strict=FAIL reason=dates_not_unique" >&2
    exit 1
  fi
  if [ "$ascending" -ne 1 ]; then
    echo "[x22-check] strict=FAIL reason=dates_not_ascending" >&2
    exit 1
  fi
  echo "[x22-check] strict=PASS"
fi
