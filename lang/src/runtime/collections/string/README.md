# String Visible Semantics

This folder is the `.hako` owner for user-visible `StringBox` policy.

It is intentionally split from the VM-facing `../string_core_box.hako`
wrapper:

- `visible_policy_box.hako`
  - method names, aliases, arity, slot vocabulary, read classification, and
    return tags
- `substrate_bridge_box.hako`
  - narrow mechanical calls into substrate-owned string storage / VM wrapper
- `core_box.hako`
  - visible owner facade over policy plus substrate bridge

## Responsibility

- Own the visible `StringBox` method vocabulary:
  `length`, `len`, `size`, `substring`, `substr`, `concat`, `indexOf`,
  `find`, `replace`, `trim`, `toUpper`, `toUpperCase`, `toLower`,
  `toLowerCase`, `lastIndexOf`, `contains`, and `startsWith`.
- Keep alias, arity, slot, effect, and return policy readable from `.hako`.
- Keep the byte/codepoint index mode and raw string storage mechanics in
  substrate until a dedicated string-kernel row moves that policy.

## Non-Responsibility

- No `String` / UTF-8 byte storage ownership.
- No VM dispatch cutover in the inventory row.
- No executable plugin function pointers.
- No string-kernel lowering or corridor optimization ownership.

## Current Slice

`STRING-VISIBLE-INVENTORY-001` lands the modular owner and policy vocabulary.
It does not replace the Rust `StringBox` method bodies or the existing
VM-facing `string_core_box.hako` wrapper.
