#!/usr/bin/env python3
"""P0c A-prime callable authority and atomic I1 activation guard."""

from __future__ import annotations

import pathlib
import re
import sys


def fail(message: str) -> None:
    raise SystemExit(f"[resolved-callable-l0] {message}")


root = pathlib.Path(sys.argv[1]).resolve()
callable_index = root / "src/mir/resolved_semantics/callable_index.rs"
header_view = root / "src/mir/resolved_semantics/callable_header_view.rs"
direct_call = root / "src/mir/canonical_direct_call.rs"
direct_call_contract = root / "src/mir/canonical_direct_call_contract.rs"
direct_call_profile = root / "src/mir/resolved_value_profile/direct_call.rs"
direct_call_profile_tests = root / "src/mir/resolved_value_profile/direct_call_tests.rs"
direct_call_lower = root / "src/mir/builder/resolved_lowering/trivial_ssa/direct_call.rs"
direct_call_lower_tests = root / "src/mir/builder/resolved_lowering/direct_call_tests.rs"
capability = root / "src/mir/canonical_direct_static_call_capability.rs"
backend_gate = root / "src/mir/canonical_direct_static_call_backend_capability.rs"
metadata = root / "src/mir/function/metadata.rs"
shared_gate = root / "src/mir/backend_capability.rs"
owner_resolver = root / "src/mir/resolved_semantics/owner_resolver.rs"
resolved_target = root / "src/mir/resolved_semantics/direct_call.rs"
resolved_unit = root / "src/mir/resolved_semantics/resolved_callable_forest.rs"

required = {
    callable_index: [
        "CanonicalCallableKeyV1",
        "ResolvedCallableRefV1",
        "CanonicalCallableSymbolV1",
        "ExactTrivialCallableSignatureV1",
        "VerifiedCallableIndexV1",
        "pub(crate) fn seal_one(",
    ],
    header_view: [
        "CallableHeaderSyntaxViewV1",
        "CallableFunctionSyntaxViewV1",
        "from_function_ast",
    ],
    owner_resolver: ["resolve_forest_with_root_callable"],
    resolved_target: ["ResolvedDirectCallTargetV1", "ResolvedCallableRefV1"],
    resolved_unit: [
        "VerifiedResolvedCallableForestV1",
        "VerifiedCallableIndexV1",
        "VerifiedSemanticOwnerForestV1",
    ],
    direct_call_contract: [
        "VerifiedTrivialDirectCallTargetV1",
        "VerifiedDirectCallEffectV1",
    ],
    direct_call: [
        "VerifiedCanonicalDirectCallEmissionV1",
        "Callee::Global",
    ],
    direct_call_profile: [
        "VerifiedTrivialDirectCallV1",
        "VerifiedTrivialDirectCallTargetV1",
        "VerifiedDirectCallEffectV1",
    ],
    capability: ["canonical_direct_static_call_v1"],
    backend_gate: [
        "canonical_direct_static_call_capabilities",
        "silent_fallback_allowed=false",
    ],
    metadata: ["canonical_direct_static_call_capabilities"],
    shared_gate: ["canonical_direct_static_call_backend_capability::enforce"],
}
for path, needles in required.items():
    if not path.is_file():
        fail(f"missing file: {path.relative_to(root)}")
    text = path.read_text()
    for needle in needles:
        if needle not in text:
            fail(f"missing contract {needle!r} in {path.relative_to(root)}")

backend_text = backend_gate.read_text()
for forbidden in ["MirInstruction", "parameter_entry_contracts", "return_exit_contract"]:
    if forbidden in backend_text:
        fail(f"backend gate infers capability from forbidden surface: {forbidden}")

direct_text = direct_call.read_text()
for forbidden in [
    "build_function_call",
    "build_legacy_function_call",
    "build_unified_function_call",
    "annotate_call_result_from_func_name",
    "effects_analyzer",
    "EffectsAnalyzerBox",
    "compute_call_effects",
    "emit_global_unified",
    "name_const",
    "FunctionCall.name",
]:
    if forbidden in direct_text:
        fail(f"canonical emission facade imports legacy authority: {forbidden}")

