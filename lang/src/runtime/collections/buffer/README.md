# Buffer Visible Semantics

This folder is the `.hako` owner for user-visible `BufferBox` policy.

It is intentionally split from the first slice:

- `visible_policy_box.hako`
  - method names, aliases, arity, mutation/read classification, and return
    vocabulary
- `substrate_bridge_box.hako`
  - narrow mechanical calls into substrate-owned byte storage
- `numeric_le_policy_box.hako`
  - typed numeric read/write little-endian and bounds policy
- `core_box.hako`
  - visible owner facade over policy plus substrate bridge

## Responsibility

- Own the visible `BufferBox` method vocabulary:
  `write`, `read`, `readAll`, `clear`, `length`, `len`, `size`, `append`,
  `slice`.
- Keep alias and arity policy out of Rust-side dispatch tables.
- Keep return / mutation classification readable from `.hako`.
- Own typed numeric read/write policy: little-endian, width, bounds, and
  `readU64` i64-return overflow behavior.
- Keep byte storage, allocation, locking, and raw buffer representation in
  substrate.

## Non-Responsibility

- No `Vec<u8>` / `RwLock` / `Arc` mechanics.
- No allocator or raw memory ownership.
- No executable plugin function pointers.
- No VM dispatch cutover in the inventory row.

## Current Slice

The landed Buffer pilot covers:

- `BUFFER-VISIBLE-INVENTORY-001`
  - modular visible method / alias / policy inventory
- `BUFFER-VISIBLE-CONTRACT-002`
  - fixture-backed `hako_check collection-visible-contract` report fields
- `BUFFER-HAKO-CORE-003`
  - first `.hako` visible owner facade
- `BUFFER-NUMERIC-LE-004`
  - typed little-endian numeric policy

It does not yet replace the Rust `BufferBox` method bodies. The first
executable VM dispatch cutover must happen in a later row.
