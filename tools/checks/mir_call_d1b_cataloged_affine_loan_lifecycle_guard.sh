#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mir-call-d1b-cataloged-affine-loan-lifecycle"
CARD="$ROOT_DIR/docs/development/current/main/investigations/mir-call-d1b-root-lineage-exact-target-loan-d0-2026-08-26.toml"
MANIFEST="$ROOT_DIR/tools/checks/guard_rows.toml"
MIR_ROOT="$ROOT_DIR/src/mir"

fail() {
  echo "[$TAG] $*" >&2
  exit 1
}

[[ $# -le 1 ]] || fail "usage: $0 [readiness|bridge_ready|observer_i0|cataloged_i0]"
# With no explicit argument, the active card owns the current guard phase.
# Historical phases remain available for explicit audit, but the manifest
# entry must never silently run an obsolete pre-bridge phase.
PHASE="${1:-}"
case "$PHASE" in
  ""|readiness|bridge_ready|observer_i0|cataloged_i0) ;;
  *) fail "unknown phase: $PHASE" ;;
esac

for file in "$CARD" "$MANIFEST" "$MIR_ROOT"; do
  [[ -e "$file" ]] || fail "missing owner ${file#$ROOT_DIR/}"
done

python3 - "$ROOT_DIR" "$CARD" "$MANIFEST" "$MIR_ROOT" "$PHASE" <<'PY'
from pathlib import Path
import sys
import tomllib

root, card_path, manifest_path, mir_root = map(Path, sys.argv[1:5])
phase = sys.argv[5]

with card_path.open("rb") as stream:
    card = tomllib.load(stream)
with manifest_path.open("rb") as stream:
    manifest = tomllib.load(stream)

if not phase:
    phase = card.get("guard_phase")
    if phase not in {"readiness", "bridge_ready", "observer_i0", "cataloged_i0"}:
        raise SystemExit("active card guard_phase is missing or unknown")

guard_id = "mir-call-d1b-cataloged-affine-loan-lifecycle"
guard_script = "tools/checks/mir_call_d1b_cataloged_affine_loan_lifecycle_guard.sh"
rows = manifest.get("rows")
if not isinstance(rows, list):
    raise SystemExit("guard_rows.toml rows table is missing")
matches = [row for row in rows if row.get("id") == guard_id]
if len(matches) != 1:
    raise SystemExit(f"expected one registry row for {guard_id}, found {len(matches)}")
row = matches[0]
if row.get("profiles") != ["pilot", "quick-static"]:
    raise SystemExit("lifecycle guard profiles drifted")
if row.get("cmd") != ["bash", guard_script]:
    raise SystemExit("lifecycle guard command drifted")
if sum(1 for item in rows if item.get("id") == guard_id) != 1:
    raise SystemExit("lifecycle guard id is duplicated")

registration_key = (
    "observer_guard_registration_row"
    if phase == "observer_i0"
    else "bridge_ready_registration_row"
    if phase == "bridge_ready"
    else "guard_registration_row"
)
registration = card.get(registration_key)
if not isinstance(registration, dict):
    raise SystemExit(f"active card {registration_key} is missing")
if phase == "observer_i0":
    if registration.get("execution_row") != "MIR-CALL-D1B-SELECTED-FUNCTIONCALL-OBSERVATION-COMPLETION-D0":
        raise SystemExit("observer execution row drifted")
    if registration.get("status") not in {"observer_i0_guard_open", "observer_i0_landed"}:
        raise SystemExit("observer status drifted")
elif phase == "bridge_ready":
    if registration.get("execution_row") != "MIR-CALL-D1B-PACKAGE-BRIDGE-READY-R0":
        raise SystemExit("bridge-ready execution row drifted")
    if registration.get("status") not in {"bridge_ready_fast_open", "bridge_ready_landed"}:
        raise SystemExit("bridge-ready status drifted")
else:
    if registration.get("execution_row") != "MIR-CALL-D1B-D0-SIG-CLOSE-E-GUARD-REGISTRATION":
        raise SystemExit("guard-only execution row drifted")
    if registration.get("status") not in {"selected_fast_guard_only", "landed_guard_only"}:
        raise SystemExit("guard-only status drifted")
