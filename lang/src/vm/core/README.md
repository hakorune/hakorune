VM Core (Skeleton) — Phase 20.12b

Responsibility
- Provide a minimal, centralized execution core for MIR(JSON v0).
- First batch: value/state/extern_iface/json_v0_reader/dispatcher + ops {const, binop, ret}.
- Goal: engines (hakorune, mini) become thin wrappers that call Core.

Scope and Guards
- This is a skeleton to establish structure and entry points.
- Parsing uses simple, escape-aware cursors from shared json cursors.
- Fail-Fast: unknown ops or malformed JSON return -1 and print a stable error.

Migration Notes
- Existing hakorune-vm remains the authoritative engine while Core grows.
- Engines under engines/{hakorune,mini} may delegate to Core as it matures.
- Re-exports or aliases can be added to bridge incrementally without broad refactors.

