# 3220 - MIRBUILDER-PROGRAMJSON-RECIPEBODIES-VERIFIER-BOUNDARY-EXPANDED-DTO-COVERAGE-PARITY-001

Status: active

## Scope

Expand ProgramJSON RecipeBodies verifier-boundary DTO coverage while reusing:

```text
ProgramJsonRecipeBodiesVerifierBoundarySnapshotBox
```

This adds more parity rows for the already selected post-verifier path. It does
not open runtime `RecipeBodies` publication, full RecipeMatcher execution,
route selection, lowering, mutation, ID allocation, or runtime route switching.

## Task Breakdown

```text
1. Reuse ProgramJsonRecipeBodiesVerifierBoundarySnapshotBox.
   - no new RecipeBodies builder owner
   - no new verifier owner
   - no runtime publication path

2. Add expanded DTO coverage rows.
   - cover local + loop + if-return + assignment + final return
   - vary local names and literal values to avoid one-shape string matching
   - keep expected summary as canonical verifier-boundary snapshot fields

3. Prove the rows through the same executable path.
   - generate a small `.hako` probe from the fixture
   - call build_summary once per row
   - compare runtime output field-by-field against the Rust-oracle summary

4. Keep the next step narrow.
   - if green, open only a scoped Rust ASTNode projector retire-candidate
   - do not proceed into runtime RecipeBodies publication without a new
     decision card
```

Rows:

```text
local_loop_body_if_branch_return
local_loop_body_if_branch_return_alt_names
```

## Stop Conditions

```text
STOP if the expanded rows require a new backend route.
STOP if the snapshot owner needs full RecipeMatcher execution.
STOP if ProgramJSON traversal is bypassed by precomputed token strings.
STOP if unsupported route rows appear in the snapshot MIR metadata.
STOP if any claim other than expanded DTO coverage parity becomes necessary.
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_verifier_boundary_expanded_dto_coverage_parity_gate.sh
```

Expected result:

```text
row_count=2
expanded_dto_coverage_rows=2
runtime_recipe_bodies_publication=0
full_recipe_matcher_execution=0
source_selfhost_claim=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-VERIFIER-BOUNDARY-EXPANDED-DTO-COVERAGE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```

After that, select the next contract explicitly. The default next decision is
whether to add more verifier-boundary DTO rows or stop for runtime
RecipeBodies publication / RecipeMatcher consultation.
