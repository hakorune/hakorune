---
Status: SSOT
Date: 2026-09-06
Scope: source-level object construction lifecycle: `new`, `birth`, field initializers, explicit reuse methods, factories, and `fini`.
Related:
  - docs/reference/language/lifecycle.md
  - docs/development/current/main/design/box-member-field-method-surface-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
  - docs/development/current/main/design/mimalloc-object-lifecycle-queue-ssot.md
  - docs/development/current/main/design/ownership-home-model-ssot.md
  - docs/development/current/main/investigations/hakorune-home-ownership-task-2026-08-04.md
---

# Constructor Birth / New Lifecycle SSOT

Decision: accepted.

## Current Capsule

- **Current decision:** common Home Flow owns caller obligations; construction
  owns unpublished-object cleanup. Neither substitutes for the other. Primary
  Fault retention is independent of bounded suppressed-diagnostic storage.
- **Current implementation status:** the final-view V2 host route reaches a
  body companion that stops before artifact; checked execution is unproven.
- **Next ordered task:** retire provisional JSON program validation at that
  terminal, then close the existing root result/ABI and physical body handoff.
- **Production stop line:** no root ABI, parameter representation, value/CFG or
  cleanup meaning may be inferred by the C consumer; see the Decision below.
- **Retirement finish line:** selected New/Birth execution and cleanup use one
  plan, with the selected old projection edges removed and fixed EXE30 proven.

This document owns construction ordering and the direct-`birth` ban. The Home
document owns Home tokens and destinations. The bounded failed-construction
decision below supplies `OWN-HOME-BIRTH-D0` without changing successful order;
retained source/exit products do not yet prove runtime adoption.

Hakorune keeps construction small and explicit:

```text
birth:
  constructor hook
  direct receiver `birth(...)` call forbidden
  fires only through new

new:
  canonical construction surface
  positional args now
  explicit per-construction field overrides now
  named args later

reuse:
  explicit lifecycle methods such as reset / reactivate / configure / clear / attach

field initializer:
  per-instance
  runs before birth

fini:
  object usable-lifetime exit / cleanup
```

## Canonical construction

Canonical source:

```hako
local page = new HakoAllocPageModel(PageId(0), Bytes(32), 2, 2)
```

The construction order is:

```text
allocate object identity
run declaration-site field initializers
run matching birth(args...)
publish the object as usable
```

`birth` is special because it initializes a fresh identity. It is not hidden
magic: it is a declared hook with normal parameters and body rules. The
special rule is only its call permission.

## Direct `birth` calls are forbidden

Forbidden source:

```hako
page.birth(PageId(0), Bytes(32), 2, 2)
```

Reason:

```text
Direct birth calls would let user code reinitialize an existing object identity.
That makes lifecycle state ambiguous and weakens verifier / allocator reasoning.
```

Parser diagnostics should point users at the canonical surface:

```text
direct receiver `birth(...)` calls are forbidden; `birth` is a constructor hook fired only by `new`; use `new HakoAllocPageModel(...)` for construction
```

Existing internal or legacy `birth` routes are compatibility residue unless a
specific row marks them as part of the canonical language. They must not be used
as permission to add source-level `page.birth(...)`.

## Field initializers

Stored field initializers are per-construction values.

```hako
box Counter {
    count: i64 = 0

    birth(start: i64) {
        me.count = start
    }
}
```

For each `new Counter(...)`, field initializers run before `birth(...)`.

Rules:

```text
field initializers:
  create the initial per-instance state
  do not replace birth parameters
  must not be shared mutable state between instances

birth:
  may override initialized fields
  owns fresh-object initialization only
  is not a reuse/reset surface
```

## New-box field initializer block

Decision: accepted for explicit field entries.

This is not a line-count reduction feature. It is a boundary/contract feature:

```text
value:
  group report construction into one initialization boundary
  make duplicate fields fail-fast
  make unknown user-defined box fields fail-fast
  make record-local carrier -> ordinary box crossing visible
  keep runtime/backend semantics unchanged

non-goal:
  reduce source line count by itself
```

The canonical object field-copy surface is:

```hako
local result = new Report {
    accepted: fields.accepted
    reason: fields.reason
}

return result
```

For successful execution, the assignment order resembles:

```hako
local result = new Report()
result.accepted = fields.accepted
result.reason = fields.reason
return result
```

This is not a failure-preserving rewrite: the `new` expression does not publish
its result before overrides succeed. An override Fault still cleans an
incomplete construction, without the outer `fini` hook.

Rules:

```text
new Box { field: expr }:
  constructs an ordinary box identity
  then assigns the listed fields in source order
  does not create a record value
  does not call a named-argument constructor
  does not open reflection or backend-specific lowering

duplicate field:
  fail-fast

unknown field on a user-defined box:
  fail-fast

unmentioned fields:
  keep declaration-site defaults / birth behavior
```

Stop lines:

```text
no wildcard copy (`fields.*`)
no shorthand copy (`fields.accepted`) until BOX-INIT-002
no named constructor arguments
no constructor overload
no record materialization
no backend route or `.inc` owner-name matcher
```

Ordering with constructor lifecycle:

```text
allocate object identity
run declaration-site field initializers
run matching birth(args...)
run new-box field initializer block assignments
publish the object as usable expression result
```

This keeps declaration-site defaults / `birth` as the constructor lifecycle
owner, while `new Box { field: expr }` remains an explicit post-construction
field override at the construction site.

For call-site line-count reduction, prefer the existing RecordFields helper
pattern:

```hako
local fields = ReportFields {
    accepted: accepted,
    reason: reason
}

return me.makeReport(fields)
```

`makeReport(fields)` may use `new Report { field: fields.field }` internally,
but the primary source-size win comes from centralizing repeated copy logic in
one same-owner helper. The initializer block exists to make that helper body
more contract-like, not to replace helper scalarization.

## Failed construction: minimal integration contract

Decision: accepted design direction on 2026-09-05 after user consultation;
implementation permission remains with CURRENT_STATE and the selected slice.
This is the existing `OWN-HOME-BIRTH-D0` contract, not a new task family.

Source authority + canonical issuer: resolved source sites, binding/place
identities and declaration-owned Home demands feed the existing Home/exit
design owners (their construction issuer is not yet implemented). The ordinary-new owner co-seals that exit
relation with its existing exact constructor key; it does not invent cleanup.
Non-authority: Unit Completion, an E0 empty list, i64 storage, runtime handles,
reference counts, `DestroyOwned` alone, Pair's name, or a successful EXE.

Keep three responsibilities, with no Birth-specific general cleanup stack:

| Owner | Responsibility |
| --- | --- |
| common source exit / Home Flow | per-cutpoint local/temporary/lexical obligations, pending Fault, caller continuation |
| construction lifecycle | unpublished receiver, initialized owning fields and committed native payload, success publication |
| physical backend/runtime | consume those decisions; release storage without moving other handles; final reporting only after cleanup |

Each Home token/native owned resource has one cleanup responsibility. Borrowed
handles have none; `share` creates another Home. Native acquisition owns its
resource immediately, including allocation failure while wrapping/registering
it; only a successful destination commit transfers responsibility. This is
not a new source registration API or a second runtime registry. Unknown native
release contracts must not be admitted as compiler-managed construction.

| Cutpoint | Owned state and failure action |
| --- | --- |
| argument preparation / allocation not successful | preserve uncommitted caller Homes; clean acquired temporaries; no nonexistent outer release |
| acquisition succeeded, destination commit pending | acquiring frame/operation retains responsibility, including wrapping failure |
| initializer / Birth / override executing | track only successful field commits; clean frame obligations, initialized fields in reverse declaration order, native payload, then outer storage |
| replacement committed, old release Faults | do not roll back the install; new field remains owned and is cleaned once; old release is not retried |
| normal publication | transfer the first Home once to the result destination; disarm incomplete-construction cleanup |
| Fault propagation | common caller exit handles its surviving obligations; outermost entry reports after cleanup; no Normal join |

The source/exit verifier also owns constructing-receiver non-escape: reject
storage, return, capture or opaque forwarding of `me` before publication unless
an exact existing non-escape contract proves the use. A physical reclaim API
cannot establish this property. Include alias-mediated escapes in the same check.

Use static cleanup edges and, where control flow requires them, initialization
flags keyed to resolved places. No runtime AST/name scan, heap cleanup list,
new Call carrier, fake empty-obligation proof, or per-instruction receipt chain.
The first Fault remains primary; later cleanup Faults are suppressed and
remaining cleanup is attempted best effort. This is resource cleanup, not
rollback of I/O. The incomplete outer `fini` never runs; complete child release
uses the existing last-Home rule. Missing plans reject before artifact, while
admitted runtime contract failures follow the cleanup path, not bare trap.
Host OOM abort/process kill is not a cleanup-complete language Fault witness.

### Ordered tasks and finish line

1. **Common exit connection:** extend existing `resolved_control_flow` / Home
   plan ownership and `ordinary_new_coseal` with exact New Fault cutpoints and
   outward continuation. Return/ImplicitVoid and declaration-only Home ABI are
   not this proof. Complete local/native ownership and field destination
   obligations through existing HOME/EXIT tasks; do not require new syntax,
   Result `?`, Shared or all-backend implementation to express this dependency.
   Caller-prefix issuance and local-install validation are connected; the
   construction-internal scalar source plan is now verified. First specify the
   missing store/release/reclaim operations and Normal/Fault status mapping;
   Call/NewBox Invoke alone is insufficient. Reuse the accepted common ABI;
   bind each source site to the control emitted by that same New owner, then
   consume the ledger into real exit operands before function finalization.
   Do not insert a source-only metadata retention checkpoint or store raw
   ValueIds in metadata. Existing backend rejection stays intact until that
   consumption is implemented. Tasks 2–3 consume this same ABI; executed
   propagation is not a prerequisite for choosing it. Task 1's control checkpoint
   permits tasks 2–3 to connect runtime/reclaim and transport; their full runtime
   proof belongs to task 4, not a circular prerequisite of the consumer.
