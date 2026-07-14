---
Status: Active — SSA-I1-COMPAT-N0a closed; next compatibility row selection stop
Date: 2026-07-15
Decision: D′ — SSA-first, control-contract-preserving, function-owner-atomic
Current blocker: RESOLVED-SEMANTIC-OWNER-FOREST-V1-DPRIME-SSA-I1-COMPAT-NEXT-ROW-SELECTION-DESIGN-STOP-001
Work mode: Refactor Series Mode followed by bounded capability slices
Supersedes:
  - mirbuilder-b0-l4-a-a2prime-implementation-task-2026-07-14.md after its closed S1 slice
  - mirbuilder-resolved-control-flow-v2-final-form-extraction-task-2026-07-14.md as the effect-bearing final form
Retains:
  - closed B0-L4-S1 exact Loop/LoopBody identity bundle
  - closed B0-L4-S2′ generic located source range and coverage schema
Related:
  - ../design/binding-ssa-first-control-lowering-ssot.md
  - mirbuilder-resolved-region-flow-v1-task-2026-07-13.md
  - mirbuilder-b0-l3b-a-plus-implementation-task-2026-07-13.md
  - mirbuilder-resolved-semantic-owner-forest-design-stop-2026-07-13.md
  - mirbuilder-ssa-rc0-owned-alias-materialization-design-stop-2026-07-14.md
  - mirbuilder-ssa-i1-trivial-profile-atomic-cutover-design-stop-2026-07-14.md
  - ../design/box-lifecycle-bprime-tombstone-adaptive-ownership-ssot.md
  - box-lifecycle-bprime-tombstone-adaptive-ownership-task-2026-07-14.md
---

# D′ Binding SSA Final-Form Taskboard

Architectural authority lives in
`../design/binding-ssa-first-control-lowering-ssot.md`. This card owns work
order, acceptance gates, current blocker, and retirement sequencing.

## Objective

Make one function-scoped Binding SSA construction mechanism the only
production `BindingRefV1 -> ValueId` merge authority for canonical source
Lowering.

```text
canonical syntax
+ VerifiedResolvedFunctionV1
        │
        ├─ VerifiedLocated*ControlV1
        │    exact source coverage
        │    exact ScopeId / RegionId topology
        │    typed reachable ports and exact targets
        │    cleanup obligations
        │    no BindingRef effect or carrier rows
        │
        ▼
CanonicalSsaFunctionLowererV2
  ResolvedIdentityLedgerV2
  ResolvedSemanticStackV1
  ControlCoverageConsumptionV2
  BindingSsaBuilderV1
        │
        ▼
verified MIR SSA
        │
        ▼
optional post-MIR derived analysis
```

The final split is:

```text
resolver:
  lexical and control identity

pre-Builder control contract:
  where control goes, which ports exist, what source is covered,
  and which cleanup obligations must run

Binding SSA:
  which ValueId reaches each BindingRef use and which PHIs are required

post-MIR analysis:
  whether a resulting PHI is an induction variable, recurrence, or invariant
```

The current `CorePlan`, legacy `LoopRouteContext`, names, value-map diffs, and
precomputed carrier rows are not canonical authorities.

Roadmap-wide completion cannot be docs-only:

```text
roadmap_docs_only_closeout = forbidden
code_or_artifact_delta_required_after_D0 = 1
```

## Accepted decision

| Boundary | Decision |
| --- | --- |
| Value merge authority | one function-owned `BindingSsaBuilderV1` |
| Production cutover unit | whole canonical function owner, never one control arm |
| Pre-Builder owner | exact coverage/topology/ports/targets/cleanup only |
| If / Loop commonality | shared SSA and CFG edge/seal substrate; family CFG boxes stay separate |
| CFG predecessor truth | MIR terminators are SSOT; cached predecessors are a checked witness |
| RegionId materialization | transaction-local roles until an independently accepted SA4 consumer |
| PHI simplification | same-input/self PHIs may remain; later generic simplifier owns removal |
| Optimizer loop facts | post-MIR derived analysis, created only for a concrete consumer |
| Legacy failure | typed failure; no retry or fallback |

The former A + A2′ plan was internally coherent while RegionFlow owned every
effect and PHI-source decision. It is superseded because a generic CFG SSA
baseline would otherwise make RegionFlow and Lower independently decide the
same PHI domain.

## Dependency DAG

The headings below remain the detailed cards. This graph is the short
dependency SSOT; optional X0/O0/O1 work is not on the blocking path.

```text
SSA-E0 -> SSA-S3 -> SSA-M0 -> SSA-RC0-D0
SSA-RC0-D0 -> SSA-RC-L0a -> SSA-RC-L0b -> SSA-RC-L0 -> SSA-RC-L1 -> SSA-RC-P0 -> SSA-RC-A0 -> SSA-RC-A1a
SSA-RC-A1a -> SSA-RC-V0 -> SSA-RC-A1b -> SSA-RC-A1c
SSA-RC-A1c -> SSA-RC-RET-P0 -> SSA-RC0 -> SSA-I0-PROFILE -> SSA-I1-T
SSA-I1-T -> SSA-I1-COMPAT -> SSA-I1-FULL -> SSA-R1
SSA-I1-FULL -> SSA-RC-RET-R1
{SSA-I1-T, exact BoxRef source producer} -> SSA-I1-O1
repository-wide ReleaseStrong caller zero -> SSA-RC-RET-R2

{SSA-I1-O1, B′ first-family ObjectCell carrier cutover}
  -> B′ first canonical ObjectCell BoxRef materialization

SSA-I1-T -> Loop-S3′ -> Loop-I1′ -> Loop-I2′
Loop-I2′ -> {N1, N2, N3} -> N4

{SSA-E0, N4} -> EXIT-S0 -> EXIT-S1 -> EXIT-S2
{Loop-I2′, EXIT-S2} -> EXIT-I1 / EXIT-I3 / EXIT-I6
{N1, EXIT-I1, EXIT-I3} -> EXIT-I2 / EXIT-I4
{SSA-I1-T, EXIT-S2} -> EXIT-I5
{EXIT-I1..I6, N4} -> EXIT-I7

SSA-I1-T -> F0 -> F1a / F1b / F1c
capture-cell authority + F0 -> F1d
{F0, required F1x, RET-I1, RET-I2} -> F2-S0 -> F2-I1
F2-I1 + canonical caller zero -> RET-R1 -> PUB-F0
repository-wide caller zero -> RET-R2

{SSA-RC-A1c, SSA-RC-V0, SSA-RC-RET-P0} -> HMI-P0 -> HMI-S0 -> HMI-S1 -> HMI-I0 -> HMI-P1
{SSA-I1-O1, HMI-P1} -> HMI-C0 -> HMI-X0 -> HMI-R1
repository-wide Rust MirInterpreter caller zero -> HMI-R2
```

Owner-family expansion may proceed independently of Loop/exit expansion when
its closed grammar does not widen. A source unit still cuts over all-or-
nothing; parent canonical / child legacy is never permitted.

B′ is the accepted later runtime constitution: explicit `fini()` is an eager
tombstone transaction, while `DestroyOwned`/last strong perform structural
drop and never user fini. Its ObjectCell, weak/generation, family rollout,
adaptive RC, plugin split, backend parity, and global Arc retirement order are
owned by the related B′ taskboard. This does not change SSA-RC-L0 or make B′ a
prerequisite for passive Ownership SSA analysis/verifier rows.

## Normative design reference

The sole architecture authority is
`../design/binding-ssa-first-control-lowering-ssot.md`. In particular it
owns the authority matrix, identity/value split, CFG seal law, Binding SSA
algorithm, open-PHI fact rule, RC/lifetime law, nesting model, physical
layout, atomic cutover rule, and final completion definition.

This taskboard owns only execution order and acceptance. Every row preserves:

```text
pre-Builder = exact source/control/cleanup, never value-merge effects
one BindingSsaBuilderV1 per canonical function owner
MIR terminators = CFG truth; cached predecessors = verified witness
family-specific If/Loop CFG boxes over the same SSA/edge substrate
whole-owner production cutover; no old-environment synchronization bridge
no canonical fallback to legacy If/Loop/CorePlan
```

Only owner-local binding rebinds enter Binding SSA. Upvar/captured-by-reference,
field, and index writes stay with their storage owners or fail preflight. New
or modified source/check files stay below 800 lines.

## Implementation order

### D0 — authority decision and taskization — closed by this card

```text
D′ is the final canonical value authority
old A+A2′ S3/I1/I2 are unauthorized
S1 identity and S2 coverage intent are retained
the next blocker is behavior-neutral SSA-P0 seam inventory
```

Production behavior delta: zero.

### B0-L4-S2′ — generic located source coverage — closed

Land only the authority-neutral portion of the preserved S2 WIP:

```text
ConsumedSourceRangeV1 with checked nonzero count
FunctionSourceViewV1-owned suffix first/range/advance
CoveredSourceSiteV1
private VerifiedLocatedSourceCoverageV1
owner/body/start/bounds/order/duplicate verifier
public constructor / Clone / into_parts = 0
Lower receives coverage separately from its family product = 0
new resolved_control_flow coverage production consumers = 0
Binding SSA and Loop production activation = 0
existing A+ If production remains unchanged
```

Prefer `resolved_control_flow/source_coverage.rs`; do not land the historical
`PlanSourceCoverage` name or carrier-oriented README wording unchanged.

Closed evidence:

```text
compiler-owned checked range navigation fixtures = 7
private coverage verification fixtures = 3
resolved_control_flow production consumers = 0
effect/carrier rows = 0
Binding SSA / Loop runtime activation = 0
existing A+ If production behavior = unchanged
```

Production behavior delta: zero.

### SSA-P0 — canonical SSA seam inventory

Before changing ownership, close one exhaustive source/caller table for:

```text
every canonical binding declaration/read/rebind and scope-exit read
every current flat value-environment access
every canonical CFG edge emitter and predecessor writer
every PHI reserve/define/expose/patch/rollback path
every assignment and scope-exit RC read/release path
every function finish/publication barrier
the exact currently accepted function-body terminal Return shape
all old If effect/join/snapshot consumers
```

Classify each row as `move to Binding SSA`, `control-only retain`, `legacy
isolate`, or `caller-zero delete`. This is evidence only: production behavior
and accepted grammar remain unchanged.

