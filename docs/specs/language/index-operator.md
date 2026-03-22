Index Operator — Design (Phase‑20.31 scoped)

Scope (Phase‑1)
- Support bracket indexing for Array/Map:
  - Read: expr[index]
  - Write: expr[index] = value
- Semantics (lowering):
  - Array
    - Read → nyash.array.get_h(handle, idx)
    - Write → nyash.array.set_hih(handle, idx, val_any)
  - Map
    - Read → nyash.map.get_hh(handle, key_any)
    - Write → nyash.map.set_hh(handle, key_any, val_any)
- Out of scope in Phase‑1: String indexing/ranges.

Parser/AST
- Add IndexExpr(target, index)
- Permit Assign(IndexExpr, value) on the LHS.

MIR Lowering (Fail‑Fast contract)
- If receiver is neither Array nor Map → error: "index operator is only supported for Array/Map".
- For Map keys/values:
  - If arguments are SSA values (handles), pass as is; otherwise wrap primitive i64 as IntegerBox via available builder utilities.

Error Policy
- Fail‑Fast, no silent fallback.
- Stable diagnostics for unsupported types and malformed LHS.

Rollout / Flags
- Implement in Rust parser first; Ny parser later.
- If needed, guard with a dev flag for initial rollout (HAKO_INDEX_OPERATOR_DEV=1), default ON in dev profile, OFF in prod profiles.

Tests (canaries)
- arr_read: [1,2][0] == 1 → true
- arr_write: arr[1]=3; arr[1]==3 → true
- map_rw: m[10]=7; m[10]==7 → true
- negative: "hello"[0] (Phase‑1) → error

Notes
- This leverages existing NyRT dotted externs already implemented for Array/Map, minimizing surface area and risk.