2. **Construction cleanup connection:** `ordinary_new_admission` and
   `new_expression` consume the same plan through allocation, Birth and
   overrides. The selected typed-object store gains stable-identity reclaim;
   no double release or early publication. Raw/native wrappers retain ownership
   until handoff; existing raw alloc/free exports alone do not satisfy this.
   Name the admitted storage profiles and their reclaim consumers explicitly;
   do not silently drop the default profile to make a proof pass. Unsupported
   profiles reject before artifact. Bind runtime store selection to the admitted
   capability before allocation; a later env choice cannot bypass the guarantee.
   Accepted independent runtime prerequisite (2026-09-06): replace the indexed
   SafeMutex/SingleThreadExact store with tombstoned optional slots. Checked
   insertion preserves existing negative handles; reclaim validates handle/type,
   takes the payload once and drops it outside the lock. Never shift or reuse
   indices. Only reclaim may recover a poisoned guard; ordinary accesses remain
   failed and poison is not cleared. Payload drop releases inert storage only,
   never child Homes or hooks. SingleThreadExact remains thread-confined;
   pinned/direct reclaim stays unsupported. Retained tombstone metadata is not
   allocator completion. Local-owner tests cover duplicate/mismatch/invalid
   reclaim, stable unrelated handles, poison and failed-store nonmutation.
   This closes the storage primitive, not HomeRelease, source-slot linkage,
   common Fault transport or executable Birth; return directly to task 1.
   Verification: six indexed-storage tests pass under each explicit safe_mutex
   and single_thread_exact profile; selected-accessor tests additionally prove
   pinned/direct rejection. Kernel quick check passes. The broader `cargo test --profile quick
   -p nyash_kernel typed_object -- --test-threads=1` with safe_mutex reports
   31/1; parent-source 98db26896b, same command/environment, reports 25/1.
   Both fail only `exports::typed_object_pinned_arena::tests::
   direct_slot_object_v0_header_and_field_offsets_are_stable` at the unchanged
   negative-handle assertion. This is separately observed baseline debt, not
   a waiver for indexed-storage failures or whole-library green evidence.
3. **Birth consumer cutover:** return to input-wire/published-C steps 2–4 in
   `workstreams/type-contract-status.md`. Normal Unit and pending Fault have
   distinct internal control transport; never expose a status as a source value
   or use Dynamic's TextScan CallOut. Consume task 1's fixed physical return
   layout with the common exit consumer, not a second Birth failure owner.
4. **Execution and retirement:** existing test owners prove acquisition-to-
   commit failures, second-store mismatch, child failure, override failure,
   replacement cleanup Fault, and a prior live caller object. Assert parent
   hook zero, correct child terminal release, stable unrelated handles,
   primary Fault, no leak/double cleanup, no failed-construction result. Include
   `apps/typed-object-birth-min/main.hako`: `execute_mir_mode` ->
   `compile_normal_with_published` -> `published_mir_object` -> C transport;
   EXE exits 30; the same owner's same-source OBJ links and executes with exit 30.
   JSON smoke/publication tests are not substitutes. Before type-contract step 5,
   name selected origin/name, tagless projection and status-to-trap file/functions
   and caller counts here; delete their exclusive code at zero in the same series.

Task 1 first connects Birth-body non-escape verification directly to
`issue_instance_constructor_semantic_batch_v1`, before semantic row publication.
The exact receiver BindingRef seeds a conservative alias fixed point over sealed
local initializer/plain-rebind relations. Every receiver/alias occurrence must
be an admitted local alias edge or exact field receiver; capture, forwarding,
stored values and unclassified uses reject as non-escape unproven. Reassignment
never clears the alias set without a reaching-definition proof. Body relations
are not a complete child graph; missing relations cannot prove safety.
Acceptance uses existing package tests for own-field/direct-alias positives and direct/alias
store, capture, forwarding and branch-rebind negatives. No new receipt is issued.
This prerequisite does not prove initializer/override non-escape or cleanup.
Checkpoints: package 77/77 covers Pair publication/pre-artifact rejection;
nested `me` stops at resolver, alias capture at the new verifier. No new assets
or legacy-file deletion claim. Package 78/78 replaces Allocation-event discovery
with exact initializer/binding relations (old discovery 1->0, not Home cleanup).

### Task 1 implementation decision: caller-prefix Home transitions

Decision: use the accepted Home semantics, not another Birth-only ownership
system. This is not Fast-path reuse: source-owned availability and its local
commit consumer do not yet exist. The read-only review closes the ownership
boundary; it does not supply an implemented cleanup receipt.

Coverage boundary: one exact callable entry -> each selected direct-local New
Fault/Normal successor and its exact local installation. Includes every
statement/expression on that straight-line prefix; excludes the later body and
construction-internal cleanup implementation. Exclusion does not waive the
whole-function or runtime finish line in tasks 2–4.

The common Home issuer belongs in `resolved_semantics`, beside the existing
Home vocabulary. It consumes one `ResolvedFunctionLoweringInputV1` source loan,
exact initializer/declaration BindingRefs, entry demand contracts, and the
already-selected ordinary constructor relation. `resolved_control_flow` owns
the outward Fault continuation, not availability, rebinding or ValueIds.
`ordinary_new_coseal` binds the resulting transition to its constructor claim;
it must not become a second Home classifier. Source statement order, not the
Allocation inventory or map order, determines the Normal-state sequence.

| Prefix outcome | Authority and behavior |
| --- | --- |
| exact entry demands, or source-proven no inputs/captures | initialize caller state; absence of a demand product is not an empty state |
| direct-local ordinary New | New Normal yields a pending first Home; exact local completion alone installs it |
| exact plain local alias | retain the supporting Home; issue no new owner and do not transfer |
| literal/source-proven Trivial expression | no Home acquisition; no MIR-type or raw-bit inference |
| uninitialized local declaration | no acquisition at declaration; later assignment still requires its own contract |
| unknown call/native/field/index operation, rebind, ownership-changing operation or structured control | no complete prefix plan; retain the existing unsupported backend capability, never skip the subtree |
| Return or other terminal before selected New | no reachable prefix; do not issue a transition for dead suffix syntax |
| known required argument/construction unwind, implementation missing | retain an explicit exact-site dependency; backend admission remains closed |
| foreign/duplicate/drifted source relation or duplicate physical consumption | fail before publication; no fallback or default state |

Each transition contains exact source coverage, destination binding/scope,
Normal pending-to-installed relation, caller Homes available on that Normal
path, and Fault continuation. Fault first requires the active argument or
construction frame's unwind, then releases prior caller Homes in reverse
declaration order and propagates outward. The failed New destination is absent.
The constructor dependency owns initialized fields/native payload; the caller
plan never copies or invents that internal set. Unknown caller-prefix meaning
prevents issuance; a known but unresolved unwind dependency prevents backend
activation. Neither is a fabricated empty cleanup or a language rejection.

Use the existing lifetime chain:

```text
ordinary_new_coseal -> package claim/local source relation
  -> ordinary_new_admission -> new_expression (including overrides)
  -> drive_local_statement_with_receipt_v1
  -> CompletedLocalBinding(ordinal, initializer, local)
  -> CallableSemanticLoweringState::record_completed_local
```

The last owner already matches the source declaration to physical values,
but does not identify allocation/Birth control cutpoints.
The first connected implementation retains destination/declaration identity in
the existing claim ledger after target take, records the result after overrides,
and verifies initializer/local/ordinal at that handoff. Pending or mismatched
completion cannot pass package finish. The source prefix issuer is now attached
to that same claim: target take checks prior Homes' completed installations,
and pending rows retain the source relation. The exact New site now carries a
control-owned outward Fault target and exact body scope, not a Normal Return
or E0 empty-cleanup receipt. Physical exit projection remains open.
Preserve the resulting Home projection for the common exit consumer; a semantic
plan stored and then dropped at scope finish is not connected completion.
Exit operand materialization must occur before function finalization: the port-aware method draft
owner immediately after `port.lower_body`, and App Main immediately after
`inner.lower_body` inside its scoped source callback. Callable state `finish()`
is too late: the cataloged draft has already entered the collector by then.
Use these existing hooks, not a second publication owner or a post-publish map.

Before body descent, those same method draft owners and
`lower_app_main_root_body_v1` must materialize the selected hidden frame ABI:
internal callees borrow the caller frame, while the outer entry owns its frame.
Only then may New/Birth emission receive its explicit frame operand. The later
pre-finalization hooks consume exit relations; they cannot retroactively define
an entry operand. Keep source parameter counts unchanged and reject missing,
foreign or wrong-type frames before publication. Do not add a port/trait axis.

Read-only readiness review (2026-09-06): `NewLocalCommitV1` holds initializer/local
ValueIds, not emission locations. `lower_ordinary_raw_new_with_port_v1` consumes
the claim into `constructor()`, emits NewBox/Birth and returns only the value.
Whole-expression completion after overrides cannot recover those cutpoints.
Bind exact source site to freshly emitted allocation/Birth control through the
existing claim port while emitting; then consume that relation before finalization.
Do not scan names, receivers or instruction order to rediscover it. This is
session-local physical correspondence, not a new semantic issuer or side table.

