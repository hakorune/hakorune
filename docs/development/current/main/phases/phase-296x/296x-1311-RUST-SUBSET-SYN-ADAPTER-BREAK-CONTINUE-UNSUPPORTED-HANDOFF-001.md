# 296x-1311 RUST-SUBSET-SYN-ADAPTER-BREAK-CONTINUE-UNSUPPORTED-HANDOFF-001

Status: closed
Date: 2026-06-19

## Purpose

Stabilize Rust loop bodies containing `break` / `continue` as an explicit
Unsupported handoff in rust-subset-to-hako.

This keeps the app-front transport honest while compiler Recipe/CorePlan loop
acceptance remains the owner for real `break` / `continue` semantics.

## Accepted Handoff

Input:

```rust
loop {
    if n == 0 {
        break;
    }
    n = n - 1;
    continue;
}
```

RustSubset:

```json
{
  "kind": "Unsupported",
  "reason": "loop with break/continue belongs to compiler Recipe/CorePlan backlog"
}
```

Emitted `.hako` skeleton:

```hako
/* TODO: loop with break/continue belongs to compiler Recipe/CorePlan backlog */
```

## Implementation

Added fixture files:

```text
apps/rust-subset-to-hako/examples/break_continue_input.rs
apps/rust-subset-to-hako/examples/break_continue_subset.json
apps/rust-subset-to-hako/examples/break_continue_expected.hako
apps/rust-subset-to-hako/convert_break_continue_fixture.hako
```

Updated:

```text
apps/rust-subset-to-hako/selftest.py
apps/rust-subset-to-hako/smoke.sh
apps/rust-subset-to-hako/README.md
apps/rust-subset-to-hako/STATUS.md
```

## Evidence

```bash
python3 apps/rust-subset-to-hako/selftest.py
cargo check -q --manifest-path apps/rust-subset-to-hako/tools/syn_adapter/Cargo.toml
cargo check -q --lib
bash apps/rust-subset-to-hako/smoke.sh
bash apps/rust-subset-to-hako/smoke_adapter.sh
```

Observed result:

```text
summary=ok
```

## Boundary

```text
break_continue_semantics_enabled=0
compiler_recipe_acceptance_changed=0
while_desugar_changed=0
converter_core_input_route_changed=0
vm_product_route=retired
```

## Next

Continue app-front source-shape selection. Reopen compiler Recipe/CorePlan only
when a fixture exposes a real compiler acceptance blocker.

```text
next_blocker=RUST-SUBSET-NEXT-APP-FRONT-TASK-SELECTION-001
```
