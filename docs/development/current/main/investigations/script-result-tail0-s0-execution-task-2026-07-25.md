# Script result TAIL0 S0

Decision: `SCRIPT-RESULT-TAIL-prime-r1`

Status: accepted design

Implementation authorization: `SCRIPT-RESULT-TAIL0-S0` only

Normative authority:

- `docs/reference/language/function-exit-and-entry-result.md`
- `FUNCTION-EXIT-SEMANTICS-prime-r1`

## Objective

Close the first canonical `ScriptLastExpressionOrUnit` implementation slice.
The source projection, rather than a Builder-returned `ValueId`, must decide
whether the final Script form is a value-producing expression or a
Unit-producing statement.

This row changes no parser grammar and admits no new expression or statement
surface. It projects the already admitted LinearScalar0 source sequence into
one source-owned Script result contract, lowers that contract exactly once,
and co-seals the physical Return, completion, and exit witness without a
fallible edge after BODY commit begins.

## Selected authority

The sole Script result authority is a source-owned structural split:

```text
Program source sequence
  -> RawScriptResultContractV1
       prelude
       terminal
  -> AST-free RawScriptBodyRecipeV1
  -> RawLoweredScriptResultV1
  -> one prepared BODY commit
```

Conceptually:

```rust
struct RawScriptResultContractV1 {
    prelude: Box<[RawLocatedScalarStmtV1]>,
    terminal: RawLocatedScriptTerminalV1,
}

enum RawLocatedScriptTerminalV1 {
    EmptyUnit,

    ValueExpression {
        expression: RawLocatedScalarExprV1,
    },

    UnitExpression {
        expression: RawLocatedScalarExprV1,
        origin: RawScriptUnitOriginV1,
    },

    UnitStatement {
        statement: RawLocatedScalarStmtV1,
        origin: RawScriptUnitOriginV1,
    },
}
```

The compiler projects that located contract once into a neutral, AST-free
recipe:

```rust
struct RawScriptBodyRecipeV1 {
    prelude: Box<[RawLinearScalarStmtV1]>,
    terminal: RawScriptTerminalRecipeV1,
}
```

`prelude` and `terminal` are separate owned fields. An ordinal flag into one
generic statement list is not sufficient because it would leave an easy path
back to Builder-last-value inference.

## Authority and non-authority

Authority:

```text
OwnedRawSourceV1 Program order
source grammar expression-versus-statement classification
RawScriptResultContractV1
RawScriptBodyRecipeV1 as its exact derived carrier
```

Non-authority:

```text
ASTNode::is_expression as a broad generic predicate
REPL auto-display heuristics
the last ValueId returned by statement lowering
the final physical Return or signature
module/function symbol spelling
collector or ledger inventory
Legacy snapshot parity
```

The source contract decides *which source form is the Script result*. Builder
type facts may validate whether the selected value has an available physical
carrier, but they never select the tail.

## Parser and grammar boundary

No parser implementation change is required for this S0. The current
canonical AST ingress already preserves the relevant distinction in the
Program sequence:

```text
bare scalar expression -> expression source form
print                 -> Print statement
local                 -> Local statement
assignment            -> Assignment statement
compound assignment   -> CompoundAssignment statement
```

This row is the later source-projection activation named by the normative
topic. It does not add a dedicated `script_tail` grammar production and does
not widen the grammar registry. A dedicated production, if later desired for
surface clarity, is a separate grammar row with independent dual-parser
evidence.

## Exact S0 source classification

Source-classified value-terminal candidates:

```text
Literal(Integer / TypedInteger / Float / Bool / String)
Variable
Unary(Minus / Not / BitNot) over the same admitted subset
ordinary Binary excluding And / Or over the same admitted subset
```

This is syntax-only classification. Builder exit preparation later requires
the selected candidate to have an exact supported non-Unit carrier. Missing,
Unknown, unsupported, or indirect-Unit carriers are typed physical-capability
rejections; they do not cause source-tail reclassification.

Unit terminal:

```text
empty Script               -> EmptyBody
final Print                -> PrintStatement
final Local                -> LocalStatement
final Assignment           -> AssignmentStatement
final CompoundAssignment   -> CompoundAssignmentStatement
final literal Void         -> ExplicitVoid
final literal Null         -> ExplicitNull
```

`UnitStatement` still owns and evaluates its statement exactly once. It does
not discard the statement itself; it discards only the statement's internal
Builder result.

