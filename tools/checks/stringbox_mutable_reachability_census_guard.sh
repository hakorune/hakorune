#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="stringbox-mutable-reachability-census"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" rg
guard_require_files "$TAG" \
  "$ROOT_DIR/src/boxes/array/ops/text.rs" \
  "$ROOT_DIR/crates/nyash_kernel/src/plugin/value_codec/borrowed_handle.rs" \
  "$ROOT_DIR/docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md"

# `as_any_mut` implementations are a trait surface.  Only these two runtime
# callers can currently obtain a mutable object and neither is a registry-held
# StringBox residence path.
mapfile -t mutable_callers < <(
  rg -n --glob '*.rs' 'as_any_mut\(\)' "$ROOT_DIR/src" "$ROOT_DIR/crates" |
    rg -v 'fn as_any_mut' |
    cut -d: -f1 |
    sort -u
)
expected_callers=(
  "$ROOT_DIR/src/boxes/array/ops/text.rs"
  "$ROOT_DIR/crates/nyash_kernel/src/plugin/value_codec/borrowed_handle.rs"
)
if (( ${#mutable_callers[@]} != ${#expected_callers[@]} )); then
  guard_fail "$TAG" "unexpected mutable Box caller count: ${#mutable_callers[@]}"
fi
for expected in "${expected_callers[@]}"; do
  if [[ ! " ${mutable_callers[*]} " =~ [[:space:]]${expected}[[:space:]] ]]; then
    guard_fail "$TAG" "expected mutable caller missing: ${expected#"$ROOT_DIR/"}"
  fi
done

if rg -n --glob '*.rs' 'Arc::(get_mut|make_mut)' "$ROOT_DIR/src" "$ROOT_DIR/crates"; then
  guard_fail "$TAG" "Arc uniqueness/recovery APIs must remain absent from the StringBox route"
fi

doc="$ROOT_DIR/docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md"
guard_expect_fixed_in_file "$TAG" \
  'every `as_any_mut` caller, `Arc` uniqueness/recovery path, sanctioned' \
  "$doc" \
  "the row-11 acceptance must retain the complete mutable-reachability census"
guard_expect_fixed_in_file "$TAG" \
  'extern/C provider, nowait/task sharing path' \
  "$doc" \
  "the row-11 acceptance must retain the complete external-path census"
guard_expect_fixed_in_file "$TAG" \
  'proof obligation for the theorem that no in-scope path can' \
  "$doc" \
  "the row-11 acceptance must make the &mut reachability theorem explicit"
guard_expect_fixed_in_file "$TAG" \
  'finds no sanctioned path; an unclassified external unsafe provider remains' \
  "$doc" \
  "the census must record the current no-reachable-path result"

echo "[$TAG] ok (all mutable callers classified; no Arc uniqueness recovery path)"
