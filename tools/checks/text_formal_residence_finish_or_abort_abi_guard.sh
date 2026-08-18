#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="text-formal-residence-finish-or-abort-abi-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

RUNTIME="$ROOT_DIR/src/runtime/text_formal_residence.rs"
EXPORT="$ROOT_DIR/crates/nyash_kernel/src/exports/text_formal.rs"
HEADER="$ROOT_DIR/include/nyrt_text_formal_residence_v1.h"
README="$ROOT_DIR/crates/nyash_kernel/src/exports/README.md"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$RUNTIME" "$EXPORT" "$HEADER" "$README"

guard_expect_in_file "$TAG" \
  'finish_text_formal_residence_or_abort_v1' "$RUNTIME" \
  "Rust runtime must own the terminal finish wrapper"
guard_expect_in_file "$TAG" \
  'std::process::abort\(\)' "$RUNTIME" \
  "nonzero finish status must fail-stop inside the runtime"
guard_expect_in_file "$TAG" \
  'hako_text_formal_residence_finish_or_abort_v1' "$EXPORT" \
  "kernel export must expose the void finish-or-abort symbol"
guard_expect_in_file "$TAG" \
  'void hako_text_formal_residence_finish_or_abort_v1' "$HEADER" \
  "C header must expose one void finish-or-abort ABI"
guard_expect_in_file "$TAG" \
  'finish_or_abort' "$README" \
  "export README must describe the terminal finish projection"

if rg -n 'hako_text_formal_residence_finish_v1' "$EXPORT" "$HEADER"; then
  guard_fail "$TAG" "retired status-returning public finish symbol remains"
fi

if rg -n 'hako_text_formal_residence_finish_or_abort_v1[^\n]*->|finish_or_abort_v1[^\n]*u32' \
  "$EXPORT" "$HEADER"; then
  guard_fail "$TAG" "public finish-or-abort ABI must not return status"
fi

if rg -n 'pub (unsafe )?fn finish_text_formal_residence_c_v1' "$RUNTIME"; then
  guard_fail "$TAG" "status-returning finish core must remain module-private"
fi

for file in "$RUNTIME" "$EXPORT"; do
  lines="$(wc -l < "$file" | tr -d '[:space:]')"
  if (( lines >= 800 )); then
    guard_fail "$TAG" "touched source exceeds 800-line hard stop: $file ($lines)"
  fi
done

echo "[$TAG] ok"
