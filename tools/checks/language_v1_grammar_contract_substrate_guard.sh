#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

guard_fail() {
  echo "[language-v1-grammar-contract-substrate] FAIL: $*" >&2
  exit 1
}

test -f grammar/language-v1-registry.toml || guard_fail "Language v1 registry missing"
test -f grammar/legacy/nyash-v1.1-codegen-input.toml \
  || guard_fail "legacy codegen input missing"
test ! -e grammar/unified-grammar.toml \
  || guard_fail "ambiguous unified grammar path returned"
test -f grammar/language-v1-grammar-contract-corpus.toml || guard_fail "corpus manifest missing"
test -f grammar/language-v1-grammar-contract-corpus/foundation.toml \
  || guard_fail "foundation corpus fragment missing"
test -f grammar/language-v1-grammar-contract-corpus/remaining-surfaces.toml \
  || guard_fail "remaining-surface corpus fragment missing"
test -f tools/language_v1/grammar_contract_hako_adapter.hako || guard_fail "Hako adapter missing"
test -f tools/language_v1/grammar_contract_drift_report.py || guard_fail "drift report missing"
test -f lang/src/compiler/parser/generated/grammar_contract_projection.hako \
  || guard_fail "generated Hako grammar projection missing"
test ! -e lang/src/compiler/parser/expr/parser_peek_box.hako \
  || guard_fail "retired ParserPeekBox source returned"
if rg -q 'ParserPeekBox|parser_peek_box' lang/src/compiler --glob '*.hako' --glob '*.toml'; then
  guard_fail "retired ParserPeekBox still has a live import or export"
fi
test "$(wc -l < lang/src/compiler/parser/parser_box.hako)" -lt 800 \
  || guard_fail "ParserBox facade exceeded the 800-line source boundary"
test "$(wc -l < lang/src/compiler/parser/program/parser_program_box.hako)" -lt 800 \
  || guard_fail "ParserProgramBox exceeded the 800-line source boundary"
for parser_source in \
  lang/src/compiler/parser/decl/parser_declaration_box.hako \
  lang/src/compiler/parser/decl/parser_delegate_exposes_box.hako \
  lang/src/compiler/parser/decl/parser_record_declaration_box.hako \
  lang/src/compiler/parser/decl/parser_box_weak_field_box.hako \
  lang/src/compiler/parser/generated/grammar_contract_projection.hako \
  lang/src/compiler/parser/legacy/parser_from_reject_box.hako \
  lang/src/compiler/parser/expr/parser_expr_box.hako \
  lang/src/compiler/parser/expr/parser_expr_context_box.hako \
  lang/src/compiler/parser/expr/parser_expr_precedence_box.hako \
  lang/src/compiler/parser/expr/parser_match_box.hako; do
  test "$(wc -l < "$parser_source")" -lt 800 \
    || guard_fail "$parser_source exceeded the 800-line source boundary"
done
if rg -q 'hako_equivalent_fixture_id' grammar/language-v1-grammar-contract-corpus; then
  guard_fail "raw Hako ProgramJSON equivalence must not replace recursive witness parity"
fi

rg -q '\[\[rows\]\]' grammar/language-v1-registry.toml || guard_fail "v1 rows missing"
rg -q '\[rows\.canonical\]' grammar/language-v1-registry.toml \
  || guard_fail "Canonical profile contracts missing"
rg -q '\[rows\.compat2025\]' grammar/language-v1-registry.toml \
  || guard_fail "Compat2025 profile contracts missing"
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
rg -q 'parse_block_delimited_head2' lang/src/compiler/parser/expr/parser_match_box.hako \
  || guard_fail "Hako Match parser does not own its scrutinee delimiter context"
rg -q 'BlockDelimitedHeadStopsBeforeTopLevelBrace' \
  lang/src/compiler/parser/expr/parser_expr_context_box.hako \
  || guard_fail "Hako Match scrutinee context policy missing"
rg -F -q 'record_literal_allowed(expr_context)' \
  lang/src/compiler/parser/expr/parser_expr_box.hako \
  || guard_fail "Hako record literal does not consume explicit expression context"
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
  parser/enum_match_duplicate_variant \
  parser/enum_match_non_exhaustive \
  parser/enum_match_unit_binding; do
  rg -q "$tag" lang/src/compiler/parser/expr/parser_match_box.hako \
    || guard_fail "Hako EnumMatch publication guard missing: $tag"
done
rg -q 'parser_inventory_id = "option"' grammar/language-v1-grammar-contract-corpus.toml \
  || rg -q 'parser_inventory_id = "option"' grammar/language-v1-grammar-contract-corpus \
  || guard_fail "shared Option inventory context missing"
if rg -q 'hako_inventory(_id)?' grammar/language-v1-grammar-contract-corpus.toml grammar/language-v1-grammar-contract-corpus; then
  guard_fail "Hako-only grammar inventory ownership must not return"
fi

python3 tools/language_v1/grammar_contract_drift_report.py --help >/dev/null
python3 tools/language_v1/generate_hako_grammar_contract.py --check \
  || guard_fail "generated Hako grammar projection is stale"
python3 -m unittest \
  tools.language_v1.test_full_gate \
  tools.language_v1.test_hako_adapter_health \
  tools.language_v1.test_hako_corpus_batch \
  tools.language_v1.test_source_migration_report \
  tools.language_v1.test_witness_projection

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

if [ "${LANGV1_GRAMMAR_FULL:-0}" = "1" ] \
  || [ "${LANGV1_HAKO_PROFILE_FULL:-0}" = "1" ]; then
  full_report="$(mktemp)"
  migration_report="$(mktemp)"
  trap 'rm -f "$health_a" "$health_b" "$observation" "$full_report" "$migration_report"' EXIT
  python3 tools/language_v1/source_migration_report.py \
    --output "$migration_report" \
    || guard_fail "Canonical-owned Hako source still contains rejected grammar"
  python3 tools/language_v1/grammar_contract_full_gate.py \
    --bin target/debug/hakorune \
    --hako-timeout-sec 180 \
    --output "$full_report" \
    || guard_fail "full Rust/Hako grammar conformance failed"
  if ! python3 - "$full_report" <<'PY'
import json
import pathlib
import sys

from tools.language_v1.grammar_contract_registry import all_registry_fixture_ids

report = json.loads(pathlib.Path(sys.argv[1]).read_text(encoding="utf-8"))
assert report["status"] == "ok"
assert report["fixture_count"] == len(all_registry_fixture_ids())
assert report["hako"]["adapter_process_count"] == 1
assert report["rust"]["status"] == "ok"
assert report["hako"]["status"] == "ok"
assert all(row["status"] != "drift" for row in report["support_matrix"])
assert sum(row["status"] == "explicitly_excluded" for row in report["support_matrix"]) == 2
assert sum(row["status"] == "migration_transport_owned" for row in report["support_matrix"]) == 2
PY
  then
    guard_fail "generated grammar support matrix drifted"
  fi
fi
cargo test -q -p hakorune-frontend-grammar

echo "[language-v1-grammar-contract-substrate] OK"
