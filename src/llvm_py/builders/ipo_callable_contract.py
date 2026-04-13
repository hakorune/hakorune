"""IPO callable-node contract owner seam.

Phase273x keeps behavior unchanged and derives static IPO-facing callable facts
from already-landed closure split meaning. ThinLTO and PGO should consume this
contract rather than raw closure thin-entry booleans.
"""

from typing import Any, Dict, Optional


def build_ipo_callable_contract(closure_contract: Optional[Dict[str, Any]]) -> Optional[Dict[str, Any]]:
    """Build a conservative IPO callable contract from a closure split contract."""

    if not isinstance(closure_contract, dict):
        return None

    proof = closure_contract.get("proof")
    policy = closure_contract.get("policy")
    lowering = closure_contract.get("lowering")
    diag = closure_contract.get("diag")
    if not isinstance(proof, dict) or not isinstance(policy, dict) or not isinstance(lowering, dict):
        return None

    accepted_class = str((diag or {}).get("accepted_class") or "")
    env_scalarization = str(policy.get("env_scalarization") or "")
    env_capture_count = int(proof.get("env_capture_count") or 0)
    thin_entry_eligible = bool(lowering.get("thin_entry_eligible"))

    if accepted_class == "empty_env":
        env_surface = "empty"
    elif env_scalarization == "scalar_single":
        env_surface = "single_scalar"
    else:
        env_surface = "aggregate_handle"

    if thin_entry_eligible and env_surface == "empty":
        thin_surface = "cross_module_direct"
        import_class = "cross_module_candidate"
    elif thin_entry_eligible:
        thin_surface = "local_only"
        import_class = "module_local_only"
    else:
        thin_surface = "none"
        import_class = "public_only"

    return {
        "proof": {
            "thin_surface": thin_surface,
            "env_surface": env_surface,
            "env_capture_count": env_capture_count,
            "addr_taken": False,
            "escape_class": "closure_value",
            "effect_facts": {
                "memory": "unknown",
                "sync": "unknown",
                "unwind": "unknown",
            },
        },
        "policy": {
            "import_class": import_class,
            "inline_bias": "neutral",
        },
        "lowering": {
            "summary_flags": {
                "thinlto_import_candidate": thin_surface == "cross_module_direct",
            },
            "call_attrs": [],
        },
        "diag": {
            "accepted_class": f"{thin_surface}.{env_surface}",
            "reject_reason": None,
        },
    }
