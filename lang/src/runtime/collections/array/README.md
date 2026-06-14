# Array Visible Semantics

This folder is the `.hako` owner for user-visible `ArrayBox` policy.

It is intentionally split from the existing VM-facing `../array_core_box.hako`
wrapper:

- `visible_policy_box.hako`
  - method names, aliases, arity, slot vocabulary, effect classification, and
    return tags
- `substrate_bridge_box.hako`
  - narrow mechanical calls into substrate-owned raw array storage
- `core_box.hako`
  - visible owner facade over policy plus substrate bridge

## Responsibility

- Own the visible `ArrayBox` method vocabulary:
  `length`, `size`, `len`, `get`, `set`, `push`, `pop`, `clear`,
  `contains`, `indexOf`, `join`, `sort`, `reverse`, `slice`, `remove`, and
  `insert`.
- Keep alias, arity, slot, effect, return, bounds, and empty-pop policy
  readable from `.hako`.
- Keep raw array storage, allocation, capacity, bounds verifier substrate, and
  ABI transport in substrate.

## Non-Responsibility

- No array storage/layout/cache ownership.
- No raw handle transport ownership.
- No VM dispatch cutover in the contract row.
- No sort / reverse / join algorithm cutover in this row.

## Current Slice

`ARRAY-VISIBLE-CONTRACT-001` lands the modular owner and policy vocabulary. It
does not replace the Rust `ArrayBox` method bodies or the existing VM-facing
`array_core_box.hako` wrapper.
