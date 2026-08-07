# Loop recipe physicalizer module split R0

Status: `landed behavior-neutral module split 2026-08-07; physical emission remains closed`
Date: 2026-08-07
Parent: `LOOP-RECIPE-OPERATION-PHYSICALIZER-DESIGN-STOP / Decision B`
Authority:
`docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md`

## Change

Perform one behavior-neutral BoxShape split of the test-only topology
physicalizer. Replace the flat module with one directory facade:

```text
src/mir/builder/resolved_lowering/loop_recipe_physicalizer/
  mod.rs       # stable parent facade and re-exports
  topology.rs  # existing recursive topology/After implementation
  tests.rs     # existing topology canary tests
```

The parent `resolved_lowering::loop_recipe_physicalizer` entry and every
`pub(super)` item used by the existing canary remain source-compatible. The
old flat `loop_recipe_physicalizer.rs` file is deleted. This row does not add
an accepted operation shape or a second physical owner.

## Non-claims

```text
no operation demand change
no physical block receipt
no operation emitter
no operation_state module
no BindingSSA/PHI owner
no function session / Completion / DraftSeal
no production selector / retry / fallback
no legacy deletion
```

The next semantic row is the separate physical block receipt. Operation leaf
emission remains closed until that receipt is sealed.

## Acceptance

- [x] The flat module is replaced by one directory facade.
- [x] Topology and focused tests are the only two implementation modules.
- [x] Existing nested Generic G0 topology behavior is unchanged.
- [x] Incomplete entry is still rejected before block allocation.
- [x] No new Builder/MIR/CFG/SSA/PHI authority is introduced.
- [x] Every touched source/test file stays below 800 lines.
- [x] Current state, workstream, code README, and reference receipt are
      updated in the same commit.

Focused gates:

```text
RUSTFLAGS='-Awarnings' cargo test --lib loop_recipe_physicalizer -- --nocapture --test-threads=1
RUSTFLAGS='-Awarnings' cargo check --lib
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
git diff --check
```

## Stop

Return to the physicalizer design stop if the split requires changing the
public-in-crate API, moving operation semantics into the topology module,
adding a test-only extraction route, or introducing a new CFG/SSA/PHI owner.
