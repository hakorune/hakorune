---
Status: accepted execution task map
Date: 2026-07-28
Decision: MIRBUILDER-INPLACE-REPLACEMENT0
Historical first executable replacement row (closed; not current):
  - CALLABLE-DRAFT-PORT-CUTOVER0-I0-R0
Policy:
  - docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md
Workstream:
  - docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md
Final convergence pointer (serial, after Loop retirement):
  - docs/development/current/main/design/repo-physical-structure-cleanup-ssot.md
Closed design-stop correction:
  - docs/development/current/main/investigations/loop-physical-prepare-design-correction-r0-task-2026-08-07.md
Current Loop product-frontier mirror (non-authoritative):
  - `LOOP-S6C-INSTALLED-CHILD-COMPOSITION-D0` (T2 BoxShape design stop)
Resume gate:
  - always resolve `current_execution_row` and `latest_card_path` from
    `CURRENT_STATE.toml`; this frontmatter is only a product-frontier summary
    and cannot resume a retired or parked row.
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

## Active Loop physicalization order

This is the compact order for the current Loop frontier. It is subordinate to
`CURRENT_STATE.toml`; it is parked while another active card is selected and
must not silently fall back to the retired prepare-only row or resume before an
explicit pointer retarget.

```text
1--5. CLOSED: demand, module split, block receipt, Const, Read
6--8. CLOSED-BY-INTEGRATION: Binary, Compare, Write through the common
      five-family dispatcher and its exact receipt/negative evidence
9. CLOSED caller-zero: seven-row Callable program through DraftSeal
10. CLOSED caller-zero: fifteen-row Generic G0 program plus carrier
11. CLOSED caller-zero semantic cohort: S6C source/Facts/Recipe/logical output,
    prephysical ingress, the normative String/Text law, and the TextEq site
    contract are issued without Builder or a production caller
12. CLOSED caller-zero: `CALLABLE-PHYSICAL-HEADER-TRANSPORT-R0` adds the
    explicit source result annotation and transports supported formal rows,
    source-backed result/header, and Completion/return proof through one
    branded package/Port cohort; no runtime wire or physical route
13. CLOSED design: `CALLABLE-PHYSICAL-TEXT-PARAMETER-ABI-D0` accepted one
    generation-checked TextFormalBorrowV1 owner; its caller-zero wire/validator
    implementation is now closed and still has no production caller
14. CLOSED caller-zero I0: `CALLABLE-PHYSICAL-TEXT-PARAMETER-ABI-I0` added only
    the Rust validator, fixed C status projection, stale-generation and exact
    Text negatives; S6C, TextEq, Builder, and session callers remain zero
15. CLOSED child BoxShape: `LOOP-S6C-INSTALLED-CHILD-COMPOSITION-D0` names
    `issue_normal_callable_semantic_package_v1` as the sole pre-install issuer
    of a total same-cohort S6C child and one move-only Completion seed; the Port
    only verifies the issued role/identity and takes/lends that child exactly
    once. Caller-supplied Facts/Recipe/slot/fixture and Port-side
    reclassification are forbidden.
16. CURRENT caller-zero I0: `LOOP-S6C-INSTALLED-CHILD-COMPOSITION-I0`
    implements the package-private seed/child models, issuer wiring, and
    focused negatives; production caller, TextFormal mapping, V2 envelope,
    Builder/session, fallback, and retry remain zero.
17. OPEN parent design stop after child I0 acceptance:
    `LOOP-S6C-COMMON-V2-PRESESSION-CONTRACT-D0` closes the one Completion owner,
    the TextFormalBorrowV1 callable-signature choice (16-byte aggregate or
    explicit lane expansion) with its call-site/session owner, and exact
    disjoint `13 operations + If + Exit = 15 placements`.
18. OPEN common-program branch after parent D0 acceptance: semantic-program co-seal,
    JoinSig transfer, bound segment input, boundary cleanup,
    `LOOP-COMMON-V2-PRESESSION-TRANSPORT-R0`, then caller-zero
    `LOOP-S6C-COMMON-V2-PRESESSION-I0`; no detached ingress/header/Completion
19. OPEN sibling TextEq-route branch: strict scalar probe, tracked route
    decision, canonical Trap owner, then only the selected route's lifecycle
    demand; route admission is pre-session and actual residence is session-local
20. OPEN fan-in structural/session coverage: Always, If, Exit, then the first
    common V2 physical session under `CanonicalSsaFunctionSessionV2`
21. OPEN gated production selection: pre-cutover authority proof, then M10b
    activation and M11/M12 legacy retirement
22. PARKED post-cutover convergence: main integration, whole-builder typed
    ingress, common finish convergence, warning/allow census, and physical
    docs/module cleanup (see the current physical-header card's parked rows)
23. OPEN: REPO-FINAL-CONVERGENCE-AUDIT0-G0
```

