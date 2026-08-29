#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mir-call-d1b-exact-target-child-guard"
BRAND="$ROOT_DIR/src/mir/builder/calls/function_call_brand_source_demand.rs"
PREFLIGHT="$ROOT_DIR/src/mir/builder/calls/function_call_preflight_route.rs"
BUILD="$ROOT_DIR/src/mir/builder/calls/build.rs"
TESTS="$ROOT_DIR/src/mir/builder/calls/function_call_preflight_route_tests.rs"
RESOLVER="$ROOT_DIR/src/mir/builder/calls/resolver.rs"
MATERIALIZER="$ROOT_DIR/src/mir/builder/calls/materializer.rs"
UNIFIED="$ROOT_DIR/src/mir/builder/calls/unified_emitter.rs"
TERMINAL="$ROOT_DIR/src/mir/builder/calls/unified_emitter/physical_terminal.rs"

fail() {
  echo "[$TAG] $*" >&2
  exit 1
}

for file in "$BRAND" "$PREFLIGHT" "$BUILD" "$TESTS"; do
  [[ -f "$file" ]] || fail "missing owner ${file#$ROOT_DIR/}"
done

python3 - "$BRAND" "$PREFLIGHT" "$BUILD" "$TESTS" "$RESOLVER" "$MATERIALIZER" "$UNIFIED" "$TERMINAL" <<'PY'
from pathlib import Path
import sys

brand, preflight, build, tests, resolver, materializer, unified, terminal = map(Path, sys.argv[1:])
brand_text = brand.read_text()
preflight_text = preflight.read_text()
build_text = build.read_text()
tests_text = tests.read_text()
resolver_text = resolver.read_text()
materializer_text = materializer.read_text()
unified_text = unified.read_text()
terminal_text = terminal.read_text()

if brand_text.count("InstalledNonBrand {") != 2:
    raise SystemExit("InstalledNonBrand caller transport drifted")
if "RawInvocationRootLineageV1::Cataloged(caller)" not in brand_text:
    raise SystemExit("Cataloged source lineage no longer feeds the child transport")
if preflight_text.count("fn prepare_cataloged_target_v1(") != 1:
    raise SystemExit("cataloged target issuer count drifted")
for token in (
    "PreparedRawOrdinaryFunctionCompletionV1::CatalogedTargeted",
    "PreparedRawOrdinaryFunctionCompletionV1::Rejected",
    "BareStaticRecoveryDecisionV1::decide",
    "CallTarget::Value(value)",
):
    if token not in preflight_text:
        raise SystemExit(f"missing child target contract: {token}")
generic_targeted = "PreparedRawOrdinaryFunctionCompletionV1::" + "Targeted"
if generic_targeted in preflight_text:
    raise SystemExit("generic Targeted completion remains in preflight")
if preflight_text.count("PreparedRawOrdinaryFunctionCompletionV1::CatalogedTargeted") != 1:
    raise SystemExit("CatalogedTargeted producer count drifted")
if preflight_text.count("bare-static-method-retired") < 2:
    raise SystemExit("bare static method retirement is not closed at both recovery edges")

if preflight_text.count("PreparedRawNonBrandRouteOriginV1::InstalledNonBrand") < 2:
    raise SystemExit("InstalledNonBrand origin is not carried through the ordinary preflight")
if preflight_text.count('"gc_collect" | "gc_stats"') != 1:
    raise SystemExit("GC exact two-name cohort drifted")
if "PreparedRawNonBrandRouteOriginV1::UnclassifiedSource" not in preflight_text:
    raise SystemExit("unclassified source boundary disappeared")

target_start = build_text.index("PreparedRawOrdinaryFunctionCompletionV1::CatalogedTargeted")
app_main_start = build_text.index("PreparedRawOrdinaryFunctionCompletionV1::AppMainTargeted", target_start)
target_window = build_text[target_start:app_main_start]
if target_window.count("lower_prepared_targeted_call_v1") != 1:
    raise SystemExit("CatalogedTargeted child handoff count drifted")
