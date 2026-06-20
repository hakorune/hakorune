#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

python3 - <<'PY'
import json
import re
from pathlib import Path

root = Path(".")
src = root / "src" / "mir"

def strip_cfg_test_modules(text):
    """Remove simple #[cfg(test)] mod tests { ... } blocks from production scans."""
    lines = text.splitlines()
    out = []
    skip = False
    brace_depth = 0
    pending_cfg_test = False
    for line in lines:
        stripped = line.strip()
        if not skip and stripped == "#[cfg(test)]":
            pending_cfg_test = True
            continue
        if pending_cfg_test and stripped.startswith("mod tests"):
            skip = True
            pending_cfg_test = False
            brace_depth = line.count("{") - line.count("}")
            if brace_depth <= 0:
                skip = False
            continue
        if pending_cfg_test:
            out.append("#[cfg(test)]")
            pending_cfg_test = False
        if skip:
            brace_depth += line.count("{") - line.count("}")
            if brace_depth <= 0:
                skip = False
            continue
        out.append(line)
    return "\n".join(out)

rust_files = [
    p for p in src.rglob("*.rs")
    if p.name != "tests.rs" and "tests" not in p.parts
]
production_by_file = {p: strip_cfg_test_modules(p.read_text()) for p in rust_files}
text = "\n".join(production_by_file.values())

some_hits = [
    str(p) for p, file_text in production_by_file.items()
    if "join_id: Some(" in file_text
]
assign_hits = [
    str(p) for p, file_text in production_by_file.items()
    if re.search(r"\.join_id\s*=", file_text)
]
assert not some_hits, some_hits
assert not assign_hits, assign_hits
assert "join_id: None" in text, "missing production join_id=None initializer"

base = Path("docs/development/current/main/design/fixtures/rust-lifecycle")
resolver = json.loads((base / "hako-lifecycle-resolver-readonly-diagnostics-v0.json").read_text())
deny = {row["id"]: row for row in resolver["deny"]}
assert deny["CarrierVar.join_id.production_lifecycle"]["decision"] == "DenyUnresolvedBoundary"
assert deny["CarrierVar.join_id.production_lifecycle"]["reason"] == "no_production_Some_ValueId_producer"
assert resolver["claims"]["join_id_dependent_paths_allowed"] is False

verifier = json.loads((base / "carrier-info-merge-from-verifier-result-v0.json").read_text())
assert "join_id producer" in set(verifier["denied_boundaries"])
assert verifier["claims"]["emission_allowed"] is False

emitter = json.loads((base / "carrier-info-merge-from-emitter-verifier-result-v0.json").read_text())
assert "join_id producer" in set(emitter["denied_boundaries"])
assert emitter["claims"]["emission_allowed"] is True
assert emitter["claims"]["emission_scope"] == "CarrierInfo::merge_from only"

surface = (base / "carrier-info-merge-from-emitter-surface-v0.hako").read_text()
assert "Denied boundary: no join_id producer is emitted here." in surface
assert "Verified boundary: join_id producer" not in surface
PY

cat <<'REPORT'
output_contract=rust-lifecycle-join-id-vocabulary-v0
production_join_id_some_producer=0
production_join_id_mutation_assignment=0
production_join_id_none_initializers=present
resolver_denies_join_id=green
verifier_denies_join_id=green
emitter_denies_join_id=green
join_id_retired_now=0
join_id_implemented_now=0
summary=ok
REPORT