Closed evidence:

```text
machine rows = 92
binding/value = 18
CFG/predecessor = 12
PHI lifecycle = 23
RC/lifetime = 7
finish/publication = 10
terminal Return = 10
old A+ If authority = 12
production behavior delta = 0
accepted grammar delta = 0
```

Evidence card:
`mirbuilder-canonical-ssa-seam-inventory-2026-07-14.md`.

### SSA-L0 — mandatory oversized PHI-helper split

`src/mir/builder/ssa/phi_input_materializer.rs` is already above 800 lines.
Before SSA-C1, SSA-P1, or SSA-S1 edits, split it by existing responsibility in
one behavior-neutral commit:

```text
facade
edge_rematerialization:
  analysis, diagnostics, recursive rematerialization, for_pred
function_repair:
  whole-function repair, pruning, missing-input completion
separate focused test modules
no API/semantic/grammar change
all existing PHI lifecycle tests green
each resulting source file below 800 lines
no Binding SSA acceptance code in the split commit
```

Closed evidence:

```text
facade = 18 lines
edge_rematerialization = 331 lines
function_repair = 166 lines
edge tests = 77 lines
function repair tests = 237 lines
shared test support = 10 lines
existing focused fixtures = 5/5 green
public/private caller API delta = 0
production behavior delta = 0
accepted grammar delta = 0
```

The whole-function repair box is explicitly legacy infrastructure. The split
does not authorize canonical SSA to depend on CFG repair, PHI pruning, or
missing-input fabrication.

### SSA-C1 — canonical CFG/seal prerequisite — closed

```text
one canonical edge facade
late predecessor veto
computed/cached predecessor equality
seal-twice and edge-after-seal errors
terminator-derived predecessor truth; cached-successor recompute is not proof
PHI analysis/update_cfg side-effect repair forbidden on canonical edges
```

Production activation remains zero. Existing If continues on its old path.

Closed evidence:

```text
one fallible CanonicalCfgSessionV1 facade
terminator-derived predecessor truth
cached successors/predecessors checked without repair
immutable per-block seal witness
duplicate edge / duplicate terminator / edge-after-seal / seal-twice typed errors
raw late-edge mutation detected at finish
focused fixtures = 15/15 green
production If/Loop/Binding SSA callers = 0
accepted grammar delta = 0
```

### SSA-P1 — PHI transaction cleanup prerequisite — closed

Close the reusable failure lifecycle independently of SSA acceptance:

```text
every pending rollback is attempted
cleanup continues after one rollback failure
primary and cleanup errors are both retained
partial PHI/function publication = 0
success commits exactly once
```

No accepted syntax or production Binding SSA call is added.

Closed evidence:

```text
PhiTxn abort attempts every pending rollback
one rollback failure does not stop later cleanup
primary plus every cleanup failure retained in PhiTxnAbortErrorV1
missing provisional PHI is a cleanup failure, not a silent success
commit with pending PHIs routes through the same rollback owner
successful commit consumes the transaction exactly once
focused fixtures = 6/6 green
production Binding SSA callers = 0
accepted grammar delta = 0
```

### SSA-V0 — canonical publication/verifier prerequisite — closed

Close the invalid-publication boundary before Binding SSA production work:

```text
post-RC MIR verifier failure is a typed compile failure
candidate module commit after verifier failure = 0
duplicate same-name canonical function publication is a typed failure
function/module publication before seal/SSA completion = 0
verification_result = Err cannot cross CanonicalModuleLoweringSessionV1::commit
```

This row changes no accepted source grammar. It must land before SSA-S1 is
connected to production. Legacy result-reporting behavior, if still required,
stays behind explicit legacy provenance rather than weakening the canonical
barrier.

Closed evidence:

```text
canonical post-RC/canonicalize verification Err -> MirVerificationFailed
CanonicalModuleLoweringSessionV1 commit after that Err = unreachable
same-name canonical draft -> typed DuplicateFunctionPublication
duplicate replacement of the first sealed draft = 0
legacy add_function and pre-RC result reporting remain explicit legacy seams
focused publication/verifier fixtures = 3/3 green
private SSA-V0 publication guard = green
production Binding SSA callers = 0
accepted grammar delta = 0
```

SSA-V0 does not fabricate a Binding SSA completion witness before the owner
exists. SSA-S1 remains disconnected; the final function-publication witness
connection is made atomically with the later production cutover.

### SSA-S1 — disconnected Binding SSA — closed

Implement `builder/ssa/binding/` with a fake/narrow IR test adapter and no AST,
source, RegionFlow, or name dependency.

Focused fixtures:

```text
entry definition and same-block overwrite
single predecessor
diamond with no/one-sided/two-sided assignment
nested diamonds
open Loop header and one backedge
zero iteration
multiple backedges
same-input and self PHIs retained
missing definition
foreign BindingRef owner
duplicate edge and late edge
unsealed/incomplete finish
PHI patch/rollback failure
all inputs are exact actual predecessors and dominate their edges
```

Production activation remains zero.

Closed evidence:

```text
one function-branded BindingSsaBuilderV1
define/read/seal/finish minimal API
immutable VerifiedPredecessorsV1 input; CFG rediscovery/repair = 0
open-block provisional PHI before recursive exposure
same-input and self PHIs retained
exact predecessor input order plus adapter-side dominance verification
typed missing/foreign/mismatch/double-seal/unfinished failures
PHI failure attempts owned rollback and poisons the instance
entry/single/diamond/nested/Loop/multi-backedge/error fixtures = 12/12 green
C1 duplicate-edge and late-edge fixtures remain green
AST/source/name/ScopeId/RegionId/RegionFlow dependencies = 0
production Binding SSA callers = 0
accepted grammar delta = 0
```

### SSA-S2 — identity/value separation — closed

Refactor the canonical Lowerer behind the old production behavior:

```text
ResolvedIdentityLedgerV2 owns claims/lifetime only
old If value path remains the sole production value owner
Binding SSA production calls = 0
all current canonical fixtures remain green
```

This is Refactor Series Mode and adds no source grammar.

Closed evidence:

```text
ResolvedIdentityLedgerV2 owns exact claims, source coverage, and retirement only
PreSsaValueEnvironmentV1 is the one temporary BindingRef-to-ValueId owner
ledger ValueId / BasicBlockId / MirBuilder dependencies = 0
old If value path remains the sole production value owner
declaration adoption -> value publication -> coverage order is preserved
scope-success preflight and value-first retirement are behavior-neutral
scope-error cleanup remains value-first, best-effort, and idempotent
canonical_coverage/finish_mismatch tag and priority are preserved
old-map / Binding SSA synchronization bridges = 0
production Binding SSA callers = 0
focused behavior-equivalence fixtures = 2/2 green
all resolved_lowering focused fixtures = 50/50 green
accepted grammar delta = 0
```

### SSA-E0 — preserved terminal Return contract — closed

Before the owner cutover, seal only the already accepted function-body
terminal Return and implicit fallthrough completion cases:

```text
exact function target
exact terminal statement site
unreachable suffix count = 0
ordered crossed-scope cleanup obligations are explicit, including empty
implicit Void completion is represented separately and closes after SSA finish
nested If/Loop Return activation = 0
accepted source grammar delta = 0
```

This row preserves existing behavior; it does not authorize a new Return port.
General Return through If/Loop waits for the later EXIT rows.

Closed evidence:

```text
explicit root Return seals exact statement site and exact function target
explicit Value / explicit Void / implicit Void remain distinct forms
implicit completion seals and consumes exact root body/end/target
ordered crossed-scope cleanup is explicit and E0-empty-only
unreachable suffix count = 0
canonical Return bypasses the legacy defer-capable emitter
post-Lower ReadyFunctionCompletionV1 is required before finalization
explicit/implicit MIR Return terminators are exactly once
root nonterminal and nested If/Loop Return remain preflight rejects
completion product fixtures = 5/5 green
production completion fixtures = 6/6 green
all resolved_lowering focused fixtures = 56/56 green
92-row seam inventory and authority guard = green
production Binding SSA callers = 0
accepted grammar delta = 0
```

### SSA-S3 — disconnected carrier-free If control product — closed

Seal the future If-side control contract without changing production:

```text
exact If/IfThen/optional IfElse topology
else=None versus else=Some(empty)
fallthrough-only V1 ports
inseparable exact source coverage
one private exact-once coverage-use vocabulary
missing, duplicate, foreign, and wrong-order claims are typed errors
typed unsupported control errors
no effects, may_rebind, join-source rows, ValueId, or BasicBlockId
```

The historical A+ product remains the sole production If path until SSA-I1-T.
Do not run both analyzers as production authorities for one function.

Closed evidence:

```text
every semantic statement-If site has exactly one source-preorder row
If/IfThen/optional IfElse topology is queried from the sealed owner product
else=None and else=Some(empty) remain distinct typed ports/topologies
fallthrough port carries zero binding-effect data
each row co-seals one exact nonempty outer statement range and coverage
nested If coverage is an exclusive function-level partition with no overlap
condition BlockExpr closes its exact prelude/tail coverage around child rows
coverage-use Missing/Duplicate/ForeignOwner/WrongOrder are typed failures
generic coverage verifier internal family consumers = 1
new If analyzer production callers = 0
old A+ If production authority remains unchanged
BindingRef effect/join/carrier rows and MIR identities = 0
focused fixtures = 9/9 green
92-row SSA seam inventory remains unchanged and authority guard = green
accepted grammar and production behavior delta = 0
```

### SSA-M0 — disconnected real-MIR Binding SSA adapter — closed

Connect the closed SSA-S1 algorithm to real MIR/PHI lifecycle types without a
production canonical caller:

```text
BindingSsaIrV1 -> MirBuilder/PhiTxn adapter
CanonicalCfgSessionV1 VerifiedPredecessorsV1 -> Binding SSA seal
provisional PHI facts remain conservative unknown while open
patched PHI facts also remain conservative unknown
accepted fact refinement set = empty
Return and every other touched block can be sealed by the same facade
production Binding SSA and adapter callers = 0
accepted grammar delta = 0
```

This card prevents SSA-I1-T from mixing a new physical MIR adapter with the
whole-owner authority cutover.

Closed evidence:

