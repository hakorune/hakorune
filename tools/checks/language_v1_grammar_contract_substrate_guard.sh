#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

guard_fail() {
  echo "[language-v1-grammar-contract-substrate] FAIL: $*" >&2
  exit 1
}

test -f grammar/unified-grammar.toml || guard_fail "physical registry missing"
test -f grammar/language-v1-grammar-contract-corpus.toml || guard_fail "shared corpus missing"
test -f tools/language_v1/grammar_contract_hako_adapter.hako || guard_fail "Hako adapter missing"
test -f tools/language_v1/grammar_contract_drift_report.py || guard_fail "drift report missing"

rg -q '\[\[language_v1_grammar_contract\.rows\]\]' grammar/unified-grammar.toml || guard_fail "v1 rows missing"
rg -q 'CompatibilityTransport' docs/reference/language/grammar-contract.md || guard_fail "transport boundary missing"
rg -q 'ParseWitness' crates/hakorune_frontend_grammar/src/contract.rs || guard_fail "witness type missing"
rg -q 'compare_witnesses' crates/hakorune_frontend_grammar/src/contract.rs || guard_fail "comparator missing"
rg -q 'HakoGrammarProfileFacade' lang/src/compiler/parser/grammar_profile_facade.hako \
  || guard_fail "Hako grammar profile facade missing"
rg -q 'parser/hako_try_reserved' lang/src/compiler/parser/stmt/parser_stmt_box/core.hako \
  || guard_fail "Hako Canonical try guard missing"
rg -q 'parser/hako_try_compat_not_normalizable' \
  lang/src/compiler/parser/stmt/parser_exception_box.hako \
  || guard_fail "Hako Compat2025 try closed-set guard missing"

python3 tools/language_v1/grammar_contract_drift_report.py --help >/dev/null
python3 -m unittest tools.language_v1.test_hako_adapter_health

cargo build -q --features vm-reference --bin hakorune
health_a="$(mktemp)"
health_b="$(mktemp)"
observation="$(mktemp)"
trap 'rm -f "$health_a" "$health_b" "$observation"' EXIT

python3 tools/language_v1/hako_adapter_health.py \
  --bin target/debug/hakorune --probe health --timeout-sec 2 >"$health_a"
NYASH_FEATURES=no-try-compat python3 tools/language_v1/hako_adapter_health.py \
  --bin target/debug/hakorune --probe health --timeout-sec 2 >"$health_b"
cmp -s "$health_a" "$health_b" \
  || guard_fail "NYASH_FEATURES changed Hako adapter health envelope"
rg -q '"schema":"language-v1-hako-adapter-health-v0"' "$health_a" \
  || guard_fail "Hako adapter health schema missing"
rg -q '"status":"ok"' "$health_a" \
  || guard_fail "Hako adapter health ping is not green"

if python3 tools/language_v1/hako_adapter_health.py \
  --bin target/debug/hakorune --probe observation --timeout-sec 0.1 >"$observation"; then
  guard_fail "unavailable Hako parser observation unexpectedly reported health success"
fi
rg -q 'parser/hako_adapter_timeout' "$observation" \
  || guard_fail "Hako adapter observation timeout tag missing"

if [ "${LANGV1_HAKO_PROFILE_FULL:-0}" = "1" ]; then
  canonical_try="$(mktemp)"
  compat_try="$(mktemp)"
  nonnormal_try="$(mktemp)"
  trap 'rm -f "$health_a" "$health_b" "$observation" "$canonical_try" "$compat_try" "$nonnormal_try"' EXIT

  if python3 tools/language_v1/hako_adapter_health.py \
    --bin target/debug/hakorune --probe observation --profile canonical \
    --source 'try { local x = 1 } catch () { local y = 2 }' \
    --timeout-sec 90 >"$canonical_try"; then
    guard_fail "Canonical Hako profile accepted statement try"
  fi
  rg -q 'parser/hako_try_reserved' "$canonical_try" \
    || guard_fail "Canonical Hako try reject tag missing"

  NYASH_FEATURES=no-try-compat python3 tools/language_v1/hako_adapter_health.py \
    --bin target/debug/hakorune --probe observation --profile compat2025 \
    --source 'try { local x = 1 } catch () { local y = 2 } cleanup { local z = 3 }' \
    --timeout-sec 90 >"$compat_try" \
    || guard_fail "explicit Compat2025 Hako try was not accepted"

  if python3 tools/language_v1/hako_adapter_health.py \
    --bin target/debug/hakorune --probe observation --profile compat2025 \
    --source 'try { local x = 1 } catch (Legacy) { local y = 2 }' \
    --timeout-sec 90 >"$nonnormal_try"; then
    guard_fail "Compat2025 Hako accepted non-normalizable try"
  fi
  rg -q 'parser/hako_try_compat_not_normalizable' "$nonnormal_try" \
    || guard_fail "Compat2025 Hako try closed-set reject tag missing"
fi
cargo test -q -p hakorune-frontend-grammar

echo "[language-v1-grammar-contract-substrate] OK"
