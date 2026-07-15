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
module_header_view = root / "src/mir/resolved_semantics/callable_module_header_view.rs"
header_source_unit = root / "src/mir/resolved_semantics/callable_header_source_unit.rs"
catalog_candidate = root / "src/mir/resolved_semantics/callable_catalog_candidate.rs"
catalog = root / "src/mir/resolved_semantics/callable_catalog.rs"
catalog_resolution_source = (
    root / "src/mir/resolved_semantics/callable_catalog_resolution_source.rs"
)
normalized_catalog = root / "src/mir/resolved_semantics/normalized_callable_catalog.rs"
catalog_tests = root / "src/mir/resolved_semantics/callable_catalog_tests.rs"
callable_index_tests = root / "src/mir/resolved_semantics/callable_index_tests.rs"
resolved_module = root / "src/mir/compiler/resolved_callable_module.rs"
resolved_module_tests = root / "src/mir/compiler/resolved_callable_module_tests.rs"
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
        "CallableCatalogCardinalityErrorV1",
        "headers_by_key",
        "key_by_callable",
        "key_by_symbol",
        "pub(crate) fn sole_header(",
        "pub(crate) fn header_for_symbol(",
        "pub(crate) fn seal_one(",
    ],
    header_view: [
        "CallableHeaderSyntaxViewV1",
        "CallableFunctionSyntaxViewV1",
        "from_function_ast",
    ],
    module_header_view: [
        "SourceCallableDeclarationSiteV1",
        "CallableModuleHeaderSyntaxViewV1",
        "LocatedCallableHeaderSyntaxViewV1",
        "UnsupportedProgramStatement",
        "CallableHeaderSyntaxViewV1::from_function_ast",
    ],
    header_source_unit: [
        "CanonicalProgramSyntaxOwnerV1",
        "VerifiedCallableHeaderSourceUnitV1",
        "seal_header_surface",
        "declaration_sites",
        "located_header",
    ],
    catalog_candidate: [
        "VerifiedOwnerFreeCallableCatalogSourceUnitV1",
        "VerifiedOwnerFreeCallableHeaderV1",
        "candidates_by_site",
        "site_by_key",
        "site_by_symbol",
        "HeaderOutsideExactI64Profile",
        "DuplicateSourceKey",
        "PhysicalSymbolCollision",
    ],
    catalog: [
        "VerifiedCallableCatalogSourceUnitV1",
        "VerifiedCallableCatalogV1",
        "VerifiedCallableDeclarationV1",
        "CatalogSealedResolverContinuationV1",
        "CallableCatalogSealOutcomeV1",
        "VerifiedCallableIndexV1::seal_many",
        "resolver.issue_owner()",
        "into_resolver(self)",
    ],
    catalog_resolution_source: [
        "CallableCatalogResolutionSourceV1",
        "LocatedCallableResolutionViewV1",
        "into_resolution_parts",
        "restore_after_resolution",
        "CallableFunctionSyntaxViewV1::from_function_ast",
    ],
    normalized_catalog: [
        "NormalizedCallableCatalogV1",
        "NormalizedCallableCatalogRowV1",
        "from_catalog",
        "header.source_key().namespace()",
        "header.signature().params()",
        "header.symbol().as_mir_name()",
    ],
    catalog_tests: [
        "normalized_catalog_is_independent_of_declaration_order_and_owner_brand",
        "declaration_reorder_preserves_exact_lookup_results",
    ],
    callable_index_tests: [
        "malformed_private_draft_rejects_duplicate_identity_and_symbol",
        "DuplicateCallableIdentity",
        "DuplicatePhysicalSymbol",
    ],
    resolved_module: [
        "ResolveCallableModuleErrorV1",
        "CallableCatalogSealOutcomeV1",
        "VerifiedResolvedCallableModuleV1",
        "VerifiedResolvedFunctionUnitV1",
        "VerifiedCallableCatalogSourceUnitV1",
        "BTreeMap<CanonicalCallableKeyV1, VerifiedResolvedFunctionUnitV1>",
        "SourceCallableDeclarationSiteV1",
        "VerifiedSemanticOwnerForestV1",
        "VerifiedSourceProjectionV1",
        "functions_by_key",
        "pub(crate) fn resolve(",
        "resolve_forest_with_reserved_root",
        "VerifiedSourceProjectionV1::seal",
    ],
    resolved_module_tests: [
        "passive_module_carrier_exposes_only_the_canonical_keyed_primary_map",
        "passive_function_unit_keeps_site_forest_and_projection_together",
    ],
    owner_resolver: [
        "resolve_forest_with_root_callable",
        "resolve_forest_with_reserved_root",
        "seal_owner_with_ancestors_and_callable_index",
    ],
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

