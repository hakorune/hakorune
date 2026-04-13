"""IPO / build-time optimization policy seam.

Phase272x centralizes build-policy ownership for future `ThinLTO` / `PGO`
widening while keeping current behavior unchanged.
"""

from dataclasses import dataclass
from typing import Any


@dataclass(frozen=True)
class IpoBuildPolicy:
    lto_mode: str
    pgo_mode: str


def resolve_ipo_build_policy() -> IpoBuildPolicy:
    """Return the current IPO build policy.

    Phase272x is owner-seam only, so both knobs stay `off`.
    """
    return IpoBuildPolicy(
        lto_mode="off",
        pgo_mode="off",
    )


def apply_ipo_build_policy(target_machine_kwargs: dict[str, Any], policy: IpoBuildPolicy) -> dict[str, Any]:
    """Apply IPO policy to target-machine kwargs.

    Current cut is a no-op by design. The helper exists so future `ThinLTO` /
    `PGO` widening has one policy owner instead of open-coded kwargs mutation.
    """
    _ = target_machine_kwargs
    _ = policy
    return target_machine_kwargs
