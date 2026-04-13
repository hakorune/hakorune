"""LoopSimdContract owner seam for numeric loop / SIMD planning.

The current cut is intentionally docs-first and non-behavioral. It turns the
already-landed numeric loop proof into one contract shape with four concerns:
proof, policy, lowering, and diagnostics.
"""

from typing import Any, Dict, List, Optional

from llvmlite import ir


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
    select_value_ids = _sorted_int_list(loop_plan.get("numeric_select_value_ids"))
    non_negative_value_ids = _sorted_int_list(loop_plan.get("numeric_non_negative_value_ids"))
    header_phi_value_ids = _sorted_int_list(loop_plan.get("header_phi_value_ids"))
    header_compare_operand_value_ids = _sorted_int_list(loop_plan.get("header_compare_operand_value_ids"))

    if reduction_value_ids:
        accepted_class = "int_reduction_candidate"
    elif select_value_ids:
        accepted_class = "int_compare_select_candidate"
    else:
        accepted_class = "int_map_candidate"

    lowering_md: Dict[str, Any]
    if accepted_class == "int_map_candidate":
        lowering_md = {
            "vectorize.enable": True,
        }
    elif accepted_class == "int_reduction_candidate":
        lowering_md = {
            "vectorize.enable": True,
            "reduction.kind": "int_add",
        }
    elif accepted_class == "int_compare_select_candidate":
        lowering_md = {
            "vectorize.enable": True,
            "compare_select.kind": "select",
        }
    else:
        lowering_md = {}

    return {
        "header": int(header_bid),
        "proof": {
            "loop_shape": "counted_simple_while_candidate",
            "body_shape": "straight_line_arithmetic_only",
            "numeric_kind": numeric_kind,
            "proof_source": str(loop_plan.get("numeric_proof_source") or ""),
            "induction_value_ids": induction_value_ids,
            "reduction_value_ids": reduction_value_ids,
            "select_value_ids": select_value_ids,
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
            "llvm_loop_md": lowering_md if lowering_md else "defer",
            "access_groups": [],
            "alias_scopes": [],
            "call_attrs": [],
        },
        "diag": {
            "accepted_class": accepted_class,
            "reject_reason": None,
        },
    }


def apply_loop_simd_metadata(module: ir.Module, terminator: Any, contract: Optional[Dict[str, Any]]) -> bool:
    """Attach a conservative llvm.loop hint for the current actual widening cut.

    Phase267x emits metadata for integer map and integer reduction candidates.
    Broader widening shapes still stay deferred.
    """

    if terminator is None or contract is None:
        return False

    lowering = contract.get("lowering") if isinstance(contract, dict) else None
    if not isinstance(lowering, dict):
        return False
    llvm_loop_md = lowering.get("llvm_loop_md")
    if not isinstance(llvm_loop_md, dict):
        return False

    operands: List[Any] = []
    if llvm_loop_md.get("vectorize.enable") is True:
        operands.append(
            module.add_metadata(
                [
                    ir.MetaDataString(module, "llvm.loop.vectorize.enable"),
                    ir.Constant(ir.IntType(1), 1),
                ]
            )
        )
    width = llvm_loop_md.get("vectorize.width")
    if isinstance(width, int):
        operands.append(
            module.add_metadata(
                [
                    ir.MetaDataString(module, "llvm.loop.vectorize.width"),
                    ir.Constant(ir.IntType(32), int(width)),
                ]
            )
        )

    if not operands:
        return False

    terminator.set_metadata("llvm.loop", module.add_metadata(operands))
    return True
