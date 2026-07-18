#!/usr/bin/env python3
"""Structural checks for the disconnected LOOP0-P0a expression port."""

from __future__ import annotations

import re
import sys
from pathlib import Path

from callable_result_i0_site0_r0_expr0_spine0_loop0 import (
    _matching_rust_brace,
    _production,
    _read,
)


PORT_PATH = "src/mir/builder/control_flow/plan/expression_port.rs"
HELPERS_VALUE_PATH = (
    "src/mir/builder/control_flow/plan/normalizer/helpers_value.rs"
)
P0A_TOUCHED_PATHS = (
    PORT_PATH,
    HELPERS_VALUE_PATH,
    "src/mir/builder/control_flow/plan/normalizer/helpers.rs",
    "src/mir/builder/control_flow/plan/normalizer/helpers_value_state.rs",
    "src/mir/builder/control_flow/plan/normalizer/common.rs",
    "src/mir/builder/control_flow/plan/normalizer/loop_body_lowering.rs",
    "src/mir/builder/control_flow/plan/normalizer/mod.rs",
    "src/mir/builder/control_flow/plan/normalizer/README.md",
    "src/mir/builder/control_flow/plan/expression_port_tests.rs",
    "src/mir/builder/control_flow/plan/mod.rs",
    "src/mir/callable_result_representation/located_legacy.rs",
)


def _function_body(text: str, name: str) -> str:
    matches = list(re.finditer(rf"\bfn\s+{re.escape(name)}(?:\s*<[^{{;]*>)?\s*\(", text))
    if len(matches) != 1:
        raise RuntimeError(
            f"LOOP0-P0a function owner drift: name={name} count={len(matches)}"
        )
    opening = text.find("{", matches[0].end())
    if opening < 0:
        raise RuntimeError(f"LOOP0-P0a function body missing: {name}")
    return text[opening:_matching_rust_brace(text, opening)]


def _production_plan_sources(root: Path) -> dict[str, str]:
    result = {}
    plan_root = root / "src/mir/builder/control_flow/plan"
    for path in sorted(plan_root.rglob("*.rs")):
        if path.name == "tests.rs" or path.name.endswith("_tests.rs") or "tests" in path.parts:
            continue
        relative = path.relative_to(root).as_posix()
        result[relative] = _production(path.read_text(encoding="utf-8"))
    return result