```text
one borrowed MirBindingSsaAdapterV1 owns only MirBuilder/PhiTxn mechanics
real provisional PHI exists before an open-header read is exposed
open and patched PHI facts remain MirType::Unknown
CanonicalCfgSessionV1 witnesses are the only predecessor/seal input
non-dominating sibling and unreachable predecessor inputs fail without repair
only still-pending provisional PHIs are individually rolled back
already-patched peers remain in a poisoned unpublished draft
rollback failure retains primary plus cleanup failure before draft discard
entry, Loop header, merge, and Return use the same seal facade
Binding SSA finish precedes PhiTxn commit and CFG finish
focused real-MIR fixtures = 5/5 green
production Binding SSA and adapter callers = 0
accepted grammar and production behavior delta = 0
```

### SSA-RC0 — ownership and scope-escape law — active

#### SSA-RC0-D0 — explicit Ownership SSA decision — closed

A′ is accepted:

```text
CopyOwned(dst, src):
  src remains valid; fresh dst owns one independently consumable token

DestroyOwned(value):
  consume exactly the named Owned token

Copy:
  ownership-neutral

ReleaseStrong:
  legacy only; canonical caller count must reach zero before retirement
```

The local VM already destroys only the named register, so no pointer-sweep
deletion task exists. This does not make `ReleaseStrong` canonical Ownership
SSA: its vector vocabulary, reference contract, Wasm no-op, and missing
forward/consume verifier remain incompatible with A′.

The accepted decision and local correction are fixed in
`mirbuilder-ssa-rc0-owned-alias-materialization-design-stop-2026-07-14.md`.
Production Binding SSA and ownership-op callers remain zero.

#### SSA-RC-L0a — instruction-diet ledger repair — closed

The L0 focused-test baseline exposed a pre-existing contract drift introduced
when `ArrayStateContractClaim` joined the kept vocabulary:

```text
actual kept tags = 41
actual removed tags = 16
actual vocabulary = 57
stale test/docs literals = 40 / 16 / 56
```

Repair only the test and machine-readable MIR reference counts. The enum,
cohorts, allowlists, schema, runtime behavior, grammar, and backend support do
not change. This is a separate prerequisite commit; do not mix it with the
physical L0 split.

Closed evidence:

```text
implementation/test/reference ledger = 41 / 16 / 57
cargo test -q --lib mir::contracts::backend_core_ops::tests::instruction_diet_ledger_counts_match -- --nocapture
  2 passed
enum/cohort/allowlist/schema/runtime/grammar/backend delta = 0
```

#### SSA-RC-L0b — VM-reference guard feature alignment — closed

The L0 quick gate exposed a pre-existing false-green in the static-const-table
load guard. Its two owned tests moved behind `feature = "vm-reference"` in
`9111364a11`, but the guard still invokes the filter without that feature:

```text
cargo test -q --lib static_const_table_load
  running 0 tests
  exit 0
```

Repair only the dedicated guard invocation so it explicitly enables
`vm-reference` and runs the three existing module tests. Do not change source grammar,
MIR behavior, backend capability, the shared test helper, or production code.
This prerequisite lands separately while the physical L0 split remains in a
named stash.

Closed evidence:

```text
bash tools/checks/k2_wide_static_const_table_load_guard.sh
  running 3 tests
  3 passed
production/source/MIR/backend delta = 0
```

#### SSA-RC-L0 — ownership transport seam split — closed

Before adding opcodes, split two near-stop files by existing responsibility in
one behavior-neutral BoxShape commit:

```text
src/mir/contracts/backend_core_ops.rs (780 lines):
  facade
  vocabulary/diet
  backend allowlists
  focused tests

src/runner/mir_json_v0.rs (749 lines):
  facade
  lifecycle opcode parser
  existing family helpers/tests
```

Every resulting source file remains below 800 lines. Public APIs, kept/removed
counts, JSON behavior, backend support, accepted grammar, and production
behavior remain unchanged. Do not touch the 796-line public
`resolved_region_flow_authority_guard.sh`.

Closed evidence:

```text
backend_core_ops facade / vocabulary / allowlists / tests = 8 / 178 / 235 / 364 lines
mir_json_v0 facade / lifecycle parser = 742 / 34 lines
backend_core_ops focused fixtures = 20/20 green
mir_json_v0 focused fixtures = 10/10 green
static-table feature-gated fixtures = 3/3 green
resolved authority guard = green
cargo fmt --check = green
cargo build --release --bin hakorune = green
dev_gate quick = 66/66 green
public API / kept-removed counts / JSON / backend / grammar / behavior delta = 0
production Binding SSA / ownership opcode callers = 0
all new or modified source/check files below 800 lines
```

#### SSA-RC-L1 — Rust interpreter frame transaction — closed

Before ownership opcodes introduce new typed executor failures, close the
existing function-frame lifecycle in a separate BoxShape row:

```text
one closure-scoped frame transaction
success and every error restore caller regs and current function
fast slots, alias/cache state, step/phi failures covered
primary execution error preserved if restoration also fails
no opcode, grammar, or accepted success-path behavior change
```

This row does not add `CopyOwned`/`DestroyOwned` and is not backend ownership
activation.

Closed evidence:

```text
exec facade / frame transaction / function loop / focused tests = 40 / 291 / 189 / 230 lines
closure-scoped transaction fixtures = 6/6 green
existing return-contract fixtures = 12/12 green
success / instruction / PHI / step-budget / missing-block restoration = green
primary execution plus restoration failure retention = green
cargo fmt --check = green
cargo build --release --bin hakorune = green
dev_gate quick = 66/66 green
opcode / grammar / accepted success behavior / ownership activation delta = 0
all new or modified source/check files below 800 lines
```

#### SSA-RC-P0 — exact ownership production profile — closed

Seal a machine-checked value-origin/storage matrix before passive vocabulary:

```text
BoxRef:
  ownership-managed after exact static witness

InlineI64 / InlineBool / InlineF64:
  trivial; reuse ValueId; ownership ops 0

BorrowedText / Array / Future / WeakRef / Void / Opaque / Unknown:
  typed preflight reject before Builder effects
```

Inventory every currently accepted receiver, parameter, local, Outbox,
literal, PHI, BlockExpr-tail, call argument, and call result origin. The
current untyped parameter/call path is not silently treated as `BoxRef`;
SSA-I1-T excludes it from the first closed profile until an
independently sealed representation and caller/callee ABI witness.
`StorageClass` is currently an inventory, not an execution proof, and
`MirType::Box` metadata does not by itself prove the same ABI class on every
JSON/backend path. P0 must promote or derive one sealed ownership
representation witness and require it at direct JSON ingress too. Its schema,
v0/v1 parse, round-trip, and type/storage mismatch rejection land with A0.
This row activates no ownership instruction and changes no grammar.

Closed evidence:

```text
machine profile rows = 17/17
trivial exact / derived trivial-only / typed reject / absent = 3 / 7 / 4 / 3
exact BoxRef source producers = 0
StorageClass::BoxRef remains inventory-only
generic BoxRef representation facts = 0
JSON storage inventory emitter / verified v0 ingress = 1 / 0
production CopyOwned / DestroyOwned callers = 0 / 0
production ownership activation = 0
first SSA-I1-T cutover profile = trivial-only until SSA-I1-O1
resolved authority guard = green
cargo fmt --check = green
cargo build --release --bin hakorune = green
dev_gate quick = 66/66 green
all new or modified source/check files below 800 lines
```

Evidence card:
`mirbuilder-canonical-ownership-production-profile-2026-07-14.md`.

#### SSA-RC-A0 — passive Ownership SSA MIR vocabulary

Add `CopyOwned { dst, src }` and singleton `DestroyOwned { value }` with:

```text
effect mask = WRITE
is_mut = true
parallel_safe / moveable / pure = false
printer, tag, dst/used-values, ID remapper
JSON emitter/schema, v0 parser, v1 bridge, and round-trip
storage/type propagation and verifier shape
backend opcode diets and transport-only classification
```

Do not consume the final free `u16` effect bit without a named optimizer or
analysis consumer. The instruction variants plus Ownership SSA verifier own
the semantic distinction; conservative `WRITE` prevents generic DCE/CSE and
matches the current lifecycle safety boundary. A dedicated effect remains a
separate future decision. Expected opcode ledger after A0:

```text
kept = 43
removed = 16
vocabulary = 59
```

Production callers, VM execution, and canonical behavior remain zero. The
historical 92-row seam inventory and its 7 RC rows stay hash-stable.

Closed evidence:

```text
passive instruction variants = 2
effect = conservative WRITE
instruction-diet ledger = 43 kept / 16 removed / 59 total
MIR JSON emitter / v0 parser / v1 bridge = present
direct JSON ownership witness = exact MirType::Box + StorageClass::BoxRef
storage/type mismatch rejection = green
transport round-trip fixtures = green
production CopyOwned / DestroyOwned callers = 0 / 0
VM / LLVM execution semantics = 0 / 0
canonical behavior and grammar delta = 0
all new or modified source/check files below 800 lines
resolved authority guard = green
cargo build --release --bin hakorune = green
dev_gate quick = 66/66 green
```

Evidence card:
`mirbuilder-ssa-rc-a0-passive-ownership-vocabulary-2026-07-14.md`.

#### SSA-RC-A1a — Rust ownership-op handlers

Implement the fixed meaning for the temporary Rust semantic oracle only:

```text
Rust MIR interpreter:
  BoxRef clone into fresh dst; exact register take on DestroyOwned
  non-BoxRef input is a typed error, never the legacy silent skip
  preflight rejects an already-defined CopyOwned dst
```

This row implements only explicit opcode handlers over the closed L1 frame
transaction. Ordinary Phi, Return, and parameter/result ABI behavior remains
unchanged until the verified ownership classification exists. Production
canonical callers remain zero.

Closed evidence:

```text
Rust MIR interpreter ownership handlers = 2
CopyOwned = exact BoxRef Arc clone into an undefined destination register
DestroyOwned = exact named-register take; same-object aliases remain live
non-BoxRef CopyOwned / DestroyOwned = typed error
already-defined CopyOwned destination = contract error before write
focused vm-reference fixtures = 5/5 green
backend contract fixtures = 21/21 green
ownership production profile = 17/17 green
production CopyOwned / DestroyOwned callers = 0 / 0
LLVM / Wasm / Hako interpreter ownership execution = 0 / 0 / 0
all new or modified source/check files below 800 lines
resolved authority guard = green
cargo build --release --bin hakorune = green
dev_gate quick = 66/66 green
```