### Enclosing construction source checkpoint and next consumer

The existing `ParserBoxSourceSealV1::box_site()` now survives ordinary coverage,
constructor syntax/semantic rows, exact New claims and pending local completion.
Final-source validation preserves the enclosing declaration and fields; its
borrow rejects foreign parser/same-ordinal rows. Constructor lookup consumes the
retained parent relation instead of ordinal-only identity. Birth keeps its real
ConstructorSourceId; NoBirthZero has no fabricated constructor occurrence.
Caller initializer/override sites remain caller-owned. New completion for a
different Box rejects without consuming the pending installation.
Verified source checkpoint: parser 41/41, package 90/90 and the exact
constructor branch-trigger test 1/1; vm-reference quick lib check passes.
The earlier source-authority filter matched zero tests and is not evidence.
No baseline change, no runtime cleanup, production retirement or EXE claim.

**Change (implemented source prerequisite, 2026-09-06):** retain
one AST-free construction plan in the existing constructor/New products. The
constructor source/Home issuer joins exact lexical `me`, resolved Plain field
assignments and the retained enclosing declaration; ordinary-new only co-seals
the result. This is task 1's code slice, not a new card or another source audit.
Old execution authority removed in this prerequisite: none; the fixed Birth
EXE/OBJ gate still requires the subsequent connected cutover and deletion.
**Contract:** field identity is the retained branded Box plus declaration ordinal,
not a layout offset. Keep declaration-ordered Home demands and source-ordered
`ResolvedAssignmentSourceV1`/field-ordinal pairs.
The plan also retains the existing ConstructorSourceId/resolver owner pair after
target take: Box identity alone cannot qualify constructor-local store sites.
NoBirth keeps explicit constructor absence; no identity is reconstructed.
Stored initializer presence is projected from the existing parser-sealed trigger
into ordinary Box coverage; normalized Birth stores cannot reconstruct this fact.
Source `i64` is classified as Trivial once; existing parameter Home ABI does not classify fields.
The first profile covers only direct own-field Plain stores from exact formal
borrows or integer literals, with every statement/expression accounted for.
No weak/default/delegation, alias/rebind, structured control, nested/native/child
acquisition or override is silently omitted; unsupported dependencies stay explicit.

| Source/outcome | Required state |
| --- | --- |
| Allocation Fault | No outer storage obligation acquired |
| Allocation Normal | Unpublished outer storage reclaim obligation acquired |
| Store Fault | This store has not committed; preserve prior initialized set |
| Store Normal | Mark the exact declared field initialized |
| NoBirthZero, no fields | Eligible source plan; outer reclaim obligation remains |
| Birth, every required field initialized | Eligible for later publication, not yet executable |
| Required field initialization unproven | `Unavailable(InitializationContractMissing)`; no zero-fill |

Initialization and Home demand are separate: Trivial fields do not imply empty
construction unwind. Unavailable is profile eligibility, not source invalidity.
**Done:** existing package tests cover renamed/reordered fields, multiple store
cutpoints, empty NoBirthZero, foreign parent/site/receiver, incomplete initialization
and hidden acquisition/residual syntax. Preserve the fixed Pair publication test,
module README/reference contract and existing M7-S/pointer gates; add no guard or
test file. Then bind this plan to actual New/entry Fault CFG in the same series.
**Stop:** never infer obligations from MIR storage, absent Call/Allocation events,
caller HomePrefix or non-escape alone. No guessed empty plan or opaque unwind stub.
Native/child/override coverage remains required by task 4, not waived by scalar
acceptance. Backend stays rejected until runtime/reclaim, typed-C, fixed EXE/OBJ
and failure proof plus old-edge deletion are complete.

The next consumer is physical entry/New/exit lowering, not another source-plan
issuer. Owning/native/child/override extension remains in tasks 2–4 with explicit
unavailable dependencies until covered. Transport is not a cleanup proof.
The worker verified that caller HomePrefix/NewFaultContinuation
leave construction unwind unresolved, `resolved_control_flow/cleanup` is E0-only,
and current DestroyOwned carries neither common Fault status nor incomplete
construction semantics. Consequently, direct Birth Fault -> DestroyOwned(prior
locals) -> ReturnFault would drop obligations even if backend execution is fenced.
Canonical cleanup needs two distinct actions: discharge a source-issued Home
once (continue on either cleanup outcome), and reclaim unpublished outer storage
without its hook after field/native cleanup. Do not hide missing obligations in
an opaque UnwindConstruction(receiver), raw handle scan or assumed empty list.
Order within tasks 1–4: exact Box/field source -> construction obligation issuance
-> entry/emission binding + complete Fault CFG -> runtime/reclaim + typed-C
-> fixed EXE/OBJ/negative proof and old-edge deletion. Runtime implementation is
not a prerequisite of source issuance; source issuance is a prerequisite of CFG.

### Physical connection decision (task 1, not a new task family)

Decision: accepted common Invoke/allocation control with normal-only projection
(2026-09-06, independent worker review); implement before physical retention.
Source authority + canonical issuer: existing constructor key/receiver/source
args, source Home prefix/NewFaultContinuation and completed local BindingRef to
ValueId relation; the existing pre-finalization owner consumes them once.
Non-authority: metadata-only ValueIds, KeepAlive, source-only copied plans,
Normal Unit, or TextScan's external/lease-shaped CheckedCallOut plan.
Fail-fast boundary: no executable Birth route until unwind/exit operands are
represented in canonical CFG and included in ordinary use/rewrite verification.
Next physical slice (after the construction-source prerequisite above): bind
the implemented control/projection vocabulary to exact emissions and consume the
ledger at the two pre-finalization hooks; no second Call target carrier.
Non-claims: no runtime cleanup, storage reclaim, completed task 1 or EXE proof.

Physical mapping decision (2026-09-06, read-only worker review consumed): extend
the finite Invoke operations with exact FieldSet, HomeRelease and
ReclaimUnpublished roles; never wrap arbitrary MirInstruction. Each is Unit
with explicit Normal/Fault successors and no source status/result projection.
Exact scalar setters currently return failure and selected C checks then traps;
`i64` admission does not prove runtime infallibility. FieldSet Normal commits,
Fault must leave the prior initialized set intact. Bind the existing exact
source field relation, receiver and value; names are not an alternate issuer.
HomeRelease consumes one source-issued Home under its admitted release contract
once, continuing on either outcome. ReclaimUnpublished consumes admitted outer
storage only: no parent fini and no recursive field/Home release. Neither is an
alias for successorless DestroyOwned. Status-to-Fault conversion records once;
ReturnFault forwards without recording again.

Construction progress remains local, not in the diagnostic FaultFrame. Birth
drains its initialized field/native subset before Fault return; the New caller
then reclaims outer storage and handles its own surviving Homes. Birth Normal
transfers the complete structural obligation to the caller through overrides
until publication. Override Fault drains that current set before outer reclaim.
Complete children use normal terminal Home release, not the parent's no-fini
rule. Straight-line scalar stores use static CFG prefixes, not a runtime bitmap
or cross-function ValueIds. Conditional owning fields require local state only
when their source control actually demands it.

Before implementing those roles, close the named physical contracts in tasks
1–3: exact source-store-to-slot linkage, terminal Home release for prior caller
objects, stable default-storage no-hook reclaim, and no mutation on rejected
stores. Existing Arc drop, empty Trivial fields and successful local tests do
not supply these contracts. Assignment-owner extraction is verified:
`builder/fields.rs` 787->536 lines, `fields/assignment.rs` 267. Existing callers
use the same re-export/inherent methods; no forwarding runtime layer or behavior
change. Quick lib check, fields tests 5/5, semantic-package tests 90/90 and
existing fieldstore/weakfield/assignment/fieldget plus pointer/M7-S guards pass.
The source total is 803 versus 787 (imports/module boundary); this is BoxShape,
not net-negative retirement. No test/ignore/baseline or production behavior change.
The bounded review located the source-store handoff at
`raw_expression_dispatch/statement_surface.rs` before child descent. Preserve
the existing assignment/constructor/Box identity there; do not use
`field_facts::declared_field_contract_identity` (origin/name lookup) as issuer.
The published object definition must retain that Box relation and project its
declaration ordinal through the existing layout algorithm.

Exact publication decision (2026-09-06, read-only worker review integrated):
use one opaque module-local `CanonicalObjectIdV1` and a field reference
`{ object, declaration_ordinal }`, not paired source/published key types.
`issue_instance_constructor_semantic_batch_v1` assigns checked IDs once from
exact ordinary-Box coverage, including empty NoBirth declarations. Its existing
invocation brand and private source correspondence prevent foreign-module reuse;
equal integers alone never prove identity. Names and constructor keys are not
object identity. Runtime type IDs remain a separate checked layout projection.
Source/transfer checkpoints retain exact identities and reject residuals; Git owns detail.
Baseline: `physical_entry_lane_adoption_tests::emits_one_direct_length_call_and_i64_receipt_in_unpublished_session` fails AlreadyIssued at line 115; identical-lock parent f42be8be61 reproduces it with `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 cargo test --profile quick --lib mir::builder::resolved_lowering::physical_entry_lane_adoption_tests:: -- --test-threads=1` (12/1).
Layout verified: package 95/95, layout 20/20, metadata 2/2, collector 10/10, refresh 6/6, receiver 4/4, JSON 121/0/1. Global routes 139/6; the six baseline-listed failures reproduce at identical assertions in f42be8be61's built binary with the same filter.
Postprocess 3/3 covers both late-refresh failures; layout tests cover backend-preflight rejection. Exact store/control checkpoint: package 95/95; binding drift 1/1, Invoke 5/5, numeric verifier 13/13, numeric contracts 7/7, receiver 5/5, scoped children 2/2, assignment 1/1, constructor 5/5, layout 21/21. Global call 180/6 matches f42be8be61's six failure transcripts (not immediate-parent/full-lib proof). vm-reference/pointer/M7-S pass, max source 757. Initial Pair SSA failure was current-change span omission, fixed through the existing block insertion API and rerun green. Birth runtime remains fenced; outward New/Home/Fault and typed-C tasks remain open.

