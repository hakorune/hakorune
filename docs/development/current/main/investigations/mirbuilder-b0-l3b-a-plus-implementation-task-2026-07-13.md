---
Status: Active — S1/S2 closed; I1a disconnected materialization infrastructure
Date: 2026-07-13
Decision: A+ pre-Builder verified flow contract
Work mode: Refactor Series Mode; one purpose, five green commits
Parent:
  - mirbuilder-b0-l3b-located-if-branch-flow-consultation-2026-07-13.md
Related:
  - mirbuilder-resolved-semantic-owner-forest-design-stop-2026-07-13.md
  - mirbuilder-resolved-region-flow-v1-task-2026-07-13.md
---

# B0-L3b A+ Implementation Task

## Objective

Lower one closed canonical family of fallthrough-only statement `If` without
giving Lower any branch-effect analysis authority.

```text
VerifiedResolvedFunctionV1
  -> exact If identity bundle
  -> VerifiedResolvedFunctionFlowV1
  -> plan-directed canonical materialization
  -> MIR
```

The series has one purpose: make the first canonical located `If` consume one
pre-Builder verified flow contract. It adds no other accepted syntax family.

## Accepted authority split

```text
resolved_semantics:
  BindingRef / ScopeId / RegionId identity
  exact If/IfThen/optional-IfElse topology

resolved_region_flow:
  condition and branch effect summaries
  fallthrough ports
  exact join source matrix
  source coverage

resolved_lowering:
  plan-directed BindingRef -> ValueId snapshots
  BasicBlock allocation
  exact predecessor collection
  final PHI definition and post-definition rebind
```

`resolved_region_flow` may not import `ValueId`, `BasicBlockId`, `MirBuilder`,
Planner, JoinIR, or legacy variable state. Lower may not diff maps, union port
sets, inspect names, or infer write effects from syntax.

## Validation corrections to the consultation response

### 1. The S1 query is self-relative, not foreign-owner detecting

`SourceStmtSiteV1` is a path relative to one function root and has no owner
brand. The accepted one-argument API remains:

```rust
VerifiedResolvedFunctionV1::if_region_bundle(&SourceStmtSiteV1)
```

Its contract means “this site within `self.owner()`”. It does not expose a
`ForeignOwner` error that the input type cannot prove.

Foreign-owner mixing is rejected one layer earlier. S2 receives the existing
owner-closed `ResolvedFunctionLoweringInputV1`, whose owner, source view,
function product, and forest can only be co-derived from one
`VerifiedResolvedSourceUnitV1`. The flow analyzer compares those owner brands
before reducing a located statement to its relative site.

This differs intentionally from the current BlockExpr Lower query: BlockExpr
is queried directly by Lower from an owner-branded located expression, while
production If Lower receives one already owner-verified flow product and never
queries the semantic bundle separately.

### 2. Optional-else proof is composite

S1 can prove the sealed arena has exactly zero or one IfElse pair. It cannot by
itself prove that this cardinality matches AST `else_body`, because
`VerifiedResolvedFunctionV1` stores no AST.

The complete proof is:

```text
S1:
  arena topology = If + required IfThen + optional IfElse

VerifiedSourceProjectionV1:
  every stored source origin projects to valid syntax

S2:
  located AST else None/Some == bundle else None/Some
```

Do not claim source optional-else totality after S1 alone.

### 3. Entry means post-condition entry

A condition BlockExpr can rebind an outer BindingRef. Therefore the join
source spelling is:

```rust
ResolvedIfPortValueSourceV1::PostConditionEntry
ResolvedIfPortValueSourceV1::BranchExit
```

The ambiguous spelling `Entry` is not accepted. The branch baseline is taken
only after the located condition has finished and every condition BlockExpr
scope has closed.

`VerifiedResolvedIfFlowV1` carries a separate condition-effect summary. Its
join rows are derived only from then/else ports; its whole-If outgoing summary
is the union of condition effects and branch join effects. A parent flow may
consume the whole summary without rescanning a nested If.

### 4. Ordered publication is explicit

