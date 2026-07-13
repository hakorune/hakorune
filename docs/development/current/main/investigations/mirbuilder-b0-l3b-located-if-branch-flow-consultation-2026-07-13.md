---
Status: Accepted — A+ pre-Builder verified flow contract
Date: 2026-07-13
Scope: B0-L3b canonical located statement-If branch-state authority
Work kind: BoxShape decision closed; implementation series opened
Related:
  - docs/development/current/main/investigations/mirbuilder-resolved-semantic-owner-forest-design-stop-2026-07-13.md
  - docs/development/current/main/investigations/mirbuilder-resolved-region-flow-v1-task-2026-07-13.md
  - src/mir/builder/resolved_lowering/
  - src/mir/resolved_semantics/
---

# B0-L3b Located If Branch Flow Consultation

## Accepted decision

A+ is accepted:

```text
branch-effect owner:
  pre-Builder VerifiedResolvedFunctionFlowV1

first runtime claim:
  fallthrough-only statement If

RegionId boundary:
  exact consume + coverage in B0-L3b
  durable role-aware block materialization in SA4

order:
  S1 exact identity bundle
  -> S2 verified function If flow
  -> I1 canonical materialization
```

The verified flow owns condition effects, fallthrough ports, outer-binding
effect summaries, join source rows, and exact source coverage. Lower owns only
plan-directed ValueId lookup, BasicBlock allocation, predecessor verification,
and final PHI publication.

Current-code validation found four mechanical refinements without reopening
the decision:

1. the bare statement site query is self-relative; foreign-owner validation
   stays at the owner-branded S2 input boundary;
2. source optional-else totality is the composite S1 topology + source
   projection + S2 located-source proof;
3. `Entry` is spelled `PostConditionEntry` because condition BlockExpr effects
   precede the branch baseline;
4. source order is stored explicitly rather than inferred from `BTreeMap` key
   order.

Executable tasks and gates are fixed in:

```text
docs/development/current/main/investigations/
  mirbuilder-b0-l3b-a-plus-implementation-task-2026-07-13.md
```

The original decision request and evidence follow as historical input.

## Decision request

Choose the owner of canonical statement-`If` branch effects before production
Lower is widened.

The existing decision says only:

```text
B0-L3b = located If route
```

It does not decide who owns:

```text
then/else BindingRef write sets
fallthrough ports
BindingRef -> ValueId branch fork/join
PHI materialization requests
If RegionId consumption versus RegionId -> BasicBlockId mapping
```

This is not an implementation detail. The accepted architecture currently
says RegionFlow owns per-port binding effects and edge-state closure, while
Lower owns ValueId/BasicBlockId materialization. Starting code without this
decision would create two semantic owners.

## Current evidence

### Already sealed

```text
source carriers:
  ExprChildRoleV1::IfCondition
  BodyChildRoleV1::IfThen
  BodyChildRoleV1::IfElse

semantic records:
  RegionKindV1::If
  RegionKindV1::IfThen / IfElse
  ScopeKindV1::IfThen / IfElse

resolver order:
  condition
  -> If control region
  -> then scope/region
  -> optional else scope/region
```

`else=None` creates no IfElse pair. `else=Some(empty)` creates one empty exact
IfElse pair. A condition BlockExpr closes before the If control region begins;
the condition BlockExpr and If are sibling regions under the same outer
region.

### Missing

```text
exact If control + branch-pair product query = 0
If-specific seal verifier = 0
BindingRef value-environment fork/join API = 0
control Region stack distinct from lexical Scope stack = 0
canonical If PHI contract = 0
```

The current `ResolvedScopeStateV1` handles BlockExpr pairs only. It cannot
model:

```text
If control Region
  ├─ IfThen Region + Scope
  └─ IfElse Region + Scope
```

because the control region sits between the current lexical region and each
branch region.

### Legacy path is not reusable

Canonical Lower must not call `lower_if_form()` or
`lower_if_form_with_condition_value()`.

