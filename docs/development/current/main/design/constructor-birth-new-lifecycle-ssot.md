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
- **Current implementation status:** scalar construction source plans retain
  exact constructor/store identity through New completion; Fault execution is not open.
- **Next ordered task:** finish layout allocation/refresh verification, then consume
  exact stores at `raw_expression_dispatch/statement_surface.rs` before child descent.
- **Production stop line:** unresolved cleanup dependencies keep the published
  backend rejection; a source plan alone cannot enable Birth execution.
- **Retirement finish line:** selected New/Birth execution and cleanup use one
  plan, with the selected old projection edges removed and fixed EXE30 proven.

This document owns construction ordering and the direct-`birth` ban. The Home
document owns Home tokens and destinations. The bounded failed-construction
decision below supplies `OWN-HOME-BIRTH-D0` without changing successful order;
source/exit products and runtime adoption remain unimplemented.

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
   `try_emit_published_static_method_exe` -> `published_mir_object` -> C transport;
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
Checkpoint evidence: package 77/77, including the fixed Pair publication and
pre-artifact rejection. Direct nested `me` is rejected by the earlier resolver;
alias capture reaches the new verifier. New receipt/guard/fixture = 0; the
unchecked Birth-row admission is closed, not a legacy-file deletion claim.
Ordinary-new admission uses exact local initializer/binding relations instead
of Allocation-event discovery (package 78/78; selected old discovery edge 1->0,
no Home-commit or cleanup claim).

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
Source IDs landed at f42be8be61; construction stores retain exact field references.
Transfer now uses the existing package port,
validates context before take and requires consumption at completion/drain;
empty transfer differs from absence. Hardened package 92/92, collector 10/10,
quick lib check and pointer/M7-S guards pass. Changed lowering owners: 40/1;
`physical_entry_lane_adoption_tests::emits_one_direct_length_call_and_i64_receipt_in_unpublished_session`
fails AlreadyIssued at line 115. Parent f42be8be61, identical Cargo.lock and
`CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 cargo test --profile quick --lib`
with filter `mir::builder::resolved_lowering::physical_entry_lane_adoption_tests::`
and `-- --test-threads=1`, also gives 12/1 at the same assertion: baseline debt.
Layout verified: package 95/95, layout 20/20, metadata 2/2, collector 10/10, refresh 6/6, receiver 4/4, JSON 121/0/1. Global routes 139/6; the six baseline-listed failures reproduce at identical assertions in f42be8be61's built binary with the same filter.
Postprocess 3/3 covers both late-refresh failures; layout tests cover backend-preflight rejection. No Birth activation; next is exact store/control consumption.

Within existing task 1, execute these three connected steps after split validation:
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
2. At the existing pre-descent assignment handoff, consume the exact plan store
   once and emit FieldSet Invoke with field reference plus real base/value
   operands. Delete the selected Birth origin-map/field-name reconstruction in
   the same series; retain unsupported execution until consumers are ready.
3. Connect the existing tasks 2–3 runtime and typed-C consumers, replacing the
   selected status-to-trap path. Table/view tests alone never close task 1 or
   authorize Birth execution; fixed Pair EXE30/OBJ and task-4 failures still gate it.

Use existing package/collector/view tests for distinct same-shaped Boxes,
renames/reordered stores, empty NoBirth, foreign source, duplicate installation,
missing ID, invalid ordinal and layout drift. Fail atomically before publishing
either functions or definitions; reject residual source/emission bindings.
Also cover mixed canonical/compatibility allocations, repeated refresh and changed
compatibility shapes, empty NoBirth, unsupported reserved positions and overflow.
The installed definition payload has one owner; the semantic batch retains only
immutable correspondence needed for exact claims, not another mutable table.
No new task card, semantic receipt, registry or per-cohort guard is required.

For prior caller
Homes, join prefix BindingRef to its retained New row and exact declaration;
construction eligibility is not complete-object destruction eligibility.
The first no-hook/plain-scalar destruction disposition must be positively
issued through the existing declaration loan, not inferred from Arc drop or
absence of events. Hook/child/native expansion remains required by tasks 2–4.
`OrdinaryNewClaimLedgerV1::is_empty` currently proves initializer/local completion,
not consumption of the retained construction plan. The two existing exit hooks
must reject residual physical bindings before finalization, independently of
expression completion. Acceptance must exercise exact store commit cutpoints,
continued cleanup after either outcome, and missing/duplicate/residual bindings;
keep backend rejection until the runtime and typed-C consumers are connected.
The selected old exact-setter status-to-trap edges must be removed with the
typed-C cutover, not revived as fallback after Fault propagation.

The retention-first premise was disproved by the existing DCE and simplify-CFG
owners: `passes/dce/elimination.rs` obtains liveness from instruction operands,
and `passes/simplify_cfg/flow.rs::rewrite_value_uses_in_function` rewrites blocks,
not arbitrary FunctionMetadata. A metadata ValueId list could become stale or
reference deleted locals. A source-only list would instead lose the exact
physical correspondence when the scoped ledger drops. Neither is the intended
exit connection. Read-only worker review independently confirmed this ordering.
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
- Consume the source/local ledger at the two named pre-finalization hooks into
  real control/cleanup operands. Update ordinary use/rewrite, CFG and verifier
  handling together; do not add a metadata table or KeepAlive repair.

