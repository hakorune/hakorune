"""RuntimeDataBox route-policy SSOT.

This module owns the env-backed route-policy source for RuntimeDataBox lowering.
The dispatch layer consumes it as a thin facade; tests reset the cached policy
through the dedicated helper below.
"""

import os
from functools import lru_cache


@lru_cache(maxsize=1)
def runtime_data_array_route_policy():
    """
    RuntimeDataBox array-route policy SSOT.

    - default (`array_mono`): allow current array-specialized route
      (`push -> slot_append_hh`, integer-key `get/set -> slot_load_hi/slot_store_hih|slot_store_hii`,
      `has -> runtime_data.has_hh`)
    - `runtime_data_only`: force `nyash.runtime_data.*` even when array hints match
    """
    raw = str(os.getenv("NYASH_RUNTIME_DATA_ARRAY_ROUTE_POLICY", "array_mono") or "array_mono")
    policy = raw.strip().lower()
    if policy in ("array_mono", "array", "default"):
        return "array_mono"
    if policy in ("runtime_data_only", "runtime_data"):
        return "runtime_data_only"
    raise RuntimeError(
        "unsupported NYASH_RUNTIME_DATA_ARRAY_ROUTE_POLICY="
        f"{raw!r} (expected: array_mono|runtime_data_only)"
    )


def prefer_array_mono_route_default():
    return runtime_data_array_route_policy() == "array_mono"


def reset_runtime_data_array_route_policy_cache_for_tests():
    runtime_data_array_route_policy.cache_clear()