def check_loop0_p0a(root: Path) -> str:
    port = _read(root, PORT_PATH)
    port_production = _production(port)
    helpers_value = _production(_read(root, HELPERS_VALUE_PATH))

    # One closed port vocabulary. The located implementation must disappear
    # completely when cfg(test) items are removed.
    owners = (
        ("trait LoopPlanExpressionPortV1", 1),
        ("struct RawLoopPlanExpressionPortV1", 1),
        ("struct LocatedLoopPlanExpressionPortV1", 1),
    )
    for owner, expected in owners:
        actual = port.count(owner)
        if actual != expected:
            raise RuntimeError(
                f"LOOP0-P0a port owner drift: owner={owner!r} expected={expected} actual={actual}"
            )
    if "LocatedLoopPlanExpressionPortV1" in port_production:
        raise RuntimeError("LOOP0-P0a located port escaped cfg(test)")
    if port.count("#[cfg(test)]\nmod located") != 1:
        raise RuntimeError("LOOP0-P0a located port must have one cfg(test) module")

    for carrier in (
        "VerifiedCallableResultLegacySourceViewV1",
        "LegacyStmtInputV1",
        "LegacyExprInputV1",
        "LegacyBodyInputV1",
    ):
        if carrier not in port:
            raise RuntimeError(f"LOOP0-P0a missing existing located carrier: {carrier}")
        if carrier in port_production:
            raise RuntimeError(
                f"LOOP0-P0a located carrier became a production dependency: {carrier}"
            )

    located_input_owners = re.findall(
        r"\benum\s+(LocatedLoopPlan(?:Expr|Stmt|Body)InputV1)\b", port
    )
    if sorted(located_input_owners) != sorted(
        (
            "LocatedLoopPlanExprInputV1",
            "LocatedLoopPlanStmtInputV1",
            "LocatedLoopPlanBodyInputV1",
        )
    ):
        raise RuntimeError(
            "LOOP0-P0a located input facade drift: "
            f"owners={located_input_owners}"
        )

    # PATH0 remains the sole path/role authority. Resolving an existing role
    # is permitted; constructing or comparing paths locally is not.
    for required in ("ExprChildRoleV1", "BodyChildRoleV1"):
        if required not in port:
            raise RuntimeError(f"LOOP0-P0a missing PATH0 role delegation: {required}")
    for forbidden in (
        "SourcePathV1",
        "SourcePathSegmentV1",
        "SourceExprSiteV1::",
        "SourceStmtSiteV1::",
        "std::ptr::eq",
        ".span()",
        "ValueId",
        "effect_ordinal",
        "target_spelling",
    ):
        if forbidden in port:
            raise RuntimeError(
                f"LOOP0-P0a second source-identity authority detected: {forbidden}"
            )
    if "==" in port:
        raise RuntimeError("LOOP0-P0a AST/path equality reconstruction detected")

    # The raw decision is one production owner. The exact located MethodCall
    # source is constructed once, and only inside the stripped test module.
    if port_production.count("CoreCallSourceV1::Unlocated") != 1:
        raise RuntimeError("LOOP0-P0a raw Unlocated decision owner drift")
    if "CoreCallSourceV1::LocatedMethodCall(" in port_production:
        raise RuntimeError("LOOP0-P0a production located source producer detected")
    if port.count("CoreCallSourceV1::LocatedMethodCall(") != 1:
        raise RuntimeError("LOOP0-P0a test-located source producer drift")
    if port.count(".activation_site()") != 1:
        raise RuntimeError("LOOP0-P0a located source must reuse one activation_site")
    if port.count(".require_expr_carrier(input)") != 1:
        raise RuntimeError("LOOP0-P0a located source must co-validate its view carrier")
    if port.index(".require_expr_carrier(input)") > port.index(".activation_site()"):
        raise RuntimeError("LOOP0-P0a located source read site before carrier validation")

    readme = _read(root, "src/mir/builder/control_flow/plan/normalizer/README.md")
    for phrase in (
        "one sealed, stack-scoped child-demand port",
        "source-view brand before exposing",
        "GenericLoop composer threading and boolean/short-circuit condition descent",
        "must not silently enter those",
    ):
        if phrase not in readme:
            raise RuntimeError(f"LOOP0-P0a README boundary drift: {phrase}")

    # The existing raw entry remains a facade over one recursive, port-driven
    # normalizer. Recursion must never jump back to the raw facade.
    if len(re.findall(r"\bfn\s+lower_value_ast\s*\(", helpers_value)) != 1:
        raise RuntimeError("LOOP0-P0a raw lower_value_ast facade drift")
    if len(re.findall(r"\bfn\s+lower_value_input\s*<", helpers_value)) != 1:
        raise RuntimeError("LOOP0-P0a port-driven value-lowering owner drift")
    facade = _function_body(helpers_value, "lower_value_ast")
    if facade.count("RawLoopPlanExpressionPortV1::new()") != 1:
        raise RuntimeError("LOOP0-P0a raw facade must construct one raw port")
    if facade.count("Self::lower_value_input(") != 1:
        raise RuntimeError("LOOP0-P0a raw facade must delegate exactly once")
    recursive = _function_body(helpers_value, "lower_value_input")
    if "Self::lower_value_ast(" in recursive:
        raise RuntimeError("LOOP0-P0a port-driven recursion escaped to raw facade")
    if recursive.count("Self::lower_value_input(") < 1:
        raise RuntimeError("LOOP0-P0a port-driven normalizer has no recursive demand")

    # P0a is disconnected from GenericLoop composition and from every claim
    # authority. P0b owns composer/pipeline signature threading.
    generic_loop_roots = (
        root / "src/mir/builder/control_flow/plan/generic_loop",
        root / "src/mir/builder/control_flow/plan/features",
        root / "src/mir/builder/control_flow/plan/recipe_tree",
    )
    for directory in generic_loop_roots:
        for path in directory.rglob("*.rs"):
            if "LoopPlanExpressionPortV1" in _production(path.read_text(encoding="utf-8")):
                raise RuntimeError(
                    "LOOP0-P0a GenericLoop port threading landed before P0b: "
                    f"{path.relative_to(root)}"
                )
    for forbidden in (
        "VerifiedCallableResultCallerLedgerV1",
        "ClaimedCallableResultActivationSiteV1",
        "ClaimedCallableResultLoopBatchV1",
        "claim_loop_batch",
        ".claim(",
        ".finish(",
    ):
        if forbidden in port + helpers_value:
            raise RuntimeError(f"LOOP0-P0a claim authority leak: {forbidden}")

    plan_sources = _production_plan_sources(root)
    located_read_allowlist = {
        "src/mir/builder/control_flow/plan/located_loop.rs": 1,
    }
    for path, text in plan_sources.items():
        expected = located_read_allowlist.get(path, 0)
        actual = text.count("CoreCallSourceV1::LocatedMethodCall(")
        if actual != expected:
            raise RuntimeError(
                "LOOP0-P0a production located occurrence drift: "
                f"path={path} expected={expected} actual={actual}"
            )
        if ".activation_site()" in text:
            raise RuntimeError(
                f"LOOP0-P0a production located carrier consumer detected: {path}"
            )

    builder_root = _production(_read(root, "src/mir/builder.rs"))
    for forbidden_field in (
        "LoopPlanExpressionPortV1",
        "RawLoopPlanExpressionPortV1",
        "LocatedLoopPlanExpressionPortV1",
        "LegacyStmtInputV1",
        "LegacyExprInputV1",
        "LegacyBodyInputV1",
    ):
        if forbidden_field in builder_root:
            raise RuntimeError(f"LOOP0-P0a MirBuilder port/carrier storage leak: {forbidden_field}")

    touched = (*P0A_TOUCHED_PATHS, __file__)
    oversized = []
    for path in touched:
        relative = str(path) if isinstance(path, str) else str(Path(path).relative_to(root))
        if len(_read(root, relative).splitlines()) >= 800:
            oversized.append(relative)
    if oversized:
        raise RuntimeError(f"LOOP0-P0a source/check files reached 800 lines: {oversized}")

    return (
        "p0a_port_owners=1 raw_ports=1 test_located_ports=1 "
        "production_located_ports=0 path_policy_owners=0 "
        "production_located_producers=0 generic_loop_consumers=0"
    )


def main() -> int:
    root = Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
    try:
        result = check_loop0_p0a(root)
    except RuntimeError as error:
        print(f"[callable-result-loop0-p0a-guard] FAIL: {error}", file=sys.stderr)
        return 1
    print(f"[callable-result-loop0-p0a-guard] ok: {result}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