Within task 1, execute these connected steps after split validation:
1. Move one exact, initially unlaid-out definition payload with callable drafts
   through `ModuleDraftCollectorV1` into the private candidate. Drain precedes
   external publication: `program_root_lowering` drains before `finalize_module`
   installs declaration metadata and calls `refresh_module_typed_object_plans`.
   Reuse that existing allocator; never allocate runtime IDs in collector drain.
   Before layout, retain declaration-only eligibility in the existing definition:
   the exact Box loan rejects inventory drift and marks inheritance/delegation,
   sync/generic/implements/native attrs unsupported, without dropping the ID.
   A layout-specific field accessor consumes this disposition; construction body,
   initializer execution and cleanup eligibility are not layout authority.
   Reserve canonical positions in object-ID order (including unsupported shapes),
   then allocate compatibility layouts after that prefix with checked arithmetic.
   Store canonical layouts only in the canonical definition table. Repeated
   semantic/backend refresh preserves and validates these allocations; it rebuilds
   only compatibility layouts. No canonical MIR-observed storage inference.
   Finalize projects definition ID -> metadata membership once; no new context/port
   axis or diagnostic-name -> definition search. Projection drift rejects.
   Refresh and semantic refresh propagate Result through existing ContractRefresh
   callers, including postprocess; no repair between initial/final verification.
   External admission borrows resolved selected layouts; old-format export is one-way.
2. Same constructor loan -> existing CallableSemanticLoweringState: non-Birth is NotConstruction; Birth Err is RetainedUnavailable with publication/backend fence preserved; Birth Ok is Selected. Caller-local OverrideUnsupported never demotes the shared definition.
   At assignment pre-descent, Selected takes its exact store once; missing/foreign/duplicate is an error, never fallback. Emit FieldSet Invoke with real base/value operands.
   Validate emitted bindings before `prepare_port_aware_draft_body_completion_v1` and on the finalized function before collector capture; scope.finish also requires completion. Take alone is insufficient. No new ChildPort field/supertrait. Existing `exact_numeric_field_contracts` consumes the published field ordinal, never receiver origin/name: Pair retains two checks bound to actual terminator/value/receiver; runtime checks remain required.
   Cut selected origin/name reconstruction in this series; retained-unavailable definitions are not retries, and execution stays fenced until consumers exist.
3. Connect the existing tasks 2–3 runtime and typed-C consumers, replacing the
   selected status-to-trap path. Table/view tests alone never close task 1 or
   authorize Birth execution; fixed Pair EXE30/OBJ and task-4 failures still gate it.

Existing tests cover same-shaped Boxes, rename/store order, empty NoBirth, foreign
source, duplicate/missing ID, ordinal/layout drift, mixed/repeated allocation,
changed compatibility shapes, unsupported reserved positions and overflow.
Reject residuals before publishing functions or definitions. The payload has one
owner; the semantic batch retains immutable correspondence, no second table.
No new card, semantic receipt, registry or per-cohort guard.

Caller binding decision (2026-09-06; existing task 1, read-only review integrated):
Prepare/emit on the existing New port record progress in NewLocalCommit; release
ledger borrows before argument descent. At intact New dispatch, queue existing
CallArgument/NewFieldInitializer source roles for actually evaluated children;
IntegerLiteral folding queues no argument. Scope restores the parent before emit.
Adapter delegation verifies package/inner ledger Rc identity and exact callable
owner/site. Missing/foreign scope or duplicate prepare/emit rejects, never retries.
`ordinary_new_admission` must consume the retained claim, not reduce it to the
constructor alone. Bind allocation, Birth, outer reclaim and prior Home cleanup
to actual Invoke operands before local installation and physical finalization.
Prior Homes join issued BindingRefs to completed New/locals and exact declarations; preserve reverse acquisition order. The last New prefix cannot prove Return.
Normal-exit Decision: raw App Main has no retained Completion; issue its first existing verify_function_completion_v1 inside the exact AppMain ordinary_new_coseal loan after initializer membership/selected New mapping. Do not widen generic selected roles or reuse the optional direct-call loan.
Extend the same Home source scan through that exact Completion, check scalar return versus escaping Home, and co-seal terminal bindings into existing cleanup obligations; unsupported suffixes stay unavailable. Completion errors/unavailable exits are unselected bookkeeping, never runtime admission: retain the backend execution fence. Generic/S6C issuance is unchanged.
The existing New ledger is created at package issuance instead of retaining a second preinstall claim-array carrier; install shares it with root Completion. Return selection precedes Match/value descent; final validation requires exit consumption and actual bindings. Connected checkpoint: package 97/97, Invoke 7/7, raw dispatch 20/20, New route 2/2; vm-reference/pointer/M7-S green. No new receipt/table/port axis; no full Home Flow or Pair EXE30 claim.
Construction eligibility is not destruction eligibility. The exact whole-Box issuer
classifies the closed plain-i64/no-hook declaration profile: all fields explicit i64,
no weak/inherited/delegated/native storage, and ordinary member roles only; legacy
fini-shaped methods, property/delegate/compatibility members remain unavailable.
No Arc/Trivial/layout inference or missing-hook-slot default. The existing New
co-seal copies definition-owned disposition plus object ID before collector transfer;
construction Err never erases either. Foreign/missing/transferred lookup rejects;
retained claims work after transfer. No second table. Connect the actual New/Fault
consumer in this series. Normal exit consumes at existing Return descent: clean suffix -> Return, any cleanup Fault -> fault-pending suffix -> ReturnFault; no new status opcode. Production CFG paths prove second/first order, one release per path and no Reclaim; direct ledger witnesses prove bookkeeping only. Unavailable suffix/foreign exit/duplicate consume must reject selection or consumption. Tasks 2–4 remain.
One frame is owned by `CallableSemanticLoweringState`: the exact App Main source
entry installs RootOwned, ordinary callable entries Borrowed. Direct-call-loan
presence is not a root witness (New-only Main has none). Construction stores
borrow this frame; independent issuance removed, no port axis. Checkpoint: fresh package 95/95, frame 1/1, binding 1/1, Invoke 6/6; vm-reference/pointer/M7-S green. Runtime/New connection remains open.
Completion covers draft preparation/capture; App Main registers its exact owner
on the existing New ledger, retained across drain and checked after final PHIs.
`OrdinaryNewClaimLedgerV1::is_empty` must not accept retained unconsumed plans
merely because initializer/local completion succeeded. Missing/duplicate/drifted
bindings reject at both checks, including entry FaultFrameEnter and local Copy.
Selected deletion: claim-to-constructor erasure, bare NewBox/Birth Call emission,
and completion without physical consumption. Preserve nonselected compatibility;
selected failures never retry it. Runtime admission remains closed until tasks
2–4, including continued cleanup on both outcomes and EXE30/OBJ, pass.

`passes/dce/elimination.rs` reads instruction operands; simplify-CFG rewrites
blocks, not arbitrary FunctionMetadata. Metadata ValueIds could become stale;
source-only lists lose physical correspondence when the scoped ledger drops.
The user accepted bounded suppressed diagnostics on 2026-09-06; the normative
policy is `docs/reference/language/semantic-kernel.md#cleanup`. This resolves
the diagnostic-capacity consultation, not runtime cleanup implementation.

Task 1 implementation contract after review:

- Keep the primary slot separate from a preallocated suppressed buffer. A Fault
  origin records once; propagation never records the same Fault again. Buffer
  overflow retains the ordered prefix and sets an omission flag, without growth,
  primary replacement or shortened cleanup. Report only at the final entry.
- Fault capture must not allocate or format text. Allocation failure uses a
  static reason/site and inline details. An already-evaluated panic message
  transfers its owned residence, not a borrow into a departing source frame.
  Consuming/discarding an omitted diagnostic must release its payload through
  an allocation-free, non-user-hook path; no leaked omitted message is allowed.
- Use per-call Normal/Fault status independently of the shared primary slot:
  successful cleanup can return Normal while an earlier primary remains pending.
  A caller-owned frame is synchronously borrowed; no TLS or global state.
- Reuse the existing MirCall/Callee for target and arguments. The selected
  representation is an Invoke control terminator with normal/fault successors,
  not a second Call target carrier or the external TextScan lease protocol.
  Unit has no result slot; non-Unit storage is readable only on the normal edge.
  The following normal-only projection is required before admitting value
  results; no reader guesses its definition or hidden ABI lanes.
- Consume the source/local ledger at the caller completion boundaries above into
  real control/cleanup operands. Update ordinary use/rewrite, CFG and verifier
  handling together; do not add a metadata table or KeepAlive repair.

Canonical control vocabulary (one owner, not an extensible effect wrapper):

