#!/usr/bin/env python3
"""Fail fast on pure-first lowering routes that MIR metadata already rejects."""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Any


FAIL_TAG = "[pure-first-route][fail]"
OK_TAG = "[pure-first-route][ok]"


def load_json(path: Path) -> dict[str, Any]:
    with path.open(encoding="utf-8") as fh:
        data = json.load(fh)
    if not isinstance(data, dict):
        raise ValueError("MIR JSON root must be an object")
    return data


def site_id(block_id: Any, index: int) -> str:
    return f"b{block_id}.i{index}"


def iter_mir_calls(fn: dict[str, Any]):
    for block in fn.get("blocks", []) or []:
        if not isinstance(block, dict):
            continue
        block_id = block.get("id")
        instructions = block.get("instructions", []) or []
        if not isinstance(instructions, list):
            continue
        for index, inst in enumerate(instructions):
            if not isinstance(inst, dict):
                continue
            if inst.get("op") != "mir_call":
                continue
            yield site_id(block_id, index), inst


def plan_callee(plan: dict[str, Any]) -> str:
    source = plan.get("source")
    if source == "extern_call_routes":
        return str(plan.get("source_symbol") or plan.get("symbol") or plan.get("source_route_id") or "?")
    if source == "global_call_routes":
        return str(plan.get("callee_name") or plan.get("target_symbol") or plan.get("source_route_id") or "?")
    if source == "user_box_method_routes":
        box_name = plan.get("box_name") or "?"
        method = plan.get("method") or "?"
        return f"{box_name}.{method}"
    return str(plan.get("source_route_id") or plan.get("symbol") or "?")


def inst_callee(inst: dict[str, Any]) -> str:
    call = inst.get("mir_call")
    if not isinstance(call, dict):
        return "?"
    callee = call.get("callee")
    if not isinstance(callee, dict):
        return "?"
    ctype = callee.get("type")
    if ctype == "Method":
        box_name = callee.get("box_name") or "?"
        name = callee.get("name") or "?"
        return f"{box_name}.{name}"
    return str(callee.get("name") or "?")


def inst_callee_type(inst: dict[str, Any]) -> str:
    call = inst.get("mir_call")
    if not isinstance(call, dict):
        return ""
    callee = call.get("callee")
    if not isinstance(callee, dict):
        return ""
    return str(callee.get("type") or "")


def route_needs_return_shape(plan: dict[str, Any]) -> bool:
    if plan.get("result_value") is None:
        return False
    if plan.get("source") not in {
        "extern_call_routes",
        "global_call_routes",
        "user_box_method_routes",
    }:
        return False
    emit_kind = plan.get("emit_kind")
    return emit_kind not in {None, "unsupported"}


def route_needs_value_demand(plan: dict[str, Any]) -> bool:
    if plan.get("return_shape") is None:
        return False
    if plan.get("source") not in {
        "extern_call_routes",
        "global_call_routes",
        "user_box_method_routes",
    }:
        return False
    emit_kind = plan.get("emit_kind")
    return emit_kind not in {None, "unsupported"}


def accepted_reason(plan: dict[str, Any]) -> bool:
    reason = plan.get("reason")
    return reason in {None, "", "none", "ok"}


def failure(
    function: str,
    site: str,
    callee: str,
    reason: str,
    owner: str,
    suggestion: str,
    *,
    layer: str = "route-preflight",
    contract: str = "metadata.lowering_plan",
) -> dict[str, str]:
    return {
        "layer": layer,
        "function": function,
        "site": site,
        "callee": callee,
        "reason": reason,
        "owner": owner,
        "contract": contract,
        "suggestion": suggestion,
    }


