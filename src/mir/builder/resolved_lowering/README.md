# Resolved lowering boundary

This directory owns the first production consumer of a sealed semantic owner.

Allowed inputs are only `CanonicalFirstFamilyPlanV1` values produced by the
whole-unit compiler preflight. Recursive lowering accepts sealed located-node
carriers and resolves lexical identity through exact source sites.

Invariants:

- `BindingRefV1 -> ValueId` is the canonical value environment.
- names are diagnostic cross-checks, never lookup keys.
- legacy `allocate_binding_id()` is structurally vetoed while an owner is installed.
- declarations, variable uses, assignment targets, and exits must all finish
  source coverage before the function draft can be published.
- canonical lowering seeds separate RegionId and ScopeId stacks from the sealed
  function/function-body roots; BlockExpr consumes one exact pair and retires
  only pair-owned BindingRefs at scope leave.
- I1b consumes one pre-Builder verified statement-If flow in source preorder.
  Both branches start from the same post-condition BindingRef baseline; the
  sealed join-source matrix selects final PHI inputs, and all PHIs define
  before one effect-authorized batch publication.
- RegionId and ScopeId stacks remain separate. Statement If consumes exact
  control/branch identities and coverage only; no durable RegionId-to-block
  map is published before SA4.
- legacy statement/expression dispatch, Planner/CorePlan, Lambda, production
  Loop activation, Main, REPL, and ProgramV0 are outside this boundary.

## Canonical V2 function finish

The three canonical V2 profile lowerers (`trivial_ssa`, `direct_accum`, and
`nested_predicate`) share one consuming finish terminal:

```text
CanonicalSsaFunctionSessionV2::finish_for_draft_seal
  -> ReadyFunctionDraftSealV1
```

Each profile closes its private effect/After/final-carrier ledger into one
move-only `ReadyCanonicalProfileCloseV1`. The common terminal consumes that
receipt and closes CFG, semantic/If control, identity/Binding SSA, PHI, the
resolved binding ledger, and Completion exactly once. It is the sole V2 issuer
of `ReadyFunctionDraftSealV1`.

The terminal accepts no raw body/site/end/target/current-block facts for
re-inference. Those identities are sealed when the exact resolved function
session opens; the profile receipt carries the already-claimed terminal
witness. A failed or duplicate close rejects before publication, and any late
failure discards the whole unpublished function. `PhiTxn` rollback is only
best-effort local cleanup; the outer function session owns atomic discard.

The existing non-V2 direct constructor caller is an explicit compatibility
allowlist entry. It may not gain callers and is retired by a later bounded row;
it is not part of the V2 finish migration. Focused guards must keep V2 direct
`ReadyFunctionDraftSealV1::new` callers at zero and keep all source files below
the repository's 800-line boundary. Every implementation slice updates this
README, the owning reference, and current-entry mirrors in the same commit.

Implementation receipt (`6bf3dd6b35`, 2026-08-07): the three V2 lowerers now
use the consuming terminal, including the previously missing DirectAccum CFG
finish. The one non-V2 constructor remains an explicit, non-growing
compatibility debt. The focused session, resolved-lowering, and draft tests,
the canonical finish guard, and the current-state pointer guard are green;
physical Loop lowering and production selection remain closed at the next
design-only operation physicalizer stop.

The caller-zero topology slice is now landed as test-only evidence behind the
`loop_recipe_physicalizer/` directory facade. It consumes one move-only common boundary and
one session-local `ReadyLoopEntryV1`, borrows the existing canonical CFG
service, and allocates only recursive header/body/step/After blocks. Owner,
entry coverage, binding ownership, parent topology, and preheader placement
are checked before allocation; an unknown parent never falls back to the root
preheader. The focused canary proves a nested Generic G0 topology and
rejects incomplete entry without allocating blocks.