Those paths own raw-AST recursion, AST clones, `String -> ValueId`
`variable_map`, name-keyed branch materialization, environment-dependent
JoinIR selection, and fallback. Reusing them would cross the exact-site and
BindingRef authority boundary.

Only thin emission primitives are reusable:

```text
next_block_id / start_new_block
finalize_branch_cond
emit_conditional or emit_conditional_edgecfg
emit_jump
phi_lifecycle::define_phi_final
```

## Options

### A — pre-Builder ResolvedIfFlowPlanV1 + materializing branch transaction

Recommended.

Add a small RegionFlow-owned immutable plan before Builder effects:

```rust
struct ResolvedIfFlowPlanV1 {
    bundle: ResolvedIfRegionBundleV1,
    then_port: ResolvedBranchPortV1,
    else_port: ResolvedBranchPortV1,
}

struct ResolvedBranchPortV1 {
    may_rebind: Box<[BindingRefV1]>,
    falls_through: bool,
}
```

The plan contains no ValueId or BasicBlockId. It is derived from exact located
source plus the sealed function product and verifies all branch assignment and
exit sites.

Canonical Lower owns only:

```text
same-base branch value snapshots
plan-declared BindingRef input lookup
BasicBlock allocation
predecessor-exact PHI emission
merged BindingRef -> ValueId publication
```

Why this is recommended:

- preserves the accepted RegionFlow/Lower boundary;
- prevents Lower from rediscovering branch effects by diffing mutable state;
- gives Loop/CorePlan a reusable per-port vocabulary later;
- keeps raw AST, names, Span, and pointer identity out of flow authority.

Tradeoff: B0-L3b becomes the first narrow RegionFlow connection instead of
keeping RegionFlow at zero until B0-L4.

### B — Lower-local BindingRef snapshot/diff transaction

Canonical Lower snapshots the complete active `BindingRef -> ValueId` map,
lowers each branch from the same baseline, and compares branch-end maps to
choose PHIs.

Advantages:

- smallest implementation;
- exact lexical identity is preserved;
- no legacy IfForm dependency.

Risk:

- Lower becomes the owner of per-port binding-effect discovery;
- the snapshot-diff policy must later be retired when RegionFlow connects;
- Loop/CorePlan can grow a second, different edge-state owner.

Choose B only if branch value diff is explicitly classified as MIR
materialization rather than semantic effect analysis.

### C — generic legacy/canonical IfForm refactor

Refactor the existing IfForm behind an identity-backend trait.

Rejected for B0-L3b. It pulls name-keyed JoinIR/PHI selection, raw AST
recursion, legacy fallback, and broad existing behavior into one slice.

### D — call the legacy IfForm from canonical Lower

Rejected. This directly violates exact located recursion and BindingRef-only
authority.

## Secondary decisions

### 1. First runtime claim

Recommended:

```text
statement If only
both branches fall through
condition = current exact expression grammar + BlockExpr
branch body = current closed statement grammar + nested fallthrough If
outer BindingRef rebind = supported
branch-local bindings = retired at branch close
If expression / early exit = later
```

This closes condition-BlockExpr runtime scope and real branch PHIs without
mixing Return coverage or CorePlan/Loop control exits.

Alternative: include branch Return and reachability now. This requires typed
`FallsThrough | Terminated`, unreachable-source disposition, and zero/one/two
predecessor merge rules in the same slice.

### 2. Region identity boundary

Recommended:

```text
B0-L3b:
  exact If/IfThen/IfElse RegionId consumption and coverage

SA4:
  durable RegionId -> BasicBlockId target authority
```

The first slice may allocate MIR blocks locally after consuming the exact
bundle, but must not claim the durable SA4 mapping cutover.

### 3. Exact product query

Recommended unconditionally:

```rust
struct ResolvedIfRegionBundleV1 {
    control: RegionId,
    then_pair: ResolvedScopeRegionPairV1,
    else_pair: Option<ResolvedScopeRegionPairV1>,
}

fn if_region_bundle(
    &self,
    owner: FunctionOwnerIdV1,
    site: &SourceStmtSiteV1,
) -> Result<ResolvedIfRegionBundleV1, ResolvedIfRegionLookupErrorV1>;
```

