"""PGO build-policy owner seam.

Phase276x adds the first actual generate/use cut while keeping LLVM-side
instrumentation and profile consumption out of scope for now.
"""

import os
from dataclasses import dataclass
from typing import Optional

_PGO_PHASE_ENV_KEYS = ("NYASH_LLVM_PGO_PHASE", "HAKO_LLVM_PGO_PHASE")
_PGO_PROFILE_ENV_KEYS = ("NYASH_LLVM_PGO_PROFILE", "HAKO_LLVM_PGO_PROFILE")


@dataclass(frozen=True)
class PgoBuildPolicy:
    phase: str
    producer: str
    artifact: str
    exclusion: str
    hotness_feed: str


def _requested_profile_path() -> Optional[str]:
    for key in _PGO_PROFILE_ENV_KEYS:
        raw = os.environ.get(key)
        if not raw:
            continue
        value = raw.strip()
        if value:
            return value
    return None


def _default_generate_artifact(output_path: Optional[str]) -> str:
    if output_path:
        if output_path.endswith(".o"):
            return output_path[:-2] + ".profraw"
        return output_path + ".profraw"
    return "module.profraw"


def pgo_sidecar_path(output_path: str, policy: PgoBuildPolicy) -> Optional[str]:
    if policy.phase == "off":
        return None
    if output_path.endswith(".o"):
        return output_path[:-2] + ".pgo.json"
    return output_path + ".pgo.json"


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


def resolve_pgo_build_policy(output_path: Optional[str] = None) -> PgoBuildPolicy:
    """Return the current PGO policy.

    Phase276x widens one actual path:
    - `generate`: resolve a conservative raw-profile artifact path
    - `use`: accept an explicit indexed-profile path and mark hotness feed as `pgo`
    LLVM-side instrumentation / use remains future work.
    """

    phase = _requested_pgo_phase()
    if phase == "generate":
        return PgoBuildPolicy(
            phase="generate",
            producer="instr_ir",
            artifact=_default_generate_artifact(output_path),
            exclusion="allow",
            hotness_feed="none",
        )
    if phase == "use":
        profile_path = _requested_profile_path()
        if profile_path and os.path.exists(profile_path):
            return PgoBuildPolicy(
                phase="use",
                producer="indexed_profdata",
                artifact=profile_path,
                exclusion="allow",
                hotness_feed="pgo",
            )
        phase = "off"

    return PgoBuildPolicy(
        phase=phase,
        producer="none",
        artifact="none",
        exclusion="allow",
        hotness_feed="none",
    )
