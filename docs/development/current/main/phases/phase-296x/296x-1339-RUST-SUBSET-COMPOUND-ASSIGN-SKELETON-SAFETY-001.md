# 296x-1339 RUST-SUBSET-COMPOUND-ASSIGN-SKELETON-SAFETY-001

Status: open
Date: 2026-06-20

## Purpose

Make Rust compound assignment expressions skeleton-safe in generated `.hako`
output.

After 296x-1338 cleared tuple-struct constructors, the selected
`hakorune_mir_core` ID-module probe advanced to the next generated skeleton
failure:

```hako
function BasicBlockIdGenerator_next(receiver: BasicBlockIdGenerator): BasicBlockId {
    local id: Unknown = null /* TODO: tuple struct constructor expression is out of v0 skeleton scope: BasicBlockId */
    receiver.next_id unsupported_op 1
    return id
}
```

MIR emit fails with:

```text
Undefined variable: unsupported_op
```

## Diagnosis

This is a RustSubset skeleton source-shape blocker. The adapter currently
represents Rust compound assignment such as:

```rust
self.next_id += 1;
```

as a binary expression with `op="unsupported_op"` instead of a statement-level
Unsupported handoff or a valid assignment skeleton.

## Scope

Allowed:

```text
focused_fixture_added=1
adapter_compound_assign_handoff_updated=1
generated_skeleton_mir_safe=1
```

Not allowed:

```text
compound_assignment_runtime_semantics_claim=0
rust_name_resolution_enabled=0
new_hako_syntax_added=0
generated_program_execution_claim=0
```

The output only needs to be skeleton-safe. It does not need to preserve
executable Rust compound-assignment semantics.

## Acceptance

Add a focused fixture containing at least:

```rust
pub struct Counter {
    next_id: u32,
}

impl Counter {
    pub fn next(&mut self) {
        self.next_id += 1;
    }
}
```

Verify:

```bash
python3 apps/rust-subset-to-hako/convert.py \
  apps/rust-subset-to-hako/examples/compound_assign_subset.json \
  -o /tmp/compound_assign_py.hako

NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-exe \
  /tmp/convert_compound_assign_fixture \
  apps/rust-subset-to-hako/convert_compound_assign_fixture.hako
```

Then re-run the selected ID-module probe:

```bash
NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune --emit-mir-json \
  /tmp/hakorune_mir_core_id_modules_generated.mir.json \
  <generated-id-module-skeleton.hako>
```

General checks:

```bash
cargo check -q --lib
RUST_SUBSET_RUN_ADAPTER=1 bash apps/rust-subset-to-hako/smoke.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
hakorune_mir_core_id_bundle_checked_in=0
compound_assignment_runtime_semantics=0
rust_name_resolution_enabled=0
new_hako_syntax_added=0
generated_program_execution_claim=0
```

## Next

After this skeleton-safety row is closed, resume:

```text
HAKORUNE-MIR-CORE-ID-MODULES-RUSTSUBSET-PILOT-001
```
