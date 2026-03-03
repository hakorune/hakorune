#!/bin/bash
# joinir_planner_first_gate.sh - common runner for planner-first gates (strict/dev)

set -e

LIB_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [ -f "$LIB_DIR/env.sh" ]; then
  source "$LIB_DIR/env.sh"
fi
if [ -f "$LIB_DIR/vm_route_pin.sh" ]; then
  source "$LIB_DIR/vm_route_pin.sh"
fi

planner_first_exit_code_allowed() {
  local exit_code="$1"
  local allowed_codes="$2"
  local code

  for code in $allowed_codes; do
    if [ "$exit_code" -eq "$code" ]; then
      return 0
    fi
  done

  return 1
}

planner_first_tag_matches() {
  local output="$1"
  local tag_spec="$2"
  local tag

  IFS='|' read -r -a tags <<<"$tag_spec"
  for tag in "${tags[@]}"; do
    if [ -z "$tag" ]; then
      continue
    fi

    # Primary contract: exact tag match.
    if grep -qF "$tag" <<<"$output"; then
      return 0
    fi

    # Legacy compatibility: allow PatternN expectation to match semantic rule tags.
    # This keeps gate TSV stable while compiler-side tag vocabulary migrates.
    local compat_tag
    compat_tag=$(planner_first_compat_tag "$tag")
    if [ -n "$compat_tag" ] && grep -qF "$compat_tag" <<<"$output"; then
      return 0
    fi
  done

  return 1
}

planner_first_pattern_semantic_rule() {
  local legacy_rule="$1"
  case "$legacy_rule" in
    Pattern1) echo "LoopSimpleWhile" ;;
    Pattern2) echo "LoopBreakRecipe" ;;
    Pattern3) echo "IfPhiJoin" ;;
    Pattern4) echo "LoopContinueOnly" ;;
    Pattern5) echo "LoopTrueEarlyExit" ;;
    Pattern6) echo "ScanWithInit" ;;
    Pattern7) echo "SplitScan" ;;
    Pattern8) echo "BoolPredicateScan" ;;
    Pattern9) echo "AccumConstLoop" ;;
    *) echo "" ;;
  esac
}

planner_first_compat_tag() {
  local tag="$1"
  local legacy_rule semantic_rule label

  if [[ ! "$tag" =~ rule=(Pattern[1-9]) ]]; then
    echo ""
    return 0
  fi
  legacy_rule="${BASH_REMATCH[1]}"

  label=""
  if [[ "$tag" =~ label=([^[:space:]|]+) ]]; then
    label="${BASH_REMATCH[1]}"
  fi

  if [ -n "$label" ]; then
    semantic_rule="$label"
  else
    semantic_rule="$(planner_first_pattern_semantic_rule "$legacy_rule")"
  fi

  if [ -z "$semantic_rule" ]; then
    echo ""
    return 0
  fi

  echo "${tag/rule=$legacy_rule/rule=$semantic_rule}"
}

run_planner_first_gate() {
  local test_name="$1"
  local fixture="$2"
  local expected="$3"
  local planner_tag="$4"
  local allowed_codes="${5:-0}"
  local timeout_secs="${6:-10}"

  if [ -z "$test_name" ] || [ -z "$fixture" ] || [ -z "$planner_tag" ]; then
    log_error "planner_first: missing required arguments"
    return 1
  fi

  export NYASH_ALLOW_USING_FILE=1

  set +e
  local output
  # Make the gate hermetic: do not inherit developer-local debug/trace envs that would
  # leak extra lines into stdout/stderr and break output matching.
  # Planner-first gates are compiler-lane contracts, so pin route to rust-vm lane.
  output=$(run_hermetic_vm_with_route_pin \
    timeout "$timeout_secs" \
    "$NYASH_BIN" --backend vm "$fixture" 2>&1)
  local exit_code=$?
  set -e

  if [ "$exit_code" -eq 124 ]; then
    log_error "$test_name: hakorune timed out (> ${timeout_secs}s)"
    return 1
  fi

  if ! planner_first_exit_code_allowed "$exit_code" "$allowed_codes"; then
    log_error "$test_name: expected exit code(s) $allowed_codes, got $exit_code"
    echo "$output"
    return 1
  fi

  local output_clean
  output_clean=$(echo "$output" | filter_noise | grep -v '^\[plugins\]' | grep -v '^\[WARN\] \[plugin/init\]' || true)

  compare_outputs "$expected" "$output_clean" "$test_name" || return 1

  # NOTE: Avoid `echo ... | grep -q` under `set -o pipefail`, because `grep -q`
  # closes the pipe early and can turn the pipeline into a SIGPIPE failure.
  if ! planner_first_tag_matches "$output" "$planner_tag"; then
    echo "[FAIL] Missing planner-first tag ($planner_tag)"
    echo "$output" | tail -n 40 || true
    test_fail "$test_name: Missing planner-first tag"
    return 1
  fi

  log_success "$test_name: PASS (exit=$exit_code)"
  return 0
}
