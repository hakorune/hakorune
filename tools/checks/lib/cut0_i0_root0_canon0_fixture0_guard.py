#!/usr/bin/env python3
"""Evidence guard for the disconnected CANON-FIXTURE0 aggregate proof.

This guard deliberately treats the aggregate fixture as a separate proof
product.  The older SOURCE-BIND0, COLLECT0, and completion tests are useful
focused evidence, but they are not a substitute for one compiler-owned
source-to-completion chain.
"""

from __future__ import annotations

import pathlib
import re


ROOT = pathlib.Path(__file__).resolve().parents[3]
FIXTURE = ROOT / "src/mir/compiler/canonical_bridge_fixture0_p0.rs"
COMPILER_MOD = ROOT / "src/mir/compiler/mod.rs"
SOURCE = ROOT / "src/mir/compiler/source_bound_package.rs"
COMPLETION = ROOT / "src/mir/compiler/canonical_physical_completion.rs"
BRAND = ROOT / "src/mir/builder/module_invocation_brand0.rs"
PHYSICAL = ROOT / "src/mir/builder/module_invocation_owner_chain.rs"
SESSION = ROOT / "src/mir/builder/module_invocation_session.rs"
CALLABLE_TX = ROOT / "src/mir/builder/resolved_lowering/callable_module_transaction.rs"
COLLISION_FIXTURE = (
    ROOT / "src/mir/builder/resolved_lowering/callable_batch_collection_p0.rs"
)
COLLISION_OWNER = (
    ROOT / "src/mir/builder/resolved_lowering/callable_module_transaction.rs"
)
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-root0-canon0-bridge-execution-task-2026-07-23.md"
)
FIXTURE_TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-root0-canon0-fixture0-execution-task-2026-07-23.md"
)
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"

# Keep this manifest explicit.  A future fixture or guard must be added here,
# otherwise the line-count and source-census proof is intentionally incomplete.
MANIFEST = (
    FIXTURE,
    COMPILER_MOD,
    SOURCE,
    COMPLETION,
    BRAND,
    PHYSICAL,
    SESSION,
    CALLABLE_TX,
    COLLISION_FIXTURE,
    COLLISION_OWNER,
    TASK,
    FIXTURE_TASK,
    STATE,
    pathlib.Path(__file__),
)


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def require_any(text: str, fragments: tuple[str, ...], label: str) -> None:
    if not any(fragment in text for fragment in fragments):
        raise AssertionError(f"missing {label}: {fragments!r}")


def production_rust_files() -> list[pathlib.Path]:
    """Return implementation files, excluding focused test modules."""

    return [
        path
        for path in ROOT.glob("src/**/*.rs")
        if not path.name.endswith("_p0.rs")
        and not path.name.endswith("_tests.rs")
        and "tests" not in path.parts
    ]


def test_registration(text: str, module: str) -> bool:
    return bool(
        re.search(rf"(?m)^\s*mod\s+{re.escape(module)}\s*;", text)
    )