The selected-Dynamic `skip_while/4` lane is a reusable authority/physical
precedent, not S6C source truth: it owns `substring/indexOf`, while forward
`ScanWithInit` requires its own exact `length/substring/TextEq` source-bound
relations. The old live general-Loop edge remains one registry/legacy route;
portable/common production selection remains zero until the current header
cohort and later common/session rows close.

Decision B forbids taking one operation from a full demand. The current demand
row and every later leaf row have one acceptance claim each. No current row
claims Return, Completion, DraftSeal, publication, backend performance,
production selection, retry/fallback removal, or legacy deletion. Each later
item requires its own task receipt and same-commit reference/README/current-
mirror updates.

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
and language/runtime/backend behavior. Its historical execution estimate used
`-68` as a five-cell target; the current policy records that trend without
using it as implementation permission.

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

The Return closeout itself selected no seventh production edge and returned to
`MIRBUILDER-NEXT-EDGE-DESIGN-STOP`; the bounded consultation below has since
accepted the next responsibility.

The bounded seventh-edge census is recorded at:

```text
docs/development/current/main/investigations/
binary-source-partition-cell-accounting-d0-consultation-2026-07-28.md
```

The consultation selected Option A and the implementation has since added one
closed manifest row. Binary has one live source selector, two disjoint semantic
owners, and zero remaining predecessor-chain symbols.

## Structural footprint observation

Structural size is a result metric, not replacement completion authority:

```text
docs/development/current/main/investigations/
mirbuilder-structural-budget-d0-consultation-2026-07-28.md
```

Fixed roots and baseline observation:

```text
roots:
  src/mir/builder
  crates/hakorune_mir_builder

source files = 952
source LOC   = 182452
test files   = 139
test LOC     = 40826
```

Accepted execution:

```text
docs/development/current/main/investigations/
mirbuilder-structural-budget0-closeout-task-2026-07-28.md
```

The closeout landed one TSV baseline row plus four measurements in the existing
shared guard. The later policy correction makes growth measurement-only: it
requires explanation but does not reject a cell. No Python checker, path
digest, disposition ledger, or final-X derivation is added.

## Accepted seventh replacement

### `BINARY-SOURCE-PARTITION-CUTOVER0-I0-R0`

Status: closed.

Execution authority:

```text
docs/development/current/main/investigations/
binary-source-partition-cell-accounting-d0-consultation-2026-07-28.md
```

Responsibility:

```text
one raw/default ASTNode::BinaryOp selector
  -> Ordinary Binary owner
  -> ShortCircuit owner

one atomic delete set:
  MirBuilder::build_binary_op
  drive_raw_ordinary_binary_expression_v1
  drive_raw_short_circuit_expression_v1
```

The source partition is total and pairwise-disjoint. Each semantic owner keeps
its own parity, failure, reuse, and child-demand proof. The cell does not claim
shared semantics and neither owner may be credited again later.

The structural observation is closed. The seventh manifest row and Binary
source edits landed together in the atomic implementation closeout.

Closeout:

```text
old Binary symbols                         = 0
ordinary/ShortCircuit external sites       = 2 / 2
focused suites                             = 16 / 16, 16 / 16, 35 / 35, 4 / 4
production Rust LOC                        = -68
five-cell rolling production Rust LOC      = -294
```

## Post-Binary selection boundary

After Binary closed, one bounded selection boundary ran:

```text
DESCENT-SPINE0-CLOSE-AUDIT
```

It selected:

