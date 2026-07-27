---
Status: accepted execution task map
Date: 2026-07-28
Decision: MIRBUILDER-INPLACE-REPLACEMENT0
First executable row:
  - CALLABLE-DRAFT-PORT-CUTOVER0-I0-R0
Policy:
  - docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md
Workstream:
  - docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
Supersedes scheduling authority of:
  - PRELOOP-STAGEB-SELECTED-CANDIDATE-SESSION0-prime-r1
  - OWN-GRAM-REJECT0-HAKO0-S0
---

# MirBuilder In-Place Replacement Task Map

## Outcome

```text
one production MirBuilder
one responsibility graph
zero detached replacement pipelines
zero old production owners after their cell closes
```

The migration unit is a responsibility with one existing production edge. It
is not a source profile, fixture, route family, or syntax-specific second
compiler.

## Audit basis

The 2026-07-18 descent series proves the model.

| Slice | Production cutover | Retired selected old work |
| --- | --- | --- |
| Local | `2a00fcdc9a` | inline initializer loop |
| Variable Assignment | `d7d3712f57` | selected Variable-target branch |
| value Return | `a264a7e809` | old value-expression path |
| statement If | `409ca5c319` | old inline statement-If orchestration |

The meaning drift began on 2026-07-21, when disconnected candidate integration
started closing under `I0` while production consumers remained zero.

This task map restores the earlier meaning:

```text
I0 = actual production switch
R0 = selected old path deletion
P0 = parity after that switch
```

## First executable replacement

### `CALLABLE-DRAFT-PORT-CUTOVER0-I0-R0`

Ceremony: T0, one atomic I0/R0 commit preferred.

Status: closed on 2026-07-28.

Closeout:

```text
new ordinary production callers = 2
old selected production callers = 0
fallback / retry                 = 0
deleted old symbols              = 4
deleted disconnected shell       = port_aware_function_draft.rs
production Rust LOC delta        = -202
```

Production seams:

```text
src/mir/builder/calls/lowering.rs
  ordinary static/free callable draft closure
  ordinary instance/constructor callable draft closure
```

Existing new owner:

```text
src/mir/builder/port_aware_function_draft_impl.rs
  build_static_method_draft_with_port_v1
  build_instance_method_draft_with_port_v1

RawExpressionDispatchPortV1
RawLegacyChildLoweringPortV1
```

The port-aware body product currently finalizes through a short-lived header
port. The ordinary production facade needs one thin Legacy completion sibling:

```text
PortAwarePreparedDraftBodyV1
  -> existing MirBuilder::finalize_function_draft
  -> MirFunction
```

That sibling may expose only consumption of the prepared `returns_value`
decision and delegation to the existing Legacy lookup facade. It must not add
a second finalizer, header catalog, publication policy, or function session.

Required change:

```text
two ordinary production closures
  -> existing port-aware draft lowering

existing publication/session/finalizer policy
  -> unchanged
```

Delete in the same commit when the compiler permits it:

```text
build_static_method_draft_v1
build_instance_method_draft_v1
lower_function_body
lower_method_body
```

If one helper still has a non-selected consumer, delete the selected caller
and record the exact residual consumer for the next cell. Do not preserve the
old callable body route as fallback.

Acceptance:

```text
new ordinary production callers              = 2
old selected production callers              = 0
fallback / retry                             = 0
static/free/instance/constructor parity       = green
loop and MethodCall body parity               = green
accepted source shape delta                   = 0
source policy / Stage-B policy delta           = 0
production Rust LOC delta                     < 0
shared replacement manifest / guard            = initialized once
all modified/new source/check files          < 800 lines
```

Focused validation:

```text
cargo check -q
relevant existing port-aware callable draft tests
relevant static/instance/constructor production-path tests
bash tools/checks/current_state_pointer_guard.sh
```

Do not invent a new guard if the existing test surface covers the contract.

## Immediate follow-up 1

### `CALLABLE-DRAFT-COLLECTOR-CUTOVER0-I0-R0`

Status: closed on 2026-07-28.

Exchange:

```text
old:
  per-function direct publication into the module

new:
  unpublished draft
  -> ModuleDraftCollectorV1
  -> complete batch
  -> atomic module insertion
```

Production seams:

```text
module_lifecycle.rs static/free draft terminal
module_lifecycle.rs instance/constructor draft terminal
function_session.rs direct Legacy publication branch
```

Acceptance:

```text
ordinary production collector callers    >= 1
old direct publication caller             = 0
partial callable publication on failure   = 0
fallback / retry                          = 0
```

This row reuses the generic collector. It does not activate the Stage-B
one-row function session.

Closeout:

```text
ordinary production collector callers    = 1
old direct production publication callers= 0
partial callable publication on failure  = 0
fallback / retry                          = 0
focused existing tests                    = 63 green
production Rust LOC delta                 = +153
two-cell rolling production Rust LOC      = -49
```

The ordinary root now owns one invocation-local `ModuleDraftCollectorV1`.
Static/free functions, instance methods, constructors, optional callable
`Main`, and raw root descent share the existing raw child port; root lifecycle
drains the complete draft set once through `try_add_functions_atomic`.
Collision evidence proves that no collected prefix reaches the live module.
The root-specific port extension owns only the canonical instance-method
observation needed by the parked Stage-B adapter; it adds no second lowering
or publication policy.

## Immediate follow-up 2

### `MODULE-CANDIDATE-SESSION-CUTOVER0-I0-R0`

Status: closed on 2026-07-28.

Exchange:

```text
old:
  compiler invokes live builder.build_module(ast)

new:
  isolated candidate Builder
  -> existing module lifecycle
  -> compiler finish
  -> success-only live replacement
```

This cell may start only after the collector cutover is green. It reuses
candidate/config/replacement machinery but keeps the existing Legacy
prepare/lower/finalize and verifier semantics.

Acceptance:

```text
default compile production caller            = 1
live Builder mutation before successful finish= 0
failed candidate leaves live Builder unchanged= 1
old live build-module entry                  = 0
fallback to old live path                    = 0
compiler reuse parity                        = green
```

Closeout:

```text
default compile production caller             = 1
live Builder mutation before successful finish= 0
failed candidate leaves live Builder unchanged= 1
old live build-module entry                   = 0
fallback to old live path                     = 0
compiler reuse parity                         = green
production Rust LOC delta                     = +44
three-cell rolling production Rust LOC        = -5
```

The Legacy request owns source and imports before opening one
`ModuleBuilderInvocationSessionV1`. Build and compiler finish operate only on
its candidate; the existing prepared external commit replaces the live
Builder after success. Successful module finalization now closes
function-local Builder state at its own terminal.

Collector parity also exposed a pre-existing field-write classification leak:
exact numeric fields were entering the typed-array carrier lane and losing
their `FieldSet`. Typed-array admission is now annotation-exact, and semantic
numeric range rejection remains verifier authority rather than a carrier
rebuild failure. Dynamic range contracts, constant proofs, and out-of-range
verification are green.

The next production edge was selected by the bounded ledger census:

```text
LOCAL-STATEMENT-DESCENT-CUTOVER0-I0-R0
docs/development/current/main/investigations/local-statement-descent-cutover0-i0-r0-task-2026-07-28.md
```

## Fourth replacement

### `LOCAL-STATEMENT-DESCENT-CUTOVER0-I0-R0`

Status: closed on 2026-07-28.

Exchange:

```text
old:
  variable_stmt::build_local_statement
  -> drive_raw_local_statement_v1
  -> drive_local_statement_v1

new/live:
  raw statement_surface ASTNode::Local
  -> RawLegacyLocalInputV1
  -> drive_local_statement_v1
```

The selected live raw/default caller is exactly one. One detached located
caller remains inactive at the production root and is guarded separately.
The cell deletes only the two old facades, rewrites their nine cfg(test)
callers through the real AST ingress, preserves Local semantics, and adds no
fallback, retry, or second selector.

Closeout:

```text
raw/default production caller             = 1
detached located caller                   = 1
detached production root activation       = 0
old facade call-shaped sites              = 0
fallback / retry                          = 0
focused Local tests and semantic helper   = green
production Rust LOC delta                 = -52
four-cell cumulative production Rust LOC  = -57
```

That closeout returned to `MIRBUILDER-NEXT-EDGE-DESIGN-STOP`; the following
bounded consultation then selected the fifth production edge.

## Fifth replacement

### `VARIABLE-ASSIGNMENT-DESCENT-CUTOVER0-I0-R0`

Status: closed on 2026-07-28.

Execution authority:

```text
docs/development/current/main/investigations/
variable-assignment-descent-cutover0-i0-r0-task-2026-07-28.md
```

Exchange:

```text
live:
  exact Variable-target Assignment
  GroupedAssignmentExpr
  -> RawLegacyVariableAssignmentInputV1
  -> drive_variable_assignment_v1

retire:
  drive_raw_variable_assignment_v1
  MirBuilder::build_grouped_assignment
```

The two raw/default syntax ingresses form the complete bounded caller set for
one variable-name reassignment input/owner. The detached located caller remains
root-inactive and separately guarded. Exact Variable retains historical
snapshot parity; Grouped retains existing production-ingress behavior only.
No Grouped historical snapshot parity is claimed.