def main() -> int:
    # Read all manifest members up front so a missing fixture is a hard failure,
    # rather than a silently skipped census row.
    missing = [path for path in MANIFEST if not path.is_file()]
    if missing:
        raise AssertionError(
            "CANON-FIXTURE0 manifest member missing: "
            + ", ".join(str(path.relative_to(ROOT)) for path in missing)
        )

    fixture = FIXTURE.read_text()
    compiler_mod = COMPILER_MOD.read_text()
    source = SOURCE.read_text()
    completion = COMPLETION.read_text()
    brand = BRAND.read_text()
    physical = PHYSICAL.read_text()
    session = SESSION.read_text()
    callable_tx = CALLABLE_TX.read_text()
    collision = COLLISION_FIXTURE.read_text()
    collision_owner = COLLISION_OWNER.read_text()
    task = TASK.read_text()
    fixture_task = FIXTURE_TASK.read_text()
    state = STATE.read_text()

    for path in MANIFEST:
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(
                f"CANON-FIXTURE0 file must remain below 800 lines: "
                f"{path.relative_to(ROOT)}"
            )

    require_any(
        state,
        (
            "CANON-FIXTURE0-S0/P0/C0/G0 is taskized and active",
            "CANON-FIXTURE0-S0/P0/C0/G0 is closed",
        ),
        "aggregate fixture row active or closed",
    )
    require(task, "CANON-FIXTURE0", "bridge lane fixture row")
    require(task, "DRAIN0", "fixture stop line")
    require_any(
        fixture_task,
        (
            "Status: **Active — CANON-FIXTURE0-S0",
            "Status: **Closed — CANON-FIXTURE0",
        ),
        "fixture task status",
    )
    require(fixture_task, "CANON-FIXTURE0-G0", "fixture guard row")
    require(
        compiler_mod,
        "mod canonical_bridge_fixture0_p0;",
        "aggregate fixture registration",
    )

    # The aggregate must use the compiler-owned bridge and must exercise every
    # exact plan variant.  These checks intentionally live in one fixture file
    # so four independent focused tests cannot masquerade as one chain.
    for fragment, label in (
        ("canonical_bridge_fixture0_four_route_aggregate", "aggregate test"),
        ("ExactCanonicalPreflightPlanV1::APlus", "A+ route assertion"),
        (
            "ExactCanonicalPreflightPlanV1::BindingSsaTrivial",
            "trivial route assertion",
        ),
        (
            "ExactCanonicalPreflightPlanV1::BindingSsaAcyclic",
            "acyclic route assertion",
        ),
        (
            "ExactCanonicalPreflightPlanV1::BindingSsaRecursive",
            "recursive route assertion",
        ),
        ("bind_canonical_source", "compiler source binding"),
        ("begin_canonical_invocation", "physical owner bridge"),
        (".lower()", "same-session lowering"),
        (".collect()", "same-collector admission"),
        (".complete()", "route completion"),
    ):
        require(fixture, fragment, label)

    # These are semantic positive proofs required by the row.  Invalid
    # synthetic identities are intentionally absent from the fixture: the
    # typed facade and the static census below prove that they are
    # unrepresentable instead of manufacturing a second fault authority.
    for fragment, label in (
        (
            "canonical_bridge_fixture0_foreign_pairing_is_rejected",
            "foreign pairing rejection",
        ),
        (
            "canonical_bridge_fixture0_condition_fn_spelling_is_canonical",
            "canonical condition_fn spelling",
        ),
        (
            "canonical_bridge_fixture0_recursive_acyclic_witness_parity",
            "recursive/acyclic witness parity",
        ),
        ("condition_fn", "condition_fn physical spelling evidence"),
    ):
        require(fixture, fragment, label)

    # The aggregate must not smuggle in the disconnected identity factory or
    # the old Builder-only completion owner.  Test-only factory usage remains
    # valid in their own focused legacy fixtures, but not in this proof.
    for forbidden, label in (
        ("TestInvocationPreflightFactoryV1", "test identity factory"),
        ("ModuleInvocationTokenV1::from_test", "test token mint"),
        ("ModuleInvocationBrandV1::legacy_test", "legacy test brand"),
        ("InvocationBranded::from_test", "post-hoc test branding"),
        ("canonical_root_completion", "legacy completion scaffold"),
    ):
        if forbidden in fixture:
            raise AssertionError(f"aggregate fixture uses forbidden {label}: {forbidden}")

    # Preserve the existing atomic late-collision proof and make its module
    # registration part of the real census rather than a prose assertion.
    require(
        collision_owner,
        "mod callable_batch_collection_p0;",
        "late-collision fixture registration",
    )
    require(
        collision,
        "fn late_collector_collision_rejects_without_delta",
        "late callable collision fixture",
    )
    require(collision, "symbol_count(), 1", "collector delta remains zero")
    require(collision, "CallableCollectorBatchPrepareErrorV1::Admission", "typed collision error")

    # Prove the intended bridge surface exists, but compute production caller
    # counts from the tree.  A static "production_consumers=0" string is not
    # evidence.  The active fixture is test-only; production CUT0 activation
    # remains forbidden at this row.
    for fragment, label in (
        ("pub(in crate::mir) fn begin_canonical_invocation", "bridge terminal"),
        ("pub(super) fn open_physical", "physical open terminal"),
        ("pub(super) fn lower", "lower terminal"),
        ("pub(in crate::mir) fn collect", "collect terminal"),
        ("pub(in crate::mir) fn complete", "completion terminal"),
        ("CollectedCanonicalPhysicalInvocationV1", "collected owner"),
    ):
        require(source + completion + compiler_mod, fragment, label)

    production = production_rust_files()
    bridge_calls: dict[str, list[pathlib.Path]] = {
        "begin": [],
        "bind": [],
        "collect": [],
        "complete": [],
    }
    for path in production:
        text = path.read_text()
        if "begin_canonical_invocation(" in text:
            bridge_calls["begin"].append(path.relative_to(ROOT))
        if "bind_canonical_source(" in text:
            bridge_calls["bind"].append(path.relative_to(ROOT))
        if ".collect()" in text and "canonical" in path.name:
            bridge_calls["collect"].append(path.relative_to(ROOT))
        if ".complete()" in text and "canonical" in path.name:
            bridge_calls["complete"].append(path.relative_to(ROOT))

    for key, paths in bridge_calls.items():
        if paths:
            raise AssertionError(
                f"CANON-FIXTURE0 requires production {key} callers = 0: {paths}"
            )

    # The source-driven collection terminal must remain the only place where
    # the generic legacy admission API is invoked for this new path.  Also make
    # sure the completion product still retains the exact receipt by value.
    require(
        brand,
        "pub(in crate::mir) fn collect_single(",
        "source-driven single admission",
    )
    require(
        brand,
        "FunctionDraftKeyV1::CanonicalResolvedOwner(header.owner())",
        "header-derived canonical single key",
    )
    require(
        brand,
        "pub(in crate::mir) fn collect_callable_batch(",
        "source-driven batch admission",
    )
    require(brand, "drafts.into_canonical_entries()", "catalog-derived batch entries")
    require(completion, "physical.receipt_brand()", "receipt retained through completion")
    require(session, "open_for_token", "shared session owner")
    require(brand, "InvocationBranded::from_source", "branded physical state")
    require(callable_tx, "into_canonical_entries", "catalog-driven batch projection")

    # Static P0 census: canonical callers cannot supply a key, policy, symbol,
    # or arity.  The physical implementation may call the legacy collector
    # internally, but only after deriving those values from a sealed header or
    # verified catalog.
    single_signature = re.search(
        r"pub\(in crate::mir\) fn collect_single\((.*?)\) ->",
        brand,
        re.DOTALL,
    )
    batch_signature = re.search(
        r"pub\(in crate::mir\) fn collect_callable_batch\((.*?)\) ->",
        brand,
        re.DOTALL,
    )
    if single_signature is None or batch_signature is None:
        raise AssertionError("typed canonical collector signatures are missing")
    for signature, label in (
        (single_signature.group(1), "single canonical collector"),
        (batch_signature.group(1), "batch canonical collector"),
    ):
        if re.search(
            r"FunctionDraftKeyV1|DraftPublicationPolicyV1|\bsymbol\b|\barity\b",
            signature,
        ):
            raise AssertionError(f"{label} exposes loose identity inputs: {signature!r}")
    require(
        brand,
        "FunctionDraftKeyV1::CanonicalResolvedOwner(header.owner())",
        "header-derived single identity",
    )
    require(brand, "drafts.into_canonical_entries()", "catalog-derived batch identity")

    forbidden_conversion = (
        "CanonicalInvocationTokenV1",
        "CanonicalInvocationBrandV1",
        "InvocationBranded::from_source(brand, receipt",
        "ModuleInvocationTokenV1::from_test",
        "TestInvocationPreflightFactoryV1",
    )
    for fragment in forbidden_conversion:
        if fragment in fixture or fragment in source:
            raise AssertionError(f"canonical fixture/source contains forbidden conversion: {fragment}")

    # Count call expressions, not definitions.  Definitions use a generic
    # lifetime (`name<'a>(...)`) and are intentionally not included by these
    # callsite patterns.  Focused `_p0` modules are excluded from the
    # production census by production_rust_files().
    call_patterns = {
        "bind": re.compile(r"\bbind_canonical_source\s*\("),
        "begin": re.compile(r"\bbegin_canonical_invocation\s*\("),
    }
    caller_paths: dict[str, list[pathlib.Path]] = {key: [] for key in call_patterns}
    physical_transition_paths: list[pathlib.Path] = []
    for path in production:
        text = path.read_text()
        for key, pattern in call_patterns.items():
            if pattern.search(text):
                caller_paths[key].append(path.relative_to(ROOT))
        if "CanonicalPhysicalInvocationV1" in text and re.search(
            r"\.lower\(\)|\.collect\(\)|\.complete\(\)", text
        ):
            physical_transition_paths.append(path.relative_to(ROOT))
    for key, paths in caller_paths.items():
        if paths:
            raise AssertionError(f"CANON-FIXTURE0 requires production {key} callers = 0: {paths}")
    if physical_transition_paths:
        raise AssertionError(
            "CANON-FIXTURE0 requires production lower/collect/complete callers = 0: "
            f"{physical_transition_paths}"
        )

    # A few legacy implementation files contain cfg(test) modules inline.
    # Restrict the factory census to canonical production surfaces so those
    # unrelated raw fixtures do not become false production callers.
    canonical_factory_callers = []
    for path in production:
        relative = path.relative_to(ROOT).as_posix()
        if not (
            relative.startswith("src/mir/compiler/")
            or relative.startswith("src/mir/builder/canonical")
        ):
            continue
        if "TestInvocationPreflightFactoryV1::new(" in path.read_text():
            canonical_factory_callers.append(path.relative_to(ROOT))
    if canonical_factory_callers:
        raise AssertionError(
            "CANON-FIXTURE0 aggregate canonical factory callers must be zero: "
            f"{canonical_factory_callers}"
        )

    if not test_registration(compiler_mod, "canonical_bridge_fixture0_p0"):
        raise AssertionError("canonical aggregate fixture is not registered as a test module")

    print(
        "[cut0-i0-root0-canon0-fixture0-guard] ok "
        "aggregate=1 routes=4 condition_fn=1 synthetic_guard=1 "
        "witness_parity=1 late_collision=1 production_callers=0 files_under_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
