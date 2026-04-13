"""LoopSimdContract owner seam for numeric loop / SIMD planning.

The current cut is intentionally docs-first and non-behavioral. It turns the
already-landed numeric loop proof into one contract shape with four concerns:
proof, policy, lowering, and diagnostics.
"""

from typing import Any, Dict, List, Optional


def _sorted_int_list(values: Any) -> List[int]:
    out: List[int] = []
    for value in values or []:
        if isinstance(value, int):
            out.append(int(value))
    return sorted(out)


def build_loop_simd_contract(loop_plan: Optional[Dict[str, Any]]) -> Optional[Dict[str, Any]]:
    """Build a conservative LoopSimdContract from an annotated loop plan.

    The phase265x cut only accepts loops that already carry the numeric proof
    seam. Actual SIMD widening is intentionally deferred to later phases.
    """

    if not isinstance(loop_plan, dict):
        return None

    numeric_kind = str(loop_plan.get("numeric_kind") or "")
    if numeric_kind != "induction":
        return None

    header_bid = loop_plan.get("header")
    if not isinstance(header_bid, int):
        return None

    induction_value_ids = _sorted_int_list(loop_plan.get("numeric_induction_value_ids"))
    reduction_value_ids = _sorted_int_list(loop_plan.get("numeric_reduction_value_ids"))
    non_negative_value_ids = _sorted_int_list(loop_plan.get("numeric_non_negative_value_ids"))
    header_phi_value_ids = _sorted_int_list(loop_plan.get("header_phi_value_ids"))
    header_compare_operand_value_ids = _sorted_int_list(loop_plan.get("header_compare_operand_value_ids"))

    accepted_class = "int_reduction_candidate" if reduction_value_ids else "int_map_candidate"

    return {
        "header": int(header_bid),
        "proof": {
            "loop_shape": "counted_simple_while_candidate",
            "body_shape": "straight_line_arithmetic_only",
            "numeric_kind": numeric_kind,
            "proof_source": str(loop_plan.get("numeric_proof_source") or ""),
            "induction_value_ids": induction_value_ids,
            "reduction_value_ids": reduction_value_ids,
            "non_negative_value_ids": non_negative_value_ids,
            "header_phi_value_ids": header_phi_value_ids,
            "header_compare_operand_value_ids": header_compare_operand_value_ids,
        },
        "policy": {
            "mode": "auto_eligible",
            "vf_pref": "auto",
            "predication": "off",
            "tail_policy": "scalar_epilogue",
        },
        "lowering": {
            "llvm_loop_md": "defer",
            "access_groups": [],
            "alias_scopes": [],
            "call_attrs": [],
        },
        "diag": {
            "accepted_class": accepted_class,
            "reject_reason": None,
        },
    }
