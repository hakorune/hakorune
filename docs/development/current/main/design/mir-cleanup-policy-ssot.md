---
Status: SSOT
Decision: accepted
Date: 2026-06-13
Scope: MIR structural cleanup policy for CURRENT-CLEAN / MIR-CLEAN tasks.
Related:
  - docs/development/current/main/investigations/mir-cleanup-inventory-2026-06-13.md
  - docs/development/current/main/design/compiler-expressivity-first-policy.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/development/current/main/DOCS_LAYOUT.md
---

# MIR Cleanup Policy SSOT

## Decision

MIR cleanup work is BoxShape-only unless a separate accepted BoxCount card says
otherwise.

```text
allowed:
  file/module organization
  README / SSOT entry maps
  test module splits
  import facade cleanup
  compat/legacy classification

forbidden:
  accepted source/MIR shape changes
  optimizer behavior changes
  route selection changes
  new silent fallback
  perf keeper claims
```

Cleanup should make the compiler easier to navigate without changing what it
accepts, plans, lowers, verifies, or optimizes.

## Series Rule

One cleanup series has one purpose.

```text
good:
  split one large test module
  classify thin mod.rs files
  document one control_flow boundary
  flatten one deep subtree behind a facade

bad:
  split tests + delete compat + adjust planner behavior
  flatten facts and plan while changing acceptance rules
  move files and update gates to accept new shapes
```

When a structural refactor needs more than one commit, use Refactor Series
Mode:

```text
all commits build
series purpose is named
behavior changes are forbidden
new acceptance fixtures are not added
compat facades are kept until consumers are migrated
```

## Gate Rule

Each cleanup card must name the smallest useful gate before editing.

Default gates:

```text
docs-only:
  bash tools/checks/current_state_pointer_guard.sh

test module split:
  cargo test --release --lib <module-filter> -- --nocapture
  cargo fmt --check

module/import move:
  cargo test --release --lib <affected-filter> -- --nocapture
  cargo fmt --check

deep path flatten:
  targeted unit tests for moved subtree
  current_state_pointer_guard when docs are touched
  cargo fmt --check
```

Do not use a broad green check to justify a cleanup whose affected seam was not
specifically exercised.

## Deep Flatten Rule

Depth reduction is allowed. It must reduce cognitive load without mixing
responsibilities.

```text
allowed:
  move a single semantic subtree into a named box
  keep facts / plan / lower / verify roles visible inside that box
  add a README before or with the move
  keep compatibility re-exports while migrating imports

forbidden:
  collapse facts and plan truth into one owner
  re-export policy across layers to make imports compile
  move multiple unrelated deep paths at once
  delete mod.rs files mechanically
```

Preferred shape for the first pilot:

```text
step_placement/
  README.md
  facts.rs
  matcher.rs
  decision.rs
  plan.rs
```

The exact file names may differ, but the boundary must stay explicit.

## Thin mod.rs Rule

Thin `mod.rs` files are not automatically bad.

Classification must precede deletion:

```text
pure_reexport:
  may be collapsed if imports stay local and readable

boundary_keep:
  keep when the file documents a layer or hides submodules

test_group_keep:
  keep when it preserves cargo test filters or fixture taxonomy
```

## Compat / Legacy Rule

Filename-based compat/legacy candidates may be classified as:

```text
keep:
  still active boundary or compatibility adapter

quarantine:
  still needed, but should be isolated and documented

retire:
  removable after named tests/gates prove no consumers remain
```

Text hits for `compat|legacy` are inventory signals only. They are not deletion
candidates without local inspection.

## Task Ladder

### CURRENT-CLEAN-001

Add this cleanup policy.

Status: landed 2026-06-13.

Acceptance:

```text
BoxShape-only cleanup
no new accepted source/MIR shape
no optimizer behavior change
no perf keeper claim
one purpose per cleanup series
minimum gate per series is named
```

### MIR-CLEAN-001

Split one large test file.

Acceptance:

```text
test module split only
no production behavior change
targeted cargo test is green
cargo fmt --check is green
```

### MIR-CLEAN-002

Classify thin `mod.rs` files.

Acceptance:

```text
thin_mod_pure_reexport_count recorded
thin_mod_boundary_keep_count recorded
thin_mod_test_group_keep_count recorded
collapse candidates listed
no deletion performed
```

### MIR-CLEAN-003

Classify compat / legacy candidates.

Acceptance:

```text
filename candidates classified as keep/quarantine/retire
text hits are not treated as deletion candidates
retire candidates name their gates
```

### MIR-CLEAN-004

Document `builder/control_flow` entry map.

Acceptance:

```text
facts / plan / lower / verify / joinir responsibilities documented
deep flatten pilot seam selected
forbidden cross-layer dependencies listed
```

### MIR-CLEAN-005

Run one deep path flatten pilot.

Acceptance:

```text
one deep subtree only
facade or compatibility imports preserved
no acceptance shape added
targeted planner/facts tests green
```

## Final Rule

```text
Cleanup changes structure.
BoxCount changes acceptance.
Do not mix them.
```
