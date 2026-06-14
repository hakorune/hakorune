# Map Visible Semantics

This folder is the `.hako` owner for user-visible `MapBox` policy.

It is intentionally split from the existing VM-facing `../map_core_box.hako`
wrapper:

- `visible_policy_box.hako`
  - method names, aliases, arity, slot vocabulary, effect classification, and
    return tags
- `substrate_bridge_box.hako`
  - narrow mechanical calls into substrate-owned raw map storage
- `core_box.hako`
  - visible owner facade over policy plus substrate bridge

## Responsibility

- Own the visible `MapBox` method vocabulary:
  `size`, `length`, `len`, `has`, `get`, `set`, `delete`, `remove`, `keys`,
  `values`, and `clear`.
- Keep alias, arity, slot, effect, return, key-normalization, and missing-key
  policy readable from `.hako`.
- Keep raw map storage, hashing, capacity, and ABI transport in substrate.

## Non-Responsibility

- No hash-table mechanics.
- No raw key/value handle transport ownership.
- No VM dispatch cutover in the contract row.
- No iterator implementation ownership.

## Current Slice

`MAP-VISIBLE-CONTRACT-001` lands the modular owner and policy vocabulary. It
does not replace the Rust `MapBox` method bodies or the existing VM-facing
`map_core_box.hako` wrapper.