for path in [module_header_view, header_source_unit]:
    text = path.read_text()
    for forbidden in [
        "FunctionSyntaxViewV1",
        "VerifiedCallableIndexV1",
        "FunctionOwnerIssuerV1",
        "MirInstruction",
    ]:
        if forbidden in text:
            fail(f"CAT0-S0 header-only surface owns forbidden authority {forbidden!r}")

candidate_text = catalog_candidate.read_text()
for forbidden in [
    "FunctionOwnerIdV1",
    "FunctionOwnerIssuerV1",
    "FunctionOriginV1",
    "VerifiedCallableIndexV1",
    "MirInstruction",
    "MirBuilder",
]:
    if forbidden in candidate_text:
        fail(f"CAT0-C0a owner-free candidate owns forbidden authority {forbidden!r}")

catalog_text = catalog.read_text()
for forbidden in [
    "FunctionSyntaxViewV1",
    "resolve_function_shadow",
    "VerifiedSemanticOwnerForestV1",
    "MirInstruction",
    "MirBuilder",
]:
    if forbidden in catalog_text:
        fail(f"CAT0-C0b catalog seal owns forbidden body/runtime authority {forbidden!r}")
if re.search(
    r"#\[derive\([^]]*Clone[^]]*\)\]\s*pub\(crate\) struct "
    r"CatalogSealedResolverContinuationV1",
    catalog_text,
):
    fail("CAT0-C0b resolver continuation must remain non-Clone and single-use")
if re.search(
    r"#\[derive\([^]]*Clone[^]]*\)\]\s*pub\(crate\) struct "
    r"VerifiedCallableCatalogV1",
    catalog_text,
):
    fail("CAT0-C0b catalog must remain attached to its owned Program source unit")
if catalog_text.count("VerifiedCallableCatalogSourceUnitV1 {") != 3:
    fail("CAT0-G0 final Program/catalog product gained a foreign pairing seam")

normalized_text = normalized_catalog.read_text()
for forbidden in [
    "FunctionOriginV1",
    "FunctionOwnerIdV1",
    "SourceCallableDeclarationSiteV1",
    "ResolvedCallableRefV1",
    "compilation_brand",
    "owner.slot",
]:
    if forbidden in normalized_text:
        fail(f"CAT0-G0 normalized parity includes invocation-local identity: {forbidden!r}")

resolved_module_text = resolved_module.read_text()
for forbidden in [
    "ASTNode",
    "MirBuilder",
    "MirInstruction",
    "lower",
]:
    if forbidden in resolved_module_text:
        fail(f"MP0-S0 passive carrier owns forbidden producer/runtime authority {forbidden!r}")
for forbidden_pattern in [
    r"impl\s+Clone\s+for\s+VerifiedResolvedCallableModuleV1",
    r"impl\s+Clone\s+for\s+VerifiedResolvedFunctionUnitV1",
    r"fn\s+(seal|from_parts)\s*\(",
]:
    if re.search(forbidden_pattern, resolved_module_text):
        fail(f"MP0-S0 passive carrier gained a construction seam: {forbidden_pattern!r}")
if resolved_module_text.count("VerifiedResolvedCallableModuleV1 {") != 2:
    fail("MP0-S0 module carrier must have one definition and one accessor impl only")
if resolved_module_text.count("VerifiedResolvedFunctionUnitV1 {") != 2:
    fail("MP0-S0 function unit must have one definition and one accessor impl only")
if resolved_module_text.count("pub(crate) fn resolve(") != 1:
    fail("MP0-R0 must have exactly one resolved-module construction entry")

resolution_source_text = catalog_resolution_source.read_text()
for forbidden in ["MirBuilder", "MirInstruction", "ValueId", "BasicBlockId"]:
    if forbidden in resolution_source_text:
        fail(f"MP0-R0 consuming source view owns runtime authority {forbidden!r}")
if resolution_source_text.count("CallableCatalogResolutionSourceV1 {") != 2:
    fail("MP0-R0 consuming source view gained a second construction authority")