Evidence card:
`mirbuilder-ssa-rc-a1a-rust-ownership-handlers-2026-07-14.md`.

#### SSA-RC-V0 — Ownership SSA verifier and forwarding

Add a verifier-owned classification, not a second reaching-value map:

```rust
enum MirOwnershipKindV1 {
    None,
    Borrowed,
    Owned,
}
```

Seal the result as `VerifiedOwnershipSsaV1`, containing owner-branded
parameter/result roots, the exact ValueId ownership kind, and verified
consuming/forwarding dispositions. The verifier, interpreter, and codegen
consume this one artifact; they do not rebuild ownership from runtime values.

Close these unconditional laws with edge/path-sensitive live-owned dataflow,
not a static instruction-count check or environment toggle:

```text
CopyOwned:
  strong-ownable src, fresh Owned dst, non-consuming src use

DestroyOwned:
  exact one Owned consuming use

Phi incoming / Return:
  forward the selected token; retain 0

canonical edge arguments:
  V1 requires absent; Phi.inputs is the sole edge transfer vocabulary

function parameter/result ABI:
  explicit Borrowed/Owned convention or typed rejection

receiver/call argument/result ABI:
  caller and callee witness must agree or the managed call shape is rejected

Borrowed V1:
  sealed ABI root only
  Borrowed Phi/Return/edge escape forbidden
  CopyOwned is the only conversion to an independent owner

canonical Copy on Owned:
  reject

duplicate consume / use after consume / reachable path without disposition:
  reject
```

##### Owned Phi is an edge-indexed forwarding consume

Do not verify an Owned Phi by counting all of its incoming operands as if they
execute together in the merge block. Each incoming is a consuming forwarding
use on one exact CFG edge:

```rust
struct OwnedPhiEdgeTransferV1 {
    predecessor: BasicBlockId,
    successor: BasicBlockId,
    source: ValueId,
    destination: ValueId,
}
```

The verifier derives this private transfer view from the already-verified CFG
and `Phi.inputs`; it is not a second public edge-argument vocabulary. For an
executed `predecessor -> successor` edge, all matching Phi sources are consumed
in parallel and all Phi destinations become Owned at successor entry. Inputs
for unselected predecessor edges do not execute. They belong to different
source lifetimes whose definitions and forwarding consumes are checked on
their own reachable paths.

This gives the following laws:

```text
one executed incoming edge:
  exactly its Phi source is forwarded; retain 0

unselected incoming edge:
  no runtime token and no runtime consume on this execution

one pre-branch Owned source forwarded on two alternative edges:
  legal when the two consumes are mutually exclusive and jointly close every
  reachable path from that source

two consumes on the same executed edge:
  reject

source live on an edge with neither forwarding nor DestroyOwned/Return:
  reject as a missing disposition

Phi destination:
  one new Owned SSA lifetime begins at merge entry and must itself be consumed
  or forwarded exactly once on every finite reachable exit path
```

`VerifiedOwnershipSsaV1` therefore records edge-branded forwarding
dispositions. Verification uses path-sensitive linear-lifetime closure:
consuming uses must not be reachable from one another and must jointly
post-dominate the definition and every non-consuming use. A raw global
`ValueId -> consume_count` ledger is insufficient and forbidden.

Required focused fixtures:

```text
then-owned / else-owned -> one Owned Phi result
pre-branch Owned source forwarded by two mutually-exclusive incoming edges
one branch forwards while the other destroys
one branch has no disposition -> reject
same source forwarded twice on one edge -> reject
Phi source used after its forwarding edge -> reject
loop header Phi with entry/backedge forwarding
multiple Owned Phis that swap values -> parallel transfer, no sequential take
unreachable predecessor or non-CFG predecessor -> reject
```

At every finite reachable function exit, every Owned token is consumed or
forwarded exactly once. An infinite path may keep a live token; this is not a
missing consume. Unreachable ownership blocks are rejected in V1.

Generic DCE/CSE/copy propagation/CFG rewrites preserve ownership effects and
may not merge `CopyOwned`, rewrite it to `Copy`, or duplicate consuming uses.

Closed evidence:

```text
physical owner = src/mir/ownership_ssa/
MirOwnershipKindV1 = None / Borrowed / Owned
sealed product = owner-branded VerifiedOwnershipSsaV1
path state = exact live-Owned ValueId set per reachable block entry
Owned Phi = exact predecessor-edge parallel consume/forward
global consume-count authority = 0
second BindingRef -> ValueId map = 0
canonical edge arguments = typed reject
unreachable blocks = typed reject
unwitnessed managed call ownership = typed reject
focused fixtures = 17/17 green
production verifier callers = 0
interpreter Phi/Return/ABI forwarding delta = 0
all new or modified source/check files below 800 lines
resolved authority guard = green
cargo build --release --bin hakorune = green
dev_gate quick = 66/66 green
```

Evidence card:
`mirbuilder-ssa-rc-v0-ownership-verifier-2026-07-14.md`.

#### SSA-RC-A1b — Rust Owned forwarding and ABI

Consume `VerifiedOwnershipSsaV1` in the Rust interpreter. Owned Phi is a
parallel move: collect selected inputs, take consumed source registers, then
publish destinations. Return and Owned parameter/result transport move rather
than clone. Borrowed roots obey V0 and cannot escape without `CopyOwned`.

Closed evidence:

```text
explicit verified function session = 1
sealed witness install/restore = success and typed error
Owned parameter transport = move from owned argument vector
Owned Phi = selected sources collected/taken before destination publication
Owned Return = exact register take
ordinary Phi/Return path = unchanged without installed witness
foreign owner = rejected before function-frame effects
focused fixtures = 3/3 green
existing function-frame fixtures = 6/6 green
canonical production callers = 0
unsupported backend activation = 0
all modified source/check files below 800 lines
```

Evidence card:
`mirbuilder-ssa-rc-a1b-rust-owned-forwarding-2026-07-14.md`.

#### SSA-RC-A1c — exact llvm_py + nyash_kernel materialization

Implement the fixed meaning only for the pinned handle provider:

```text
llvm_py / llvmlite object lane:
  strict BoxRef handle + VerifiedOwnershipSsaV1 witness
  nyrt_handle_retain_h -> fresh dst
  nyrt_handle_release_h(value)

runtime provider:
  nyash_kernel only
```

The root proof-of-concept shim, llvm_py PyVM harness, Wasm, `.hako` reference
interpreter, native llvmc, archived Cranelift/JIT, and every unproved consumer
fail a typed `owned-value-lifecycle-v1` capability preflight. In particular,
the PyVM unknown-op skip must not hide either opcode. No backend may lower an
ownership opcode as a no-op or silently map old `release_strong` JSON to
`destroy_owned`. Production canonical callers remain zero.

Closed evidence:

```text
strict transported VerifiedOwnershipSsaV1 preflight = before LLVM effects
Rust ownership witness transport emitters = 1
sealed ABI/operation inventory = witness-owned, not emitter-reconstructed
post-seal CFG/Phi/Return/ownership-op mutation = full-product preflight reject
llvm_py CopyOwned = nyrt_handle_retain_h fresh result
llvm_py DestroyOwned = exact nyrt_handle_release_h
accepted provider = nyash_kernel only
PyVM ownership unknown-op skip = 0
unsupported backend capability rejection = explicit
Python focused fixtures = 4/4 green
Rust backend preflight fixtures = 3/3 green
Rust JSON transport fixture = 1/1 green
backend core-op fixtures = 21/21 green
nyash_kernel handle lifecycle fixtures = 3/3 green
canonical production callers = 0
all touched source/check files <= 672 lines
release build = green
dev_gate quick = 66/66 green
```

Evidence card:
`mirbuilder-ssa-rc-a1c-llvm-py-ownership-2026-07-14.md`.

#### SSA-RC-RET-P0 — legacy ReleaseStrong inventory and isolation

Status: closed

Create a separate machine ledger for every `ReleaseStrong` producer, consumer,
opcode surface, document, pass, and fixture. Do not mutate the historical
92-row SSA seam evidence. Classify each row as:

```text
canonical caller-zero delete
legacy builder isolate
optional RC insertion isolate
optimizer/CFG rewrite isolate until ownership-profile preservation is proven
backend/JSON compatibility isolate
dead after repository caller zero
```

Connect a private ownership helper beneath
`tools/checks/lib/resolved_binding_ssa_contract.sh`; add no public guard and do
not grow the 796-line authority guard.

Closed evidence:

```text
tracked surfaces = 118
exact token occurrences = 266
canonical caller-zero delete = 1
legacy builder isolate = 4
optional RC insertion isolate = 3
optimizer/CFG rewrite isolate = 11
backend/JSON compatibility isolate = 31
dead after repository caller zero = 68
historical 92-row inventory mutations = 0
semantic/backend/opcode/JSON/caller delta = 0
new public guards = 0
new/modified source and check files <= 217 lines
authority guard = green
release build = green
dev_gate quick = 66/66 green
current-state pointer guard = green
```

Evidence card:
`mirbuilder-ssa-rc-ret-p0-legacy-release-inventory-2026-07-14.md`.

#### SSA-RC0 — disconnected ownership transition planner

Status: closed

Seal the bounded ownership contract before production Binding SSA activation:

```text
assignment materializes next before destroying old and installing definition
local/Outbox declaration copies a borrowed strong initializer or transfers an
owned initializer
exact BindingRef self-assignment emits ownership ops 0
successful scope exit reads and destroys the current reaching value
BlockExpr tail/current aliases transfer or CopyOwned exactly once
outer-binding and scope-local tail cases remain distinct
unpublished draft discard emits no duplicate runtime cleanup
local/parameter/receiver versus Upvar/cell/place storage stays separated
remaining locals destroy in reverse source declaration order
terminal Owned Return transfers and leaves the destroy set
terminal BorrowedStrong Return materializes CopyOwned before transfer
Void/fallthrough destroys every current Owned root in reverse declaration order
return result ownership matches the sealed function ABI profile
```

The disconnected pure planner emits typed plans, not `MirInstruction`, and
allocates no `ValueId`. It distinguishes `Trivial`, `Owned`, and
`BorrowedStrong` provenance. This row activates neither Binding SSA nor new
source grammar and does not claim a whole-language ownership solution.

