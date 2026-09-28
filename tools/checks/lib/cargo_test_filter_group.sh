#!/usr/bin/env bash
# Shared helper for guard scripts that need several cargo test filters.
#
# Call after cd-ing to the repository root. This helper intentionally targets the
# main crate lib test target: the quick first-row guards below own library-unit
# contract tests plus separate route/file locks, not workspace-wide discovery.
# Keep filters narrow enough that the guard still documents exactly which
# contract family it owns.
#
# Tests already recorded in tools/checks/manifests/cargo_lib_red_baseline.failures.txt
# are known baseline debt (repo rule: classified debt is non-blocking). When a
# filter would match such a test, the test is skipped via --skip and the skip is
# echoed, so coverage restores itself automatically once the debt is repaid.
# Tests that are red but NOT in the baseline still fail the guard normally.

CARGO_TEST_RED_BASELINE_FILE="tools/checks/manifests/cargo_lib_red_baseline.failures.txt"

run_cargo_test_filter_group() {
  local tag="$1"
  local label="$2"
  shift 2

  echo "[${tag}] --- ${label} ---"
  local filter
  for filter in "$@"; do
    local skips=()
    if [[ -f "$CARGO_TEST_RED_BASELINE_FILE" ]]; then
      local red
      while IFS= read -r red; do
        [[ -z "$red" ]] && continue
        if [[ "$red" == *"$filter"* ]]; then
          echo "[${tag}] skipping known-baseline-debt test: ${red}"
          skips+=(--skip "$red")
        fi
      done < "$CARGO_TEST_RED_BASELINE_FILE"
    fi
    echo "[${tag}] >>> cargo test -q --lib ${filter}"
    cargo test -q --lib "$filter" -- --nocapture ${skips[@]+"${skips[@]}"}
  done
}