The repository has no generic Rust `OrderedMap` type. The function flow product
uses a sealed source-preorder boxed slice:

```rust
Box<[VerifiedResolvedIfFlowV1]>
```

Each row owns its exact `SourceStmtSiteV1`; lookup is read-only and does not
create a second mutable map authority. Do not equate `BTreeMap` key order with
source execution order.

## Closed V1 grammar

```text
statement If only

condition:
  current canonical expression grammar, including BlockExpr

then:
  explicit fallthrough body

else:
  ImplicitIdentity
  or explicit fallthrough body, including Some(empty)

branch body:
  current closed statement grammar
  plus nested fallthrough statement If

supported effects:
  outer BindingRef rebind
  branch-local declaration/use/rebind
  same-name branch-local shadow
```

Pre-Builder rejection remains mandatory for Return, QMark, Break, Continue,
Throw, Try/Catch/Finally, Loop/CorePlan, If expression results, Lambda runtime,
and every other unsupported branch route.

## Product contracts

### ResolvedIfRegionBundleV1

```rust
pub(crate) struct ResolvedIfRegionBundleV1 {
    control: RegionId,
    then_pair: ResolvedScopeRegionPairV1,
    else_pair: Option<ResolvedScopeRegionPairV1>,
}
```

The private index is stored only in `VerifiedResolvedFunctionV1`. Draft/data
state remains index-free. The seal verifier constructs the index exactly once
from the arena and returns it to the verified product. It is a rebuildable seal
witness containing IDs only, not a second region authority.

### VerifiedResolvedIfFlowV1

```rust
pub(crate) struct VerifiedResolvedIfFlowV1 {
    site: SourceStmtSiteV1,
    regions: ResolvedIfRegionBundleV1,
    condition_effects: ResolvedIfConditionEffectsV1,
    then_port: ResolvedFallthroughPortV1,
    else_port: ResolvedElseFallthroughV1,
    join: ResolvedIfJoinContractV1,
    coverage: VerifiedIfFlowCoverageV1,
}
```

```rust
pub(crate) struct ResolvedFallthroughPortV1 {
    may_rebind_outer: Box<[BindingRefV1]>,
}

pub(crate) enum ResolvedElseFallthroughV1 {
    ImplicitIdentity,
    Explicit(ResolvedFallthroughPortV1),
}
```

V1 has no `falls_through: bool`. Type construction proves fallthrough.

### Join contract

```rust
pub(crate) struct ResolvedIfJoinBindingV1 {
    binding: BindingRefV1,
    then_source: ResolvedIfPortValueSourceV1,
    else_source: ResolvedIfPortValueSourceV1,
}

pub(crate) enum ResolvedIfPortValueSourceV1 {
    PostConditionEntry,
    BranchExit,
}
```

The ordered join rows are the only join-domain authority. Lower does not
recompute a union from the ports.

```text
then port contains binding -> then source BranchExit
otherwise                  -> then source PostConditionEntry

explicit else contains binding -> else source BranchExit
otherwise                      -> else source PostConditionEntry

implicit else -> PostConditionEntry for every join row
```

Branch-local bindings are never rows. The verifier proves each row is an
entry-visible outer BindingRef and the rows exactly represent the port union.

### Coverage

Each exact assignment site is owned once by its nearest flow traversal.
Effects may be propagated through multiple ancestor summaries without
duplicating source-site ownership.

```text
direct condition assignment:
  condition coverage/effect

direct branch assignment outside nested If:
  then or else coverage/effect

nested If:
  child flow owns its source sites
  parent consumes child whole-effect summary
```

The function product proves a bijection between located If sites and sealed If
flow rows. Duplicate/missing rows, uncovered assignment targets, and partial
draft publication fail before Builder effects.

## Materialization contract

### No full-map diff

`BranchValueSnapshotV1` contains only bindings named by verified join rows.
Each materialized assignment records its exact BindingRef against the current
verified port before the value environment is changed. This is contract
verification, not effect discovery.

