#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-rune-contract-repeat"
cd "$ROOT_DIR"

echo "[$TAG] running M11c-contract-repeat guard"

cargo test -q parser_accepts_distinct_contract_runes_on_same_declaration
cargo test -q parser_rejects_duplicate_contract_rune_value

rg -F -q 'fn repeatable_rune_name(name: &str) -> bool' src/parser/runes.rs
rg -F -q 'matches!(name, "Contract")' src/parser/runes.rs
rg -F -q 'rune.args.join' src/parser/runes.rs
rg -F -q '_repeatable_rune_name(rune_name)' lang/src/compiler/parser/rune/rune_contract_box.hako
rg -F -q 'if name == "Contract" { return 1 }' lang/src/compiler/parser/rune/rune_contract_box.hako
rg -F -q '_contains_rune_name_arg0' lang/src/compiler/parser/rune/rune_contract_box.hako
rg -F -q 'M11c-contract-repeat' docs/development/current/main/design/inline-plan-ssot.md
rg -F -q 'M11c-contract-repeat is live as a parser metadata shape.' \
  docs/development/current/main/phases/phase-293x/293x-057-M11C-CONTRACT-REPEATABLE-PARSER.md

if rg -F -q 'fn repeatable_rune_name' lang/c-abi/shims -g '*.inc'; then
  echo "[$TAG] ERROR: .inc must not own repeatable rune policy" >&2
  exit 1
fi

echo "[$TAG] ok"