```text
InvokeOperation = Call(existing MirCall with dst=None)
                | NewBox(exact CanonicalObjectIdV1; allocation only)
                | FieldSet(exact CanonicalFieldRefV1, base, value) | HomeRelease(object, value) | ReclaimUnpublished(object, value)
Invoke(operation, fault_frame operand, normal_landing, fault_landing)
InvokeNormalResult(originating Invoke block, dst)
ReturnFault(fault_frame operand)
FaultFrameEnter(dst, RootOwned | Borrowed)
```

Invoke defines no SSA value. NewBox's handle is defined by the projection in
its dedicated Normal landing, not on the allocation terminator; Birth Unit
has no projection. Allocation consumes the existing construction object's ID,
not class text or Birth arguments; arguments lower once before allocation and
belong only to the Birth call. Module verification requires the exact definition;
runtime-layout availability remains a separate backend capability check.
ValueId rewriting preserves the object ID. The earlier string/args Invoke payload
had only verifier-fixture writers and is replaced in place, not kept as a variant.
A value-returning Call uses its canonical result ABI and
the same projection. This preserves the current block-local SSA definition
model (`verification/ssa.rs` and `verification/utils.rs`), which otherwise makes
a terminator's dst visible on both successors. Require distinct Normal/Fault
landings (Normal is neither function entry nor Invoke origin), exactly one value projection (zero for Unit), first after any PHIs,
and the origin Invoke as the Normal block's sole predecessor. Verify projected
uses and PHI predecessor edges under Normal dominance at publication, even if
ambient `verify_allow_no_phi` disables ordinary compatibility checks. CFG rewrites
must preserve/rewrite the explicit origin link; they must not merge it away.

Physical ABI: per-call Normal/Fault status return, borrowed caller-owned Fault
frame and a normal-result out slot only for value results. These are hidden
physical lanes, not receiver/source arguments or source result values. Existing
Return publishes Normal; ReturnFault propagates without recording a new Fault.
The frame's entry definition and every forwarding use must be explicit and
type-checked; missing frame cannot be synthesized from numeric zero or a global.
Read-only entry review selects FaultFrameEnter as the intrinsic internal
operand definition, not a source MirType or an extra `MirFunction.params`
element (which would corrupt receiver-offset and exact entry-count checks).
Require one entry-prologue definition, no incoming CFG re-entry, and exact
Invoke/ReturnFault frame uses; forbid ordinary arguments/Copy/Phi/store/Return
escape and source-type metadata. RootOwned comes only from the selected outer
entry role; internal methods borrow. ValueIds are function-local, so scoped
source-owner checks, not matching raw numbers across functions, prove origin.
Allocation Fault cleans prior caller Homes without reclaiming a nonexistent
object. Birth Fault also reclaims the incomplete allocated object. Both outcomes
of each cleanup continue to the next required cleanup. Admission stays closed
until these consumers exist; the control checkpoint alone is not runtime proof.

Task-4 acceptance retains injected allocation failure, cleanup Normal with pending primary, later-Fault ordering, overflow/last cleanup, propagation without rerecording, message lifetime after source-frame release and no leaked omitted payload. ABI unit tests never replace fixed production EXE30/OBJ proof or runtime release/reclaim.
Source/control checkpoint: exact source arguments lower once, cleanup uses real operands, Invoke results are Normal-only and Unit Birth has no result. Artifact-specific validation now consumes retained eligibility in the existing Completed owner and exhausts root lifecycle sites, sibling functions and empty Birth definitions before callback. Fresh locked quick lib build; package97/97, pipeline20/20, construction2/2, module negative1/1 and pointer/M7-S pass. The added negative initially attempted insertion after a terminator; corrected to a separate block, then rerun green without weakening the assertion. Aliases add no Home; rebind/unknown acquisition/entry gaps/Handle arguments/overrides remain unavailable. Generic lifecycle admission stays fenced; tasks2–4 and fixed EXE30/OBJ remain required, no whole-lib or execution claim.

**Change (tasks 2–3):** replace escaped mutable-MIR artifact emission with one synchronous consumer terminal in `NormalDefaultPublishedPipelineV1`; reuse its build/finish path.
**Contract:** worker premise review was required because EXE and OBJ have different post-compile owners and Rust privacy must prevent raw-module lifecycle admission. Source authority remains retained root/Birth products, never observation metadata or successful bookkeeping validation.
**Done:** both named artifact callers consume the same final borrowed view; strict verification and fallible commit preparation precede the callback. Fixed Pair EXE30/OBJ, final-binding/eligibility rejection, callback-count-zero before admission, no retry and generic-clone rejection remain acceptance.
**Stop:** no source eligibility, uncovered lifecycle function, final verifier failure or commit-preparation failure terminates before callback/artifact. No second pipeline, permission receipt, fixture workaround or source expansion.
Census boundary: MIR-mode EXE / LLVM-mode OBJ -> normal compiler -> published host object -> C API; includes post-compile mutation and view reconstruction; excludes already-closed collector transport and nonselected compatibility callers.

| Existing retained state | Diagnostic validation | Selected lifecycle artifact |
| --- | --- | --- |
| Root payload absent | Preserve absence; no source root validation claimed | No root lifecycle permission; reject uncovered root lifecycle sites |
| Root Unregistered | Preserve NotIssued; bookkeeping is not root validation | No root lifecycle permission; reject uncovered root lifecycle sites |
| Root NoSelectedLocalNew | Validate registered root despite empty selected-New set | No root lifecycle permission; reject uncovered root lifecycle sites |
| Root source Unavailable(reason) | Preserve reason and validate existing bindings | Reject; historical SourceComplete observation cannot upgrade it |
| Root source complete, exact final bindings | Recheck the same source obligations | Eligible only inside final verified borrow |
| Birth NotConstruction / absent payload | No construction claim | No Birth permission; reject uncovered Birth lifecycle sites |
| Birth RetainedUnavailable(reason) | Bookkeeping may succeed | Reject; Ok(()) is not admission |
| Birth Selected, completed, exact final bindings | Recheck same owner/key/frame/stores | Eligible only inside final verified borrow |
| Pending / duplicate / foreign / residual / drift | Existing exact failure | Callback zero, artifact zero |

Owner sequence: compiler finishing -> retained eligibility/binding checks exhausting lifecycle sites against exact root/Birth bindings -> strict final verifier -> external-commit preparation -> view/callback -> infallible builder commit. Worker review: function-key membership alone misses extra Invoke/frame/ReturnFault sites; validate actual sites and uncovered siblings, including empty Birth targets. Existing Completed owner selects a named artifact validator before consuming retained state; no public allow flag or new receipt. Callback failure propagates; no later mutable finishing or admitted module return.
Privacy Decision: rehome the existing view implementation under `normal_default_pipeline`; the lifecycle constructor is `pub(super)`, generic `try_new` stays fenced. Preserve old import paths with thin re-exports, not another view/token/trait. Preserve existing logical test paths and bodies; baseline manifests are not rewritten for this move.
EXE switch: `runner/modes/mir.rs` snapshots artifact intent before compile and invokes the consumer instead of emitting from returned `result.module`.
OBJ switch: `runner/product/llvm/mod.rs` snapshots OBJ intent before compile; existing CompileOptionsBox/MirCompilerBox forward the same terminal. Selected OBJ exits before MethodIdInjector/PyVM/harness processing, not after a new post-validation mutation.
Host switch: `published_mir_object` consumes the supplied view once; the caller-zero `published_mir_emit` relay is removed. EXE linking reuses its object result instead of calling module-based admission again.
Selected delete-set: escaped-MIR EXE emission; post-mutation OBJ emission; duplicate host view construction; selected compatibility continuation/retry. Nonselected routes remain explicit and lifecycle-fenced.

Root post-finishing checkpoint: retained ledger/exact root key now revalidate New/frame/local/exit bindings after `finish_built_module`, before external commit; observation remains non-authorizing. Package97/97, raw compatibility2/2 and Invoke7/7 pass. Same locked quick lib `normal_default_root_catalog_lifecycle_tests -- --test-threads=1` at parent ed10ce1ad6 and current gives9/4 with identical failure locations/reasons below: known parent debt, no baseline/test weakening.

- `mir::builder::normal_default_root_catalog_lifecycle::normal_default_root_catalog_lifecycle_tests::actual_string_helpers_general_result_row_reaches_its_first_loop_carrier`: raw-compat/runtime-box-fate-retired/static.
- `mir::builder::normal_default_root_catalog_lifecycle::normal_default_root_catalog_lifecycle_tests::parser_scan_package_passes_callable_source_handoff_without_fallback`: static-result-ingress/no-exact-static-target instead of target-unavailable.
- `mir::builder::normal_default_root_catalog_lifecycle::normal_default_root_catalog_lifecycle_tests::source_backed_package_failure_is_terminal_before_builder_effects`: RootExpansion instead of CallableSemanticSeal.
- `mir::builder::normal_default_root_catalog_lifecycle::normal_default_root_catalog_lifecycle_tests::source_bound_static_result_owner_reaches_the_raw_terminal`: raw-compat/runtime-box-fate-retired/static.