```text
before branch:
  save post-condition entry values for join rows only

after then:
  collect BranchExit values requested by then sources
  restore changed plan-authorized bindings without MIR Release

after else:
  collect BranchExit values requested by else sources
  restore changed plan-authorized bindings without MIR Release
```

An assignment outside the current verified port is an under-approximation
contract failure. A nested If reports its verified whole-effect bindings to
the enclosing branch transaction; the parent does not inspect child maps.

### PHI lifecycle

Every join row creates one fresh final PHI, even when both incoming ValueIds
are equal. Same-input elimination belongs to a later canonical simplifier.

```text
fresh ValueId
  -> define_phi_final(exact predecessor inputs)
  -> publish_join_value(binding, entry_value, defined_phi)
```

`publish_join_value` is an environment rebind, not declaration publication and
not assignment-source coverage. It must reject publication before final PHI
definition and must not emit `ReleaseStrong`.

### Implicit else CFG

Do not use `emit_conditional_edgecfg()` for implicit else; mapping its else
fragment to merge can create a join self-loop.

```text
ensure then and merge blocks exist
emit_conditional(header, condition, then, merge)
then actual exit -> emit_jump(merge)

false predecessor = header
```

Explicit else uses existing low-level conditional/jump primitives and records
the actual then/else exit blocks. All target blocks must exist before
`emit_conditional()` so predecessor registration is complete.

### Semantic stacks

Replace the BlockExpr-only pair stack with separate logical region and lexical
scope stacks. Seed them from sealed function and function-body roots; do not
scan the arena from Lower.

```text
condition BlockExpr:
  push/pop BlockExpr region and scope before branch baseline

If:
  push control RegionId only

then/explicit else:
  push branch RegionId and branch ScopeId independently
```

The I1a infrastructure must add a sealed exact function-body root query or
equivalent verified root carrier so the Sequence region/scope is not
rediscovered by Lower.

On error, semantic stacks and the plan-directed value journal restore
explicitly. Partial MIR remains in the unpublished function draft and the
outer function transaction discards it. Cleanup errors are combined with the
primary error rather than replacing it.

## Five-commit series

### 1. Decision/task acceptance — this commit

```text
consultation = Accepted A+
decision-time blocker = S1 exact If region bundle
production If activation = 0
code delta = 0
```

Acceptance:

```bash
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/resolved_region_flow_authority_guard.sh
git diff --check
```

### 2. S1 — exact If region bundle

Files:

```text
src/mir/resolved_semantics/if_region.rs
src/mir/resolved_semantics/if_region_tests.rs
src/mir/resolved_semantics/{mod.rs,product.rs,verifier.rs,README.md}
tools/checks/lib/resolved_if_lowering_contract.sh
```

Required behavior:

```text
one exact If control region per indexed site
control lexical_scope = None
required one exact IfThen scope/region pair
zero or one exact IfElse scope/region pair
branch region parent = control
branch scope parent = surrounding lexical scope
kind/origin/reciprocal links exact
all If/IfThen/IfElse records accounted; orphan = reject
private verified-product index only
production query callers = 0; the only allowed later caller is RegionFlow
```

Fixtures:

```text
same-Span If sites remain distinct
None gives no else pair
Some(empty) gives an empty else pair
missing control/then
wrong kind/origin/parent/scope parent
broken reciprocal pair
orphan branch records
index IDs resolve to the authoritative arena records
```

S1 claim stops at arena topology. Syntax optional-else totality remains S2.

Gates:

```bash
cargo test -q --lib mir::resolved_semantics::if_region_tests
bash tools/checks/resolved_region_flow_authority_guard.sh
tools/checks/dev_gate.sh quick
bash tools/checks/current_state_pointer_guard.sh
```

S1 closed on 2026-07-13. The sealed product now owns one private,
rebuildable, ID-only index from each self-relative statement site to its exact
If control, required IfThen pair, and optional IfElse pair. The seal rejects
wrong control scope/parent, derived branch origin, branch parent/scope parent,
broken reciprocity, duplicate topology, and orphan branch records before
publication.

Closeout evidence:

