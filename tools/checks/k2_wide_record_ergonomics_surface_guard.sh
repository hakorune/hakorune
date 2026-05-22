#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-record-ergonomics-surface"
cd "$ROOT_DIR"

echo "[$TAG] validating record defaults/shorthand/with surface"

cargo test --release source_to_program_json_v0_fills_record_literal_defaults_when_omitted -- --nocapture >/dev/null
cargo test --release source_to_program_json_v0_record_literal_keeps_type_namespace_with_same_value_name -- --nocapture >/dev/null
cargo test --release parser_record_literal_surface_allows_same_type_and_value_name -- --nocapture >/dev/null

rg -q "Constructor lookup is type-namespace based" docs/reference/language/quick-reference.md
rg -q "omitted defaulted fields are materialized in declaration order" docs/reference/language/stage-profiles.md
rg -q "Constructor IDENT is resolved in the type namespace" docs/reference/language/EBNF.md

echo "[$TAG] ok"