allowed_files = registration.get("allowed_files")
expected_files = {
    guard_script,
    "tools/checks/guard_rows.toml",
    "docs/development/current/main/investigations/mir-call-d1b-root-lineage-exact-target-loan-d0-2026-08-26.toml",
    "docs/development/current/main/CURRENT_STATE.toml",
}
if phase == "observer_i0":
    expected_files.update({
        "tools/checks/mir_call_d1b_cataloged_affine_loan_lifecycle_guard.sh",
        "src/mir/resolved_semantics/direct_call.rs",
        "src/mir/resolved_semantics/direct_call_inventory_gate.rs",
        "src/mir/resolved_semantics/product.rs",
        "src/mir/resolved_semantics/mod.rs",
        "src/mir/resolved_semantics/resolver.rs",
        "src/mir/resolved_semantics/owner_resolver.rs",
        "src/mir/resolved_semantics/shadow/traversal_profile.rs",
        "src/mir/resolved_semantics/source_site_inventory.rs",
        "src/mir/resolved_semantics/brand_source_relation_tests.rs",
        "src/mir/resolved_semantics/tests.rs",
        "src/mir/resolved_semantics/function_root_tests.rs",
        "src/mir/resolved_semantics/if_region_tests.rs",
        "src/mir/resolved_semantics/loop_region_tests.rs",
        "src/mir/resolved_semantics/block_expr_tests.rs",
        "src/mir/resolved_semantics/README.md",
        "src/mir/builder/normal_callable_semantic_source.rs",
        "src/mir/callable_semantic_batch/issuer.rs",
        "src/mir/normal_callable_semantic_package/resolver_deferred_tests.rs",
    })
elif phase == "bridge_ready":
    expected_files.update({
        "src/mir/builder/normal_callable_package_bridge.rs",
        "src/mir/builder/program_root_lowering.rs",
        "src/mir/builder/normal_default_root_catalog_lifecycle.rs",
        "src/mir/builder/normal_default_root_catalog_post_install.rs",
        "src/mir/builder/README.md",
        "src/mir/normal_callable_semantic_package/install.rs",
        "src/mir/normal_callable_semantic_package/README.md",
    })
if set(allowed_files or []) != expected_files:
    raise SystemExit(f"{registration_key} allowed file boundary drifted")

if phase == "observer_i0":
    observer_status = card.get("status")
    if observer_status not in {"observer_i0_fast_open", "observer_i0_landed"}:
        raise SystemExit("observer card status drifted")
    expected_permission = observer_status == "observer_i0_fast_open"
    if card.get("implementation_permission") is not expected_permission:
        raise SystemExit("observer implementation permission/status drifted")
    if card.get("guard_phase") != "observer_i0":
        raise SystemExit("observer card guard phase drifted")
elif phase == "bridge_ready":
    bridge_status = card.get("status")
    if bridge_status not in {"bridge_ready_fast_open", "bridge_ready_landed"}:
        raise SystemExit("bridge-ready card status drifted")
    expected_permission = bridge_status == "bridge_ready_fast_open"
    if card.get("implementation_permission") is not expected_permission:
        raise SystemExit("bridge-ready implementation permission/status drifted")
    if card.get("guard_phase") != "bridge_ready":
        raise SystemExit("bridge-ready card guard phase drifted")
else:
    if card.get("implementation_permission") is not False:
        raise SystemExit("semantic implementation permission opened during guard-only row")
readiness_audit = card.get("d0_sig_close_f_readiness_audit_2026_08_26")
if not isinstance(readiness_audit, dict):
    raise SystemExit("active card readiness audit is missing")
spec = readiness_audit.get("guard_spec", "")
if guard_script not in spec or "Unknown phase fails closed" not in spec:
    raise SystemExit("active card does not name the fail-closed lifecycle guard")

