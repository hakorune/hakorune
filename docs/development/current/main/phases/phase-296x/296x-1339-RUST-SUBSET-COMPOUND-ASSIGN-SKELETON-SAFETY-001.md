# 296x-1339 RUST-SUBSET-COMPOUND-ASSIGN-SKELETON-SAFETY-001

Status: closed
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

## Result

```text
focused_fixture_added=1
adapter_compound_assign_handoff_updated=1
generated_skeleton_mir_safe_for_compound_assign=1
compound_assignment_runtime_semantics=0
rust_name_resolution_enabled=0
new_hako_syntax_added=0
generated_program_execution_claim=0
summary=ok
```

Implementation:

```text
apps/rust-subset-to-hako/tools/syn_adapter/src/stmts.rs
```

Compound assignment now becomes a statement-level `Unsupported` handoff instead
of an expression-level `Binary(op=unsupported_op)` node. The converter core
remains unchanged and does not claim executable `+=` semantics.

Focused fixtures:

```text
apps/rust-subset-to-hako/examples/compound_assign_input.rs
apps/rust-subset-to-hako/examples/compound_assign_subset.json
apps/rust-subset-to-hako/examples/compound_assign_expected.hako
apps/rust-subset-to-hako/convert_compound_assign_fixture.hako
```

Verification:

```text
python_reference_compound_assign_fixture=green
syn_adapter_compound_assign_fixture=green
hako_converter_compound_assign_fixture_exe_parity=green
RUST_SUBSET_RUN_ADAPTER=1 apps/rust-subset-to-hako/smoke.sh=green
cargo_check_lib=green
current_state_pointer_guard=green
git_diff_check=green
```

Selected ID-module re-probe:

```text
unsupported_op_skeleton_statement=cleared
next_failure=Unresolved function: 'Self_new'
next_blocker=RUST-SUBSET-SELF-QUALIFIED-CALL-SKELETON-SAFETY-001
```

## Next

After this skeleton-safety row is closed, resume:

```text
HAKORUNE-MIR-CORE-ID-MODULES-RUSTSUBSET-PILOT-001
```

The selected ID-module pilot is still blocked, but by the next skeleton source
shape:

```hako
function BasicBlockIdGenerator_default(): Self {
    return Self_new()
}
```
