---
Status: Landed
Date: 2026-04-28
Scope: make once poison storage a two-sided parser emission contract
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/parser/declarations/box_def/members/property_emit.rs
  - src/tests/parser_unified_members_property_emit.rs
---

# 291x-648: Once Poison Write

## Goal

Close the stale half-contract in synthetic `once` getter emission.

This is BoxShape cleanup for the existing `once` failure contract. It does not
change `once` syntax, computed properties, or `birth_once` lowering.

## Evidence

The synthetic `__get_once_<name>` getter checked:

```text
__once_poison_<name>
```

and threw when it was present, but no emitted path ever wrote that slot. That
made the poison read path dead surface and left the documented "failed once
stays failed" behavior only partially represented in the AST.

## Decision

Wrap the compute/cache path in a synthetic `TryCatch`:

```text
try:
  value = me.__compute_once_<name>()
  me.setField("__once_<name>", value)
  return value
catch:
  me.setField("__once_poison_<name>", "once '<name>' previously failed")
  throw "once '<name>' previously failed"
```

Later reads rethrow the stored poison message.

## Boundaries

- Do not change `once` surface syntax.
- Do not alter the cache-hit path.
- Do not attempt exact original exception preservation in this card; the current
  MIR catch path does not expose catch binding as a local value.
- Do not touch `birth_once` prologue emission.

## Acceptance

```bash
cargo fmt
cargo test parser_unified_members_property_emit --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- `once` getter emission now writes the poison slot when compute fails.
- Parser AST coverage asserts both header-first and block-first `once` getters
  include the poison write/throw handler.