if phase == "observer_i0":
    source_files = [path for path in mir_root.rglob("*.rs")]
    source_text = "\n".join(path.read_text() for path in source_files)
    required = (
        "ResolvedDirectCallObservationV1",
        "direct_call_observations",
        "ObserveOnly",
        "UnissuedDirectCallObservation",
    )
    for token in required:
        if token not in source_text:
            raise SystemExit(f"observer_i0 is not landed: missing {token}")
    traversal = (mir_root / "resolved_semantics/shadow/traversal_profile.rs").read_text()
    if "SelectedCallableV1 => !matches!(expression, ASTNode::FunctionCall" in traversal:
        raise SystemExit("SelectedCallableV1 still rejects ordinary FunctionCall")
    observer_surface_files = [
        mir_root / "resolved_semantics/direct_call.rs",
        mir_root / "resolved_semantics/product.rs",
        mir_root / "resolved_semantics/mod.rs",
        mir_root / "resolved_semantics/resolver.rs",
        mir_root / "resolved_semantics/owner_resolver.rs",
        mir_root / "resolved_semantics/shadow/traversal_profile.rs",
        mir_root / "resolved_semantics/source_site_inventory.rs",
        mir_root / "builder/normal_callable_semantic_source.rs",
        mir_root / "callable_semantic_batch/issuer.rs",
    ]
    observer_surface_text = "\n".join(path.read_text() for path in observer_surface_files)
    for token in (
        "RawDirectCallDispositionLoanV1",
        "RawDirectCallDispositionPortV1",
        "CalleeResolverBox",
    ):
        if token in observer_surface_text:
            raise SystemExit(f"observer_i0 crossed its target/loan boundary: {token}")
    resolver = (mir_root / "resolved_semantics/resolver.rs").read_text()
    if '"direct calls require a callable index"' not in resolver:
        raise SystemExit("FullFunction unindexed rejection disappeared")
    source = (mir_root / "builder/normal_callable_semantic_source.rs").read_text()
    if "NormalCallableSemanticAdmissionV1::Rejected" not in source:
        raise SystemExit("package-admission observation terminal is missing")
    helper = (mir_root / "resolved_semantics/direct_call_inventory_gate.rs").read_text()
    if ".owners()" not in helper:
        raise SystemExit("forest-wide observation helper is missing owners() coverage")
    if "forest.roots()" in helper:
        raise SystemExit("forest-wide observation helper regressed to roots()")
    for surface in (
        mir_root / "callable_semantic_batch/issuer.rs",
        mir_root / "builder/normal_callable_semantic_source.rs",
    ):
        if "forest_has_unissued_direct_call_observation_v1" not in surface.read_text():
            raise SystemExit(f"forest-wide observation helper is not delegated: {surface}")
    if "direct_call_targets" not in source_text:
        raise SystemExit("direct-call target field census disappeared")

elif phase == "readiness":
    # This phase freezes the pre-implementation boundary.  It must fail if a
    # partial semantic surface appears before the dedicated implementation row.
    forbidden_semantic_symbols = (
        "BuilderInstallConsumerV1",
        "BuilderInstallTokenV1",
        "BuilderPrivateInstalledCallableBundleV1",
        "RawDirectCallDispositionLoanV1",
        "RawDirectCallDispositionPortV1",
        "with_normal_callable_install_once",
    )
    source_files = [path for path in mir_root.rglob("*.rs")]
    source_text = "\n".join(path.read_text() for path in source_files)
    for token in forbidden_semantic_symbols:
        if token in source_text:
            raise SystemExit(f"semantic surface appeared during guard-only row: {token}")

    lifecycle = (mir_root / "builder/normal_default_root_catalog_lifecycle.rs").read_text()
    lowering = (mir_root / "builder/program_root_lowering.rs").read_text()
    post_install = (mir_root / "builder/normal_default_root_catalog_post_install.rs").read_text()
    for token in ("prepare_install", ".commit()", "with_bound_source"):
        if token not in lifecycle:
            raise SystemExit(f"readiness caller census edge disappeared: {token}")
    if "source_ast()" not in lifecycle:
        raise SystemExit("readiness compatibility source edge disappeared")
    for token in ("NormalCallableSemanticPackageMode::Installed", "begin_lowering"):
        if token not in lowering:
            raise SystemExit(f"readiness lowering edge disappeared: {token}")
    if "NormalCallableSemanticPackageMode" not in post_install:
        raise SystemExit("readiness post-install relay disappeared")

    if "RawDirectCallDispositionLoanV1" not in readiness_audit.get("raw_capability_decision", ""):
        raise SystemExit("active card no longer names the raw loan boundary")