def classify_plan(fn_name: str, plan: dict[str, Any]) -> dict[str, str] | None:
    site = str(plan.get("site") or f"b{plan.get('block')}.i{plan.get('instruction_index')}")
    source = str(plan.get("source") or "lowering_plan")
    callee = plan_callee(plan)

    if plan.get("tier") == "Unsupported" or plan.get("emit_kind") == "unsupported":
        return failure(
            fn_name,
            site,
            callee,
            "unsupported_tier",
            source,
            "select a supported LoweringPlan tier before pure-first EXE build",
            layer="route-preflight",
            contract="tier/emit_kind",
        )

    if source in {"global_call_routes", "user_box_method_routes"}:
        if plan.get("target_exists") is False:
            return failure(
                fn_name,
                site,
                callee,
                "target_exists=false",
                source,
                "fix resolver/module target publication before backend emission",
                layer="semantic-route",
                contract="target_exists",
            )
        if plan.get("arity_matches") is False:
            return failure(
                fn_name,
                site,
                callee,
                "arity_mismatch",
                source,
                "fix call arity or route arity contract before backend emission",
                layer="semantic-route",
                contract="arity_matches",
            )

    if source == "user_box_method_routes":
        if plan.get("target_body_supported") is False:
            return failure(
                fn_name,
                site,
                callee,
                "target_body_supported=false",
                source,
                "split a generic user-box method body acceptance row",
                layer="semantic-route",
                contract="target_body_supported",
            )
        if not accepted_reason(plan):
            return failure(
                fn_name,
                site,
                callee,
                "typed_user_box_method_contract_missing",
                source,
                "publish a supported typed user-box method route contract",
                layer="semantic-route",
                contract="reason/proof",
            )

    if source == "global_call_routes" and not accepted_reason(plan):
        return failure(
            fn_name,
            site,
            callee,
            "typed_global_call_contract_missing",
            source,
            "publish a supported typed global-call route contract",
            layer="semantic-route",
            contract="reason/proof",
        )

    if route_needs_return_shape(plan) and plan.get("return_shape") is None:
        return failure(
            fn_name,
            site,
            callee,
            "return_shape_missing",
            source,
            "publish return_shape or make the route diagnostics-only",
            layer="semantic-route",
            contract="return_shape",
        )

    if route_needs_value_demand(plan) and not plan.get("value_demand"):
        return failure(
            fn_name,
            site,
            callee,
            "value_demand_mismatch",
            source,
            "publish value_demand compatible with return_shape",
            layer="semantic-route",
            contract="value_demand",
        )

    if (
        source in {"global_call_routes", "user_box_method_routes"}
        and plan.get("return_shape") == "object_handle"
        and not plan.get("target_result_box_name")
    ):
        return failure(
            fn_name,
            site,
            callee,
            "object_return_target_box_missing",
            source,
            "publish target_result_box_name for object_handle route results",
            layer="semantic-route",
            contract="return_shape=object_handle requires target_result_box_name",
        )

    return None


def classify_missing_plans(
    fn_name: str,
    plans_by_site: dict[str, dict[str, Any]],
    mir_calls: list[tuple[str, dict[str, Any]]],
) -> list[dict[str, str]]:
    failures: list[dict[str, str]] = []
    for site, inst in mir_calls:
        if site in plans_by_site:
            continue
        if inst_callee_type(inst) != "Extern":
            continue
        failures.append(
            failure(
                fn_name,
                site,
                inst_callee(inst),
                "lowering_plan_missing",
                "extern_call_routes",
                "refresh extern_call_routes/lowering_plan for this extern call site",
                layer="route-preflight",
                contract="metadata.lowering_plan[site]",
            )
        )
    return failures


def capability_plan_allows(plan: dict[str, Any], capability: str) -> bool:
    allow = plan.get("allow")
    return isinstance(allow, list) and capability in {str(item) for item in allow}


def classify_random_capability_plans(
    fn_name: str,
    metadata: dict[str, Any],
) -> list[dict[str, str]]:
    plans = metadata.get("capability_plans")
    if not isinstance(plans, list):
        return []

    failures: list[dict[str, str]] = []
    for plan in plans:
        if not isinstance(plan, dict) or not capability_plan_allows(plan, "hako.random"):
            continue
        failures.append(
            failure(
                fn_name,
                "<capability>",
                "hako.random",
                "random_capability_route_unsupported",
                "capability_plans",
                "select a supported random route row or keep the caller on proof-only deterministic keys",
                layer="route-preflight",
                contract="metadata.capability_plans[hako.random]",
            )
        )
    return failures


def collect_failures(
    data: dict[str, Any],
    *,
    reject_unsupported_random: bool = False,
) -> tuple[list[dict[str, str]], int, int]:
    functions = data.get("functions", [])
    if not isinstance(functions, list):
        return (
            [
                failure(
                    "<module>",
                    "<schema>",
                    "?",
                    "lowering_plan_missing",
                    "mir_json",
                    "MIR JSON root must contain functions[]",
                    layer="mir-schema",
                    contract="functions[]",
                )
            ],
            0,
            0,
        )

    fn_by_name: dict[str, dict[str, Any]] = {}
    for fn in functions:
        if not isinstance(fn, dict):
            continue
        fn_by_name[str(fn.get("name") or "<anonymous>")] = fn

    reachable = reachable_functions(fn_by_name)

    failures: list[dict[str, str]] = []
    plan_count = 0
    for fn in functions:
        if not isinstance(fn, dict):
            continue
        fn_name = str(fn.get("name") or "<anonymous>")
        if fn_name not in reachable:
            continue
        metadata = fn.get("metadata")
        if not isinstance(metadata, dict):
            failures.append(
                failure(
                    fn_name,
                    "<metadata>",
                    "?",
                    "lowering_plan_missing",
                    "metadata",
                    "function metadata must contain lowering_plan array",
                    layer="mir-schema",
                    contract="function.metadata.lowering_plan",
                )
            )
            continue
        if reject_unsupported_random:
            failures.extend(classify_random_capability_plans(fn_name, metadata))
        plans = metadata.get("lowering_plan")
        if not isinstance(plans, list):
            failures.append(
                failure(
                    fn_name,
                    "<metadata>",
                    "?",
                    "lowering_plan_missing",
                    "metadata.lowering_plan",
                    "function metadata.lowering_plan must be an array",
                    layer="mir-schema",
                    contract="function.metadata.lowering_plan",
                )
            )
            continue

        plans_by_site: dict[str, dict[str, Any]] = {}
        for plan in plans:
            if not isinstance(plan, dict):
                continue
            plan_count += 1
            site = str(plan.get("site") or f"b{plan.get('block')}.i{plan.get('instruction_index')}")
            if site in plans_by_site:
                failures.append(
                    failure(
                        fn_name,
                        site,
                        plan_callee(plan),
                        "lowering_plan_missing",
                        "metadata.lowering_plan",
                        "duplicate lowering_plan entries for one site",
                        layer="mir-schema",
                        contract="unique metadata.lowering_plan.site",
                    )
                )
            plans_by_site[site] = plan
            plan_failure = classify_plan(fn_name, plan)
            if plan_failure is not None:
                failures.append(plan_failure)

        mir_calls = list(iter_mir_calls(fn))
        failures.extend(classify_missing_plans(fn_name, plans_by_site, mir_calls))

    return failures, len(reachable), plan_count


