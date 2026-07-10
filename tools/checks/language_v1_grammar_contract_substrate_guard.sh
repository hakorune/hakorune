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
test ! -e lang/src/compiler/parser/expr/parser_peek_box.hako \
  || guard_fail "retired ParserPeekBox source returned"
if rg -q 'ParserPeekBox|parser_peek_box' lang/src/compiler --glob '*.hako' --glob '*.toml'; then
  guard_fail "retired ParserPeekBox still has a live import or export"
fi
test "$(wc -l < lang/src/compiler/parser/parser_box.hako)" -lt 800 \
  || guard_fail "ParserBox facade exceeded the 800-line source boundary"
test "$(wc -l < lang/src/compiler/parser/program/parser_program_box.hako)" -lt 800 \
  || guard_fail "ParserProgramBox exceeded the 800-line source boundary"
rg -q 'hako_equivalent_fixture_id = "match_compat"' \
  grammar/language-v1-grammar-contract-corpus.toml \
  || guard_fail "peek-to-Match replacement parity fixture missing"

rg -q '\[\[language_v1_grammar_contract\.rows\]\]' grammar/unified-grammar.toml || guard_fail "v1 rows missing"
rg -q 'CompatibilityTransport' docs/reference/language/grammar-contract.md || guard_fail "transport boundary missing"
rg -q 'ParseWitness' crates/hakorune_frontend_grammar/src/contract.rs || guard_fail "witness type missing"
rg -q 'compare_witnesses' crates/hakorune_frontend_grammar/src/contract.rs || guard_fail "comparator missing"
rg -q 'HakoGrammarProfileFacade' lang/src/compiler/parser/grammar_profile_facade.hako \
  || guard_fail "Hako grammar profile facade missing"
rg -q 'parser/try_reserved' lang/src/compiler/parser/stmt/parser_stmt_box/core.hako \
  || guard_fail "Hako Canonical try guard missing"
rg -q 'parser/try_compat_not_normalizable' \
  lang/src/compiler/parser/stmt/parser_exception_box.hako \
  || guard_fail "Hako Compat2025 try closed-set guard missing"
rg -q 'parser/peek_legacy_replaced_by_match' \
  lang/src/compiler/parser/expr/parser_expr_box.hako \
  || guard_fail "Hako Canonical peek guard missing"
rg -q 'parser/peek_compat_not_normalizable' \
  lang/src/compiler/parser/expr/parser_expr_box.hako \
  || guard_fail "Hako Compat2025 peek normalization guard missing"
rg -q 'parser/hako_record_fields_expected_canonical' \
  lang/src/compiler/parser/expr/parser_expr_box.hako \
  || guard_fail "Hako record-field progress fail-fast guard missing"
if rg -q 'ParserPeekBox\.parse' lang/src/compiler/parser/expr/parser_expr_box.hako; then
  guard_fail "legacy Hako Peek JSON route is still live"
fi
if rg -q 'option_inventory|\"name\":\"Option\"' \
  lang/src/compiler/parser/grammar_profile_facade.hako; then
  guard_fail "Hako grammar facade still owns an Option-specific inventory"
fi
rg -F -q 'set_enum_inventory_json(inventory_json)' \
  lang/src/compiler/parser/grammar_profile_facade.hako \
  || guard_fail "Hako grammar facade does not consume generic inventory context"
rg -F -q '\"enum_decls\":' lang/src/compiler/parser/program/parser_program_box.hako \
  || guard_fail "Hako ProgramJSON does not publish the invocation enum inventory"
for tag in \
  parser/hako_enum_match_duplicate_variant \
  parser/hako_enum_match_non_exhaustive \
  parser/hako_enum_match_unit_binding; do
  rg -q "$tag" lang/src/compiler/parser/expr/parser_match_box.hako \
    || guard_fail "Hako EnumMatch publication guard missing: $tag"
done
rg -q 'hako_inventory_id = "option"' grammar/language-v1-grammar-contract-corpus.toml \
  || guard_fail "shared Option inventory context missing"

python3 tools/language_v1/grammar_contract_drift_report.py --help >/dev/null
python3 -m unittest \
  tools.language_v1.test_hako_adapter_health \
  tools.language_v1.test_hako_corpus_batch

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
  batch_report="$(mktemp)"
  trap 'rm -f "$health_a" "$health_b" "$observation" "$batch_report"' EXIT
  python3 tools/language_v1/hako_corpus_batch.py \
    --bin target/debug/hakorune \
    --fixture-id try_statement_canonical_reject \
    --fixture-id try_statement_compat_normalizable \
    --fixture-id try_statement_compat_not_normalizable \
    --fixture-id match_canonical \
    --fixture-id match_compat \
    --fixture-id match_missing_arm \
    --fixture-id match_user_enum_canonical \
    --fixture-id match_missing_arrow \
    --fixture-id match_user_enum_wildcard_canonical \
    --fixture-id match_user_enum_non_exhaustive \
    --fixture-id match_user_enum_duplicate_variant \
    --fixture-id match_user_enum_unit_binding \
    --fixture-id match_missing_close \
    --fixture-id peek_canonical_reject \
    --fixture-id peek_compat_normalizable \
    --fixture-id peek_compat_not_normalizable \
    --include-registry-transport-exclusions \
    --timeout-sec 180 >"$batch_report" \
    || guard_fail "Hako compile-once grammar corpus batch failed"
  rg -q '"adapter_process_count":1' "$batch_report" \
    || guard_fail "Hako grammar corpus did not use one adapter process"
  rg -q '"status":"ok"' "$batch_report" \
    || guard_fail "Hako grammar corpus batch report is not green"
  if ! python3 - "$batch_report" <<'PY'
import json
import pathlib
import sys

from tools.language_v1.grammar_contract_registry import (
    HAKO_TRANSPORT_EXCLUSION_TAG,
    RUST_MIGRATION_TRANSPORT_OWNER,
    hako_transport_fixture_ids,
)

report = json.loads(pathlib.Path(sys.argv[1]).read_text(encoding="utf-8"))
expected_ids = set(hako_transport_fixture_ids())
excluded = [row for row in report["rows"] if row["row_status"] == "excluded"]
assert {row["fixture_id"] for row in excluded} == expected_ids
assert report["excluded_fixture_count"] == len(expected_ids)
assert report["adapter_fixture_count"] + len(expected_ids) == report["fixture_count"]
assert all(row["stable_reject_tag"] == HAKO_TRANSPORT_EXCLUSION_TAG for row in excluded)
assert all(row["transport_owner"] == RUST_MIGRATION_TRANSPORT_OWNER for row in excluded)
assert all(row["hako_adapter_invoked"] is False for row in excluded)
PY
  then
    guard_fail "Hako compatibility-transport exclusion report drifted"
  fi
fi
cargo test -q -p hakorune-frontend-grammar

echo "[language-v1-grammar-contract-substrate] OK"
