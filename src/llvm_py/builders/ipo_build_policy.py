"""IPO / build-time optimization policy seam.

Phase274x centralizes build-policy ownership for the first narrow `ThinLTO`
artifact cut while keeping default behavior unchanged.
"""

import os
from dataclasses import dataclass
from typing import Any, Mapping, Optional
from builders.pgo_build_policy import resolve_pgo_build_policy

_LTO_ENV_KEYS = ("NYASH_LLVM_LTO_MODE", "HAKO_LLVM_LTO_MODE")


@dataclass(frozen=True)
class IpoBuildPolicy:
    lto_mode: str
    pgo_mode: str
    thinlto_import_candidate_count: int = 0


def summarize_ipo_contracts(
    callable_contracts_by_function: Optional[Mapping[str, Mapping[int, Mapping[str, Any]]]],
    edge_contracts_by_function: Optional[Mapping[str, Mapping[int, Mapping[str, Any]]]],
) -> dict[str, int]:
    """Summarize landed IPO callable/edge contracts for build-policy consumers."""

    thin_candidates = 0
    direct_thin_edges = 0

    for rows in (callable_contracts_by_function or {}).values():
        if not isinstance(rows, Mapping):
            continue
        for contract in rows.values():
            proof = contract.get("proof") if isinstance(contract, dict) else None
            lowering = contract.get("lowering") if isinstance(contract, dict) else None
            if not isinstance(proof, dict) or not isinstance(lowering, dict):
                continue
            if str(proof.get("thin_surface") or "") != "cross_module_direct":
                continue
            if bool((lowering.get("summary_flags") or {}).get("thinlto_import_candidate")):
                thin_candidates += 1

    for rows in (edge_contracts_by_function or {}).values():
        if not isinstance(rows, Mapping):
            continue
        for contract in rows.values():
            proof = contract.get("proof") if isinstance(contract, dict) else None
            if not isinstance(proof, dict):
                continue
            if str(proof.get("call_shape") or "") == "DirectThin":
                direct_thin_edges += 1

    return {
        "thinlto_import_candidate_count": thin_candidates,
        "direct_thin_edge_count": direct_thin_edges,
    }


def _requested_lto_mode() -> str:
    for key in _LTO_ENV_KEYS:
        raw = os.environ.get(key)
        if not raw:
            continue
        value = raw.strip().lower()
        if value == "thin":
            return "thin"
        if value in ("off", "0", "false", "no"):
            return "off"
    return "off"


def resolve_ipo_build_policy(module_summary: Optional[Mapping[str, int]] = None) -> IpoBuildPolicy:
    """Return the current IPO build policy.

    Phase274x keeps default behavior unchanged. `ThinLTO` only becomes active
    when explicitly requested and the landed callable/edge contracts prove that
    at least one direct thin import candidate exists.
    """
    summary = dict(module_summary or {})
    thin_candidate_count = int(summary.get("thinlto_import_candidate_count") or 0)
    direct_thin_edge_count = int(summary.get("direct_thin_edge_count") or 0)

    lto_mode = "off"
    if _requested_lto_mode() == "thin" and thin_candidate_count > 0 and direct_thin_edge_count > 0:
        lto_mode = "thin"

    return IpoBuildPolicy(
        lto_mode=lto_mode,
        pgo_mode=resolve_pgo_build_policy().phase,
        thinlto_import_candidate_count=thin_candidate_count,
    )


def apply_ipo_build_policy(target_machine_kwargs: dict[str, Any], policy: IpoBuildPolicy) -> dict[str, Any]:
    """Apply IPO policy to target-machine kwargs.

    Current cut is a no-op by design. The helper exists so future `ThinLTO` /
    `PGO` widening has one policy owner instead of open-coded kwargs mutation.
    """
    _ = target_machine_kwargs
    _ = policy
    return target_machine_kwargs


def thinlto_companion_path(output_path: str, policy: IpoBuildPolicy) -> Optional[str]:
    """Return the ThinLTO companion bitcode path for the current policy."""

    if policy.lto_mode != "thin":
        return None
    if output_path.endswith(".o"):
        return output_path[:-2] + ".thinlto.bc"
    return output_path + ".thinlto.bc"