elif phase == "cataloged_i0":
    # This phase is intentionally future-facing.  Running it before the
    # Cataloged implementation lands must fail closed rather than silently
    # treating readiness evidence as implementation evidence.
    source_files = [path for path in mir_root.rglob("*.rs")]
    source_text = "\n".join(path.read_text() for path in source_files)
    required = (
        "RawDirectCallDispositionLoanV1",
        "take_once",
        "residual",
    )
    for token in required:
        if token not in source_text:
            raise SystemExit(f"cataloged_i0 phase is not landed: missing {token}")
    if "PreparedRawOrdinaryFunctionCompletionV1::Targeted" in source_text:
        raise SystemExit("generic Targeted payload remains")
    if "PreparedRawOrdinaryFunctionCompletionV1::CatalogedTargeted" in source_text:
        raise SystemExit("Cataloged direct payload remains")
    if source_text.count("MirInstruction::call(") < 1:
        raise SystemExit("canonical Call issuer is missing")
    for token in ("resolve_call_target", "try_unique_static_method_recovery", "make_name_const_result"):
        if token in source_text:
            raise SystemExit(f"late recovery/name-Const edge remains: {token}")

elif phase == "bridge_ready":
    # This phase is opened only after the package-only bridge lands.  It must
    # prove that the package lifecycle is closed without smuggling in a
    # not-yet-issued direct-call loan.  The loan and Cataloged payload remain
    # explicitly pending until cataloged_i0.
    package_only_bundle = "BuilderPrivateInstalledCallablePackageBundleV1"
    source_files = [path for path in mir_root.rglob("*.rs")]
    source_text = "\n".join(path.read_text() for path in source_files)
    if package_only_bundle not in source_text:
        raise SystemExit("bridge_ready phase is not landed: package-only bundle is missing")
    if "with_normal_callable_install_once" not in source_text:
        raise SystemExit("bridge_ready phase is not landed: one-shot bridge is missing")
    for token in (
        "RawDirectCallDispositionLoanV1",
        "RawDirectCallDispositionPortV1",
        "WithDirectCalls",
    ):
        if token in source_text:
            raise SystemExit(f"bridge_ready must not provision a direct-call loan: {token}")

    lifecycle_path = mir_root / "builder/normal_default_root_catalog_lifecycle.rs"
    lowering_path = mir_root / "builder/program_root_lowering.rs"
    post_install_path = mir_root / "builder/normal_default_root_catalog_post_install.rs"
    install_path = mir_root / "normal_callable_semantic_package/install.rs"
    constructor_path = mir_root / "normal_callable_semantic_package/instance_constructor_loan.rs"
    loan_port_path = mir_root / "builder/normal_callable_semantic_loan_port.rs"
    lifecycle = lifecycle_path.read_text()
    lowering = lowering_path.read_text()
    post_install = post_install_path.read_text()
    install = install_path.read_text()
    constructor = constructor_path.read_text()
    loan_port = loan_port_path.read_text()

    for token in ("prepare_install", "with_bound_source"):
        if token in lifecycle:
            raise SystemExit(f"bridge_ready package-only lifecycle escape remains: {token}")
    # Compatibility roots are the one explicit AST-retaining branch.  The
    # forbidden package escape is specifically a source getter on the
    # installed package, not the compatibility owner’s source view.
    for token in ("package.source_ast()", "installed_package.source_ast()"):
        if token in lifecycle:
            raise SystemExit(f"bridge_ready installed package source getter remains: {token}")
    for token in ("NormalCallableSemanticPackageMode::Installed(&",):
        if token in lowering or token in install:
            raise SystemExit(f"bridge_ready shared-reference lifecycle remains: {token}")
    if "NormalCallableSemanticPackageMode::Installed(&" in post_install:
        raise SystemExit("bridge_ready post-install still relays a bare Installed package")
    # The low-level installed-port fixture may keep a cfg(test)-only
    # compatibility helper while production uses the bridge's scoped method.
    if "begin_lowering(&self," in install:
        marker = install.find("begin_lowering(&self,")
        prefix = install[max(0, marker - 120):marker]
        if "#[cfg(test)]" not in prefix:
            raise SystemExit("bridge_ready production begin_lowering escape remains")
    if "self.installed.source_ast()" in constructor or "self.installed.source_ast()" in loan_port:
        raise SystemExit("bridge_ready hidden installed source_ast handoff remains")
    if "source_ast" in install and "fn source_ast" in install:
        raise SystemExit("bridge_ready Installed::source_ast getter remains")
    if "with_normal_program_source_loan" not in source_text:
        raise SystemExit("bridge_ready source HRTB loan is missing")
    transition = card.get("d0_sig_close_f_transition_amendment_2026_08_26")
    if not isinstance(transition, dict) or "cataloged_provision_shape" not in transition:
        raise SystemExit("active card does not keep CatalogedI0 provisioning separate")

print(f"[{guard_id}] phase={phase} ok")
PY
