#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mir-root-import-hygiene-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"
cd "$ROOT_DIR"

CONTRACT="docs/development/current/main/design/mir-root-facade-contract-ssot.md"

guard_require_command "$TAG" rg
guard_require_files "$TAG" "$CONTRACT"

echo "[$TAG] checking MIR root import hygiene"

bad_wildcards="$(
  rg -n '^\s*use\s+crate::mir::(\*|\{[^}]*\*[^}]*\})\s*;' src tests -g '*.rs' || true
)"
if [ -n "$bad_wildcards" ]; then
  echo "[$TAG] ERROR: MIR root wildcard import found" >&2
  printf '%s\n' "$bad_wildcards" >&2
  exit 1
fi

bad_root_vocab="$(
  rg -n 'crate::mir::(StringCorridor|SumPlacement|ThinEntry|PlacementEffect|StorageClass|ValueConsumer)' \
    src/mir src/runner -g '*.rs' || true
)"
if [ -n "$bad_root_vocab" ]; then
  echo "[$TAG] ERROR: semantic metadata vocabulary imported through MIR root" >&2
  printf '%s\n' "$bad_root_vocab" >&2
  exit 1
fi

bad_root_detect_callers="$(
  rg -n 'crate::mir::detect_(skip_whitespace_shape|read_digits_loop_true_shape|continue_shape|parse_number_shape|parse_string_shape|escape_skip_shape)' \
    src/mir -g '*.rs' || true
)"
if [ -n "$bad_root_detect_callers" ]; then
  echo "[$TAG] ERROR: loop-canonicalizer detection helper imported through MIR root" >&2
  printf '%s\n' "$bad_root_detect_callers" >&2
  exit 1
fi

bad_root_detect_bridge="$(
  rg -n 'pub\(crate\)\s+use\s+builder::detect_(skip_whitespace_shape|read_digits_loop_true_shape|continue_shape|parse_number_shape|parse_string_shape|escape_skip_shape)' \
    src/mir/mod.rs || true
)"
if [ -n "$bad_root_detect_bridge" ]; then
  echo "[$TAG] ERROR: crate-internal detection bridge reintroduced in MIR root" >&2
  printf '%s\n' "$bad_root_detect_bridge" >&2
  exit 1
fi

echo "[$TAG] ok"
