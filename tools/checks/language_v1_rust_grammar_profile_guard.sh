#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="language-v1-rust-grammar-profile"
BIN="target/debug/hakorune"
NORMAL="tools/language_v1/fixtures/try_compat_normalizable.hako"
NON_NORMAL="tools/language_v1/fixtures/try_compat_not_normalizable.hako"
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

echo "[$TAG] OK"
