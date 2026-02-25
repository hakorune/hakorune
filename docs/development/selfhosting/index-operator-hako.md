Index Operator in Hakorune Compiler (selfhost)

Scope
- Bring Hako-side behavior up to parity with Rust parser/MIR builder for Phase‑20.31.

User-facing spec (Phase‑1)
- Read: expr[index] for Array/Map
- Write: expr[index] = value for Array/Map
- String indexing/ranges: out of scope in Phase‑1
- Unsupported receiver: Fail‑Fast with a stable diagnostic

Required changes (Hako compiler)
- Parser
  - Add IndexExpr(target, index)
  - Permit Assign(IndexExpr, value) on LHS
- Lowering (MIR emit)
  - Array: index read/write → BoxCall("get"/"set") on ArrayBox
  - Map:   index read/write → BoxCall("get"/"set") on MapBox
  - Optional (AOT): dotted extern mapping remains as today (nyash.array.get_h, nyash.map.set_hh …)
- Diagnostics
  - If receiver type cannot be resolved to ArrayBox/MapBox, emit: "index operator is only supported for Array/Map"

Smokes (opt‑in, external HAKO_BIN)
- tools/smokes/v2/profiles/quick/core/index_operator_hako.sh
  - Requires HAKO_BIN; skips with WARN when missing
  - Canaries: array read/write, map rw, negative string

Rollout
- No flags are required; follow Rust side semantics.
- Keep Phase‑2 (String/range) for later work.