The query and seal verifier must prove exact owner, origin, kind, parent,
scope/region reciprocal link, required then pair, and exact optional-else
cardinality. Lower must not scan the arena to rediscover this bundle.

## Recommended implementation order after the decision

### B0-L3b-S1 — passive exact If bundle

```text
ResolvedIfRegionBundleV1
if_region_bundle
If-specific seal verification
foreign/missing/wrong-parent/else-cardinality fixtures
production activation delta = 0
```

### B0-L3b-S2 — branch flow authority

If A is selected:

```text
ResolvedIfFlowPlanV1
exact per-port BindingRef rebind sets
fallthrough-only first contract
no ValueId / BasicBlockId
Builder connection = 0
```

If B is selected, explicitly document the temporary Lower-owned diff policy
and its retirement condition before implementation.

### B0-L3b-I1 — canonical statement If

```text
located condition/then/else only
same post-condition branch base
control-region and lexical-scope stacks separated
branch-local retirement
outer BindingRef PHI join
existing emission primitives only
coverage before function publication
```

## File layout

Keep existing near-limit files unchanged:

```text
resolved_semantics/tests.rs               670 lines
resolved_semantics/verifier.rs            706 lines
compiler/tests.rs                         706 lines
resolved_region_flow_authority_guard.sh   791 lines
```

Use new bounded files:

```text
src/mir/resolved_semantics/if_region.rs
src/mir/resolved_semantics/if_region_tests.rs
src/mir/builder/resolved_lowering/if_lowering.rs
src/mir/builder/resolved_lowering/branch_identity.rs
src/mir/builder/resolved_lowering/if_control.rs
src/mir/builder/resolved_lowering/if_tests.rs
tools/checks/lib/resolved_if_lowering_contract.sh
```

If A is selected, place the neutral plan outside Builder in the existing
RegionFlow-neutral MIR boundary chosen by the response; do not hide it under
`resolved_lowering/`.

## Required fixtures after decision

```text
same-Span condition/then/else sites remain distinct
else=None versus else=Some(empty)
condition BlockExpr closes before branch scope
true/false outer rebind values
one-sided rebind with no else
branch-local shadow does not leak or join
nested If consumes each exact bundle once
branch error restores scope/control/value baseline
wrong/foreign/missing bundle rejects
PHI inputs match exact CFG predecessors
verified MIR and VM-reference values
```

If early exit is included, also require zero/one/two reachable predecessor
fixtures and explicit source-coverage disposition after termination.

## Stop conditions

```text
legacy lower_if_form enters canonical Lower
raw branch AST is passed to legacy recursive dispatch
String/name/Span/pointer/encounter order selects a binding or site
branch-local BindingRef enters the merge domain
then state leaks into else compilation
Lower rediscovers semantic write sets when A is selected
RegionFlow allocates ValueId or BasicBlockId
PHI is emitted before exact bundle/port verification
IfElse pair is fabricated for else=None
condition BlockExpr scope remains active in a branch
cleanup error overwrites the primary error
function publishes before bundle/coverage/stack verification
Loop/CorePlan/Lambda/ProgramV0/default-route widening enters this slice
any source file reaches 800 lines
```

## Must not claim at this stop

```text
canonical If runtime support
RegionFlow production connection
RegionId -> BasicBlockId authority cutover
If expression result lowering
Return/Break/Continue branch flow
Loop/CorePlan/Lambda support
legacy IfForm retirement
default source route cutover
Hako typed-source parity
```

## Reply template

Please decide:

1. branch-effect owner: **A** pre-Builder flow plan or **B** Lower-local diff;
2. first claim: **fallthrough statement If** or **include early exit**;
3. RegionId boundary: **consume only until SA4** or **map now**;
4. any correction to the exact bundle/query and implementation order.

Recommended answer:

```text
A
fallthrough statement If
consume only until SA4
exact bundle/query and S1 -> S2 -> I1 order accepted
```
