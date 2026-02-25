#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
CURRENT_TASK_DOC="$ROOT_DIR/CURRENT_TASK.md"
NOW_DOC="$ROOT_DIR/docs/development/current/main/10-Now.md"
LANE_MAP_DOC="$ROOT_DIR/docs/development/current/main/design/de-rust-lane-map-ssot.md"
PORT_PACK_DOC="$ROOT_DIR/docs/development/current/main/design/joinir-port-task-pack-ssot.md"
FAST_GATE_CASES="$ROOT_DIR/tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv"
FAST_GATE_SCRIPT="$ROOT_DIR/tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh"
source "$(dirname "$0")/lib/guard_common.sh"

TAG="phase29bq-joinir-port-sync-guard"

cd "$ROOT_DIR"

task_id=""
next_id=""
done_id=""
done_num=0
fixed_order_max=""
fixed_order_max_num=0

require_prereqs() {
  guard_require_command "$TAG" rg
  guard_require_command "$TAG" sed
  guard_require_files "$TAG" "$CURRENT_TASK_DOC" "$NOW_DOC" "$LANE_MAP_DOC" "$PORT_PACK_DOC" "$FAST_GATE_CASES" "$FAST_GATE_SCRIPT"
}

extract_line_or_fail() {
  local file="$1"
  local pattern="$2"
  local msg="$3"
  local mode="${4:-first}"
  local line=""
  if [ "$mode" = "last" ]; then
    line=$(rg -n "$pattern" "$file" | tail -n1 | cut -d: -f2- || true)
  else
    line=$(rg -n "$pattern" "$file" | head -n1 | cut -d: -f2- || true)
  fi
  if [ -z "$line" ]; then
    guard_fail "$TAG" "$msg"
  fi
  printf '%s' "$line"
}

extract_backtick_value_or_fail() {
  local line="$1"
  local msg="$2"
  local value
  value=$(printf '%s' "$line" | sed -n 's/.*`\([^`]*\)`.*/\1/p')
  if [ -z "$value" ]; then
    guard_fail "$TAG" "$msg"
  fi
  printf '%s' "$value"
}