Canonical control vocabulary (one owner, not an extensible effect wrapper):

```text
InvokeOperation = Call(existing MirCall with dst=None)
                | NewBox(existing box_type and allocation arguments)
Invoke(operation, fault_frame operand, normal_landing, fault_landing)
InvokeNormalResult(originating Invoke block, dst)
ReturnFault(fault_frame operand)
FaultFrameEnter(dst, RootOwned | Borrowed)
```

Invoke defines no SSA value. NewBox's handle is defined by the projection in
its dedicated Normal landing, not on the allocation terminator; Birth Unit
has no projection. A value-returning Call uses its canonical result ABI and
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

The existing task-4 acceptance must exercise primary preservation under injected
allocation failure, cleanup success with an already-pending primary, later-Fault
ordering, full-buffer omission, propagation without duplicate recording, message
lifetime after source-frame release and no leaked omitted payload. Capacity
exhaustion must still reach the last required cleanup and final entry report.
The same connected series must reach the selected typed-C Birth consumer and
fixed production proof; ABI-only unit tests do not close it. Runtime release,
construction reclaim and tasks 2–4 remain required, not silently scoped away.

The source prefix walks every preceding statement. Plain aliases add no Home;
rebind, unknown acquisition, entry demand gaps, Handle arguments and nonempty
overrides are unavailable, never empty cleanup. Read-only review identified
the override/capture omissions in the draft; both were corrected before the
package gate. Three New declarations verify reverse order and prior-install
rejection; the real two-New compiler path preserves package completion and
backend rejection. Construction obligation issuance precedes retained physical
exit projection in this series; this is not a completed task-1/runtime claim.
Focused package gate: 83 passed / 0 failed, control suite 33/33. Exact outward
target, foreign-owner and non-New rejection join existing prefix negatives;
pointer/M7-S guards green, changed source maximum 688 lines.
No whole-lib/no-new-red claim or baseline edits.

**Change:** replace the selected New path's physical-control drop with exact
emission binding through its existing claim port and consumption at the two
pre-finalization hooks. Reuse MirCall/Callee; introduce no target reissuer.
**Contract:** source arguments lower once; cleanup references are real operands.
Invoke separates normal/fault control; Unit Birth has no source result. Allocation
still needs its normal-only handle, and selected non-Unit calls need normal-only
SSA result definition/projection. Unit-first is not permission to omit either.
**Done:** existing package/local tests reject missing/foreign/duplicate emission
binding and residual consumption; entry tests reject missing/foreign/wrong-type
frames and preserve source parameter counts; ordinary verifier/CFG/rewriter/optimization
tests preserve cleanup operands and reject Fault-edge normal-result use. Keep
the package evidence and fixed Pair publication, plus the existing M7-S guard.
This is the task-1 control checkpoint only; tasks 2–4 remain the series terminal.
**Stop:** missing allocation/result mapping or release capability keeps the
selected executable admission closed. Fix the same owner; no empty cleanup,
name/order recovery, detached source proof or wider grammar.

Control-schema checkpoint (2026-09-06): Invoke/normal-only result/ReturnFault
and intrinsic FaultFrameEnter verification are implemented; no scalar frame or
ordinary-value escape is admissible. Invoke tests now live with the existing
verifier owner: 4/4; instruction tests 18/18, vocabulary doc-sync 1/1;
quick lib and vm-reference checks pass.
The incremental test compile was cancelled at the memory safety boundary;
the fresh `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1` run passed. No tests were
ignored/deleted or baseline changed. Backend admission stays UnsupportedBeforeObject.
Entry emission, exact New/Birth binding and pre-finalization consumption remain
next; task 1/runtime are not complete and no production old-edge deletion is claimed.

At tasks 2–3, extend the existing published transport and
`hako_llvmc_ffi_mir_call_dispatch.inc`, not its legacy method-birth branch.
`PublishedCallKindV1` currently lacks Birth/control rows and the published view
rejects Birth. Stable indexed reclaim is checkpointed; HomeRelease, source/CFG
binding and common Fault transport remain open. Verify connected single release,
failed allocation and unrelated live handles before activation. Storage capability must
match the selected runtime profile; retain default-profile acceptance.

Current natural-source grammar permits one initialized local per declaration.
Multiple initializer AST carriers are not a new prerequisite; if later
admitted, protect pending HomeValues before any local installation.
Normal Completion/E0 remain insufficient; do not reuse TextScan's successorless
Fault terminal as caller propagation. No input-ABI reconsultation is needed.
Missing Home field/native products are explicit dependencies, not assumed empty. The
full Home program and general unsafe raw ownership stay parked; selected
construction obligations cannot be waived or reduced to a Pair-only success.
No new guard/fixture/card is planned. Split source owners before 800 lines;
do not create a general framework merely to save a few cleanup edges.
The remapper is now at the 760-line split trigger: before further growth,
separate its value-discovery responsibility from instruction remapping within
the same module; retain its public owner and existing tests, no new facade.

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