```text
focused exact If fixtures = 10 green
same-Span site distinction = verified
implicit versus explicit-empty else arena topology = verified
nested exact bundle independence = verified
private verified-product index = 1
draft/data index fields = 0
production query callers = 0
source optional-else totality = deferred to S2
RegionFlow / Builder / Lower connection = 0
canonical If runtime activation = 0
resolved_region_flow_authority_guard = green
dev_gate quick = PASS 66/66
top authority guard lines = 794
all S1 Rust source files < 800 lines
selected next slice = B0-L3b-S2 verified function If flow
```

### 3. S2 — verified function If flow

Files:

```text
src/mir/resolved_region_flow/README.md
src/mir/resolved_region_flow/mod.rs
src/mir/resolved_region_flow/ports.rs
src/mir/resolved_region_flow/if_flow.rs
src/mir/resolved_region_flow/analyzer.rs
src/mir/resolved_region_flow/coverage.rs
src/mir/resolved_region_flow/verifier.rs
src/mir/resolved_region_flow/if_flow_tests.rs
src/mir/mod.rs
```

The analyzer accepts the existing owner-closed
`ResolvedFunctionLoweringInputV1` and consumes its immutable located source
view. This is the only allowed leaf dependency on compiler source transport.
`resolved_region_flow` may not import compiler capability/orchestration or any
Builder module. Document the direction in its README.

Required behavior:

```text
lifetime-free VerifiedResolvedFunctionFlowV1
source-preorder If rows
typed fallthrough ports; no bool
ImplicitIdentity versus Explicit empty else
PostConditionEntry versus BranchExit matrix
condition effects separated from branch join rows
branch-local and shadow bindings excluded
nested If flows built postorder and consumed by parent summaries
exact source coverage and bundle/site bijection
no ValueId / BasicBlockId / Builder / Planner / JoinIR
production caller = 0
```

Fixtures cover no-If, one-sided/two-sided rebind, all join-source matrices,
condition BlockExpr rebind, explicit empty/implicit else, branch-local shadow,
nested If propagation/order, same Span, missing/duplicate coverage, malformed
bundle/cardinality, owner-closed input, and failure without partial publish.

Gates:

```bash
cargo test -q --lib mir::resolved_region_flow
bash tools/checks/resolved_region_flow_authority_guard.sh
tools/checks/dev_gate.sh quick
bash tools/checks/current_state_pointer_guard.sh
```

S2 closure evidence (2026-07-13):

```text
owner-closed analyzer entry = 1
lifetime-free function product = 1
source-preorder If rows = verified
nested composition = postorder child summary
typed fallthrough bool fields = 0
condition effects / branch join = separated
implicit identity / explicit empty else = distinct
branch-local join rows = 0
assignment coverage = exact once
bundle / source site / flow row bijection = verified
focused tests = 10/10
production flow callers = 0
Builder / Lower / runtime activation = 0
top authority guard lines = 794
all S2 Rust source files < 800 lines
selected next slice = B0-L3b-I1a disconnected materialization infrastructure
```

### 4. I1a — disconnected materialization infrastructure

Files:

```text
src/mir/builder/resolved_lowering/semantic_stack.rs
src/mir/builder/resolved_lowering/branch_transaction.rs
src/mir/builder/resolved_lowering/if_materialization.rs
src/mir/builder/resolved_lowering/if_materialization_tests.rs
src/mir/builder/resolved_lowering/{identity.rs,lowerer.rs,mod.rs,README.md}
```

Required behavior:

```text
separate RegionId and ScopeId stacks
sealed function/function-body root seed
BlockExpr migrated without behavior change
join-domain-only entry snapshot
plan-authorized rebind journal and restore
implicit false direct edge unit fixture
actual predecessor verification
same-input final PHI
define before publish_join_value
canonical If syntax acceptance = 0
production flow transport = 0
```

All existing BlockExpr focused and VM-reference fixtures must remain green.

Gates:

```bash
cargo test -q --lib mir::builder::resolved_lowering
cargo test -q --features vm-reference --lib \
  mir::builder::resolved_lowering::block_expr_tests
bash tools/checks/resolved_region_flow_authority_guard.sh
tools/checks/dev_gate.sh quick
bash tools/checks/current_state_pointer_guard.sh
```

