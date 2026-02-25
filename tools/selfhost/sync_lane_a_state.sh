#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"

CURRENT_TASK_DOC="$ROOT_DIR/CURRENT_TASK.md"
NOW_DOC="$ROOT_DIR/docs/development/current/main/10-Now.md"
LANE_MAP_DOC="$ROOT_DIR/docs/development/current/main/design/de-rust-lane-map-ssot.md"
PORT_PACK_DOC="$ROOT_DIR/docs/development/current/main/design/joinir-port-task-pack-ssot.md"

TAG="lane-a-state-sync"

usage() {
  cat <<'EOF'
Usage:
  bash tools/selfhost/sync_lane_a_state.sh

Source of truth:
  CURRENT_TASK.md の compiler lane block（active/done/next）

Effect:
  CURRENT_TASK.md を読み取り、次の mirror を同期する:
    - docs/development/current/main/10-Now.md
    - docs/development/current/main/design/de-rust-lane-map-ssot.md
    - docs/development/current/main/design/joinir-port-task-pack-ssot.md
EOF
}

if [ "${1:-}" = "-h" ] || [ "${1:-}" = "--help" ]; then
  usage
  exit 0
fi

if [ -n "${1:-}" ]; then
  echo "[$TAG] FAIL: unknown arg: $1" >&2
  usage >&2
  exit 2
fi

require_cmd() {
  local cmd="$1"
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo "[$TAG] FAIL: command not found: $cmd" >&2
    exit 2
  fi
}

require_file() {
  local file="$1"
  if [ ! -f "$file" ]; then
    echo "[$TAG] FAIL: file not found: $file" >&2
    exit 2
  fi
}

extract_line_or_fail() {
  local file="$1"
  local pattern="$2"
  local mode="${3:-first}"
  local line=""
  if [ "$mode" = "last" ]; then
    line=$(rg -n "$pattern" "$file" | tail -n1 | cut -d: -f2- || true)
  else
    line=$(rg -n "$pattern" "$file" | head -n1 | cut -d: -f2- || true)
  fi
  if [ -z "$line" ]; then
    echo "[$TAG] FAIL: pattern not found ($pattern) in $file" >&2
    exit 2
  fi
  printf '%s' "$line"
}

extract_backtick_value_or_fail() {
  local line="$1"
  local value
  value=$(printf '%s' "$line" | sed -n 's/.*`\([^`]*\)`.*/\1/p')
  if [ -z "$value" ]; then
    echo "[$TAG] FAIL: failed to parse backtick value from line: $line" >&2
    exit 2
  fi
  printf '%s' "$value"
}

replace_prefixed_line() {
  local file="$1"
  local prefix="$2"
  local replacement="$3"
  local tmp
  tmp="$(mktemp)"
  if ! awk -v prefix="$prefix" -v replacement="$replacement" '
    BEGIN { count = 0 }
    index($0, prefix) == 1 {
      print replacement
      count++
      next
    }
    { print }
    END {
      if (count != 1) {
        printf("expected exactly one line for prefix [%s] in %s (actual=%d)\n", prefix, FILENAME, count) > "/dev/stderr"
        exit 42
      }
    }
  ' "$file" > "$tmp"; then
    rm -f "$tmp"
    echo "[$TAG] FAIL: replace failed for prefix [$prefix] in $file" >&2
    exit 2
  fi
  mv "$tmp" "$file"
}

require_cmd rg
require_cmd sed
require_cmd awk
require_cmd mktemp

require_file "$CURRENT_TASK_DOC"
require_file "$NOW_DOC"
require_file "$LANE_MAP_DOC"
require_file "$PORT_PACK_DOC"

compiler_line=$(extract_line_or_fail "$CURRENT_TASK_DOC" '^- compiler lane:')
task_id=$(printf '%s' "$compiler_line" | sed -n 's/.*phase-29bq \/ \([^`]*\)`.*/\1/p')
if [ -z "$task_id" ]; then
  echo "[$TAG] FAIL: failed to parse active task id from CURRENT_TASK.md compiler lane" >&2
  exit 2
fi

active_label=$(printf '%s' "$compiler_line" | sed -n 's/.*（active: \(.*\)）/\1/p')

next_line=$(extract_line_or_fail "$CURRENT_TASK_DOC" '^  - next: `(JIR-PORT-[0-9][0-9]|none)`')
next_id=$(extract_backtick_value_or_fail "$next_line")

done_line=$(extract_line_or_fail "$CURRENT_TASK_DOC" '^  - done: `JIR-PORT-[0-9][0-9]`' "last")
done_id=$(extract_backtick_value_or_fail "$done_line")
done_suffix="${done_id#JIR-PORT-}"
done_range="JIR-PORT-00..$done_suffix"

if [ "$task_id" = "none" ]; then
  joinir_mode_line="- JoinIR port mode（lane A）: monitor-only（failure-driven）"
  lane_row_status='active（monitor-only, blocker=`none`）'
  lane_blocker_line='  - current blocker は `none`（monitor-only）。'
  port_pack_blocker='- lane A blocker: `none`（monitor-only）'
else
  joinir_mode_line="- JoinIR port mode（lane A）: proactive（manual override: $task_id）"
  lane_row_status="active（proactive, blocker=\`$task_id\`）"
  if [ -n "$active_label" ]; then
    lane_blocker_line="  - current blocker は \`$task_id\`（$active_label）。"
    port_pack_blocker="- lane A blocker: \`$task_id\`（active, $active_label）"
  else
    lane_blocker_line="  - current blocker は \`$task_id\`。"
    port_pack_blocker="- lane A blocker: \`$task_id\`（active）"
  fi
fi

replace_prefixed_line "$NOW_DOC" "- Compiler lane:" "- Compiler lane: \`phase-29bq\`（$done_range done / active blocker=\`$task_id\` / next=\`$next_id\`）"
replace_prefixed_line "$NOW_DOC" "- JoinIR port mode（lane A）:" "$joinir_mode_line"

replace_prefixed_line "$LANE_MAP_DOC" "| A | Compiler Meaning |" "| A | Compiler Meaning | JoinIR / Planner / CorePlan の受理・意味決定 | \`de-rust-compiler-thin-rust-roadmap-ssot.md\` + JoinIR gate SSOT | $lane_row_status |"
replace_prefixed_line "$LANE_MAP_DOC" "  - done range は " "  - done range は \`$done_range\`（詳細は \`joinir-port-task-pack-ssot.md\` の Current State）。"
replace_prefixed_line "$LANE_MAP_DOC" "  - current blocker は " "$lane_blocker_line"
replace_prefixed_line "$LANE_MAP_DOC" "  - next は " "  - next は \`$next_id\`（tail active）。"

replace_prefixed_line "$PORT_PACK_DOC" "- lane A blocker:" "$port_pack_blocker"
replace_prefixed_line "$PORT_PACK_DOC" "- next:" "- next: \`$next_id\`（tail active）"

echo "[$TAG] synced lane A mirrors from CURRENT_TASK.md"
echo "[$TAG] active=$task_id done_range=$done_range next=$next_id"
