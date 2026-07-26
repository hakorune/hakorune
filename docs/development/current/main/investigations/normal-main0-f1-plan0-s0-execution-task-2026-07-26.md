---
Status: active execution task
Date: 2026-07-26
Decision: NORMAL-SOURCE-PLAN0-prime-r1
Row: NORMAL-MAIN0-F1-PLAN0-S0
Scope: one Program-owned embedded resolved Main.main/0 function plan
ceremony_tier: T1 bounded owner extension inside accepted NORMAL-CANONICAL-CORE0
series_mode: BoxShape only; 2-5 buildable commits; no new accepted source shape
sunset_id: NORMAL-CANONICAL-CORE0-PROOF-SUNSET-001
proof_inventory_before: closed SOURCE0 exact-site source owner
new_proofs: one disconnected Main semantic/completion/F1 plan family
retired_or_merged_proofs: none in F1-PLAN0
net_proof_delta: one temporary disconnected Main F1 proof
sunset_row: NORMAL-FILE-CANONICAL-CORE0-G0
retire_when: sole canonical-core dispatcher consumes the Main F1 plan and disconnected consumer count is zero
Related:
  - docs/development/current/main/investigations/normal-source-plan0-design-stop-2026-07-26.md
  - docs/development/current/main/investigations/normal-main0-source0-s0-execution-task-2026-07-26.md
  - docs/reference/language/function-exit-and-entry-result.md
  - src/mir/compiler/normal_source_plan/
  - src/mir/resolved_semantics/
  - src/mir/compiler/capability.rs
---

# NORMAL-MAIN0-F1-PLAN0-S0

## Outcome

Resolve the already verified embedded `Main.main/0` function without extracting
or cloning it from its original Program, then consume the existing function
completion and F1 authorities:

```text
VerifiedNormalMainFunctionSourceUnitV1
  -> prepare_embedded_resolved_main(self)
  -> VerifiedNormalMainResolvedSourceUnitV1
       owns the complete Program-backed Main source unit
       owns one semantic owner forest
       owns one exact source projection
       owns one Main function role
  -> borrow_function_input()
  -> ResolvedFunctionLoweringInputV1
  -> NormalMainFunctionPreflightV1::seal()
  -> VerifiedNormalMainFunctionPlanV1
       CanonicalTrivialBindingSsaPlanV1
       VerifiedFunctionCompletionV1
       SealedFunctionExitContractV1
       exact Main role evidence
```

This row remains disconnected. It creates no function draft, physical thunk,
module transaction, publication, VM execution, profile admission, or runner
consumer.

## Existing borrow-based seam

Do not call:

```rust
VerifiedResolvedSourceUnitV1::resolve_function(ASTNode)
```

That API consumes a standalone function AST and would require extracting or
cloning the embedded Main method.

The accepted seam is the existing borrow-based primitive chain:

```text
NormalMainFunctionSourceViewV1::function()
  -> FunctionSyntaxViewV1::from_ast(&function)
  -> FunctionSemanticResolverSessionV1::resolve_forest(view)
  -> VerifiedSourceProjectionV1::seal(&function, &forest)
  -> FunctionSourceViewV1::from_exact_parts(
       &function,
       root_owner,
       &forest,
       &projection,
     )
```

The new owner aggregates these products. It does not duplicate the semantic
resolver, source navigator, owner issuer, completion verifier, or value-profile
analyzer.

Because `VerifiedSourceProjectionV1` stores source paths rather than source
references, the resolved owner can retain the complete Program-backed source
unit and recreate borrow-only function views safely. Self-referential pointers
and unsafe lifetime extension are forbidden.

## Owner vocabulary

Conceptual minimum:

```rust
pub(crate) struct VerifiedNormalMainResolvedSourceUnitV1 {
    source: VerifiedNormalMainFunctionSourceUnitV1,
    forest: VerifiedSemanticOwnerForestV1,
    projection: VerifiedSourceProjectionV1,
    role: VerifiedNormalMainRoleV1,
    _seal: VerifiedNormalMainResolvedSourceUnitSealV1,
}

pub(crate) struct VerifiedNormalMainFunctionPlanV1<'unit> {
    unit: &'unit VerifiedNormalMainResolvedSourceUnitV1,
    lowering: CanonicalTrivialBindingSsaPlanV1<'unit>,
    role: VerifiedNormalMainRoleV1,
    _seal: VerifiedNormalMainFunctionPlanSealV1,
}
```

The exact physical representation may vary to satisfy Rust borrowing, but these
laws do not:

```text
Program source owner                         = 1
semantic resolver session                    = 1
semantic owner forest                        = 1
source projection                            = 1
Main role                                    = 1
AST clone/extraction                         = 0
second source-family classification          = 0
```

No public or crate-wide constructor may accept independently supplied source,
forest, projection, owner, or Main role facts.

## Main role policy

`CanonicalLoweringPreflightV1` currently rejects `name == "main"` as an
ordinary first-family function. Do not delete that fence or silently widen the
ordinary callable profile.

Add one explicit role-scoped entry:

```text
NormalMainFunctionPreflightV1
  -> shared canonical body/completion/value-profile implementation
  -> Main-specific header policy
```