```text
RECORD-HELPER-BODY-DESCENT0-D0
  closed decision = Candidate A
  ceremony        = T1
  execution       = RECORD-HELPER-BODY-DESCENT0-I0-R0
  contract        = declaration-body short reborrow without reusing
                    call-site location/ledger authority
```

Authority:

```text
docs/development/current/main/investigations/
record-helper-body-descent0-d0-consultation-2026-07-28.md
```

Still parked:

```text
BINARY-SOURCE-PARTITION-PROOF-CONSOLIDATION0
RAW-BODY-FACADE-RETIRE0
non-Program root fallback
```

The selection commit does not create an eighth manifest row. The atomic
implementation adds the row as `closed` only after the two old direct edges
are zero and the focused gates are green. Keep the root non-Program fallback
in a separate compiler-residue decision.

## Accepted eighth replacement

### `RECORD-HELPER-BODY-DESCENT0-I0-R0`

Responsibility:

```text
raw/default MethodCall
-> prepared InlineRecord / InlineSetter helper execution
-> callable-catalog helper body statement/expression descent
```

New responsibility edge:

```text
MethodCallArgumentDescentV1
-> one tagged catalog-child operation
-> exact associated MethodCallDescentPortV1
```

Completion owner:

```text
record_helper_args.rs
lower_record_helper_body_until_return
```

Atomic old-edge deletion:

```text
self.build_expression(*expr.clone())
self.build_statement(stmt.clone())
try_inline_same_module_helper_setter_call
try_inline_same_module_helper_setter_call_with_descent
try_inline_same_module_helper_setter_call_from_receiver_with_descent
```

Focused gates:

```text
record_helper_args and MethodCall tests
private M0 route helper after exact caller census
real allocator record-construction helper guard
shared replacement guard
four-metric structural observation
```

Hard stop:

```text
new helper source/provenance authority
call-site location/ledger/token reuse
helper grammar or Return completion change
located production activation
fallback / retry / reselection
new unowned source/test/check file
unexplained source/test or rolling-LOC growth
```

Closeout:

```text
manifest state                              = closed
old direct helper-body edges                = 0
dead same-family facades                    = 0
focused suites                              = 4 / 4, 4 / 4, 13 / 13, 17 / 17
private M0 / real CLI / shared guards       = green / green / green
source/test files                           = 952 / 139
source/test LOC                             = 182430 / 40820
production Rust LOC                         = +46
five-cell rolling production Rust LOC       = -292
```

## Closed ninth replacement

```text
FIELD-PROPERTY-GETTER-DESCENT0-I0-R0
  pack / ceremony = CALL-OBJECT0 / T1
  Candidate / terminal = A / A1 lookup-none
```

```text
ASTNode::FieldAccess
-> build_field_access_with_port_v1
-> selected port lowers object to ValueId
-> try_lower_property_read_with_port_v1
-> PropertyGetterCompletionV1
-> shared standard prepare / execute
-> selected-port catalog child OR raw A1 terminal
```

The old value-only entry, raw standard handler, Legacy argument adapter, and
dead field facade are absent. MethodCall source fabrication, receiver
re-descent, header authority, fallback, retry, and reselection are zero.

```text
authority = investigations/field-property-getter-descent0-d0-consultation-2026-07-28.md
manifest = one closed row
source/test = 952 files / 182452 LOC; 139 files / 40809 LOC
cell / rolling production LOC = +22 / -218
```

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

### Active default compiler ingress D0

The current normal runners, including ordinary `--backend vm`, still enter
`compile_with_source* -> compile_legacy_request`. Existing canonical
entrypoints admit narrower explicit families and have no normal/default
production caller.

Active authority:

```text
CANONICAL-DEFAULT-COMPILER-INGRESS0-D0
docs/development/current/main/investigations/
canonical-default-compiler-ingress0-d0-consultation-2026-07-28.md
```

Before a tenth row, the D0 must accept one typed request carrying AST, source
identity, imports, configuration, and compatibility provenance; one total
source-family classifier before Builder effects; one backend-neutral result;
the exact normal/default caller family; and one atomic old-edge delete set.

`COMPILER-RESIDUE0` cannot close until normal/default route selection is once,
`compile_with_source*` Legacy production callers are zero, family-specific
canonical entrypoints are internal or retired, full corpus/backend parity is
green, and canonical rejection never retries Legacy.

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