Closed evidence:

```text
closed local-binding classes = Receiver / Parameter / Local / Outbox
closed value provenance = Trivial / Owned / BorrowedStrong
exact BindingRef self-assignment authority = 1
raw ValueId self-assignment authority = 0
materialization order = next -> commit -> previous destroy
scope/function destroy order = reverse source declaration order
MirBuilder / MirInstruction / BasicBlockId imports = 0
BindingRef-to-ValueId planner maps = 0
ValueId allocation / MIR emission = 0
production callers / grammar / runtime activation = 0
focused fixtures = 18/18 green
resolved-lowering group = 74/74 green
authority guard / release build / quick 66/66 = green
largest source/check file = 500 lines
```

Evidence card:
`mirbuilder-ssa-rc0-ownership-transition-planner-2026-07-14.md`.

### SSA-I0-PROFILE — disconnected exact trivial owner profile

Status: closed

Seal one executable whole-owner proof before any production Binding SSA
effect. `src/mir/resolved_value_profile` owns the closed
`InlineI64`/`InlineBool`/`InlineF64` vocabulary, exact value and definition
coverage, homogeneous If merge profiles, and explicit terminal dispositions.

The analyzer is co-sealed with:

```text
ResolvedFunctionLoweringInputV1
VerifiedFunctionCompletionV1
VerifiedResolvedFunctionIfControlV1
```

The carrier-free If product contributes only exact control topology and
coverage. The trivial profile records homogeneous representation at the
merge, but never decides whether a PHI exists or where Binding SSA places it.

Closed evidence:

```text
profile manifest files = 8
focused profile fixtures = 10/10 green
carrier-free If control / compiler capability fixtures = 9/9 / 4/4 green
private profile validator = green
public authority guard / release build / quick 66/66 = green
literal authority = AST LiteralValue tag only
return; / implicit fallthrough = distinct no-value dispositions
parameters / receiver / Outbox / String / Void value / Null value = typed reject
profile / Binding SSA / Ownership SSA production callers = 0
CopyOwned / DestroyOwned production callers = 0
current A+ behavior and accepted grammar delta = 0
largest profile source file = 610 lines
largest private check helper = 468 lines
```

Evidence card:
`mirbuilder-ssa-i0-trivial-owner-profile-2026-07-15.md`.

### SSA-I1-T — atomic trivial-profile owner cutover — closed

In one production commit, select one whole source unit whose executable
`VerifiedTrivialCanonicalOwnerV1` is admitted and move that entire owner to
Binding SSA:

```text
zero receiver / parameter / Outbox value origins
initialized local declarations with an exact trivial profile
variable reads
binding assignments with exact trivial propagation
straight-line statements
BlockExpr
fallthrough statement If, including nested If
homogeneous trivial merge profiles, without precomputed PHI rows
exact trivial Return value, explicit return;, or implicit fallthrough
```

If remains a family-specific CFG/semantic box. It stops querying effect sets
or join-source rows. Both branches use block-local definitions, merge closes
after every predecessor is known, and later `read` creates only required PHIs.

Route selection happens once before Builder effects:

```text
TrivialBindingSsa profile or temporary CurrentCanonicalAPlus profile
one source unit / one owner / one selected value authority
no body/site-level mixing
no retry to A+ after a Binding-SSA failure
Binding-SSA canonical finalization skips legacy insert_rc_instructions
```

Atomic acceptance:

```text
SSA-M0, SSA-RC0, and SSA-I0-PROFILE are closed
all admitted trivial If/BlockExpr runtime fixtures green
all canonical declaration/read/rebind operations use Binding SSA
then definitions do not leak into else compilation state
scope leave retires identity without deleting historical SSA definitions
self-assignment uses exact BindingRef provenance, not raw ValueId equality
trivial BlockExpr tail forwarding fixtures are green
selected Binding-SSA route legacy ReleaseStrong calls = 0
selected Binding-SSA route optional RC double-destroy paths = 0
legacy insert_rc_instructions calls on the selected route = 0
flat value-map merge authority calls = 0
canonical If may_rebind/join-source queries = 0
manual branch snapshot/restore = 0
canonical materialize_all_phi_inputs repair calls = 0
co-sealed control coverage is consumed exactly once before finish
coverage finish before candidate function publication
Return and every touched block seal through the C1 witness
function verifier green before publication
canonical failure legacy retry = 0
production Ownership SSA witness/install/verifier calls = 0
production CopyOwned / DestroyOwned callers = 0
```

No Loop syntax or ownership-managed value is activated in this commit.

Closed evidence:

```text
pre-Builder whole-unit route match = exactly one
production BindingSsaBuilderV1 caller files = 1
production MirBindingSsaAdapterV1 caller files = 1
production CanonicalCfgSessionV1 caller files = 1
located resolved-lowering fixtures = 75/75 green
capability / finish schedule = 5/5 / 2/2 green
trivial profile / carrier-free If control = 10/10 / 11/11 green
VM-reference nested/fallthrough If = 6/6 green
exact Float BinOp / selected-route ReleaseStrong = green / 0
selected Binding-SSA route legacy RC insertion = 0
production Ownership SSA / CopyOwned / DestroyOwned activation = 0
accepted grammar delta = 0
largest new production source file = 605 lines
private SSA-I1-T authority validator = green
```

Evidence card:
`mirbuilder-ssa-i1-t-trivial-binding-ssa-cutover-2026-07-15.md`.

### SSA-I1-COMPAT — representation/ABI compatibility rows

Close one currently accepted representation family per bounded row:

```text
parameter and receiver ABI
Outbox / Void disposition
BorrowedText
Null
```

Each row either seals an executable representation/ABI witness or remains a
typed whole-unit profile rejection. It may not add a body-level bridge between
A+ and Binding SSA.

Selected first row:

```text
SSA-I1-COMPAT-N0a:
  exact LiteralValue::Null -> NullSentinel profile
  existing ConstValue::Null / MirType::Void / runtime no-value representation
  local/read/assignment/BlockExpr/homogeneous If only
  terminal result remains InlineBool or existing no-value completion
  ownership/call ABI/backend vocabulary delta = 0
```

Selection card:
`mirbuilder-ssa-i1-compat-null-sentinel-selection-2026-07-15.md`.

Implementation evidence:
`mirbuilder-ssa-i1-compat-null-sentinel-implementation-2026-07-15.md`.

N0a is closed with 12/12 profile fixtures, 3/3 focused VM/reference fixtures,
18/18 production-profile inventory rows, ownership operations zero, release
build, authority guard, and quick 66/66 green.

The remaining rows stay separate: exact typed parameters do not include the
receiver owner family; Void disposition does not imply Outbox identity;
BorrowedText requires its own lifetime/ABI decision.

### SSA-I1-FULL — current canonical A+ caller-zero cutover

After every currently accepted canonical source unit has an admitted
Binding-SSA profile, route the whole family through Binding SSA and prove:

```text
temporary A+ production callers = 0
unit-internal profile mixing = 0
canonical retry/fallback = 0
one BindingRef -> ValueId authority for the whole current family
```

Physical retirement of old A+ value/effect authority remains SSA-R1 and may
start only after this caller-zero proof.

### SSA-I1-O1 — first exact BoxRef Ownership SSA activation

After one exact source `BoxRef` producer and its caller/result ABI witness are
sealed, atomically activate that one closed owner profile:

```text
production CopyOwned/DestroyOwned callers > 0 only inside the selected owner
VerifiedOwnershipSsaV1 consumed by verifier and supported backends
declaration/assignment/BlockExpr/Return ownership plans all active together
Owned Phi/call shapes either fully verified or rejected before Builder effects
legacy ReleaseStrong and optional RC insertion on the same owner = 0
```

This is the first row allowed to claim production Ownership SSA. It adds no
new source syntax and does not broaden beyond the exact producer/profile.

### SSA-RC-RET-R1 — canonical legacy ownership isolation

After SSA-I1-FULL, prove `ReleaseStrong` canonical callers are exactly zero. Keep
remaining legacy builder, optional RC insertion, JSON compatibility, and
backend callers behind explicit legacy provenance. Do not change opcode
meaning or delete repository vocabulary in this row.

### SSA-RC-RET-R2 — physical ReleaseStrong retirement

Only after repository-wide exact producer/consumer zero, remove
`ReleaseStrong` from MIR enum, printer, JSON/schema, backend handlers,
reference docs, and opcode diets. Expected final ledger if no replacement op
is removed:

```text
kept = 42
removed = 17
vocabulary = 59
```

This row is not implied by canonical caller zero.

### SSA-R1 — retire old If value-flow authority

After SSA-I1-FULL is green and exact production caller counts are zero, physically
delete the old canonical If effect/join products and branch snapshot
transaction: condition/whole-effect summaries, `may_rebind_outer`, join-source
rows, and the effect-driven PHI materializer. Temporary isolation is not a
completion state. Keep exact If topology, source coverage, semantic stacks,
predecessor checks, and the runtime fixtures.

Also require exact caller/definition zero for:

```text
PreSsaValueEnvironmentV1
BranchValueStoreV1 / DefinedJoinValueStoreV1 adapters
old active-effect stack
old manual join publication
old resolved_region_flow value/effect transport shell
every old-environment / Binding SSA synchronization bridge
```

Production behavior delta: zero; authority count decreases.

### B0-L4-S3′ — carrier-free Loop control contract

Add a disconnected `VerifiedLocatedLoopControlV1` owning only:

```text
exact Loop statement site
closed Loop/LoopBody bundle from S1
condition-false-only V1 topology
fallthrough-only body proof
inseparable exact located coverage from S2′
typed unsupported control errors
```

It contains no `may_rebind`, carrier rows, ValueId, BasicBlockId, or source
matrix. Nested If/Loop and nonlocal exits remain preflight rejects in this
first product.

Do not recreate the old `SharedPostState` restriction. Outcome-dependent
binding state is a CFG/SSA concern. A short-circuit condition may be rejected
in the first slice only when the current exact expression/control grammar
cannot lower its CFG, never because a carrier/effect row is unavailable.

Production Loop activation: zero.

### B0-L4-I1′ — disconnected Loop CFG transaction