Birth retention checkpoint: existing state/frame move with pending draft/exact collector key/normal drain into the post-finishing FnOnce validator. NotConstruction remains absent; validated Selected or RetainedUnavailable moves to Transferred, never fake absence/completion; subsequent use rejects. Worker-audited replacement/raw/canonical/unkeyed/bare/observation terminals reject payload loss and repeat attachment. Package97/97, collector37/37, construction1/1, frame1/1, session6/6, raw2/2 pass. Two transport tests added; existing test modules physically split, names preserved. No baseline/fixture/guard addition or executable admission claim; strict final verification and synchronous consumer remain required.
Borrowed-terminal checkpoint 2f124053db: existing normal pipeline prepares commit and strictly verifies final typed MIR before synchronous consumption; EXE and llvm-boundary OBJ callers are connected, generic lifecycle admission stays fenced. Fresh normal pipeline20/20 (including callback once/error/no-retry, compatibility bypass and fixed Pair rejection), view22/22 with unchanged test names, package97/97; quick lib llvm-boundary+vm-reference and pointer/M7-S green. Deleted unused EXE relay and duplicate host readmission. Subsequent650374b82c closed pre-artifact residual/retry and967cd73cbd connected retained eligibility. Remaining: Home source coverage, typed C control/result ABI and fixed Pair EXE30/OBJ; no CLI Birth execution claim.
Runtime checkpoint: kernel exports89/1 versus parent9164286970 79/1 with identical Cargo.lock (--locked); both fail only `exports::typed_object_pinned_arena::tests::direct_slot_object_v0_header_and_field_offsets_are_stable` at tests.rs:85 (negative handle assertion). Known parent debt, no baseline/test weakening. Current-source Fault8/8 and indexed storage8/8 pass; SingleThreadExact Fault8/8 and pinned/direct rejection also pass. C/C++ ABI header and pointer/M7-S checks green. Typed-C/EXE30 proof still open.

Tasks 2–3 runtime-first Decision: one runtime-owned repr(C) frame, version/presence/length/omitted u32 header, primary plus eight inline suppressed records (reason u32, reserved u32, site u64, two i64 details). C borrows, never copies/mutates records; initialize once on fresh aligned storage, final entry reports then disposes once. Extend existing published transport and `hako_llvmc_ffi_mir_call_dispatch.inc`, never legacy birth dispatch.
Runtime owner: `typed_object_store_backend` checked indexed insert/store/reclaim and `exports/fault` C APIs landed.650374b82c already closes pre-object residuals and typed retry (fresh C build, ABI no-object negative, view22/22). Remaining order: exact FieldGet publication; pipeline-private lifecycle view; existing synchronous C frame extension; same-borrow body export without clone/refresh; shared typed operation dispatch; selected hidden-frame/status definitions and returns. Existing kinds1–7 lack control/frame/result lanes; generic JSON stays fenced. Unit Birth has no result; allocation loads its out-slot only on Normal.
Checked allocation must validate the admitted exact layout and use fallible payload reservation before indexed insertion; do not wrap `nyash_object_new_typed_hi`/`default_layout_fields` (missing/poisoned layout defaults and infallible payload construction). Failure leaves the out-slot unchanged and drops uninstalled payload.
Exact FieldSet validates expected object identity/type and slot under the same storage owner before mutation. Bind the admitted storage profile across allocation/write/read/reclaim: SafeMutex and SingleThreadExact only; pinned/direct mismatch rejects, never fallback. Checked ObjectFieldGet runtime checkpoint: same indexed owner checks profile/handle/type/slot under one guard; Normal writes i64 out, InvalidContract leaves it unchanged, never source Fault/zero substitution. Fresh Fault9/9, indexed storage8/8, SingleThreadExact Fault9/9, pinned/direct rejection and C/C++ header checks pass, including zero/min/max, foreign/missing/reclaimed handle, wrong type/slot/profile, null out and unchanged storage. Legacy field_get_i64_hii stays excluded (default zero/storage retry). No new source Invoke/receipt; release runtime rebuild and typed C consumer remain mandatory.
ReclaimUnpublished and published PlainI64NoHook HomeRelease share only inert detach/drop-outside-lock; runtime must not infer destruction from slot tags, names or absent hooks. Prove duplicate/type rejection, unrelated handle stability and one drop. Typed-C connected series (worker reviewed): extend the same temporary frame with exact function coordinates/roles and lifecycle operands/control/result/layout rows; replace selected definition registry/name scanning in same_module_function_plan/definition_emit/function_emit, then shared prepass and both main/same-module dispatch. Prepass peeks; emission takes once. Role-driven hidden ABI/Return/ReturnFault must replace hardcoded i64 Birth handling. No frame-only completion: fixed Pair EXE30/OBJ30, Normal-only allocation result, Unit/no-out, cleanup Fault preservation, coordinate/role/operand/layout/residual negatives, generic lifecycle fence, and selected old allocation/field/return/generic-fusion edges unreachable are mandatory.
Frame ABI: Normal=0/Fault=1; InvalidContract=2 is physical caller failure, never a third source outcome or ordinary cleanup edge. Validate version/header/null requirements before mutation; arbitrary dangling/misaligned pointers are outside the trusted ABI. Record static reason/site/details first; owned-message handoff stays Rust-private (Box bytes prepared earlier), never accept arbitrary C ptr/len to free. No-message null is absence, never a Rust slice.
Frame acceptance: primary/order/overflow, Normal with pending Fault, propagation records zero, record/overflow allocate zero, invalid header preserves out-slot, payload disposal once. Root observation remains NotIssued / NoSelectedLocalNew / Unavailable(reason) / SourceCompleteAtFinalization, never execution permission. Selected publication Decision: existing completed invocation retains root ledger and exact key plus Birth owner/ConstructionState/CallableFaultFrame with exact collector draft; run mutable finishing, final binding validators, strict verifier and fallible commit preparation before synchronous borrowed view consumption. Callback returns artifact/diagnostics, not mutable admitted MIR; general try_new(MirModule) keeps lifecycle fenced, including clones. No second pipeline/result copy, snapshot, source metadata or semantic receipt. Birth uses existing payload session; alternative drains reject rather than drop payload. Test post-finishing root cleanup/store/frame drift, missing/unavailable/no-New, duplicate, foreign key, residual and stale-observation rejection. Fixed EXE30/OBJ and remaining Home coverage remain mandatory.

Current natural-source grammar permits one initialized local per declaration.
Multiple initializer AST carriers are not a new prerequisite; if later
admitted, protect pending HomeValues before any local installation.
Normal Completion/E0 remain insufficient; do not reuse TextScan's successorless
Fault terminal as caller propagation. No input-ABI reconsultation is needed.
Home source checkpoint: existing ordinary_new_coseal loan joins selected New BindingRefs, successful construction_for(source, actual arity), and synchronously borrowed definitions. Terminal-only nonweak i64 field reads/Integer Add now qualify; shared argument/prefix value_class is unchanged. Fresh package97/97, pipeline20/20, view22/22 pass, including unchanged Pair SourceComplete, renamed Page/fields, alias and reverse two-Home cleanup, Bool/unknown field/Subtract/nested field/prefix negatives, missing/repeated/no-Birth initialization, non-i64 and foreign/transferred definition rejection. Initial pipeline19/1 expected old source rejection; updated to exact C UnsupportedBeforeObject with callback panic retained, then20/20. No ignore/baseline/fixture change or CLI execution claim.
Exact FieldGet checkpoint: existing New ledger retains read/receiver sites, actual receiver BindingRef (alias distinct from Home), Home and CanonicalFieldRef; whole-terminal success commits rows once. Existing field port lowers receiver once and compares its binding ValueId before mandatory ObjectFieldGet. Final root/compiler checks reject operand/field drift, missing/duplicate emission and exact reads in uncovered siblings or covered Birth bodies. Fresh package99/99, pipeline20/20, view22/22, artifact-negative2/2 and remapper/DCE2/2 pass; quick tests/lib and vm-reference+llvm-boundary check green. No new receipt/port axis, fallback, ignore, fixture or baseline change. Fixed Pair retains two exact reads but C admission remains UnsupportedBeforeObject; EXE30/OBJ still open.
FieldGet acceptance: unchanged Pair two exact reads; repeated field at distinct sites; alias exact receiver; failed Add sibling leaves zero committed read rows; duplicate/foreign/missing site, receiver/ordinal mismatch, residual emission and post-finish drift reject. Exact selected rows emit in Prepared or Unavailable root-exit states: unavailable cleanup preserves diagnostic exact reads but never artifact eligibility; wrong phase rejects, only absent rows remain unselected. Completed-module coverage rejects exact reads outside the retained root, including otherwise covered Birth bodies; root ledger owns their exhaustive validation. Selected property/local_field_base/type/origin/name inference and string FieldGet are bypassed without retry. Then replace selected clone/refresh export, name-plan allocation, i64 Birth handling, FieldSet trap conversion and lifecycle i64 returns in the same series. Full Home/native/unsafe ownership remains separately bounded; fixed Pair EXE30/OBJ cannot be waived or replaced.
Same-borrow export Decision: selected view uses existing readonly contract checks (object membership/layout plus all retained-contract validation), explicit CanonicalV1 output and leading-Phi coordinate validation; no clone/refresh/repair. Selected backend preflight shares existing capability checks but rejects any LegacyCallV0, canonical Extern Call or residual extern_call_routes before C: current legacy route validators do not prove missing-row completeness. Generic compatibility keeps its existing owner; no retry or Extern promotion. Test current selected positives, unchanged metadata/site coordinates, stale contracts, nonleading Phi and mixed/residual extern negatives. This closes the host clone/refresh edge before lifecycle transport, not Pair execution.
Same-borrow checkpoint: fresh export5/5, pipeline20/20, view22/22 (array OBJ/EXE Result0), shared capability3/3 and separate-process compatibility-selector export1/1 pass. Initial view20/2 exposed synthetic array inputs with absent prepublication contracts, formerly repaired by backend refresh; both now prepare contracts with the existing Verifier owner before borrowing, without changing executable/unsupported expectations, and recheck22/22 passes. Host selected clone/refresh calls are removed; generic compatibility is unchanged. Physical test split908->638+312 preserves test paths. No new guard/fixture/card, ignore or baseline change; Pair execution remains open.
Private root-binding Decision (worker reviewed): `into_artifact_parts` returns its already retained root key after exhaustive validation. Preserve the existing single view projection/route classification and unchanged ExplicitCompatibility exit; selected strict verification and commit preparation precede a parent-private consuming `bind_retained_root` on that same view. It borrows the exact key from that view's module; missing rejects, None stays absent, and Some is identity only (even for no-New roots), never lifecycle permission. Binding changes no route, rows or capability and performs no second scan/receipt/table/clone. Tests cover real-source callback root retention, renamed exact physical key, missing/absent identity, unchanged route and generic root absence. Existing missing/unavailable/foreign/residual validation remains mandatory. Later typed consumer work must extend the single projection scan and derive Birth roles from existing canonical keys; generic try_new/clones stay lifecycle-fenced, with no premature production activation.
Root-binding checkpoint: fresh pipeline21/21, view22/22 including array OBJ/EXE Result0 and artifact negatives2/2 pass. Same final module/key flows through the existing validator/consumer; renamed/missing/absent root and generic root absence are covered without changing admission. Remapper/test physical splits retain test paths. Initial unavailable-cleanup FINI fixture was replaced by real-source-row physical-state evidence, not source/EXE proof.
Typed-C prepass checkpoint: existing same-module call prepass peeks at published rows before legacy name/plan lookup; emission validates/takes once. Peek leaves residuals, duplicate call/array take is malformed (never absence), and u32 coordinates cannot wrap. Fresh FFI build and existing C test cover repeated peek, residual, malformed shape, duplicate consumption, foreign/overflow coordinate, nested FreeFunction OBJ without legacy call plan, wrong-arity before artifact and real ABI residual/no-object. Existing root test binary re-run view22/22 includes array OBJ/EXE Result0; no fresh Cargo/whole-lib or Birth execution claim. User-requested restart pause; next connected lifecycle consumer and fixed Pair acceptance remain open.