if generic_targeted in build_text:
    raise SystemExit("generic Targeted completion remains in build")
if build_text.count("PreparedRawOrdinaryFunctionCompletionV1::CatalogedTargeted") != 1:
    raise SystemExit("CatalogedTargeted consumer count drifted")

helper_start = build_text.index("fn lower_prepared_targeted_call_v1")
helper_end = build_text.index("impl MirBuilder", helper_start)
helper_window = build_text[helper_start:helper_end]
if helper_window.index("drive_call_arguments_v1") > helper_window.index("emit_prepared_cataloged_call_v1"):
    raise SystemExit("targeted helper emits before ordered argument descent")
if any(token in helper_window for token in ("build_resolved_function_call", "try_unique_static_method_recovery", "make_name_const_result")):
    raise SystemExit("targeted helper re-entered a late resolver/recovery/name-Const edge")

emit_start = build_text.index("fn emit_prepared_cataloged_call_v1")
emit_end = build_text.index("\n}\n\n#[cfg(test)]", emit_start)
emit_window = build_text[emit_start:emit_end]
if emit_window.count("MirInstruction::call(") != 1:
    raise SystemExit("targeted child canonical issuer count drifted")
if "MirInstruction::Call {" in emit_window or "make_name_const_result" in emit_window:
    raise SystemExit("targeted child retained a legacy Call literal or name Const")
for retired in (
    "PreparedRawOrdinaryFunctionCompletionV1::Resolved",
    "build_resolved_function_call",
    "build_unified_function_call",
    "try_unique_static_method_recovery",
):
    if retired in build_text:
        raise SystemExit(f"caller-zero resolved edge reappeared: {retired}")

for token in (
    "cataloged_target_preflight_applies_total_shadow_order",
    "cataloged_bare_static_rejects_before_children",
    "cataloged_local_value_target_is_consumed_once_before_canonical_call_publication",
):
    if token not in tests_text:
        raise SystemExit(f"missing focused child evidence: {token}")
if "bare-static-method-retired" not in tests_text:
    raise SystemExit("missing focused bare-static retirement evidence")

resolve_start = resolver_text.index("pub fn resolve(&self, target: CallTarget)")
resolve_end = resolver_text.index("    /// Call引数の検証", resolve_start)
resolve_window = resolver_text[resolve_start:resolve_end]
if any(token in resolve_window for token in ("Err(", "return Err")):
    raise SystemExit("resolver totality changed: an Err arm needs a new design row")
for token in (
    "CallTarget::Global",
    "CallTarget::Method",
    "CallTarget::Constructor",
    "CallTarget::Extern",
    "CallTarget::Value",
    "CallTarget::Closure",
):
    if token not in resolve_window:
        raise SystemExit(f"resolver totality matrix lost {token}")

unified_start = unified_text.index("let resolver = super::resolver::CalleeResolverBox::new")
unified_end = unified_text.index("        // Structural guard FIRST", unified_start)
unified_window = unified_text[unified_start:unified_end]
if unified_window.count("resolver.resolve(target.clone())?") != 1:
    raise SystemExit("unified emitter resolver consume is not exactly one direct propagation")
for token in (
    "try_global_additional_resolvers_with_authority",
    "GlobalPresenceAuthorityV1",
    "AdditionalGlobalResolver",
):
    if token in unified_window:
        raise SystemExit(f"unified emitter retained deleted recovery token: {token}")

for token in (
    "GlobalPresenceAuthorityV1",
    "try_global_additional_resolvers_with_authority",
    "make_name_const_result",
    "MirInstruction::Call",
    "BareStaticRecoveryDecisionV1",
):
    if token in materializer_text:
        raise SystemExit(f"materializer retained deleted Global recovery token: {token}")
if materializer_text.count("materialize_receiver_in_callee") != 1:
    raise SystemExit("active receiver materialization owner disappeared")
if "AdditionalGlobalResolver" in terminal_text:
    raise SystemExit("alternate Global recovery route remains in physical terminal")
PY

echo "[$TAG] ok"
