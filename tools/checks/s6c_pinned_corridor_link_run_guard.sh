#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="s6c-pinned-corridor-link-run-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

ROOT_FEATURE="$ROOT_DIR/Cargo.toml"
KERNEL_FEATURE="$ROOT_DIR/crates/nyash_kernel/Cargo.toml"
ROOT_SUPPORT="$ROOT_DIR/src/runtime/promotion_test_support.rs"
KERNEL_SUPPORT="$ROOT_DIR/crates/nyash_kernel/src/exports/promotion_test_support.rs"
RUNNER="$ROOT_DIR/lang/c-abi/tests/s6c_pinned_corridor_link_run.c"
SMOKE="$ROOT_DIR/tools/checks/s6c_pinned_corridor_link_run_smoke.sh"

guard_require_files "$TAG" \
  "$ROOT_FEATURE" "$KERNEL_FEATURE" "$ROOT_SUPPORT" "$KERNEL_SUPPORT" \
  "$RUNNER" "$SMOKE"
guard_expect_fixed_in_file "$TAG" 'promotion-test-support = []' "$ROOT_FEATURE" \
  "root test support must remain default-off"
guard_expect_fixed_in_file "$TAG" \
  'promotion-test-support = ["nyash-rust/promotion-test-support"]' \
  "$KERNEL_FEATURE" "NyRT test ABI must explicitly opt into the root issuer"
guard_expect_fixed_in_file "$TAG" '#[cfg(feature = "promotion-test-support")]' \
  "$ROOT_DIR/crates/nyash_kernel/src/exports/mod.rs" \
  "test-only exports must stay feature-gated"
guard_expect_fixed_in_file "$TAG" 'to_handle_text_with_lease_identity' "$ROOT_SUPPORT" \
  "test wires must be issued with generation in the allocation transaction"
guard_expect_fixed_in_file "$TAG" 'hako_s6c_candidate' "$RUNNER" \
  "runner must execute the real linked candidate"
guard_expect_fixed_in_file "$TAG" 'candidate_traps(zero, zero)' "$RUNNER" \
  "zero wire negative must remain"
guard_expect_fixed_in_file "$TAG" 'candidate_traps(stale, stale)' "$RUNNER" \
  "stale wire negative must remain"
guard_expect_fixed_in_file "$TAG" 'candidate_traps(non_text, non_text)' "$RUNNER" \
  "non-Text negative must remain"
guard_expect_fixed_in_file "$TAG" 'residence_can_reenter' "$RUNNER" \
  "both normal exits must prove reusable Residence lifecycle"
guard_expect_fixed_in_file "$TAG" 'multi-scalar' "$RUNNER" \
  "multi-scalar needle counterexample must remain"
guard_expect_fixed_in_file "$TAG" 'objcopy --redefine-sym' "$SMOKE" \
  "link runner may rename only the exact emitted symbol"
if rg -n 'promotion_test_support|hako_promotion_test_' \
  "$ROOT_DIR/include" "$ROOT_DIR/lang/c-abi/shims"; then
  guard_fail "$TAG" "test wire issuer must not enter the production ABI headers/shims"
fi
for source in "$ROOT_SUPPORT" "$KERNEL_SUPPORT" "$RUNNER" "$SMOKE"; do
  lines="$(wc -l <"$source")"
  if (( lines >= 760 )); then
    guard_fail "$TAG" "source reached the split trigger: ${source#"$ROOT_DIR/"} ($lines lines)"
  fi
done

echo "[$TAG] ok (test-only wire issuer + real-object oracle/lifecycle coverage)"