### 5. I1b — atomic canonical statement If activation

This commit alone connects all production pieces:

```text
canonical preflight builds and seals function flow before Builder effects
CanonicalFirstFamilyPlanV1 owns the flow and is no longer Copy
Lower receives the function flow, not a separate semantic bundle
preflight admits only the closed fallthrough statement-If grammar
located condition/then/else materialization
exact bundle/flow/stack/coverage consumption
verified MIR and VM-reference runtime behavior
```

Preflight acceptance without Lower support, or Lower support without the
verified flow, is forbidden intermediate state.

Runtime fixtures:

```text
condition BlockExpr rebind is observed by both branches
then-only outer rebind with implicit false identity
else-only outer rebind
both branches outer rebind
explicit empty else differs from implicit else topology
branch-local declaration/use/rebind then retirement
same-name branch-local shadow restores outer binding
nested If consumes each exact flow once
then state never leaks into else compilation
same-input PHI remains final before optimization
actual nested branch exit is the PHI predecessor
branch error restores semantic stacks/value journal/current block
cleanup error preserves primary error
partial function publication on error = 0
MIR verifier and VM-reference results green
```

Final gates:

```bash
cargo test -q --lib mir::resolved_semantics::if_region_tests
cargo test -q --lib mir::resolved_region_flow
cargo test -q --lib mir::builder::resolved_lowering
cargo test -q --features vm-reference --lib \
  mir::builder::resolved_lowering::if_tests
bash tools/checks/resolved_region_flow_authority_guard.sh
tools/checks/dev_gate.sh quick
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Guard structure

`tools/checks/resolved_region_flow_authority_guard.sh` is already 791 lines.
Do not add contract logic inline. Source and call one new helper only:

```text
tools/checks/lib/resolved_if_lowering_contract.sh
```

The helper owns required files, focused test invocation, positive anchors,
forbidden imports/calls, and phase counters. The top-level guard must remain
below 800 lines.

Permanent forbidden anchors include:

```text
falls_through: bool in V1 flow
legacy lower_if_form / lower_if_form_with_condition_value
emit_conditional_edgecfg for implicit else
build_expression / build_statement from canonical If
variable_map / LexicalScopeGuard / ASTNode::Program
Lower map difference or port-set union
ValueId / BasicBlockId / MirBuilder in resolved_region_flow
semantic IfElse pair for ImplicitIdentity
BindingRef environment publication before final PHI definition
durable RegionId -> block map before SA4
```

## Series-wide nonclaims

```text
If expression result
Return/QMark/Break/Continue/Throw branch ports
zero/one reachable predecessor early-exit merge
Loop/CorePlan/Lambda runtime
durable role-aware RegionId materialization
SA4 cutover
legacy IfForm retirement
default source route cutover
ProgramV0 canonical support
Hako typed-source parity
complete general ResolvedRegionFlow V1
```

## Series stop conditions

Stop publication if any row:

```text
analyzes effects after Builder starts
lets Lower diff branch maps or derive join rows
uses a pre-condition branch baseline
starts else from then working state
fabricates an IfElse semantic pair for implicit else
mixes the control RegionId and lexical ScopeId stacks
creates a statement-If result PHI
omits an actual CFG predecessor from a final PHI
publishes a fresh PHI ValueId before define_phi_final succeeds
lets a branch-local BindingRef enter an outgoing port/join row
silently retries legacy IfForm or legacy source route
widens early exits, Loop, CorePlan, Lambda, or ProgramV0
puts implementation into an existing near-800-line file
leaves any source file at 800 lines or more
```

## Closeout claims after I1b only

```text
canonical fallthrough statement If branch effects have one pre-Builder owner
condition BlockExpr closes before the shared post-condition branch baseline
branch-local identity never enters the join domain
outer BindingRef state is materialized from sealed join rows only
implicit false and explicit else predecessors are exact
each sealed If/branch RegionId is consumed exactly once
durable RegionId -> BasicBlockId mapping remains zero until SA4
```