lowering_text = "\n".join(
    [
        direct_call_lower.read_text(),
        (root / "src/mir/builder/resolved_lowering/trivial_ssa/lowerer.rs").read_text(),
    ]
)
for forbidden in [
    "build_function_call",
    "build_legacy_function_call",
    "build_unified_function_call",
    "annotate_call_result_from_func_name",
    "FunctionCall { name",
    "callee: None",
    "LegacyImplicitShare",
]:
    if forbidden in lowering_text:
        fail(f"canonical P0c Lower imports forbidden authority: {forbidden}")

for path in [callable_index, direct_call_contract, direct_call_profile]:
    for forbidden in ["CurrentFunction", "CurrentFunctionCall"]:
        if forbidden in path.read_text():
            fail(
                "generic callable vocabulary contains current-function-specific "
                f"authority: {path.relative_to(root)}: {forbidden}"
            )

allowed_by_pattern = {
    r"VerifiedCanonicalDirectCallEmissionV1": {
        direct_call,
        direct_call_lower,
        root / "src/mir/canonical_direct_call_tests.rs",
    },
    r"VerifiedCallableIndexV1::seal_one": {
        root / "src/mir/canonical_direct_call_tests.rs",
        root / "src/mir/resolved_semantics/callable_index_tests.rs",
        owner_resolver,
    },
    r"resolve_forest_with_root_callable": {
        owner_resolver,
        root / "src/mir/compiler/lowering_input.rs",
        root / "src/mir/resolved_semantics/direct_call_tests.rs",
    },
    r"VerifiedCanonicalDirectCallEmissionV1::conservative_from_header": {
        root / "src/mir/canonical_direct_call_tests.rs",
    },
    r"VerifiedCanonicalDirectCallEmissionV1::from_verified_profile": {
        direct_call_lower,
    },
    r"analyze_trivial_canonical_owner_with_direct_call_v1": {
        root / "src/mir/compiler/capability.rs",
        root / "src/mir/resolved_value_profile/mod.rs",
        direct_call_profile_tests,
    },
    r"\.claim_direct_call\(": {
        direct_call_profile_tests,
        root / "src/mir/builder/resolved_lowering/trivial_ssa/lowerer.rs",
    },
    r"canonical_direct_static_call_capabilities\s*\.push": {
        root / "src/mir/backend_capability.rs",
        direct_call_lower,
        root / "src/mir/canonical_direct_static_call_backend_capability_tests.rs",
    },
    r"CanonicalDirectStaticCallCapabilityV1::v1": {
        root / "src/mir/backend_capability.rs",
        direct_call_lower,
        root / "src/mir/canonical_direct_static_call_backend_capability_tests.rs",
    },
}
for pattern, allowed in allowed_by_pattern.items():
    actual = {
        path
        for path in (root / "src").rglob("*.rs")
        if re.search(pattern, path.read_text())
    }
    unexpected = sorted(path.relative_to(root) for path in actual - allowed)
    if unexpected:
        fail(f"L0 production caller/producer escaped for {pattern!r}: {unexpected}")

for path in [
    *required,
    root / "src/mir/resolved_semantics/callable_index_tests.rs",
    root / "src/mir/canonical_direct_call_tests.rs",
    direct_call_profile_tests,
    direct_call_lower,
    direct_call_lower_tests,
    root / "src/mir/canonical_direct_static_call_backend_capability_tests.rs",
]:
    lines = len(path.read_text().splitlines())
    if lines >= 800:
        fail(f"source/check reached 800-line stop boundary: {path.relative_to(root)} ({lines})")

if direct_call_lower_tests.read_text().count("#[test]") != 5:
    fail("P0c-I1 focused runtime fixture count must remain exactly five")

print("[resolved-callable-l0] ok")
