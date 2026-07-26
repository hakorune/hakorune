---
Status: active execution task
Date: 2026-07-26
Decision: NORMAL-SOURCE-PLAN0-prime-r1
Row: NORMAL-MAIN0-SOURCE0-S0
Scope: one Program-owned embedded Main.main/0 source unit
ceremony_tier: T1 bounded owner extension inside accepted NORMAL-CANONICAL-CORE0
sunset_id: NORMAL-CANONICAL-CORE0-PROOF-SUNSET-001
proof_inventory_before: sealed Main0 source-family product and closed INPUT0/G0 proof
new_proofs: one disconnected exact-site Main source-unit fixture family
retired_or_merged_proofs: none in SOURCE0
net_proof_delta: one temporary disconnected Main source proof
sunset_budget: one bounded proof family until canonical-core production dispatch
sunset_row: NORMAL-FILE-CANONICAL-CORE0-G0
retire_when: canonical-core dispatcher consumes the Main source unit and disconnected consumer count is zero
budget_repayment_evidence: NORMAL-FILE-CANONICAL-CORE0-G0
Related:
  - docs/development/current/main/investigations/normal-source-plan0-design-stop-2026-07-26.md
  - docs/development/current/main/investigations/normal-source-plan0-input0-s0-execution-task-2026-07-26.md
  - src/mir/compiler/normal_source_plan/
  - src/mir/builder/main_expansion.rs
  - src/mir/compiler/function_input.rs
---

# NORMAL-MAIN0-SOURCE0-S0

## Outcome

Consume the already sealed `ScalarRoot::Main0` product into one embedded
function-source unit while the original Program remains the sole AST owner:

```text
SealedNormalMainSourceV1
  -> prepare_function_source(self)
  -> VerifiedNormalMainFunctionSourceUnitV1
       original PreparedNormalSourcePlanInputV1
       exact Main box site
       exact static main/0 method site
  -> borrow_exact_function()
  -> NormalMainFunctionSourceViewV1<'_>
```

This row is source-only and disconnected. It does not resolve semantics,
classify function completion, open a Builder, lower MIR, create a physical
entry thunk, or publish a module.

## Authority law

The accepted source-family classifier remains the only family classifier.
SOURCE0 consumes its sealed exact sites; it does not scan the Program to decide
whether the source is Script, Main0, or CallableModule.

```text
source-family authority =
  NormalSourcePlanClassifierV1

Main exact-site relation authority =
  VerifiedNormalMainFunctionSourceUnitV1

function completion / return authority =
  later NORMAL-MAIN0-F1-PLAN0-S0
```

`VerifiedMainExpansionV1::from_program()` is a useful historical Raw source
owner, but SOURCE0 must not call it: doing so would independently reclassify
Main and its helpers after `SealedNormalSourcePlanV1` already selected the
family.

## Ownership and navigation

The Main source unit owns the complete original
`PreparedNormalSourcePlanInputV1`. It also owns the exact source sites moved
from `SealedNormalMainSourceV1`.

The only view is borrow-only:

```rust
pub(crate) struct NormalMainFunctionSourceViewV1<'src> {
    function: &'src ASTNode,
    main_statement_index: usize,
    method_key: &'src str,
}
```

The production surface must not expose:

```text
into_ast
program_mut
function_mut
clone_program
clone_function
rewrite
```

The exact-site validator checks, without changing ownership:

```text
root remains Program
statement index resolves
statement remains static box Main
method key remains main
method declaration name remains main
method remains static
arity remains zero
sealed site facts equal observed exact-site facts
```

This is relation verification, not source-family reclassification.

## Product and rejection

Suggested owner vocabulary:

```rust
pub(crate) struct VerifiedNormalMainFunctionSourceUnitV1 {
    input: PreparedNormalSourcePlanInputV1,
    main_box: NormalTopLevelSiteV1,
    main_method: NormalMainMethodSiteV1,
    _seal: VerifiedNormalMainFunctionSourceUnitSealV1,
}

pub(crate) struct RejectedNormalMainFunctionSourceV1 {
    owner: SealedNormalMainSourceV1,
    error: NormalMainFunctionSourceErrorV1,
}
```

Typed errors:

```text
RootNotProgram
MainStatementMissing
MainStatementDrift
MainMethodMissing
MainMethodShapeDrift
MainMethodNameDrift
MainMethodStaticDrift
MainMethodArityDrift
```

The rejection exposes only:

```text
error()
discard(self)
```

No owner extraction, retry, alternate site, Raw expansion, or AST repair is
allowed.

## File boundary

Keep this source owner beside the classifier that issued its exact sites:

```text
src/mir/compiler/normal_source_plan/
  main_source.rs
  main_source_tests.rs
```

`product.rs` may add one consuming delegation and the minimum site accessors
needed by `main_source.rs`. It must not return loose AST/site parts.

Do not put this implementation in `compiler/mod.rs`,
`normal_file_vm_frontdoor.rs`, or `builder/main_expansion.rs`.

Every modified/new source or check file remains below 800 lines.

## Fixtures

Use direct AST fixtures so this row tests exact-site ownership rather than a
second parser path:

```text
static Main.main/0
  -> one verified embedded function view

Main site with unrelated top-level source retained
  -> original Program remains owned

main body / annotation / explicit Return
  -> source view preserves syntax without classifying it

tampered Main statement index
  -> retained typed rejection

tampered method key/name/static/arity relation
  -> retained typed rejection

source unit borrowed repeatedly
  -> same exact AST identity; no clone or move escape
```

Tests may use private fixture constructors to create drift that the public
classifier cannot emit. Production constructors remain sealed.

## Guard

Extend the existing manifest-backed family guard:

```text
tools/checks/run_row_guard.sh --only normal-source-plan0
```

Freeze:

```text
Main source-unit producer              = 1
exact-site borrow terminal             = 1
source-family reclassification         = 0
VerifiedMainExpansion re-entry         = 0
AST clone/rewrite                      = 0
Builder/MIR/backend/runner reference   = 0
production consumer                    = 0
all source/check files                 < 800 lines
```

Do not create a new row-specific shell or manifest row.

## Acceptance

```bash
cargo check --lib
cargo test -q --lib mir::compiler::normal_source_plan
tools/checks/run_row_guard.sh --only normal-source-plan0
python3 tools/checks/lib/normal_file_vm0_frontdoor_forge_guard.py
bash tools/checks/mir_root_facade_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Immediate continuation

```text
NORMAL-MAIN0-SOURCE0-S0
-> NORMAL-MAIN0-F1-PLAN0-S0
```

F1-PLAN0 is where the embedded Main view must enter the existing function
semantic/completion owners. If evidence proves that the original Program
cannot remain owned while entering that projection, stop at the accepted
reconsult blocker instead of cloning or rewriting the function AST.

## Non-claims

```text
semantic resolution
function completion or return acceptance
Main lowering
helper catalog or call graph
physical main thunk
module transaction/publication
VM execution/process projection
profile admission or dispatch
new CLI/default caller
existing narrow-route behavior change
```
