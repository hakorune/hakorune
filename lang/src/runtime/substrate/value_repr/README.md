# hako.value_repr — Current-Lane Value Representation Helpers

Responsibility:
- Keep narrow runtime value-lane predicates in one substrate-owned place.
- Provide helper predicates used by low-level capability facades before exact
  unsigned storage is live.

Current live row:
- `CurrentLaneBox.is_usize_i64(value)` returns `1` when `value` is representable
  by the current non-negative `Integer(i64)` subset used for provisional
  `usize` facades, otherwise `0`.

Non-responsibility:
- Do not implement exact pointer-sized unsigned storage here.
- Do not implement wrapping or checked arithmetic here.
- Do not own allocator, OSVM, RawBuf, or ABI policy.