parse_current_task_state() {
  local compiler_line next_line done_line
  compiler_line=$(extract_line_or_fail "$CURRENT_TASK_DOC" '^- compiler lane:' "CURRENT_TASK.md missing compiler lane line")
  task_id=$(printf '%s' "$compiler_line" | sed -n 's/.*phase-29bq \/ \([^`]*\)`.*/\1/p')
  if [ -z "$task_id" ]; then
    guard_fail "$TAG" "failed to parse active task id from CURRENT_TASK.md compiler lane"
  fi

  next_line=$(extract_line_or_fail "$CURRENT_TASK_DOC" '^  - next: `(JIR-PORT-[0-9][0-9]|none)`' "failed to parse next line from CURRENT_TASK.md")
  next_id=$(extract_backtick_value_or_fail "$next_line" "failed to parse next id from CURRENT_TASK.md")

  done_line=$(extract_line_or_fail "$CURRENT_TASK_DOC" '^  - done: `JIR-PORT-[0-9][0-9]`' "failed to parse done line from CURRENT_TASK.md" "last")
  done_id=$(extract_backtick_value_or_fail "$done_line" "failed to parse latest done id from CURRENT_TASK.md")
  done_num=$((10#${done_id#JIR-PORT-}))
}

parse_fixed_order_tail() {
  local fixed_order_line
  fixed_order_line=$(extract_line_or_fail "$PORT_PACK_DOC" '^## Fixed Order \(JIR-PORT-00\.\.[0-9][0-9]\)' "failed to parse fixed order line from joinir-port-task-pack-ssot.md")
  fixed_order_max=$(printf '%s' "$fixed_order_line" | sed -n 's/.*JIR-PORT-00\.\.\([0-9][0-9]\).*/\1/p')
  if [ -z "$fixed_order_max" ]; then
    guard_fail "$TAG" "failed to parse fixed order max from joinir-port-task-pack-ssot.md"
  fi
  fixed_order_max_num=$((10#$fixed_order_max))
}

check_sequence_contract() {
  if [ "$task_id" = "none" ]; then
    if [ "$next_id" != "none" ]; then
      guard_fail "$TAG" "CURRENT_TASK.md sequence broken: active=none requires next=none (actual=$next_id)"
    fi
    if [ "$done_num" -ne "$fixed_order_max_num" ]; then
      guard_fail "$TAG" "CURRENT_TASK.md terminal mismatch: active=none requires latest done JIR-PORT-$fixed_order_max (actual=$done_id)"
    fi
  else
    local task_num next_num
    task_num=$((10#${task_id#JIR-PORT-}))
    if [ "$task_num" -ne $((done_num + 1)) ]; then
      guard_fail "$TAG" "CURRENT_TASK.md sequence broken: done=$done_id active=$task_id"
    fi

    if [ "$next_id" = "none" ]; then
      if [ "$task_num" -ne "$fixed_order_max_num" ]; then
        guard_fail "$TAG" "CURRENT_TASK.md terminal mismatch: active=$task_id next=none requires fixed-order tail JIR-PORT-$fixed_order_max"
      fi
    else
      next_num=$((10#${next_id#JIR-PORT-}))
      if [ "$next_num" -ne $((task_num + 1)) ]; then
        guard_fail "$TAG" "CURRENT_TASK.md sequence broken: active=$task_id next=$next_id"
      fi
    fi
  fi

  if [ "$done_num" -gt "$fixed_order_max_num" ]; then
    guard_fail "$TAG" "CURRENT_TASK.md done id exceeds fixed-order max: done=$done_id max=JIR-PORT-$fixed_order_max"
  fi

  if [ "$task_id" != "none" ] && ! printf '%s' "$task_id" | rg -q '^JIR-PORT-[0-9][0-9]$'; then
    guard_fail "$TAG" "CURRENT_TASK.md active id format invalid: $task_id"
  fi
  if [ "$next_id" != "none" ] && ! printf '%s' "$next_id" | rg -q '^JIR-PORT-[0-9][0-9]$'; then
    guard_fail "$TAG" "CURRENT_TASK.md next id format invalid: $next_id"
  fi
}

check_now_doc_sync() {
  local now_line now_blocker now_next
  now_line=$(extract_line_or_fail "$NOW_DOC" '^- Compiler lane:' "10-Now.md missing Compiler lane line")
  now_blocker=$(printf '%s' "$now_line" | sed -n 's/.*active blocker=`\([^`]*\)`.*/\1/p')
  now_next=$(printf '%s' "$now_line" | sed -n 's/.*next=`\([^`]*\)`.*/\1/p')
  if [ "$now_blocker" != "$task_id" ]; then
    guard_fail "$TAG" "10-Now.md active blocker mismatch: expected=$task_id actual=${now_blocker:-<empty>}"
  fi
  if [ "$now_next" != "$next_id" ]; then
    guard_fail "$TAG" "10-Now.md next mismatch: expected=$next_id actual=${now_next:-<empty>}"
  fi
}

check_lane_map_sync() {
  local lane_row lane_blocker map_current_line map_next_line map_current map_next
  lane_row=$(extract_line_or_fail "$LANE_MAP_DOC" '^\| A \| Compiler Meaning \|' "de-rust-lane-map-ssot.md missing lane A table row")
  lane_blocker=$(printf '%s' "$lane_row" | sed -n 's/.*blocker=`\([^`]*\)`.*/\1/p')
  if [ "$lane_blocker" != "$task_id" ]; then
    guard_fail "$TAG" "lane map table blocker mismatch: expected=$task_id actual=${lane_blocker:-<empty>}"
  fi

  map_current_line=$(extract_line_or_fail "$LANE_MAP_DOC" '^  - current blocker は `(JIR-PORT-[0-9][0-9]|none)`' "de-rust-lane-map-ssot.md missing lane A current blocker snapshot")
  map_next_line=$(extract_line_or_fail "$LANE_MAP_DOC" '^  - next は `(JIR-PORT-[0-9][0-9]|none)`' "de-rust-lane-map-ssot.md missing lane A next snapshot")
  map_current=$(extract_backtick_value_or_fail "$map_current_line" "failed to parse lane map current blocker")
  map_next=$(extract_backtick_value_or_fail "$map_next_line" "failed to parse lane map next")
  if [ "$map_current" != "$task_id" ]; then
    guard_fail "$TAG" "lane map snapshot blocker mismatch: expected=$task_id actual=${map_current:-<empty>}"
  fi
  if [ "$map_next" != "$next_id" ]; then
    guard_fail "$TAG" "lane map snapshot next mismatch: expected=$next_id actual=${map_next:-<empty>}"
  fi
}

check_task_pack_sync() {
  local pack_blocker_line pack_next_line pack_blocker pack_next
  pack_blocker_line=$(extract_line_or_fail "$PORT_PACK_DOC" '^- lane A blocker: `(JIR-PORT-[0-9][0-9]|none)`' "joinir-port-task-pack-ssot.md missing lane A blocker line")
  pack_next_line=$(extract_line_or_fail "$PORT_PACK_DOC" '^- next: `(JIR-PORT-[0-9][0-9]|none)`' "joinir-port-task-pack-ssot.md missing lane A next line")
  pack_blocker=$(extract_backtick_value_or_fail "$pack_blocker_line" "failed to parse joinir task pack blocker")
  pack_next=$(extract_backtick_value_or_fail "$pack_next_line" "failed to parse joinir task pack next")
  if [ "$pack_blocker" != "$task_id" ]; then
    guard_fail "$TAG" "joinir-port-task-pack blocker mismatch: expected=$task_id actual=${pack_blocker:-<empty>}"
  fi
  if [ "$pack_next" != "$next_id" ]; then
    guard_fail "$TAG" "joinir-port-task-pack next mismatch: expected=$next_id actual=${pack_next:-<empty>}"
  fi

  if ! rg -Fq -- "- $done_id: done" "$PORT_PACK_DOC"; then
    guard_fail "$TAG" "joinir-port-task-pack missing done marker for latest done id: $done_id"
  fi

  if [ "$task_id" = "none" ] && ! rg -Fq -- "monitor-only" "$PORT_PACK_DOC"; then
    guard_fail "$TAG" "joinir-port-task-pack must declare monitor-only state when active=none"
  fi
}

check_promotion_contract() {
  if ! rg -Fq -- "joinir_port04_phi_exit_invariant_min" "$FAST_GATE_CASES"; then
    guard_fail "$TAG" "phase29bq_fast_gate_cases.tsv missing JIR-PORT-04 promotion case_id"
  fi
  if ! rg -Fq -- "phase29bq_joinir_port04_phi_exit_invariant_lock_vm" "$FAST_GATE_SCRIPT"; then
    guard_fail "$TAG" "phase29bq_fast_gate_vm.sh missing JIR-PORT-04 promotion lock smoke step"
  fi
}

main() {
  echo "[$TAG] checking lane A joinir port sync"
  require_prereqs
  parse_current_task_state
  parse_fixed_order_tail
  check_sequence_contract
  check_now_doc_sync
  check_lane_map_sync
  check_task_pack_sync
  check_promotion_contract
  echo "[$TAG] ok"
}

main "$@"
