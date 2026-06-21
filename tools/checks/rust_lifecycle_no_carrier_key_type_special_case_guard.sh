#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
from pathlib import Path

source_path = Path("src/mir/ordered_map_origin_plan.rs")
source = source_path.read_text()
impl_source = source.split("#[cfg(test)]", 1)[0]

for forbidden in [
    'key == "requested_names"',
    "key == 'requested_names'",
    "seed_string_key",
    "seed_text_key",
    'MirType::Box("StringBox".to_string())',
]:
    if forbidden in impl_source:
        raise SystemExit(
            f"carrier_key_type_special_case=fail forbidden={forbidden!r}"
        )

required = [
    'seed_array_key(schema, output_origin, "requested_names")',
    'seed_array_key(schema, output_origin, "carrier_names")',
    'seed_array_key(schema, output_origin, "carrier_host_ids")',
]
for needle in required:
    if needle not in impl_source:
        raise SystemExit(
            f"carrier_key_type_special_case=fail missing_required={needle!r}"
        )

fixture = Path(
    "docs/development/current/main/design/fixtures/rust-lifecycle/"
    "variable-context-explicit-carrier-snapshot-derived-artifact-verifier-result-v0.json"
).read_text()
if '"requested_names_transport": "ArrayBox"' not in fixture:
    raise SystemExit(
        "carrier_key_type_special_case=fail requested_names_transport_not_arraybox"
    )

print("output_contract=rust-lifecycle-no-carrier-key-type-special-case-v0")
print("no_requested_names_stringbox_override=green")
print("no_key_name_type_override=green")
print("requested_names_transport=ArrayBox")
print("summary=ok")
PY
