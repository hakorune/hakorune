#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$root"

ingress="src/mir/builder/static_result_publication_ingress.rs"
bridge="src/mir/builder/calls/static_result_publication_physical_bridge.rs"
member="src/mir/builder/calls/member_route.rs"
me="src/mir/builder/method_call_handlers/static_current_owner_policy.rs"
terminal="src/mir/builder/calls/method_call_terminal.rs"
transport="src/mir/builder/raw_invocation_source_transport"

test -f "$ingress"
test -f "$bridge"
test ! -e src/mir/builder/raw_static_result_publication.rs

grep -q 'Unavailable' "$ingress"
grep -q 'NoExactStaticTarget' "$ingress"
grep -q 'TargetOnly' "$ingress"
grep -q 'Selected' "$ingress"
grep -q 'SourceContextMissing' "$ingress"
grep -q 'SourceLocationLost' "$ingress"
grep -q 'ForeignLineage' "$ingress"
grep -q 'self.callable_ledger.is_some()' "$ingress"
grep -q 'expected_lineage: Some(RawInvocationRootLineageV1::Cataloged' "$ingress"
grep -q 'StaticResultPublicationIngressV1::Selected' "$member"
grep -q 'StaticResultPublicationIngressV1::NoExactStaticTarget' "$member"
grep -q 'StaticResultPublicationIngressV1::Unavailable' "$member"
grep -q 'handle_me_method_call_with_publication_ingress' "$member"
grep -q 'resolve_me_call_with_publication_ingress' "$me"

# Check the rejecting arm itself, not a diagnostic elsewhere in the file.
python3 - "$ingress" "$member" "$me" <<'PY_GUARD'
import pathlib
import re
import sys

ingress, *consumers = [pathlib.Path(path) for path in sys.argv[1:]]
source = ingress.read_text()
enum = re.search(r"enum StaticResultPublicationIngressV1\s*\{(.*?)\n\}", source, re.S)
assert enum, "publication ingress enum missing"
for state in ("Unavailable", "NoExactStaticTarget", "TargetOnly", "Selected"):
    assert re.search(r"^\s*" + state + r"\b", enum[1], re.M), state
assert not re.search(r"\bAbsent\b", enum[1]), "obsolete Absent ingress state"
for path in consumers:
    source = path.read_text()
    arm = re.search(
        r"Ok\(StaticResultPublicationIngressV1::TargetOnly\(target\)\)\s*=>\s*\{"
        r"(.*?)\n\s*Ok\(StaticResultPublicationIngressV1::NoExactStaticTarget\)",
        source, re.S,
    )
    assert arm, f"{path}: TargetOnly rejection arm missing"
    assert re.search(r"^\s*(?:return\s+)?Err\(format!\(", arm[1]), path
    for token in ("static-result-ingress/target-only/", "target.reason()", "target.target().mir_symbol_projection()"):
        assert token in arm[1], f"{path}: missing {token}"
    assert not re.search(r"lower_|descent\.|handle_", arm[1]), f"{path}: TargetOnly must reject before descent"
PY_GUARD

if rg -n 'try_emit_source_bound_static_call_result_v1|raw_static_result_publication' src/mir/builder; then
  echo "legacy terminal publication hook remains" >&2
  exit 1
fi
if rg -n 'ASTNode::MethodCall|ASTNode::Call' "$ingress" "$bridge"; then
  echo "publication ingress/bridge added a second AST matcher" >&2
  exit 1
fi
if rg -n 'Option::None|unwrap_or\(|unwrap_or_default\(' "$ingress" "$bridge"; then
  echo "publication ingress collapsed an outcome through a default" >&2
  exit 1
fi

for source in "$transport"/*.rs; do
  test "$(wc -l < "$source")" -le 759
done
test "$(wc -l < "$ingress")" -lt 760
test "$(wc -l < "$bridge")" -lt 760
test "$(wc -l < "$terminal")" -lt 760

echo "script static-result publication ingress guard: PASS"