## Typed C program handoff Decision

### CONSTRUCTOR-BIRTH-ABI-HANDOFF-I0 — Birth ABI handoff BoxShape (2026-09-06)

Decision: retain source-issued ABI lanes before C program lowering.
Source authority + canonical issuer: `VerifiedInstanceConstructorSemanticRowV1`
issues one Birth formal relation from its exact DeclaredInstance resolver root:
source ID/target, receiver `BindingRefV1`, ordered parameter `BindingRefV1`s,
source arity and the explicit receiver-lane-0 / argument-lane-N+1 mapping.
`issue_ordinary_new_claims_v1` co-seals that relation with the existing Birth
recipe; `NewLocalCommitV1` is its sole transfer owner after claim take. The
root result is projected only by the existing terminal I64 relation at final
handoff sealing.
Non-authority: `MirFunction.signature`, physical parameter count, Birth name,
JSON, C defaults, positional lane rules and the Pair fixture.
Fail-fast boundary: missing/foreign/duplicate/residual root result, Birth
source/ABI, receiver/formal or one-way transfer rejects before final-view
lifecycle activation and before C.
Smallest next slice: implement this BoxShape in the constructor semantic row,
ordinary-New local commit/final handoff, final-view binder and lifecycle C-frame
projection. The frame must copy the issued lanes, never synthesize wire/input
kind from physical ordinal. Test root I64/Birth Unit, receiver lane, ordered
arguments and missing/foreign/duplicate/drift rejection before C.
Non-claims: C opcode lowering, source execution, EXE30/OBJ30, a new semantic
source interpretation, generic C migration or legacy deletion.

The two read-only audits established the prior gap and the issuer boundary.
This is behavior-preserving retention of an already accepted Birth source form;
it does not make the C body consumer executable.

Decision: provisional JSON validators remain retired at the no-artifact terminal. Preserve each already-issued Birth recipe's canonical target when a root New claim becomes a local commit, then co-seal it with root Completion and the canonical construction draft after finalization.
Source authority + canonical issuer: `OrdinaryNewClaimLedgerV1` owns root Completion and issued Birth recipes; its private finalizer validates one exact retained construction draft and issues the opaque final root/Birth handoff before Atomic Publish.
Non-authority: root symbol, `FunctionSignature`, `MirParamDecl`, instruction position, JSON, C frame, Pair names, canonical-core role, ObjectFieldGet, metadata and positional/default ABI rules.
Fail-fast boundary: absent/foreign/duplicate root, recipe, Birth key, construction draft, Completion or local completion rejects before view, C or artifact; NoBirthZero has an explicit zero-Birth terminal.
Smallest next slice: landed BoxShape—retain the existing Birth target in `NewLocalCommit`, seal the private finalized handoff at artifact validation, and make the final view consume it instead of the production root-name-only bind.
Non-claims: field admission, C implementation, Pair EXE30/OBJ30, ABI/formal projection, Fault cleanup, retry or broader legacy deletion.

### Terminal consumer design (2026-09-06)

Decision: reuse the existing FieldRead ledger and binding resolver in one AST-free terminal consumer; do not create another semantic receipt or registry.
Source authority + canonical issuer: the Completion ownership walk in `home_new_prefix` issues the terminal relation; package co-seal retains its exact FieldRead rows and canonical fields.
Non-authority: raw receiver AST, class/field spellings, MIR types, JSON and physical ValueIds cannot select the terminal meaning.
Fail-fast boundary: foreign/missing/duplicate relation or reads, unresolved binding, uninstalled Home, wrong exit phase, residual consumption or emitted operand/result drift rejects before artifact; selected failure never retries raw lowering.
Smallest next slice: landed. The connected terminal consumer reserves its relation,
consumes two receiver demands and FieldRead rows in order, emits their i64 Add,
then passes the same result to the existing Home exit owner. Selected raw
FieldRead re-entry rejects.
Non-claims: C execution, root/Birth ABI closure, Pair EXE30/OBJ30, whole raw-dispatch retirement or backend parity.

Worker `fieldread_api_audit` confirmed that `take_terminal_field_read(site,
resolve_receiver)` already returns exact `(base, canonical field)` and
`record_terminal_field_read`/`validate_field_reads` enforce consumption and
emission correspondence. The missing piece is physical wiring: the current
`PreparedRawFieldReadV1::exact_object` path lowers receiver AST again. No source
authority is missing for the selected two reads. Earlier worker-unavailable
claims were unsupported: the worker remained running and returned this audit.

Boundary: selected root terminal relation -> root artifact validation.
Includes Completion issuer, package ledger, invocation/package/structured ports,
`build_return_with_port_v1`, exact reads, Add and existing Home exit owner.
Excludes C transport, external compatibility, other result shapes and backends.
This is a bounded implementation inventory, not a caller-zero/Exhausted claim.

Before consumer cutover, correct the `181d7f8e92` classifier regression:
the parent recursively returns Integer for Add, but the new I64Add arm is
rejected as an operand. Thus `(pair.left + pair.right) + 1` loses prior scalar
coverage. Also cover literal-only and mixed nested Add without treating them
as a direct two-field terminal. Keep scalar eligibility and exact direct
FieldRead-pair shape separate; only two direct field operands issue the relation.
Verify parent/current behavior with the same focused commands before calling
this runtime evidence. No new fixture has been added by this design review.

The consumer belongs under `ordinary_new_admission::selected`; enter it from
the live Return port before raw value descent. Validate relation owner/Return
site and reserve consumption once, resolve both bindings, then take/emit/record
the two reads in source order using existing ValueId/block allocation. Emit
integer Add from the issued relation; reuse the existing Home exit owner for
cleanup and Return, retaining the exact sum value through Normal cleanup.
Do not bypass Fault cleanup with a direct Return. Keep physical progress in the
existing ledger, separate from immutable source relations; artifact validation
must detect missing/duplicate Add, reversed operands, wrong sum/Return value,
unconsumed relation and selected raw re-entry. A failed emission poisons this
compilation; no retry or reuse of half-consumed rows.

Deletion unit: the selected terminal's raw receiver/binary/value-return descent
edge. Shared raw helpers still have other callers and are not whole-file delete
assets. Absence alone is not canonical compatibility permission: eligibility
must be fixed at source issuance, and an expected-but-missing relation rejects.
Acceptance uses the actual source pipeline plus ledger mutation negatives;
two earlier relation unit tests are dependency evidence only. Keep changed
sources below 800 lines (design splitting at 760), update module README and
the existing lifecycle reference when changing a public contract, and reuse
the M7-S guard. Do not split this into disconnected API-only tasklets.

Checkpoint: the actual typed-object-birth-min source pipeline reaches artifact
validation with exactly two owned `ObjectFieldGet` values, one ordered `Add`,
and the existing cleanup `Return` carrying that Add result. Physical progress is
owned by `OrdinaryNewClaimLedgerV1`; it retains reservation, exact Add binding
and completion separately from the immutable source relation. Final, finishing
and artifact validation reject unconsumed, reversed, drifted or missing physical
results. This closes only the selected MIRBuilder terminal edge; C physical
program and Pair EXE30/OBJ30 remain open.

### Premise audit and authority boundary

