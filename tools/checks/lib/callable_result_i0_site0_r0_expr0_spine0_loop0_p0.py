#!/usr/bin/env python3
"""Structural checks for LOOP0-P0a and the disconnected P0b-F0 support."""

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
P0B_F0_FIXTURE_PATH = (
    "src/mir/callable_result_representation/tests/actual_parser_add_fixture.rs"
)
P0B_F0_ENV_PATH = (
    "src/mir/builder/control_flow/plan/generic_loop/facts/extract/test_support.rs"
)
P0B_F0_TOUCHED_PATHS = (
    P0B_F0_FIXTURE_PATH,
    P0B_F0_ENV_PATH,
    "src/mir/builder/control_flow/plan/generic_loop/facts/extract/mod.rs",
    "src/mir/callable_result_representation/mod.rs",
    "src/mir/callable_result_representation/tests/mod.rs",
    "src/mir/callable_result_representation/tests/activation.rs",
    "src/mir/callable_result_representation/tests/loop_claim_batch.rs",
    "src/mir/builder/control_flow/edgecfg/api/verify.rs",
    "src/mir/builder/control_flow/facts/loop_cond_return_in_body/tests.rs",
    "src/mir/builder/control_flow/plan/facts/loop_tests_parts/multi_candidate.rs",
    "src/mir/builder/control_flow/plan/loop_cond/true_break_continue.rs",
    "src/mir/builder/control_flow/plan/normalizer/value_join_demo_if2.rs",
    "src/mir/builder/control_flow/plan/parts/wiring_tests.rs",
    "src/tests/mir_direct_route_decode_escapes.rs",
    "src/tests/mir_joinir_if_select_parts/helpers.rs",
    "src/tests/mir_joinir_stage1_using_resolver_min.rs",
    "src/tests/mir_loopform_complex.rs",
    "src/tests/mir_move_contract.rs",
    "src/tests/mir_stage1_staticcompiler_receiver.rs",
)
P0B_F0_MODE_KEYS = (
    "NYASH_JOINIR_DEV",
    "HAKO_JOINIR_PLANNER_REQUIRED",
    "HAKO_JOINIR_STRICT",
    "NYASH_JOINIR_STRICT",
    "HAKO_JOINIR_DEBUG",
    "NYASH_JOINIR_DEBUG",
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

    # One closed port vocabulary. O0-R0 promotes the located implementation to
    # a passive production schema while keeping execution callers at zero.
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
    if port_production.count("struct LocatedLoopPlanExpressionPortV1") != 1:
        raise RuntimeError("LOOP0-O0-R0 passive located port schema drift")
    if "#[cfg(test)]\nmod located" in port:
        raise RuntimeError("LOOP0-O0-R0 located port remained test-only")

    for carrier in (
        "VerifiedCallableResultLegacySourceViewV1",
        "LegacyStmtInputV1",
        "LegacyExprInputV1",
        "LegacyBodyInputV1",
    ):
        if carrier not in port:
            raise RuntimeError(f"LOOP0-P0a missing existing located carrier: {carrier}")
        if carrier not in port_production:
            raise RuntimeError(
                f"LOOP0-O0-R0 passive located carrier missing from production schema: {carrier}"
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

    # The raw decision and exact located MethodCall source each remain one
    # passive production owner. No located execution root is connected here.
    if port_production.count("CoreCallSourceV1::Unlocated") != 3:
        raise RuntimeError("LOOP0-O0-R0 raw/located Unlocated decision drift")
    if port_production.count("CoreCallSourceV1::LocatedMethodCall(") != 1:
        raise RuntimeError("LOOP0-O0-R0 passive located source producer drift")
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

    # O0-R0 permits one analysis-only GenericLoop representation consumer and
    # still forbids composition, lowering, and every claim authority.
    generic_loop_roots = (
        root / "src/mir/builder/control_flow/plan/generic_loop",
        root / "src/mir/builder/control_flow/plan/features",
        root / "src/mir/builder/control_flow/plan/recipe_tree",
    )
    generic_loop_port_allowlist = {
        "src/mir/builder/control_flow/plan/generic_loop/located_representation/mod.rs": 2,
        "src/mir/builder/control_flow/plan/generic_loop/located_representation/direct_preflight.rs": 3,
        "src/mir/builder/control_flow/plan/generic_loop/located_representation/lowering_view.rs": 9,
        "src/mir/builder/control_flow/plan/generic_loop/located_representation/recipe_seal.rs": 4,
        "src/mir/builder/control_flow/plan/features/generic_loop_located_composer.rs": 3,
    }
    observed_generic_loop_ports = {}
    for directory in generic_loop_roots:
        for path in directory.rglob("*.rs"):
            if path.name == "tests.rs" or path.name.endswith("_tests.rs") or "tests" in path.parts:
                continue
            relative = path.relative_to(root).as_posix()
            count = _production(path.read_text(encoding="utf-8")).count(
                "LocatedLoopPlanExpressionPortV1"
            )
            if count:
                observed_generic_loop_ports[relative] = count
    if observed_generic_loop_ports != generic_loop_port_allowlist:
        raise RuntimeError(
            "LOOP0-O0-R0 passive GenericLoop located consumer drift: "
            f"{observed_generic_loop_ports}"
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
        PORT_PATH: 1,
        "src/mir/builder/control_flow/plan/lowerer/emission_port.rs": 1,
    }
    for path, text in plan_sources.items():
        expected = located_read_allowlist.get(path, 0)
        actual = text.count("CoreCallSourceV1::LocatedMethodCall(")
        if actual != expected:
            raise RuntimeError(
                "LOOP0-P0a production located occurrence drift: "
                f"path={path} expected={expected} actual={actual}"
            )
        if ".activation_site()" in text and path != PORT_PATH:
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

    # P0b-F0 owns one actual ParserBox extraction/activation fixture and one
    # complete process-scoped mode lock. Later P0b rows must borrow both.
    fixture = _read(root, P0B_F0_FIXTURE_PATH)
    actual_method_owners = []
    mir_root = root / "src/mir"
    for path in mir_root.rglob("*.rs"):
        count = path.read_text(encoding="utf-8").count(
            '.find("\\n  static_const_parse_add(text, pos) {")'
        )
        actual_method_owners.extend([path.relative_to(root).as_posix()] * count)
    if actual_method_owners != [P0B_F0_FIXTURE_PATH]:
        raise RuntimeError(
            "LOOP0-P0b-F0 actual ParserBox extraction owner drift: "
            f"owners={actual_method_owners}"
        )
    for owner in (
        "source",
        "selected_static_sites",
        "plan",
        "caller",
    ):
        definitions = re.findall(rf"\bpub\(crate\)\s+fn\s+{owner}\s*\(", fixture)
        if len(definitions) != 1:
            raise RuntimeError(
                f"LOOP0-P0b-F0 shared fixture owner drift: {owner}"
            )
    sites = _function_body(fixture, "selected_static_sites")
    site_segments = re.findall(
        r"SourcePathSegmentV1::(?:Body\(\d+\)|LoopBody\(\d+\)|Value)",
        sites,
    )
    expected_segments = [
        "SourcePathSegmentV1::Body(3)",
        "SourcePathSegmentV1::Value",
        "SourcePathSegmentV1::Body(4)",
        "SourcePathSegmentV1::LoopBody(5)",
        "SourcePathSegmentV1::Value",
    ]
    if sites.count("site(vec![") != 2 or site_segments != expected_segments:
        raise RuntimeError(
            "LOOP0-P0b-F0 exact selected-site drift: "
            f"segments={site_segments}"
        )
    callable_mod = _read(root, "src/mir/callable_result_representation/mod.rs")
    tests_mod = _read(root, "src/mir/callable_result_representation/tests/mod.rs")
    if tests_mod.count("pub(crate) mod actual_parser_add_fixture;") != 1:
        raise RuntimeError("LOOP0-P0b-F0 fixture module registration drift")
    if callable_mod.count("pub(crate) use tests::actual_parser_add_fixture;") != 1:
        raise RuntimeError("LOOP0-P0b-F0 fixture re-export drift")
    if "actual_parser_add_fixture" in _production(callable_mod):
        raise RuntimeError("LOOP0-P0b-F0 fixture escaped cfg(test)")
    live_test_text = "\n".join(
        path.read_text(encoding="utf-8")
        for path in (root / "src/mir/callable_result_representation/tests").rglob("*.rs")
    )
    if re.search(r"\bfn\s+actual_plan\s*\(", live_test_text):
        raise RuntimeError("LOOP0-P0b-F0 legacy local actual_plan owner remains")

    env_support = _read(root, P0B_F0_ENV_PATH)
    if "static ENV_LOCK" in env_support or "Mutex<" in env_support:
        raise RuntimeError("LOOP0-P0b-F0 added a second environment lock")
    for key in P0B_F0_MODE_KEYS:
        if env_support.count(f'"{key}"') != 1:
            raise RuntimeError(f"LOOP0-P0b-F0 mode-key owner drift: {key}")
    if len(re.findall(r"\bfn\s+with_default_and_strict_modes\s*<", env_support)) != 1:
        raise RuntimeError("LOOP0-P0b-F0 two-mode owner drift")
    mode_pair = _function_body(env_support, "with_default_and_strict_modes")
    for token, expected in (
        ("ScopedTestConfig::apply", 1),
        ("GenericLoopTestModeV1::Default", 1),
        ("set_mode(", 1),
        ("GenericLoopTestModeV1::StrictPlannerRequired", 1),
    ):
        if mode_pair.count(token) != expected:
            raise RuntimeError(
                f"LOOP0-P0b-F0 mode-pair step drift: token={token}"
            )
    ordered_steps = (
        mode_pair.index("ScopedTestConfig::apply"),
        mode_pair.index("GenericLoopTestModeV1::Default"),
        mode_pair.index("set_mode("),
        mode_pair.index("GenericLoopTestModeV1::StrictPlannerRequired"),
    )
    if list(ordered_steps) != sorted(ordered_steps):
        raise RuntimeError("LOOP0-P0b-F0 default/strict order drift")
    if "with_env_vars" in mode_pair or "Mutex" in mode_pair:
        raise RuntimeError("LOOP0-P0b-F0 mode pair acquired a second lock")

    direct_env_pattern = re.compile(
        r"(?:std::)?env::(?:set_var|remove_var)\(\s*\"("
        + "|".join(map(re.escape, P0B_F0_MODE_KEYS))
        + r")\""
    )
    direct_writers = []
    for path in (root / "src").rglob("*.rs"):
        matches = direct_env_pattern.findall(path.read_text(encoding="utf-8"))
        direct_writers.extend(
            f"{path.relative_to(root).as_posix()}:{key}" for key in matches
        )
    if direct_writers:
        raise RuntimeError(
            "LOOP0-P0b-F0 mode keys bypass the process-state lock: "
            f"writers={direct_writers}"
        )

    f0_touched = (*P0B_F0_TOUCHED_PATHS, __file__)
    f0_oversized = []
    for path in f0_touched:
        relative = str(path) if isinstance(path, str) else str(Path(path).relative_to(root))
        if len(_read(root, relative).splitlines()) >= 800:
            f0_oversized.append(relative)
    if f0_oversized:
        raise RuntimeError(
            f"LOOP0-P0b-F0 source/check files reached 800 lines: {f0_oversized}"
        )

    return (
        "p0a_port_owners=1 raw_ports=1 passive_located_ports=1 "
        "production_located_execution_callers=0 path_policy_owners=0 "
        "production_located_producers=1 generic_loop_passive_consumers=1 "
        "p0b_f0_actual_fixtures=1 p0b_f0_mode_locks=1"
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
