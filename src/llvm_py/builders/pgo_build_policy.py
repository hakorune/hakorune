"""PGO build-policy owner seam.

Phase275x adds one owner seam for future profile-generate / profile-use cuts
while keeping current behavior unchanged.
"""

import os
from dataclasses import dataclass

_PGO_PHASE_ENV_KEYS = ("NYASH_LLVM_PGO_PHASE", "HAKO_LLVM_PGO_PHASE")


@dataclass(frozen=True)
class PgoBuildPolicy:
    phase: str
    producer: str
    artifact: str
    exclusion: str
    hotness_feed: str


def _requested_pgo_phase() -> str:
    for key in _PGO_PHASE_ENV_KEYS:
        raw = os.environ.get(key)
        if not raw:
            continue
        value = raw.strip().lower()
        if value in ("generate", "use"):
            return value
        if value in ("off", "0", "false", "no"):
            return "off"
    return "off"


def resolve_pgo_build_policy() -> PgoBuildPolicy:
    """Return the current PGO scaffold policy.

    Phase275x is scaffold-only, so the policy shape is fixed but behavior stays
    off even when future env plumbing appears.
    """

    phase = _requested_pgo_phase()
    if phase != "off":
        # Scaffold only: keep generate/use disabled until a later cut.
        phase = "off"

    return PgoBuildPolicy(
        phase=phase,
        producer="none",
        artifact="none",
        exclusion="allow",
        hotness_feed="none",
    )