Worker-reviewed at `bd517a7bd5`. This inventory covers the selected final normal
artifact borrow -> shared OBJ/EXE host -> lifecycle V2 companion -> no-artifact
terminal. It includes root/Birth definitions, ordinary physical body, all V2
operation/control families, layout/profile and matching/residual handling. It
excludes generic JSON ingress, explicit compatibility, other backend parity,
and declared-instance `sum()` selection; those are not newly certified parked.
The downstream execution cutover is **CutoverBlockerOpen**, not Exhausted.

- Semantic unit: a published root and its exact cataloged Birth dependencies,
  with source-authorized result/Completion and parameter representations.
  `Pair` is the acceptance witness, never a compiler selector.
- Root/Birth identity handoff landed at `9a28db8d2d`; it retains issued Birth
  targets through final validation and view binding. This does not close the
  root result/parameter ABI. Source-backed/compatibility and app/script roles
  remain selected upstream and cannot be recovered by root name.
- `resolved_value_profile` cannot repair this missing handoff. Initialized-field
  and ObjectFieldGet facts are not root-result or Birth-formal authority.
- Birth definitions in `PublishedStaticMethodCFrameV2::from_view` are cataloged,
  but formal kind/wire tags are positional defaults. They are not evidence of
  source-backed parameter representation. The frame has no explicit root
  result/ABI relation; a hardcoded i64 root or Unit-by-absence is forbidden.
- The body emitter uses `PublishedLifecycleV2`, but unlike the ordinary
  published egress it skips leading-PHI/readonly contract validation. Generic
  root emission moves PHIs. Therefore original instruction coordinates and
  emitted array positions are not yet proven identical.
- The C companion's former JSON validators are retired: its nonempty body path
  only reaches `body-consumer-pending` after frame/site validation. This fixes
  their cross-function value leakage and does not validate a physical program.
- Opaque/transferred content: generic metadata/plans and unsupported operations
  must not enter a selected body as silent unconsumed siblings. Physical
  params, literals, Copy/Add, PHI, jump/branch, lifecycle normal/fault blocks
  and tails require exact final-function membership, not fixture-derived counts.

| Family | Required consumer relation / current execution disposition |
| --- | --- |
| Root / Birth / other function | Explicit source role, result and parameter ABI; root handoff open, foreign or unsupported role stops |
| Call / NewBox | Exact target/operands; NewBox checked status and Normal-only result, Birth Unit without out slot; execution stopped |
| FieldSet / FieldGet | Exact object/layout/ordinal/value representation; no name/type recovery; execution stopped |
| HomeRelease / Reclaim | Published release vs unpublished cleanup and primary Fault preservation remain distinct; execution stopped |
| NormalResult / FaultFrameEnter | Unique Invoke-result relation and root-owned vs borrowed frame; execution stopped |
| Return / ReturnFault | Source result vs Fault completion, no value on Unit/Fault and no inferred root i64; execution stopped |
| Ordinary physical body | Per-function params/values/blocks, literal/type payload, PHI incoming edges and exact terminators; handoff open |
| Layout / fields / profile | Canonical layout plus one final-borrow SafeMutex or SingleThreadExact profile; missing/drifted/unsupported stops |

### Accepted handoff constraints and evidence limits

Keep V1 ABI layout unchanged. The synchronous V2 frame and companion site
array borrow the same final view; no environment reread selects storage in C.
V2 direct preartifact ingress remains a separate declared ABI. Do not add V3 or
new semantic receipts to work around a missing source relation. Retire redundant
BodySite relations only after the existing V2 controls' exact correspondence is
proved; matching names alone cannot establish it.

The future C owner may place already-issued values/blocks and lower explicit
operations; it cannot resolve calls/types/fields or infer source exit meaning.
Parse/project once, scope ValueIds by function, preserve instruction coordinates,
seed formal values, connect PHIs in two phases and consume each selected row
exactly once. Any unsupported or residual sibling rejects before publication.
Reuse inert LLVM construction/artifact utilities only after their dependencies
are isolated; generic/same-module whole-body lowerers are not valid successors.
`nyash.object.checked_new_v1` consumes frame/profile/site/type/layout/out;
`MIR I64=4 -> checked wire I64=1` is explicit. Diagnostic site encoding must
have one physical issuer before execution; no unexplained constant/default.

Evidence correction: the pipeline test's callback constructs a frame and
returns `consumer-pending` itself; it does not execute body egress/host/C. The
C preartifact test uses incomplete synthetic JSON and proves only its exercised
structural checks and no-artifact stop. Neither proves complete program
validation, actual source-to-companion acceptance or checked Birth execution.
The existing `typed_object_birth_min_exe.sh` uses standalone JSON/ny-llvmc and
legacy route metadata, so its result cannot certify the typed published caller.
No new runtime tests were run for this read-only audit/documentation change.

The bounded next task, gated sequence, acceptance and exclusive deletion sets
are owned by the [workstream](../workstreams/mirbuilder-inplace-replacement-current.md#acceptance-incident-and-bounded-repair-order-2026-09-05).

## Reuse is explicit

Object reuse must use ordinary, named lifecycle methods.

```hako
page.reactivate()
page.resetForReuse(Bytes(64), 4)
page.configure(policy)
page.clear()
page.attach(owner)
```

These methods are normal public methods. They should express lifecycle rules
with contracts and transitions when available:

```text
requires:
  pre-state and input validity

ensures:
  post-state and observer facts

transition:
  allowed state movement
```

Do not reuse `birth` for reset/reactivation. This keeps construction,
reconfiguration, and cleanup separate.

### Current allocator reuse inventory

Current `hako_alloc` reuse surface is explicit ordinary method surface:

| Method surface | Owner file | Role |
| --- | --- | --- |
| `HakoAllocPageModel.reactivate()` | `lang/src/hako_alloc/memory/page_box.hako` | Move an empty, committed page back to active reusable state. |
| `HakoAllocPageModel.reuse()` | `lang/src/hako_alloc/memory/page_box.hako` | Guarded wrapper over `canReuse()` and `reactivate()`. |
| `HakoAllocObjectLifecycle*Result.reset()` | `lang/src/hako_alloc/memory/object_lifecycle_facade_result_box.hako` | Clear result capsule observer state before a new facade operation. |
| `HakoAllocObjectLifecycleFacadePageSourceAttach.attachFreshPage(...)` | `lang/src/hako_alloc/memory/object_lifecycle_facade_page_source_box.hako` | Attach a newly sourced page to the object-lifecycle facade. |

These methods are normal public methods. Future `configure`, `clear`, or
`attach*` methods are allowed only as explicit lifecycle methods with their own
contracts / transitions or row guard. They must not be implemented as direct
receiver `birth(...)` calls.

Compatibility exception:

```text
lang/src/hako_alloc/memory/arc_box.hako: arc.birth(ptr)
```

This remains a legacy non-constructor host facade exception. It is not
permission to add source-level receiver `birth(...)` lifecycle reuse.

## Factories

Named construction variants belong in factory methods or factory boxes, not in
extra constructor keywords.

Example shape:

```hako
box HakoAllocPageFactory {
    makeSmall(page_id: PageId): HakoAllocPageModel {
        return new HakoAllocPageModel(page_id, Bytes(32), 2, 2)
    }
}
```

Factories may choose constructor arguments and policies. They do not weaken the
`birth` direct-call ban.

## Named arguments are later

This is readable but not part of the current MVP:

```hako
local page = new HakoAllocPageModel(
    page_id: PageId(0),
    block_size: Bytes(32),
    capacity: 2,
    reserved: 2
)
```

Named constructor arguments require a separate row because they affect parser
surface, diagnostics, argument binding, and metadata transport.

Current MVP:

```text
new Box(positional_args...)
new Box { field: expr }
new Box(positional_args...) { field: expr }
```

Later row:

```text
new Box(named_args...)
```

## `fini` boundary

`fini {}` is the optional non-callable terminal Home hook selected by C′. Like
`birth`, it is compiler-owned rather than an ordinary receiver call; unlike
`birth`, it runs only for a fully constructed object whose last Home is being
released. It is not a direct physical-free API.

Relationship:

```text
new -> field initializers -> birth -> usable methods
-> terminal Home release -> fini hook -> reverse field release -> structural drop
```

Direct `obj.fini()` and `fini()` method declarations are rejected targets.
Failed unpublished outer construction never runs the outer hook; already
complete child Homes are released and may finalize only when terminal, as
defined by `docs/reference/language/lifecycle.md`.

## Stage ownership

```text
Stage0:
  parse birth declarations
  parse new expressions
  reject or diagnose direct source birth calls
  transport constructor metadata
  no lifecycle checker

Stage1:
  constructor resolution
  field initializer ordering facts
  verifier-visible lifecycle facts
  explicit reuse method contracts
  direct-birth negative diagnostics

LLVM/EXE:
  primary acceptance for object-heavy allocator routes

VM:
  semantic reference / scalar smoke only
```

## Task rows

The active task placement is the phase-293x mimalloc taskboard.

Immediate rows:

```text
LIFECYCLE-BIRTH-001:
  document and enforce new-only birth policy

PARSER-BIRTH-001:
  add negative source fixture for page.birth(...)

PARSER-BIRTH-002:
  improve parser diagnostic with new Box(...) hint

NEW-NAMED-ARGS-001:
  parked; design named constructor args later

REUSE-LIFECYCLE-001:
  keep allocator reuse as explicit methods with contracts/transitions
```

Stop line:

```text
Do not accept source-level receiver.birth(...) as a quick fix for constructor
or lifecycle routing failures.
```
