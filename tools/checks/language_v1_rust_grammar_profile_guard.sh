#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="language-v1-rust-grammar-profile"
BIN="target/debug/hakorune"
NORMAL="tools/language_v1/fixtures/try_compat_normalizable.hako"
NON_NORMAL="tools/language_v1/fixtures/try_compat_not_normalizable.hako"
PEEK_NORMAL="tools/language_v1/fixtures/peek_compat_normalizable.hako"
PEEK_NON_NORMAL="tools/language_v1/fixtures/peek_compat_not_normalizable.hako"
FROM_BOX_CLOSED="tools/language_v1/fixtures/from_box_closed.hako"
FROM_CALL_CLOSED="tools/language_v1/fixtures/from_call_closed.hako"
FROM_BOX_NOT_CLOSED="tools/language_v1/fixtures/from_box_not_closed.hako"
FROM_CALL_NOT_CLOSED="tools/language_v1/fixtures/from_call_not_closed.hako"
AST_JSON="$(mktemp)"
OUT="$(mktemp)"
ERR="$(mktemp)"
trap 'rm -f "$AST_JSON" "$OUT" "$ERR"' EXIT

fail() {
  echo "[$TAG] FAIL: $*" >&2
  exit 1
}

cargo test -q -p hakorune-frontend-parser
cargo test -q --test parser_grammar_profile
cargo test -q --test parser_stage3
cargo build -q --bin hakorune

if "$BIN" --emit-ast-json "$AST_JSON" "$NORMAL" >"$OUT" 2>"$ERR"; then
  fail "Canonical default accepted statement try"
fi
rg -q 'parser/try_reserved' "$ERR" || fail "Canonical try reject tag missing"

"$BIN" --grammar-profile compat2025 --emit-ast-json "$AST_JSON" "$NORMAL" >"$OUT" 2>"$ERR" \
  || fail "Compat2025 rejected normalizable statement try"

if "$BIN" --grammar-profile compat2025 --emit-ast-json "$AST_JSON" "$NON_NORMAL" >"$OUT" 2>"$ERR"; then
  fail "Compat2025 accepted non-normalizable statement try"
fi
rg -q 'parser/try_compat_not_normalizable' "$ERR" \
  || fail "Compat2025 non-normalizable reject tag missing"

if "$BIN" --grammar-profile future >"$OUT" 2>"$ERR"; then
  fail "unknown grammar profile accepted"
fi
rg -q 'parser/profile_unknown' "$ERR" || fail "unknown profile reject tag missing"

if NYASH_FEATURES=no-try-compat "$BIN" --grammar-profile canonical >"$OUT" 2>"$ERR"; then
  fail "explicit profile and legacy env conflict accepted"
fi
rg -q 'parser/profile_legacy_env_conflict' "$ERR" || fail "legacy conflict tag missing"

if NYASH_FEATURES=no-try-compat "$BIN" --emit-ast-json "$AST_JSON" "$NORMAL" >"$OUT" 2>"$ERR"; then
  fail "legacy env selected a non-Canonical profile"
fi
rg -q 'parser/try_reserved' "$ERR" || fail "legacy env changed Canonical rejection"

if "$BIN" --emit-ast-json "$AST_JSON" "$PEEK_NORMAL" >"$OUT" 2>"$ERR"; then
  fail "Canonical accepted legacy peek"
fi
rg -q 'parser/peek_legacy_replaced_by_match' "$ERR" || fail "Canonical peek tag missing"

"$BIN" --grammar-profile compat2025 --emit-ast-json "$AST_JSON" "$PEEK_NORMAL" >"$OUT" 2>"$ERR" \
  || fail "Compat2025 rejected normalizable peek"

if "$BIN" --grammar-profile compat2025 --emit-ast-json "$AST_JSON" "$PEEK_NON_NORMAL" >"$OUT" 2>"$ERR"; then
  fail "Compat2025 accepted non-normalizable peek"
fi
rg -q 'parser/peek_compat_not_normalizable' "$ERR" || fail "Compat2025 peek tag missing"

if "$BIN" --emit-ast-json "$AST_JSON" "$FROM_BOX_CLOSED" >"$OUT" 2>"$ERR"; then
  fail "Canonical default accepted legacy box-from"
fi
rg -q 'parser/from_inheritance_legacy' "$ERR" || fail "Canonical box-from tag missing"

if "$BIN" --emit-ast-json "$AST_JSON" "$FROM_CALL_CLOSED" >"$OUT" 2>"$ERR"; then
  fail "Canonical default accepted legacy from-call"
fi
rg -q 'parser/from_call_legacy' "$ERR" || fail "Canonical from-call tag missing"

for fixture in "$FROM_BOX_CLOSED" "$FROM_CALL_CLOSED"; do
  if "$BIN" --grammar-profile compat2025 --emit-ast-json "$AST_JSON" "$fixture" >"$OUT" 2>"$ERR"; then
    fail "Compat2025 semantic parser accepted transport-only from syntax: $fixture"
  fi
  rg -q 'parser/from_compat_transport_only' "$ERR" \
    || fail "Compat2025 transport-only tag missing for $fixture"
done

for fixture in "$FROM_BOX_NOT_CLOSED" "$FROM_CALL_NOT_CLOSED"; do
  if "$BIN" --grammar-profile compat2025 --emit-ast-json "$AST_JSON" "$fixture" >"$OUT" 2>"$ERR"; then
    fail "Compat2025 accepted non-closed from transport: $fixture"
  fi
  rg -q 'parser/from_transport_not_closed_form' "$ERR" \
    || fail "non-closed transport tag missing for $fixture"
done

echo "[$TAG] OK"