The current S0 does not use a final variable or compound expression whose
exact result is `Void` as an inferred Unit authority. Direct source
`Void`/`Null` literals are the admitted Unit-expression forms. Other indirect
Unit-result expressions require a later typed Value-or-Unit carrier row and
fail with a typed capability error in this slice.

Unsupported final or prelude surfaces remain fail-fast:

```text
Call / MethodCall / Field / Index / New
Await / QMark / Match / Lambda
Array / Map / Record / BlockExpr / grouped assignment
Weak unary / And / Or
If / Loop / Return / Break / Continue / ScopeBox
```

They reject during source/recipe projection before the Builder root function
opens. This is a capability boundary, not a fallback to Legacy lowering.

## Route-specific recipe law

`RawRootBodyRecipeV1` must stop exposing one generic statement sequence as
the Script policy. Its private payload becomes route-specific:

```text
Script:
  RawScriptBodyRecipeV1
    prelude
    terminal

App:
  current App statement recipe
  current App/F1 migration status unchanged
```

The exact Rust enum/struct nesting may remain private, but these laws are
mandatory:

```text
generic Script statements() policy access = 0
ScriptLastValueOrVoid live authority       = 0
Script route inferred from symbols         = 0
App policy changed by this row             = 0
```

## Lowering and result carrier

The Builder performs:

```text
lower every prelude statement in source order
  -> evaluate once
  -> preserve effects
  -> discard internal ValueId

lower exactly one terminal
  -> ValueExpression: retain only its exact ValueId
  -> UnitExpression: evaluate once, retain Unit provenance
  -> UnitStatement: execute once, retain Unit provenance
  -> EmptyUnit: emit no source operation
```

The Script-specific lowerer returns a typed carrier:

```rust
enum RawLoweredScriptResultV1 {
    Value {
        value: ValueId,
    },
    Unit {
        origin: RawScriptUnitOriginV1,
        evaluated_unit: Option<ValueId>,
    },
}
```

This carrier is derived from the sealed terminal variant. It is not computed
by accumulating the last result of generic statement lowering.

`evaluated_unit` is `Some` only for an explicit final `Void`/`Null`
expression. Empty and statement-Unit terminals use `None`.

The generic statement primitive may still return an internal `ValueId` needed
by assignment/local mechanics, but the Script-result terminal must not expose
that primitive as result policy. Prelude lowering should expose a
`Result<(), _>`-shaped boundary to the Script orchestrator.

## One prepared BODY commit

The current BODY transaction has a temporal gap:

```text
commit_raw_root_exit_v1 mutates signature and Return
-> seal_root_body_preserving still returns Result
```

This row closes that gap. Borrow-only preparation must aggregate the exit plan
and tracker completion before mutation:

```rust
struct PreparedRootBodyCompletionV1 {
    // consumed tracker and predetermined completion
}

struct PreparedRawRootBodyCommitPlansV1 {
    exit: PreparedRawRootExitPlanV1,
    completion: PreparedRootBodyCompletionV1,
}
```

Those are nested plans, not the complete owner. The issued outer product must
own every capability needed by commit:

```rust
struct PreparedRawRootBodyCommitV1 {
    session: ModuleBuilderInvocationSessionV1,
    physical: PreparedRawRootBodyPhysicalCompletionV1,
    open: RawOpenRootFunctionV1,
    result: RawLoweredScriptResultV1,
    plans: PreparedRawRootBodyCommitPlansV1,
}
```

The exact private nesting may differ, but no ambient Builder, loose tuple, or
borrowed tracker may be reacquired after issue. Then one private consuming
commit performs:

```text
physical Return and exact signature
-> RawRootBodyExitWitnessV1
-> CompletedRootBodyV1
-> function draft extraction and function-state cleanup
-> CompletedRawRootBodyPhysicalV1
```

No `Result`, lookup, verification, retry, repair, or source observation is
allowed after `PreparedRawRootBodyCommitV1` is issued.

For a Script value:

```text
selected tail ValueId
  = physical Return operand
  = CompletedRootBodyV1::Value operand
  = exit-witness operand
```

For Script Unit:

```text
signature = Void
completion = NoValue
exit witness retains the exact Unit origin

EmptyUnit / UnitStatement:
  one synthetic Void operand
  physical Return(synthetic Void)

UnitExpression(Void / Null):
  reuse the exact evaluated Unit operand
  physical Return(the same operand)
  no second synthetic Void
```