Build the family-specific CFG box over the common canonical CFG and Binding
SSA substrate:

```text
preheader
header, open until every backedge exists
body entry
latch
after, open until every false/break edge exists
transaction-local RegionId roles
LoopBody exact scope/region enter/leave around body only
```

The condition is lowered outside the LoopBody pair on every runtime
iteration. No carrier snapshot, restore, or publication API exists.

Production Loop activation: zero.

### B0-L4-I2′ — atomic first canonical Loop

Connect S1 + S2′ + S3′ + I1′ in one production commit.

Accepted first grammar:

```text
statement Loop
condition-false external exit only
condition/tail/RHS expressions:
  Literal
  Variable
  eager non-And/Or BinaryOp
  closed BlockExpr over the same expression set
body statements:
  local declaration
  BindingRef assignment
  expression statement from the closed set
zero or more runtime iterations
```

Preflight rejects before Builder effects:

```text
And / Or short-circuit control
Call / MethodCall / FunctionCall
Outbox
nested If / Loop
Break / Continue / Return
LoopRange / ForRange
QMark / Throw / Try
Lambda execution
every expression/statement kind not listed above
```

The grammar list is fixed by S3′ fixtures. I2′ may not inherit newly landed
expression capabilities implicitly.

Required runtime fixtures:

```text
zero / one / multiple iterations
condition-only, body-only, and combined outer rebind
multiple independent bindings
binding with no downstream read does not require a predeclared carrier/PHI row
condition BlockExpr outer rebind reaches body and after
condition/body locals and same-name shadow do not leak
after-loop read receives the correct SSA value
header seals only after latch edge
after seals only after every V1 exit edge
actual/cached predecessor equality
all PHIs pass CFG, SSA, dominance, and RC verification
VM/reference result parity
```

Legacy `LoopRouteContext`, current `CorePlan`, normalization suffix, name
lookup, map diff, and retry counts remain zero on the canonical route.

### N1 — If inside Loop

Add exactly one fallthrough nesting shape:

```text
If in Loop
BlockExpr at every condition/body boundary
```

The inner If merge seals before the Loop latch consumes its reaching
definitions. No family effect summary is propagated.

### N2 — Loop inside If

Add exactly one fallthrough nesting shape. The inner Loop after block seals
before the outer branch merge reads its definitions.

### N3 — Loop inside Loop

Add exactly one fallthrough nesting shape. The inner Loop after definitions
feed the outer latch/backedge through the same function SSA instance.

### N4 — bounded depth-independent nesting proof

Add no new syntax. Fix bounded witnesses such as:

```text
Loop -> If -> Loop -> If
same-name shadows at multiple depths
condition BlockExpr at nested boundaries
error cleanup at each child session boundary
```

Only after N4 may the supported If/Loop grammar claim finite nesting under the
same depth-independent rules.

### EXIT-S0 — semantic exit, cleanup, and disposition contract

Before adding a nonlocal exit shape, seal a disconnected pre-Builder
`ResolvedExitCleanupContractV1` owning only:

```text
exact source exit site and typed port kind
exact target RegionId or function region
ordered crossed-scope cleanup obligations
unreachable source disposition:
  Materialized
  SkippedAfterTerminator
  OwnedByChildFunction
ValueId / BasicBlockId / transaction-local block roles = 0
new exit production activation = 0
existing Binding SSA production remains unchanged
```

### EXIT-S1 — disconnected Lower target-role registry

Add `ActiveControlTargetsV1` under resolved Lowering as a separate,
transaction-local materialization registry:

```text
RegionId -> accepted Continue/Break target roles
function region -> Return target role
ordered cleanup emission cursor
durable publication = 0
pre-Builder semantic ownership = 0
new exit production activation = 0
```

It may hold materialized block roles because it is Lower-owned. It never
becomes resolver/RegionFlow authority and is discarded with the function
transaction.

### EXIT-S2 — multi-completion and family-port upgrade

General exits cannot reuse the single root-terminal E0 enum. Before an EXIT-Ix
activation, co-seal a disconnected product that can represent:

```text
zero or more explicit exact exits plus optional implicit fallthrough
If fallthrough / Return reachable port variants
Loop false / Continue / Break / Return reachable port variants
family topology + exact cleanup + unreachable disposition as one product
zero / one / two reachable predecessor contracts without fabricated values
ValueId / BasicBlockId / materialized target roles = 0
```

Each EXIT-Ix atomically connects only the needed closed port variant and its
Lower behavior. Partial bools and independently recombined exit sidecars are
forbidden.

### EXIT-I1 — Continue from the current Loop body

Activate one shape: straight-line Continue in the current Loop body, targeting
the exact current Loop RegionId. The Loop CFG contract selects its continue
role; Binding SSA observes only the emitted edge.

### EXIT-I2 — Continue through nested If

Activate one shape: Continue inside an already-supported nested If branch to
that If's enclosing current Loop. Prove branch cleanup and exact RegionId
routing; do not add labeled/outer-Loop syntax.

### EXIT-I3 — Break from the current Loop body

Activate one shape: straight-line Break in the current Loop body to its exact
after role. Keep the after block open through every accepted current-Loop exit,
then seal once.

### EXIT-I4 — Break through nested If

Activate one shape: Break inside an already-supported nested If branch to that
If's enclosing current Loop. Prove branch cleanup and exact predecessor
accounting; do not add labeled/outer-Loop syntax.

### EXIT-I5 — Return through If

Activate one shape: Return from a statement-If branch to the exact current
function region, with its sealed cleanup obligations. Cover one- and
zero-reachable branch merge cases without fabricating a value for unreachable
source.

### EXIT-I6 — Return through Loop

Activate one shape: Return from the current Loop body to the exact current
function region. Cover the remaining condition-false path and Return path
without forcing unreachable declarations into ValueIds.

### EXIT-I7 — nested exit closure proof

Add no syntax or port kind. Combine only already activated shapes to prove:

```text
Continue/Break in nested If inside Loop
Return through nested If/Loop compositions
cleanup order is inner-to-outer exactly once
zero/one/two reachable predecessor handling
every unreachable declaration has one disposition
all resulting blocks seal from actual predecessor sets
```

Only after I7 may the supported grammar claim nested exit closure.

### EXIT-P0 — parked labeled or outer-Loop targets

Do not infer labeled Break/Continue or an inner-Loop-to-outer-Loop transfer
syntax from exact RegionId infrastructure. If the language later accepts such
a source form, open a separate language decision and one-shape implementation
row. Until then activation is zero.

QMark, Throw, and Try/Finally remain separate design-stop rows until their
language, resolver, and cleanup contracts are independently accepted:

```text
pre-Builder contract:
  exact target + ordered cleanup chain + reachable port kind

Lower:
  emit cleanup and exact CFG edge/terminator

Binding SSA:
  resolve values on the resulting CFG only
```

Unsupported syntax fails before Builder effects. Do not add partial bool or
Option-shaped port support.

### X0 — non-blocking parked three-family control-only extraction

This optional appendix never blocks F1, retirement, or canonical-source
completion. It opens only when its independent evidence gate is satisfied.

After If, Loop, and one independent third control family are production-closed,
inventory their landed code mechanically. Extract a private common envelope
only when all three prove identical ownership for:

```text
source coverage lifetime
owner/source closure
typed control port vocabulary
cleanup/rollback/commit lifecycle
```

Effect ordering, may-rebind sets, carriers, and family PHI lifecycles are not
extraction candidates. If the three families do not prove a smaller useful
envelope, keep the family wrappers separate.

### F0 — whole-unit canonical capability closure matrix

Before broad owner expansion or the default route switch, inventory every
ordinary-source capability against an explicit disposition:

```text
source owner kind and child-owner worklist closure
statement / expression / control family
required resolver, control, cleanup, SSA, RC, and backend capability
canonical supported
explicit legacy-only: ProgramV0 / REPL
separate language or design decision
typed unsupported before Builder effects
```

The matrix is exhaustive and guarded. It defines the compatibility threshold
for ordinary-source cutover; “whatever current preflight accepts” is not a
self-justifying completion condition. Any missing capability becomes one
bounded `G1x` row rather than silently widening an F1/F2 commit.

### F1a — instance method and constructor owner family

Cut over one closed receiver-bearing owner capability set. Receiver,
parameters, locals, reads, writes, RC, SSA finish, and publication switch
atomically; accepted control grammar does not widen in this row.

### F1b — source entry owner family

Cut over one closed source-entry owner capability set without changing the
synthetic wrapper policy or adding Main/Lambda behavior.

### F1c — Main.main owner plus entry thunk

Lower source `Main.main` exactly once as one source owner. The synthetic entry
is a call-only thunk; inline and callable copies of the same source body are
forbidden.

### F1d — Lambda child owner family

Open only after capture mode, cell/slot layout, child-owner transport, and
Upvar storage authority are independently accepted. Cut over the complete
parent/child source unit atomically; parent canonical / child legacy is
forbidden.

F0 must explicitly classify whether F1d is required by the selected F2
compatibility threshold or remains a typed unsupported capability. It cannot
silently block the roadmap or be silently omitted from an all-source claim.

Every later function owner family gets its own `F1x` row. REPL and ProgramV0
remain explicit legacy inputs until their separate lifetime/source-authority
decisions.

### RET-P0 — legacy caller inventory

Inventory every remaining caller of:

```text
LoopRouteContext and current CorePlan
legacy IfForm value-map joins
manual If/Loop carrier classification
name-keyed final_values
raw &[ASTNode] + consumed usize source protocol
route-specific PHI materializers
PreSsaValueEnvironmentV1 and old join-value adapters
resolved_region_flow imports/re-exports and effect transport
canonical materialize_all_phi_inputs repair
raw canonical Branch/Jump/predecessor mutations
unchecked canonical add_function paths
ordinary compile entrypoints that still select BareAst legacy provenance
```

Create a new retirement inventory rather than mutating the frozen 92-row SSA-P0
evidence. Classify callers as canonical source, explicit BareAst legacy,
ProgramV0, REPL, test-only, or dead. This row changes no production behavior.

### RET-I1 — canonical legacy-call veto

Make legacy If/Loop/CorePlan imports, constructors, calls, name lookups, and
retry paths zero on every supported canonical source route. Guard the boundary
without deleting explicit legacy consumers prematurely.