Closeout:

```text
raw/default production callers              = 2
detached located caller                     = 1
detached production root activation         = 0
old facade call-shaped sites                = 0
fallback / retry                            = 0
focused tests and private ASN0 helper       = green
production Rust LOC delta                   = -77
five-cell rolling production Rust LOC       = -134
```

The following bounded Return census selected the sixth production edge.

## Sixth replacement

### `RETURN-SOURCE-PARTITION-CUTOVER0-I0-R0`

Status: closed on 2026-07-28.

Execution authority:

```text
docs/development/current/main/investigations/
return-source-partition-cutover0-i0-r0-task-2026-07-28.md
```

Exchange:

```text
live value:
  ASTNode::Return Some
  -> RawLegacyValueReturnInputV1
  -> drive_value_return_statement_v1

split Void:
  ASTNode::Return None
  -> build_void_return_statement

retire:
  build_return_statement(Option<Box<ASTNode>>)
  drive_raw_value_return_statement_v1
```

This T1 cell changes only the responsibility interface. It preserves the
Return ABI, Match/defer/cleanup/completion owners, located inactive adapter,
and language/runtime/backend behavior. Its measured `src/**/*.rs` delta must
be `<= -68` so the new five-cell rolling total remains non-positive.

Closeout:

```text
raw/default value caller                  = 1
raw/default exact Void caller             = 1
detached located value caller             = 1
old facade call-shaped sites              = 0
fallback / retry                          = 0
focused tests and private Return helper   = green
production Rust LOC delta                 = -141
five-cell rolling production Rust LOC     = -73
```

No seventh production edge is selected. Return to
`MIRBUILDER-NEXT-EDGE-DESIGN-STOP`.

The bounded seventh-edge census is recorded at:

```text
docs/development/current/main/investigations/
binary-source-partition-cell-accounting-d0-consultation-2026-07-28.md
```

It does not select a manifest row. Binary has one live source selector and one
dead predecessor chain but two semantic owners; D0 must choose source-partition
or split semantic accounting before execution resumes.

## Macro pack order

The first three replacements above are fixed. After them, the shared ledger
selected Local as the first historical live replacement credit inside this
fixed order.

```text
DESCENT-SPINE0
  body -> statement -> expression -> argument recursion

FUNCTION-STATE0
  variable / binding / type / PHI / current function state

CALL-OBJECT0
  FunctionCall / MethodCall / new / field / index / collection / lambda

CONTROL0
  If residual / Loop / LoopRange / Match / QMark / cleanup / async

FUNCTION-LIFECYCLE0
  draft / collector / finalizer / function close

MODULE-LIFECYCLE0
  declaration / catalog / multi-box / module transaction

COMPILER-RESIDUE0
  old facades / selectors / Legacy orchestration / detached assets
```

No new macro pack may be inserted.

## Shared replacement ledger

Stable paths:

```text
manifest:
  docs/development/current/main/design/fixtures/mirbuilder-inplace-replacement-v1.tsv

shared guard:
  tools/checks/mirbuilder_inplace_replacement_guard.sh
```

Create both in the first callable-draft cutover series and register the shared
guard in `docs/tools/check-scripts-index.md`. Do not create a guard per cell.

The manifest records each cell:

```text
id
pack
production caller
new owner
delete target
parity gate
state = pending | active | closed | explicit-residual
```

It also records detached assets:

```text
IntegrateNow | ReuseNeutral | FixtureOnly | Delete
```

The manifest is not permission for an inventory-only detour. Add its first
entries in the callable draft cutover series and keep it current as cells
land.

## Stage-B disposition

Keep and reuse:

```text
shared static-current-owner receiver policy
prepared member / Me / Standard route seams
source-neutral successful unified Call receipt
generic collector / verifier / candidate replacement machinery
```

Do not production-activate:

```text
ParserBox exact one-row source selection
preloop special activation plan
selected one-function Stage-B session
special outer-carrier type publisher
compile_request special-source branch
```

After generic callable/module cutovers, rerun the real Stage-B case through
the ordinary production path. Fix only a remaining general responsibility
gap. Do not revive the special route.

## Completion gate

```text
fixed macro packs                              = 8
replacement ledger remaining                  = 0
accepted AST classification                   = 57 / 57
old selected production edges                 = 0
detached production-capable routes            = 0
Legacy orchestration consumers                = 0
fallback / retry / reselection                 = 0
full accepted corpus and backend parity        = green
```

## Non-claims

```text
independent MirBuilder V2
whole compiler rewrite
new language semantics
Stage-B special production profile
default Raw/Canonical cutover
ownership grammar/runtime activation
.hako selfhost MirBuilder/parser migration
backend or VM policy change
```