source_unit_users = []
for path in (root / "src").rglob("*.rs"):
    if path in {
        header_source_unit,
        catalog_candidate,
        catalog,
        catalog_resolution_source,
        root / "src/mir/resolved_semantics/callable_header_source_unit_tests.rs",
        root / "src/mir/resolved_semantics/callable_catalog_candidate_tests.rs",
        catalog_tests,
        resolved_module_tests,
        root / "src/mir/resolved_semantics/mod.rs",
    }:
        continue
    if "VerifiedCallableHeaderSourceUnitV1" in path.read_text():
        source_unit_users.append(str(path.relative_to(root)))
if source_unit_users:
    fail(f"CAT0-S0 source unit has production callers: {source_unit_users}")

candidate_users = []
for path in (root / "src").rglob("*.rs"):
    if path in {
        catalog_candidate,
        catalog,
        root / "src/mir/resolved_semantics/callable_catalog_candidate_tests.rs",
        catalog_tests,
        resolved_module_tests,
        root / "src/mir/resolved_semantics/mod.rs",
    }:
        continue
    if "VerifiedOwnerFreeCallableCatalogSourceUnitV1" in path.read_text():
        candidate_users.append(str(path.relative_to(root)))
if candidate_users:
    fail(f"CAT0-C0a candidate product has production callers: {candidate_users}")

catalog_users = []
for path in (root / "src").rglob("*.rs"):
    if path in {
        catalog,
        catalog_tests,
        catalog_resolution_source,
        resolved_module,
        resolved_module_tests,
        root / "src/mir/resolved_semantics/mod.rs",
    }:
        continue
    if "VerifiedCallableCatalogSourceUnitV1" in path.read_text():
        catalog_users.append(str(path.relative_to(root)))
if catalog_users:
    fail(f"CAT0-C0b final catalog escaped its passive MP0-S0 carrier: {catalog_users}")

resolved_module_users = []
for path in (root / "src").rglob("*.rs"):
    if path in {
        resolved_module,
        resolved_module_tests,
        root / "src/mir/compiler/mod.rs",
    }:
        continue
    text = path.read_text()
    if (
        "VerifiedResolvedCallableModuleV1" in text
        or "VerifiedResolvedFunctionUnitV1" in text
    ):
        resolved_module_users.append(str(path.relative_to(root)))
if resolved_module_users:
    fail(f"MP0-S0 passive carrier has production producers/consumers: {resolved_module_users}")

resolution_source_users = []
for path in (root / "src").rglob("*.rs"):
    if path in {
        catalog,
        catalog_resolution_source,
        resolved_module,
        root / "src/mir/resolved_semantics/mod.rs",
    }:
        continue
    if "CallableCatalogResolutionSourceV1" in path.read_text():
        resolution_source_users.append(str(path.relative_to(root)))
if resolution_source_users:
    fail(f"MP0-R0 consuming source view escaped exact producer path: {resolution_source_users}")

normalized_users = []
for path in (root / "src").rglob("*.rs"):
    if path in {
        normalized_catalog,
        catalog_tests,
        root / "src/mir/resolved_semantics/mod.rs",
    }:
        continue
    if "NormalizedCallableCatalogV1" in path.read_text():
        normalized_users.append(str(path.relative_to(root)))
if normalized_users:
    fail(f"CAT0-G0 normalized product has production callers: {normalized_users}")

callable_index_text = callable_index.read_text()
for forbidden in ["only_header(", ".values().find", ".values()\n            .find"]:
    if forbidden in callable_index_text:
        fail(f"callable index retains linear/panicking compatibility seam: {forbidden}")

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
    callable_index_tests,
    catalog_tests,
    root / "src/mir/canonical_direct_call_tests.rs",
    direct_call_profile_tests,
    direct_call_lower,
    direct_call_lower_tests,
    resolved_module,
    resolved_module_tests,
    catalog_resolution_source,
    root / "src/mir/canonical_direct_static_call_backend_capability_tests.rs",
]:
    lines = len(path.read_text().splitlines())
    if lines >= 800:
        fail(f"source/check reached 800-line stop boundary: {path.relative_to(root)} ({lines})")

if direct_call_lower_tests.read_text().count("#[test]") != 5:
    fail("P0c-I1 focused runtime fixture count must remain exactly five")

print("[resolved-callable-l0] ok")
