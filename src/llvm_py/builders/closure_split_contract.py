"""
Closure split contract owner seam.

Phase269x keeps closure split conservative:
- capture classification only
- env scalarization deferred
- thin-entry specialization deferred
"""

from typing import Any, Dict, Iterable, List, Optional


def _capture_id(entry: Any) -> Optional[int]:
    if isinstance(entry, dict) and "id" in entry:
        try:
            return int(entry["id"])
        except (TypeError, ValueError):
            return None
    if isinstance(entry, int):
        return int(entry)
    return None


def _sorted_unique(values: Iterable[int]) -> List[int]:
    return sorted(dict.fromkeys(int(v) for v in values))


def build_closure_split_contract(
    params: Optional[List[Dict[str, Any]]],
    captures: Optional[List[Any]],
    me_capture: Optional[Any],
) -> Dict[str, Any]:
    """
    Build a conservative closure split contract.

    The contract owns capture classification only. Actual env scalarization and
    thin-entry specialization stay deferred for later cuts.
    """
    capture_value_ids = [
        capture_id
        for capture_id in (_capture_id(entry) for entry in (captures or []))
        if capture_id is not None
    ]
    me_capture_id = _capture_id(me_capture)

    env_capture_value_ids = list(capture_value_ids)
    if me_capture_id is not None:
        env_capture_value_ids.append(me_capture_id)

    env_capture_count = len(env_capture_value_ids)
    if capture_value_ids and me_capture_id is not None:
        accepted_class = "capture_env_with_me"
    elif capture_value_ids:
        accepted_class = "capture_env_only"
    elif me_capture_id is not None:
        accepted_class = "me_only_env"
    else:
        accepted_class = "empty_env"

    ctor_name = "nyash.closure.new_with_captures" if env_capture_count > 0 else "nyash.closure.new"

    return {
        "proof": {
            "param_count": len(params or []),
            "capture_value_ids": _sorted_unique(capture_value_ids),
            "me_capture_id": me_capture_id,
            "env_capture_value_ids": env_capture_value_ids,
            "env_capture_count": env_capture_count,
        },
        "policy": {
            "env_shape": accepted_class,
            "env_scalarization": "defer",
            "thin_entry_specialization": "defer",
        },
        "lowering": {
            "ctor_name": ctor_name,
            "use_capture_ctor": env_capture_count > 0,
        },
        "diag": {
            "accepted_class": accepted_class,
        },
    }