ROOTBATCH remains a borrowed witness validator. It gains no result
classification, type lookup, Return writer, signature repair, collector
identity, or ledger identity authority.

## Failure law

Source/recipe failures retain the exact source-owned route package and reject
before Builder effects.

Lowering/exit/commit preparation failures retain the exact unpublished BODY
owner:

```text
session
open root function and TypeContext
physical tracker
route-specific recipe
lowered terminal result when available
typed stage and nested cause
```

Public rejection capability remains:

```text
stage()
error()
discard(self)
```

Forbidden:

```text
into_owner
retry / resume
Legacy fallback
tail reclassification
signature or Return repair
postprocess/public-adapter repair
partial ROOTBATCH entry
```

## File topology and implementation order

Two parent files are already at the structural limit:

```text
src/mir/compiler/raw_root_source_facts.rs         = 788 lines
src/mir/builder/raw_root_environment_install.rs   = 797 lines
```

Do not add semantic logic to either parent. Use a short Refactor Series under
this one objective:

```text
SCRIPT-RESULT-STRUCTURE0
  behavior-neutral child-module extraction

  src/mir/compiler/raw_root_source_facts/recipe_projection.rs
    existing post-install recipe projection

  src/mir/builder/raw_root_environment_install/body_transaction.rs
    BODY transaction products and drive_root_body

SCRIPT-RESULT-CONTRACT0
  src/mir/compiler/raw_root_source_facts/script_result.rs
    RawScriptResultContractV1 source classifier

  src/mir/compiler/raw_root_source_facts/script_result_p0.rs
    focused source-contract fixtures; child test module only

  src/mir/raw_script_result_recipe.rs
    neutral AST-free terminal recipe and Unit provenance

SCRIPT-RESULT-RECIPE0
  route-specific RawRootBodyRecipeV1 payload
  one located-to-neutral projection
  path/order/provenance uniqueness including terminal

SCRIPT-RESULT-LOWER0
  src/mir/builder/raw_script_result_lowering.rs
  prelude discard + one typed terminal result

SCRIPT-RESULT-COMMIT0
  PreparedRootBodyCompletionV1
  PreparedRawRootBodyCommitV1
  no post-commit fallible edge

SCRIPT-RESULT-WITNESS0
  exact completion/Return/signature/Unit-origin relation
  existing ROOTBATCH borrowed validation only

SCRIPT-RESULT-G0
  focused fixtures
  reuse the existing Raw BODY family guard
```

Child modules preserve parent-private field access and avoid widening
compiler- or Builder-wide APIs. `raw_root_decl_access.rs` receives only a
net-neutral narrow route accessor change if required.

## Guard and proof budget

```text
ceremony_tier             = T2 new source authority
sunset_id                 = none; existing family guard is durable
proof_inventory_before    = existing Raw BODY guard + BODY fixtures
new_proofs                = focused Script result fixtures only
retired_or_merged_proofs  = old live ScriptLastValue guard expectation
net_proof_delta           = 0 public guards
sunset_budget             = 0
sunset_row                = none
retire_when               = only when a replacement Raw BODY family guard lands
budget_repayment_evidence = existing guard evolved in place; no new shell guard
```

Evolve the existing compatibility entry:

```text
tools/checks/lib/
  cut0_i0_root0_raw_source0_lower_root_body0_s0_guard.py
```

into the reusable Raw BODY / Script-result family guard. Keep the filename for
current compatibility and update the check index description. Do not add a
per-row shell guard and do not extend the ordinary-callable
`resolved_control_flow_contract.sh`.

The historical BODY0 card may retain its old wording as migration evidence.
The live-source guard must require:

```text
RawScriptResultContractV1 producer                = 1
Script source-result consumer                     = 1
ScriptLastValueOrVoid in live production          = 0
Builder-last-ValueId accumulator                  = 0
post-commit fallible tracker seal                 = 0
ROOTBATCH result classifier/Return repair         = 0
all modified/new source/check files               < 800 lines
```

## Acceptance matrix

Success:

