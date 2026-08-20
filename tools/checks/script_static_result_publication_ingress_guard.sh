#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$root"

ingress="src/mir/builder/static_result_publication_ingress.rs"
bridge="src/mir/builder/calls/static_result_publication_physical_bridge.rs"
member="src/mir/builder/calls/member_route.rs"
me="src/mir/builder/method_call_handlers.rs"
terminal="src/mir/builder/calls/method_call_terminal.rs"
transport="src/mir/builder/raw_invocation_source_transport.rs"

test -f "$ingress"
test -f "$bridge"
test ! -e src/mir/builder/raw_static_result_publication.rs

grep -q 'Unavailable' "$ingress"
grep -q 'Absent' "$ingress"
grep -q 'Selected' "$ingress"
grep -q 'SourceContextMissing' "$ingress"
grep -q 'SourceLocationLost' "$ingress"
grep -q 'ForeignLineage' "$ingress"
grep -q 'self.callable_ledger.is_some()' "$ingress"
grep -q 'expected_lineage: Some(RawInvocationRootLineageV1::Cataloged' "$ingress"
grep -q 'StaticResultPublicationIngressV1::Selected' "$member"
grep -q 'StaticResultPublicationIngressV1::Absent' "$member"
grep -q 'StaticResultPublicationIngressV1::Unavailable' "$member"
grep -q 'handle_me_method_call_with_publication_ingress' "$member"
grep -q 'resolve_me_call_with_publication_ingress' "$me"

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

test "$(wc -l < "$transport")" -le 759
test "$(wc -l < "$ingress")" -lt 760
test "$(wc -l < "$bridge")" -lt 760
test "$(wc -l < "$terminal")" -lt 760

echo "script static-result publication ingress guard: PASS"
