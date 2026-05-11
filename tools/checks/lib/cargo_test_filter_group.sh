#!/usr/bin/env bash
# Shared helper for guard scripts that need several cargo test filters.
#
# Call after cd-ing to the repository root. This helper intentionally targets the
# main crate lib test target: the quick first-row guards below own library-unit
# contract tests plus separate route/file locks, not workspace-wide discovery.
# Keep filters narrow enough that the guard still documents exactly which
# contract family it owns.

run_cargo_test_filter_group() {
  local tag="$1"
  local label="$2"
  shift 2

  echo "[${tag}] --- ${label} ---"
  local filter
  for filter in "$@"; do
    echo "[${tag}] >>> cargo test -q --lib ${filter}"
    cargo test -q --lib "$filter" -- --nocapture
  done
}