### RET-I2 — explicit legacy isolation

Confine remaining legacy control routes behind explicit
`LegacyModuleLoweringInputV1` provenance. Canonical failure never enters this
boundary.

### F2-S0 — disconnected default-route producer

Connect the ordinary source frontend to `VerifiedResolvedSourceUnitV1`, build
the complete owner worklist, and run whole-unit capability preflight. Route
selection remains unchanged and production canonical calls do not increase.

### F2-I1 — default canonical source route

Switch the ordinary canonical source frontend atomically only after F0's
compatibility threshold and whole-unit preflight prove every required owner
and control family supported. Failure never retries the legacy source route.

### PUB-F0 — typed final publication closure

Close the final external publication protocol after the default route and
canonical caller-zero retirement boundary:

```text
only coverage/seal/SSA-complete function witnesses enter the candidate module
synthetic entry/thunk/stub publication uses the same checked boundary
canonical materialize_all_phi_inputs repair calls = 0
optimizer / RC / canonicalize complete before final verification
MIR mutation after final verification = 0
CanonicalModuleLoweringSessionV1::commit consumes an unforgeable ready witness
or is closure-scoped so an unverified commit is unrepresentable
```

SSA-V0 remains the early fail-fast prerequisite. PUB-F0 is the final temporal
API proof across every now-supported owner and synthetic function family.

### RET-R1 — caller-zero manual authority retirement

Delete caller-zero mechanisms:

```text
legacy IfForm value-map joins
manual LoopForm carrier classification
name-keyed final_values
route-specific PHI materializers
```

### RET-R2 — conditional CorePlan retirement

Delete current `CorePlan`, `LoopRouteContext`, and the raw suffix protocol only
after the complete repository caller inventory reaches zero. If ProgramV0,
REPL, or another explicit legacy input still calls them, keep the isolated
implementation and claim only that canonical source has zero legacy authority.

ProgramV0 compatibility is never silently promoted or deleted by this series.

### PARK-LEGACY-SUFFIX-001 — independent normalization suffix defect

Keep `LEGACY-NORMALIZATION-SUFFIX-CONSUMED-INDEX-001` outside the D′ authority
series. Its own bounded task is:

```text
focused final-Loop suffix reproducer
0 < consumed <= remaining.len() validation
explicit continue after suffix advance
exact-once lowering proof
separate commit from canonical Loop/SSA work
```

### O0 — optional durable RegionId materialization

Open SA4 only when a named production consumer needs durable region-to-MIR
roles. The product must be role-aware (`entry`, ports, owned blocks), derived
from verified materialization, and invalidated with MIR changes. A scalar
`RegionId -> BasicBlockId` map remains forbidden.

### O1 — optional derived Loop analysis

Create post-MIR `DerivedLoopAnalysisV1` only when a named production optimizer
consumer exists and proves that completed SSA cannot be queried directly.

Possible derived fields:

```text
header PHIs
preheader/backedge incoming values
induction candidates
recurrences
invariants
```

The result is invalidated by MIR/CFG changes and never becomes source or
Lower route authority. Structured `LoopRegionSignature` is a separate IR
decision and cannot coexist as a second generic-baseline truth.

### HMI — Rust VM to `.hako` MIR interpreter migration

This is a required selfhost retirement branch, not a new product VM route and
not part of the active SSA-RC0 design decision. Its durable policy and detailed
acceptance live in:

```text
../design/vm-active-lane-retirement-ssot.md
```

The execution order is:

```text
HMI-P0:
  inventory Rust handlers/callers and choose one sealed MIR ingress

HMI-S0/S1:
  seal the minimal portable opcode subset and normalized observation contract

HMI-I0:
  implement a disconnected `.hako` MIR interpreter in small boxes

HMI-P1:
  prove independent Rust/`.hako` normalized parity

HMI-C0:
  switch the closed semantic-reference subset to `.hako` without fallback

HMI-X0:
  expand one named MIR instruction family per slice

HMI-R1/R2:
  isolate remaining Rust callers, then delete only after repository caller zero
```

The first closed subset covers `Const`, `Copy`, `CopyOwned`, `DestroyOwned`,
`BinOp`, `Jump`, `Branch`, `Phi`, and `Return`. Legacy `ReleaseStrong` is not
part of the new portable subset. EXE/AOT remains the production route
throughout.

HMI-C0 requires the first canonical Binding-SSA owner so parity includes a
real canonical control/ownership fixture. HMI-P0 through HMI-P1 may otherwise
proceed as disconnected work only after SSA-RC-A1c, SSA-RC-V0, and
SSA-RC-RET-P0. Hako interpreter parity is not Hako compiler-Lower parity.

## Production activation table

| Milestone | Binding SSA production | If production owner | Loop production | Source grammar delta |
| --- | ---: | --- | ---: | ---: |
| D0 / S2′ / SSA-P0 / SSA-L0 / SSA-C1 / SSA-P1 / SSA-V0 / SSA-S1 / SSA-S2 / SSA-E0 / SSA-S3 / SSA-M0 / SSA-RC0-D0 / SSA-RC-L0a / SSA-RC-L0b / SSA-RC-L0 / SSA-RC-L1 / SSA-RC-P0 / SSA-RC-A0 / SSA-RC-A1a / SSA-RC-V0 / SSA-RC-A1b / SSA-RC-A1c / SSA-RC-RET-P0 / SSA-RC0 / SSA-I0-PROFILE | 0 | current A+ path | 0 | 0 |
| SSA-I1-T | admitted trivial whole unit | Binding SSA + If CFG box; temporary A+ remains whole-unit only | 0 | 0; ownership activation = 0 |
| SSA-I1-FULL | all current canonical whole units | Binding SSA + If CFG box | 0 | 0 |
| SSA-I1-O1 | 1 exact BoxRef owner | Binding SSA + Ownership SSA | 0 | 0 |
| SSA-R1 / S3′ / I1′ | 1 whole owner | Binding SSA | 0 | 0 |
| I2′ | 1 whole owner | Binding SSA | 1 closed Loop family | +1 family |
| N1-N3 | 1 whole owner | Binding SSA | bounded nesting expansion | one nesting shape per slice |
| N4 / EXIT-S0 / EXIT-S1 / EXIT-I7 | 1 whole owner | Binding SSA | existing accepted families | 0 |
| EXIT-I1-I6 | 1 whole owner | Binding SSA | bounded typed exits | one source shape per slice |
| F1a-F1d / F2 | bounded then all supported owners | Binding SSA | Binding SSA | one owner family per F1 row |

## Required counters

Before SSA-I1-T:

```text
BindingSsaBuilder production sessions = 0
carrier-free If/Loop control production consumers = 0
canonical accepted grammar delta = 0
```

At SSA-I1-T and thereafter on an admitted trivial canonical route:

```text
BindingRef value merge authorities = 1
variable reads bypassing Binding SSA = 0
binding definitions bypassing Binding SSA = 0
flat map branch snapshots = 0
If may_rebind queries = 0
old A+ If join-source queries = 0
Lower full-map diff = 0
String/name binding lookup = 0
Reserve-only PHI publication = 0
edge-after-seal acceptance = 0
silent legacy retry/fallback = 0
```

At B0-L4-I2′ and thereafter:

```text
canonical Loop carrier rows = 0
canonical LoopRouteContext constructions = 0
canonical current CorePlan calls = 0
coverage inferred from consumed usize = 0
durable RegionId materialization publication = 0 until SA4
```

## Error and finish gates

Inject failures at:

```text
edge emission
provisional PHI definition
recursive predecessor read
PHI patch
block seal
declaration/read/assignment coverage
scope/region leave
RC validation
SSA finish
function verification
function finalization
```

Every failure proves:

```text
all pending cleanup attempts run
primary and cleanup failures are both preserved
caller current function/block/context is restored
semantic stacks are restored or the draft is discarded
partial function/module publication = 0
canonical legacy retry = 0
```

Per-block seal order:

```text
1. every accepted incoming edge is emitted
2. terminator-derived and cached predecessors are exactly equal
3. CanonicalCfgSession seals and yields VerifiedPredecessors
4. BindingSsaBuilder seals from that witness
5. incomplete PHIs for the block are patched
```

Success and publication order after every block seal:

```text
1. all control coverage consumed
2. identity coverage complete
3. scope/region stacks balanced
4. every touched block sealed
5. incomplete PHIs = 0
6. PhiTxn committed
7. Ownership SSA forwarding/consuming-use verification green
8. resolved authority finished and function draft finalized
9. function session restores caller state
10. sealed function draft enters the unpublished candidate module
11. candidate module finalization and RC insertion complete
12. final CFG/SSA/dominance/accepted-RC/MIR reverify green
13. canonical module session commits externally
```

Step 10 is internal candidate publication, not externally visible commit.
Verifier failure between steps 10 and 13 discards the candidate module.

## Guard plan

Do not create another public row guard.

```text
stable public entry:
  tools/checks/resolved_region_flow_authority_guard.sh

private reusable helpers:
  tools/checks/lib/resolved_control_lowering_contract.sh
  tools/checks/lib/resolved_control_flow_contract.sh
  tools/checks/lib/resolved_binding_ssa_contract.sh
  tools/checks/lib/resolved_if_lowering_contract.sh
  tools/checks/lib/resolved_loop_lowering_contract.sh
```

The top authority guard must remain below 800 lines. The control aggregator
sources the control-flow, Binding SSA, If, and Loop helpers. The control-flow
helper owns the module manifest, forbidden effect/MIR imports, co-sealed
coverage boundary, production caller counts, and source-size check. Update
each private helper in the slice that changes its real contract; do not
front-load speculative regexes.

Guard transition is ordered:

```text
S2′:
  add disconnected resolved_control_flow contract checks

SSA-S3:
  line-neutrally admit resolved_control_flow as a disconnected consumer
  keep old production If S2/I1 checks

SSA-RC-L0a:
  require implementation/test/reference ledger equality at 41/16/57

SSA-RC-L0b:
  require the dedicated static-const-table load guard to execute 3 tests under vm-reference

SSA-RC-L0:
  assert both split facades preserve public behavior and every file is <800

SSA-RC-L1:
  require Rust interpreter caller-frame restoration on all injected errors

SSA-RC-A0/A1a/V0/A1b/A1c:
  add one private resolved_binding_ssa_owned_mir helper
  require opcode/effect/JSON/transport/backend/verifier contracts
  keep production ownership callers zero and 92-row evidence unchanged

SSA-RC-RET-P0:
  add a separate ReleaseStrong producer/consumer retirement ledger
  forbid canonical ownership aliases to the legacy opcode

SSA-I1-T atomic commit:
  replace production If effect/join assertions with Binding SSA/control-only assertions
  require exactly one function Binding SSA production session
  require flat value owner and old adapters to have zero production callers
  require canonical ReleaseStrong and optional-RC double-destroy paths to be zero
  freeze the 92-row artifact as historical evidence and move live ownership
  caller counts to the RET-P0 ledger

SSA-R1:
  assert exact old symbol and caller counts are zero
  physically remove old effect/join files and their allowlist

Loop-I2′:
  require canonical CorePlan / LoopRouteContext / raw suffix callers to stay zero

EXIT-Ix:
  require exact port + cleanup + disposition + target-role consumption

RET-I1/I2 and F2-I1:
  require canonical legacy imports/calls/retry zero and explicit legacy provenance only

PUB-F0:
  require two-stage publication witness, canonical repair zero,
  and zero MIR mutation after final verification
```

Common per-code-slice gates:

```bash
bash tools/checks/resolved_region_flow_authority_guard.sh
cargo build --release --bin hakorune
tools/checks/dev_gate.sh quick
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Add focused unit/runtime commands named by each milestone before committing.

## May claim

| Milestone | Maximum claim |
| --- | --- |
| S2′ | exact located range/coverage schema exists; its production consumers are zero and existing A+ If is unchanged |
| SSA-P0/L0/P1/C1 | the inventory, physical split, CFG seal, and PHI cleanup prerequisites are closed; production SSA calls are zero |
| SSA-V0 | canonical verifier failure and duplicate function publication cannot commit; grammar delta is zero |
| SSA-S1 | disconnected Binding SSA handles tested CFG shapes; production calls are zero |
| SSA-S2 | identity and temporary value ownership are separated; production Binding SSA calls remain zero |
| SSA-E0 | the already accepted terminal Return has an exact preservation contract; grammar delta is zero |
| SSA-S3 | one carrier-free If control product is sealed; production If still uses A+ |
| SSA-RC-L0a | stale instruction-diet literals are repaired to the unchanged 41/16/57 implementation vocabulary |
| SSA-RC-L0b | the static-const-table load guard executes its three feature-gated VM-reference module tests instead of accepting zero tests |
| SSA-RC-L0/L1/P0 | ownership transport/frame seams are closed and the exact BoxRef/trivial/reject profile is sealed; production ownership is unchanged |
| SSA-RC-A0 | passive CopyOwned/DestroyOwned transport exists; production callers and backend execution are zero |
| SSA-RC-A1a/V0/A1b/A1c | supported backends and path-sensitive Ownership SSA verification implement the closed BoxRef profile; canonical callers are zero |
| SSA-RC-RET-P0 | every legacy ReleaseStrong surface is classified without changing its meaning |
| SSA-M0/RC0 | the real-MIR adapter and bounded pure ownership plans are sealed; production Binding SSA calls remain zero |
| SSA-I0-PROFILE | one executable whole-owner trivial profile is co-sealed with completion and carrier-free If control; all production callers remain zero |
| SSA-I1-T | every admitted trivial whole unit has one BindingRef value/PHI authority; non-admitted current units remain whole-unit A+ only |
| SSA-I1-FULL | the whole current canonical family uses Binding SSA and temporary A+ production callers are zero |
| SSA-I1-O1 | one exact BoxRef source profile uses production CopyOwned/DestroyOwned with verified forwarding |
| SSA-RC-RET-R1/R2 | canonical legacy ownership callers are isolated, then repository-wide caller-zero vocabulary is physically retired |
| SSA-R1 | old canonical If value authority and the temporary flat environment are caller-zero; explicit legacy mechanisms may remain |
| S3′ | one carrier-free Loop control contract is sealed; Builder connection is zero |
| I1′ | one disconnected Loop CFG transaction exists; production Loop activation is zero |
| I2′ | the first closed canonical Loop uses exact control plus generic Binding SSA |
| N1/N2/N3 | the selected single nesting shape uses one function SSA authority |
| N4 | the supported If/Loop grammar has bounded depth-independent nesting evidence |
| EXIT-I1-I6 | the selected single Continue/Break/Return source shape is production-supported |
| EXIT-I7 | accepted nested exit cleanup and predecessor closure are proven without a new shape |
| EXIT-S0/S1/S2 | exit semantics, Lower roles, and multi-port contracts are sealed; new exit runtime activation is zero |
| F1x | only the selected closed owner family is cut over atomically |
| RET-I1/I2 | canonical legacy calls are zero and remaining explicit legacy provenance is isolated |
| F2-I1 | F0-required ordinary canonical source owners use the no-retry SSA route |
| RET-R1/R2 | only repository-wide caller-zero mechanisms are physically removed |
| PUB-F0 | every supported canonical and synthetic function crosses one typed final publication barrier |
| HMI-P0/S0/S1/I0 | one disconnected portable `.hako` MIR-interpreter subset exists; Rust remains the temporary reference |
| HMI-P1 | the closed subset has independent normalized Rust/`.hako` parity |
| HMI-C0 | `.hako` is the sole semantic-reference owner for the closed subset; product execution remains EXE/AOT |
| HMI-X0 | only the selected named MIR instruction family joins the `.hako` subset |
| HMI-R1/R2 | Rust interpreter callers are isolated, then physically removed only at repository caller zero |

## Must not claim

```text
all source/control families supported before their capability gates
captured-by-reference or Upvar layout from local Binding SSA
field/index writes are local SSA definitions
typed open-PHI facts without a separate proof
Break/Continue/Return/Try support from generic SSA alone
durable RegionId materialization or SA4 completion
ProgramV0 source authority
REPL owner lifetime completion
Hako Lower parity
product VM parity from `.hako` semantic-reference parity
Rust MirInterpreter physical retirement before HMI-R2 caller zero
current CorePlan retirement before its final callers close
post-MIR recurrence authority without a production consumer
ordinary-source compatibility before F0 and F2-I1 close
ownership correctness beyond the bounded SSA-RC0/I1 contract
global legacy deletion from canonical caller-zero evidence alone
narrow preflight acceptance as proof of ordinary-source compatibility
BorrowedText/Opaque/Unknown ownership support from the BoxRef first profile
ReleaseStrong physical retirement before repository-wide exact caller zero
```

## Stop conditions

Stop implementation or publication if any slice:

```text
uses Binding SSA only for Loop while the same owner's If uses a flat map
seeds SSA at a Loop boundary and exports all visible bindings afterward
adds an old-environment/SSA synchronization bridge or recursive mode Option
keeps effect/carrier rows as permanent PHI-placement verification
passes AST, SourceSite, RegionId, Span, pointer, or name into Binding SSA
puts ValueId, BasicBlockId, or materialized target roles in a pre-Builder product
adds an independently mutable third predecessor truth
emits an edge and registers its predecessor through unguarded separate calls
adds a predecessor after seal
uses cached successors as the terminator-truth predecessor proof
repairs canonical CFG or missing PHI inputs during post-Lower materialization
exposes a Reserve-only PHI dst
infers a concrete open-PHI fact from only the entry input
erases historical SSA definitions on lexical scope leave
routes Upvar/field/index writes through local Binding SSA
uses ordinary Copy to create an Owned alias
maps CopyOwned directly to legacy ReleaseStrong
lets DestroyOwned inspect or remove a same-object alias
infers BoxRef ownership from Unknown/Opaque runtime data
lowers ownership ops as an unsupported-backend no-op
publishes an Owned Phi by cloning without a verified forwarding law
treats every Owned Phi incoming as a merge-block consume instead of an exact edge use
checks Ownership SSA with a global consume count instead of path-sensitive lifetime closure
changes legacy ReleaseStrong meaning during migration
discovers unsupported control after Builder effects
lets PHI/cleanup failure overwrite the primary error
publishes before SSA/coverage/stack/function verification finishes
commits a canonical module while verification_result is Err
silently overwrites a same-name canonical function
retries legacy If/Loop/CorePlan after canonical failure
lets one activation row accept more than one source/control shape
inherits a newly landed expression kind without updating the row's closed grammar
adds a universal optional-field control product before three-family evidence
adds DerivedLoopAnalysis without a named consumer
lets a new or modified source/check file reach 800 lines
translates the Rust VM file-for-file instead of a sealed MIR subset
uses AST or ProgramV0 as the `.hako` MIR-interpreter authority
falls back to Rust after `.hako` semantic-reference cutover
deletes Rust interpreter code before exact repository caller zero
```

## Final completion definition

This roadmap reaches its canonical-source final form when:

```text
one BindingSsaBuilder instance owns all local BindingRef reaching values per function
all canonical CFG edges use one late-edge-safe facade
pre-Builder products contain only source/control/cleanup semantics
If, Loop, and nested typed exits use family boxes over the same SSA substrate
all supported source owners cut over atomically with no fallback
old canonical effect/carrier/manual-PHI callers are zero
remaining legacy mechanisms are isolated behind LegacyModuleLoweringInputV1
repository-wide physical deletion occurs only after global caller zero
function publication is gated by exact coverage, seal, SSA, CFG, RC, and MIR verification
F0's whole-unit capability matrix and compatibility threshold are closed
optimizer loop facts are derived from completed MIR only when consumed
```

ProgramV0, REPL lifetime, Hako Lower parity, and structured-loop IR remain
independent decisions. They are not hidden prerequisites or accidental claims
of this final form. The `.hako` MIR-interpreter migration is a separate required
selfhost retirement branch: it does not redefine D-prime compiler completion,
but Rust semantic-reference ownership is not the repository's selfhost final
state.

## Immediate next action

Stop at **SSA-I1-COMPAT next-row selection**. Select exactly one remaining row
before another implementation card:

```text
exact typed parameter ABI
explicit Void value disposition
Outbox identity after Void is decided
BorrowedText lifetime / ABI
```

Receiver remains a separate owner-family decision and must not be bundled with
the parameter row. Production Ownership SSA, Loop activation, and whole-unit
fallback behavior remain unchanged until a later card explicitly selects them.