This is not a production physicalizer or selector. It emits no operation MIR,
Return, DraftSeal, publication, retry, fallback, or legacy deletion. The
DirectAccum binding port remains profile-specific and must not be reused as
the common port; no second CFG/SSA/PHI owner is allowed. The current row is
the passive `LOOP-RECIPE-OPERATION-EFFECT-S0` product: it issues an item-keyed
exact source/effect ledger before operation emission is opened. The product is
test-only, moves the verified Core once, and emits no operation MIR, Return,
DraftSeal, selector, retry, fallback, or production route. The callable
adapter is now closed as a separate source-evidence receipt. The Generic G0
anchor ledger is also closed: its 15 item keys are issued before source facts
are dropped, with item 3 matching the child-entry carrier relation.
Cross-profile parity is also closed as a diagnostic-only receipt; it compares
neither profile counts nor source order. Reviewed Decision B separates full
demand/preflight from leaf emission. The Builder-free complete operation
demand, topology module split, and the private logical-to-physical block
receipt are now closed. The private
`LOOP-RECIPE-OPERATION-EMITTER-CONST-S0` and the bounded
`LOOP-RECIPE-OPERATION-EMITTER-READ-I0` leaf canaries are closed. ReadBinding
projects the complete prepared program, claims the exact source through the
canonical BindingSSA/PHI owner, validates explicit entry requirements, and
returns distinct logical/physical receipts. Pre-claim rejects are typed;
claim/read/type/receipt failures terminate the unpublished function session as
one discard transaction. No single-operation demand extraction was added.
Full operation physicalization, production activation, carrier seeds,
continuation/Tail, Return/DraftSeal, selector, retry/fallback retirement, and
legacy deletion remain closed until a named production caller switch is
authorized.

The current bounded preparation slice adds a private Prepared-product
move-only handoff, complete WriteBinding projection, and typed pure leaf
bridges for Const/Binary/Compare. The focused fixture proves a Const -> Binary
-> Compare chain without introducing a second CFG/SSA/PHI owner. This is not
yet the five-family dispatcher or the callable Prelude -> Loop -> Tail canary;
those adapters, `finish_for_draft_seal`, and DraftSeal integration remain the
next implementation boundary.

## Full operation demand P0

The Builder-free `VerifiedLoopOperationPhysicalDemandV1` now consumes the
resolver semantic context, complete operation/effect ledger, and one logical
After continuation as a move-only product. `prepare_all` derives Recipe
Loop/Block/Item order and proves complete seven-row Callable and fifteen-row
Generic G0 coverage before any Builder effect. The resolved-lowering layer is
not imported by this product; physical block allocation, leaf operation
emission, session opening, and DraftSeal remain later boundaries.

## Caller-zero Loop physical prepare

`src/mir/compiler/loop_physical_prepare.rs` is a test-only pre-effect contract
boundary for `LOOP-PHYSICAL-PREPARE-P0`. It brands the exact resolved callable
input, derives a prelude target/result capability from the existing callable
index/header, and seals one Tail/ABI/Completion compatibility relation before
any Builder session opens. The moved topology-only compatibility
`VerifiedLoopPhysicalDemandV1` owns the co-sealed logical product for the
closed P0 topology probe; the retained resolved input remains a borrowed view.
It is not the current full-operation demand and cannot feed operation emission.

The current `helper.to_i64(n)` MethodCall fixture intentionally has no
resolver-issued direct callable target and therefore rejects with typed
`NoSafeSlice::MissingPreludeTarget`. It remains a typed `Method` negative. The
bounded `CALLABLE-STATIC-PREFIX-S0` fixture is separate: top-level
`int_to_str(n: i64)` calls catalog-backed `to_i64(n: i64)` as a real
`FunctionCall`, and the observer records only the resolver-issued target and
explicit `FreeStatic` shape. No target injection, name lookup, AST rematch,
physical ID, Builder effect, selector, retry, fallback, or production caller
is opened by this row.

The neutral shape vocabulary remains in
`callable_single_loop_source_shapes.rs`; embedded syntax/source-map/static-
fixture tests remain sibling test-only modules and all touched files stay
below the 800-line limit. `CALLABLE-STATIC-PREFIX-MAP-S1` is now closed as a
source-only relation: same-brand different-owner resolver targets are kept,
while foreign compilation brands reject as `ForeignOwner`. The next bounded
cell is `CALLABLE-STATIC-PREFIX-P0` for declaration-derived ABI/Prepared
evidence; no physicalizer or production route is opened.

`CALLABLE-STATIC-PREFIX-P0` is now closed: the static fixture yields one
positive Prepared relation whose caller ABI comes from the completion/header
declarations and whose callee ABI comes from the resolver target header. ABI
is no longer accepted as an external argument at this boundary. The next
step is a design-only audit of the common physicalizer/session finish seam;
physical Builder effects remain closed.

## Disconnected canonical CFG prerequisite

`canonical_cfg/` owns the SSA-C1 edge/seal substrate. It emits a terminator and
its cached predecessor witness as one fallible operation, derives predecessor
truth directly from terminators, and rejects late edges or cache drift without
calling CFG repair. During SSA-C1 it has zero production If, Loop, and Binding
SSA callers; the existing A+ If path remains unchanged.