def entrypoint_names(fn_by_name: dict[str, dict[str, Any]]) -> set[str]:
    if "main" in fn_by_name:
        return {"main"}

    suffix_matches = {name for name in fn_by_name if name.endswith(".main/0")}
    if suffix_matches:
        return suffix_matches

    return set(fn_by_name)


def plan_target_function(plan: dict[str, Any]) -> str | None:
    source = plan.get("source")
    if source == "global_call_routes":
        target = plan.get("target_symbol") or plan.get("symbol") or plan.get("callee_name")
    elif source == "user_box_method_routes":
        target = plan.get("target_symbol") or plan.get("symbol")
    else:
        return None

    if not target:
        return None
    return str(target)


def plan_is_direct_edge(plan: dict[str, Any]) -> bool:
    if plan.get("tier") == "Unsupported" or plan.get("emit_kind") == "unsupported":
        return False
    if plan.get("target_exists") is False or plan.get("arity_matches") is False:
        return False
    return plan.get("emit_kind") == "direct_function_call"


def reachable_functions(fn_by_name: dict[str, dict[str, Any]]) -> set[str]:
    """Approximate pure-first EXE function emission reachability.

    The MIR ledger can contain diagnostic lowering_plan rows for functions that
    are not emitted into the current EXE. Preflight should fail before the C shim
    only for routes that the pure-first function emitter can reach from main.
    """

    seen = set(entrypoint_names(fn_by_name))
    work = list(seen)
    while work:
        fn_name = work.pop()
        fn = fn_by_name.get(fn_name)
        if not isinstance(fn, dict):
            continue
        metadata = fn.get("metadata")
        if not isinstance(metadata, dict):
            continue
        plans = metadata.get("lowering_plan")
        if not isinstance(plans, list):
            continue
        for plan in plans:
            if not isinstance(plan, dict) or not plan_is_direct_edge(plan):
                continue
            target = plan_target_function(plan)
            if target is None or target not in fn_by_name or target in seen:
                continue
            seen.add(target)
            work.append(target)
    return seen


def print_failure(item: dict[str, str]) -> None:
    print(FAIL_TAG, file=sys.stderr)
    print(f"layer={item['layer']}", file=sys.stderr)
    print(f"function={item['function']}", file=sys.stderr)
    print(f"site={item['site']}", file=sys.stderr)
    print(f"callee={item['callee']}", file=sys.stderr)
    print(f"reason={item['reason']}", file=sys.stderr)
    print(f"owner={item['owner']}", file=sys.stderr)
    print(f"contract={item['contract']}", file=sys.stderr)
    print(f"suggestion={item['suggestion']}", file=sys.stderr)


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(
        description="Preflight pure-first MIR lowering_plan route contracts."
    )
    parser.add_argument(
        "--reject-unsupported-random",
        action="store_true",
        help="fail on reachable hako.random capability metadata until a random route row lands",
    )
    parser.add_argument("mir_json", type=Path)
    args = parser.parse_args(argv)

    data = load_json(args.mir_json)
    failures, function_count, plan_count = collect_failures(
        data,
        reject_unsupported_random=args.reject_unsupported_random,
    )
    if failures:
        for item in failures:
            print_failure(item)
        return 1

    print(f"{OK_TAG} layer=route-preflight functions={function_count} plans={plan_count}")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main(sys.argv[1:]))
    except Exception as exc:  # noqa: BLE001 - shell guard should get compact failure.
        print(FAIL_TAG, file=sys.stderr)
        print("layer=mir-schema", file=sys.stderr)
        print("function=<preflight>", file=sys.stderr)
        print("site=<schema>", file=sys.stderr)
        print("callee=?", file=sys.stderr)
        print("reason=lowering_plan_missing", file=sys.stderr)
        print("owner=mir_json", file=sys.stderr)
        print("contract=valid MIR JSON object", file=sys.stderr)
        print(f"suggestion={exc}", file=sys.stderr)
        raise SystemExit(1)
