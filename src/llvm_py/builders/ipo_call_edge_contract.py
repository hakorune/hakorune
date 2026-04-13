"""IPO call-edge contract owner seam.

Phase273x derives a conservative static call-edge view from callable-node facts
without enabling ThinLTO or PGO yet.
"""

from typing import Any, Dict, Optional


def build_ipo_call_edge_contract(callable_contract: Optional[Dict[str, Any]]) -> Optional[Dict[str, Any]]:
    """Build a conservative IPO call-edge contract from a callable contract."""

    if not isinstance(callable_contract, dict):
        return None

    proof = callable_contract.get("proof")
    policy = callable_contract.get("policy")
    if not isinstance(proof, dict) or not isinstance(policy, dict):
        return None

    thin_surface = str(proof.get("thin_surface") or "")
    if thin_surface == "cross_module_direct":
        call_shape = "DirectThin"
    else:
        call_shape = "DirectThick"

    return {
        "proof": {
            "call_shape": call_shape,
            "hotness_overlay": None,
        },
        "policy": {
            "import_class": str(policy.get("import_class") or "public_only"),
        },
        "lowering": {
            "summary_hotness": "none",
        },
        "diag": {
            "accepted_class": call_shape.lower(),
            "reject_reason": None,
        },
    }