The shared verifier remains sole authority for body shape, located source,
completion, resolved control, Binding SSA profile, and result representation.
Only the header/role policy differs.

First Main role:

```text
name              = main
static            = true
arity             = 0
receiver          = absent
capture/Lambda    = absent
uses/contracts    = empty
attrs/override    = absent
direct calls      = 0
```

The ordinary `CanonicalLoweringPreflightV1::verify_function()` behavior and
its rejection of `main` remain unchanged.

## F1 completion and result matrix

Admit only the already accepted exact 0-or-1 terminal-root slice:

| Source disposition | Expected F1 contract |
| --- | --- |
| empty Main body | `ImplicitUnit(EmptyBody)` |
| non-empty fallthrough | `ImplicitUnit(ImplicitFallthrough)` |
| final expression statement | evaluated/discarded, then `ImplicitUnit` |
| `return;` | `ExplicitUnit(BareReturn)` |
| `return void` | `ExplicitUnit(ExplicitVoid)` |
| `return null` | `ExplicitUnit(ExplicitNull)` |
| `return Integer` | `ExplicitValue` |
| `return Bool` | `ExplicitValue` |
| `return Float` | `ExplicitValue` |

Declared result:

```text
unannotated + Unit/value     = admit when physical carrier is supported
: void + Unit/exact Void     = admit
: void + non-Unit literal    = typed contract rejection
: i64 + exact Integer        = admit through existing ReturnExitContract
non-Void annotation + fallthrough/Unit = typed missing-return rejection
other annotation/carrier     = typed capability rejection
```

This row does not add String, Box, Array, Future, WeakRef, object, dynamic, or
heterogeneous result carriers.

## Explicit exclusions

Reject before Builder effects:

```text
multiple Return
nested/all-path Return
non-terminal Return
cleanup-bearing Return
direct call
instance Main / receiver
parameters
capture / Lambda / Outbox
String function result
owner-bearing or dynamic result
unsupported annotation
```

The rejection must preserve the complete Program-owned resolved Main unit plus
typed stage/cause. It exposes inspection and `discard(self)` only; no retry,
fallback, alternate role, or legacy entry is allowed.

## Internal commit order

This row is one BoxShape series:

```text
F1-A:
  Program-owned embedded resolved source owner
  borrow-only forest/projection assembly

F1-B:
  explicit VerifiedNormalMainRoleV1
  shared preflight entry without ordinary-profile widening

F1-C:
  completion/result matrix
  one VerifiedNormalMainFunctionPlanV1

F1-D:
  rejection retention, reuse fixtures, structural guard
```

Each commit is buildable. No accepted source shape is added outside the
accepted Main capability matrix.

## File boundary

Prefer bounded files beside the source-plan owner:

```text
src/mir/compiler/normal_source_plan/
  main_resolved_source.rs
  main_function_plan.rs
  main_function_plan_tests.rs
```

Small reusable role-policy plumbing may live beside `capability.rs`, but do not
grow `compiler/mod.rs`, `capability.rs`, or `lowering_input.rs` past 800 lines.

Do not place Main semantic policy in:

```text
runner
front door
Raw source facts
VM execution
process projection
```

## Required fixtures

```text
success:
  empty
  non-empty fallthrough
  final expression remains Unit
  return;
  return void
  return null
  return Integer
  return Bool
  return Float
  :void + Unit
  :i64 + Integer

typed rejection:
  :void + Integer
  :i64 + fallthrough
  unsupported annotation
  multiple/nested/non-terminal Return
  direct call
  parameter/receiver/capture
  String/owner/dynamic carrier

ownership:
  repeated source view keeps exact AST identity
  resolution failure retains Program-backed owner
  preflight failure retains resolved owner
  rejection -> later success with a fresh resolver/compiler owner
```

## Structural guard

Extend the existing `normal-source-plan0` family guard:

```text
Program-owned embedded resolved Main producer = 1
FunctionSemanticResolverSession consumer       = 1
VerifiedSourceProjection producer              = 1
Normal Main role producer                      = 1
shared completion/F1 consumer                  = 1

standalone Function AST construction           = 0
AST clone/rewrite/extraction                    = 0
ordinary callable main admission               = 0
second completion/return classifier            = 0
Raw Main expansion re-entry                    = 0

Builder / MirInstruction / publication         = 0
VM / process / runner consumer                 = 0
fallback/retry                                 = 0
all modified/new source and check files        < 800 lines
```

Do not add a row-specific shell. Grow the existing manifest-backed family
guard.

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
NORMAL-MAIN0-F1-PLAN0-S0
-> NORMAL-MODULE-TX0-L0
```

`NORMAL-MODULE-TX0-L0` defines the common atomic candidate-module transaction
schema used by Main-only and Main-plus-helper modules. It does not yet emit the
physical thunk.

## Reconsult boundary

Stop and reopen the accepted design only if immutable Program ownership cannot
be retained while using the existing semantic forest/source projection.

Do not treat an ordinary Rust borrowing inconvenience as permission to clone
or rewrite the Main function AST.

## Non-claims

```text
function draft lowering
physical Main thunk
module transaction/publication
VM execution/process projection
Main direct calls
helper catalog/call graph
profile admission/dispatch
new CLI/default caller
imports/using
dynamic/object carriers
```