```text
empty Script
  -> Unit / EmptyBody

final Integer/String/Bool/Float expression
  -> Value

final Variable/Unary/ordinary Binary with exact supported non-Unit type
  -> Value

prelude expression + final expression
  -> earlier result discarded
  -> final expression only supplies Value

final Print/Local/Assignment/CompoundAssignment
  -> operation executes exactly once
  -> Unit with exact origin
  -> one synthetic Void is the Return operand

final literal Void/Null
  -> Unit with distinct source provenance
  -> evaluated Unit ValueId is the Return operand
  -> second synthetic Void = 0

same compiler:
  success -> success
  reject -> success
```

Typed rejection:

```text
unsupported prelude/final source surface
unsupported final operator
undefined final variable
missing/Unknown/unsupported result carrier
indirect Unit result not admitted by S0
route/contract drift
preterminated block
tracker non-closure
```

For every rejection:

```text
live module publication delta = 0
collector/ledger delta         = 0
partial root draft publication = 0
fallback/retry                 = 0
```

Focused gates:

```bash
RUSTFLAGS='-Awarnings' cargo test -q --lib raw_script_result_p0 -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test -q --lib raw_root_body -- --test-threads=1
python3 tools/checks/lib/cut0_i0_root0_raw_source0_lower_root_body0_s0_guard.py
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Durable forward task order

This row does not authorize later work, but it fixes the dependency order:

```text
SCRIPT-RESULT-TAIL0-S0

-> ENTRY-RESULT-PROJECTION0-D0
     T2 design stop
     inventory source-entry selection, physical entry, and process-status callers
     select SelectedSourceEntry, SourceEntryResult, physical thunk,
     and ProcessTermination ownership without merging them

-> ENTRY-RESULT-PROJECTION0-S0
     CONTRACT0
     ENTRY-SELECTION0
     SOURCE-ENTRY0
     PHYSICAL-THUNK0
     VM-REFERENCE0
     EXE-AOT0
     PARITY-G0

-> RAW-BODY-RETURN-COMPAT-P0
     test-only Legacy observation
     canonical/public consumers zero

-> FUNCTION-EXIT-APP-SCRIPT-PARITY0
     canonical witness chain
     Legacy differences remain explicit

-> NORMAL-ENTRY-D0
     T2 design stop for production semantic-profile selection

-> NORMAL-ENTRY-PROFILE0-S0
-> NORMAL-ENTRY-CANARY0-S0
-> NORMAL-ENTRY-CUTOVER0-S0
     cutover requires separate explicit authorization

-> ENTRY-EXIT-CODE-COMPAT-RETIRE0-S0
     sunset_id = ENTRY-EXIT-CODE-COMPAT-SUNSET-001
-> RAW-BODY-RETURN-COMPAT-RETIRE0-S0
     sunset_id = RAW-BODY-RETURN-COMPAT-SUNSET-001
-> OLD-RAW-RETIRE0
     sunset_id = RAW-PUBLICATION-SUNSET-001
```

`ENTRY-RESULT-PROJECTION0-D0` must keep evaluation result and process status
separate. Its required starting inventory includes VM, MIR-interpreter,
quiet-MIR, JoinIR bridge, HV1 inline, native `ny_main`, and LLVM harness
callers. It must explicitly account for the common `vm_execution.rs` adapter,
standalone MIR mode, quiet-MIR modulo/truncation behavior, the LLVM mock
42/0 behavior, historical PyVM status transport, duplicated entry selection
in `entry_selection.rs` and `MirInterpreter::execute_module`, and the NyRT
positive-`i64` handle heuristic. It must not start by adding a generic
`to_exit_code(Box)` helper.

`NORMAL-ENTRY-D0` must select the complete semantic profile before changing
`compile_with_source`. An existing explicit Raw ingress is not itself default
route authority. Its D0 evidence must include an exact direct-caller census
and must keep the grammar profile distinct from the function/Script/process
semantic profile, even if a later selected envelope carries both.

## Non-claims

```text
new parser syntax or dedicated script_tail production
ordinary function/Main F1 widening
App return-policy change
nested/all-path/cleanup-bearing completion
indirect Unit-expression result support
physical source-entry thunk
process-exit projection
normal-entry profile or cutover
JSON / Program(JSON v0)
executor / selfhost / fastmem
old Raw-chain retirement
public-adapter repair
CUT0
```

## Closeout condition

This card cannot close docs-only:

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

The row closes only when the source contract, route-specific recipe,
Script-specific lowering, infallible BODY commit, witness relation, focused
fixtures, reused family guard, pointer guard, and below-800-line boundary are
all green.
