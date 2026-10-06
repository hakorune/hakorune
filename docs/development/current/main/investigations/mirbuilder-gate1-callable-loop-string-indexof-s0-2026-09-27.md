# MirBuilder Gate 1 — current design and acceptance

Status: NULLABLE-RESULT-ABI-S0 landed — `local x = me.m(..)` against a `NullableObject(C)` callee lowers to `NullableHandle` + `HomeReleaseIfLive` and publishes `ordinary_nullable_handle`/`nullable_handle`/`const_null`/`home_release_if_live` through the C shim; card compressed to tombstone history
Date: 2026-09-29
Scope: MIRBUILDER-GATE1-INSTANCE-ENTRY-HOME-S0; compact Gate-1 frontier.
Related: CURRENT_STATE.toml; workstream row H; RULES.md;
  ownership-home-model-ssot.md; own-home-callable-abi-d0-design-task-2026-08-09.md.

## Current decision

The 2026-09-29 review at `339674c77b` invalidates the blanket
`AllSixRedsAwaitNamedParkedFamilies` conclusion from `053b659637`.
Gate 1 is still unsatisfied. The user selected organization and design;
this turn closes the entry contract and selects the next bounded S0 without
executing it. One read-only worker audited the exact authority and consumers;
the primary integrated the decision below. The original
`MIRBUILDER-GATE1-MULTI-RETURN-RESULT-NEW-D0` is decomposed into entry admission,
per-exit flow/consumption, and later argument/result contracts.

```text
Decision: adapt the sole Home ABI owner to lend current-batch entry demands;
  full call-site result ABI and per-exit Home flow remain separate contracts.
Source authority + canonical issuer: exact resolver callable batch/common
  parameter catalog -> CallableHomeAbiIssuerV1 -> scoped source entry-flow loan.
Non-authority: receiver spelling, MIR types, absent result annotation, an empty
  cleanup vector, and a retained Construction terminal.
Fail-fast boundary: foreign/incomplete entry rejects; captures stay unavailable;
  unresolved result never issues Unit or a complete call-site Home ABI.
Smallest next slice: MIRBUILDER-GATE1-INSTANCE-ENTRY-HOME-S0, replacing only
  receiver-presence => EntryDemandMissing for the exact admitted entry cohort.
Non-claims: no multi-return flow, constructor argument/result widening,
  field destruction, selected-C/backend activation, app PASS or Gate-1 completion.
```

The canonical ABI D0's complete-call-ABI rule is preserved with a narrow
[entry-flow loan amendment](own-home-callable-abi-d0-design-task-2026-08-09.md#source-entry-flow-projection-2026-09-29).
Updating that owner is required because it specified only a complete ABI
aggregate. The new entry-only loan stays inside the owner aggregate, keeps the
result unresolved and cannot reach a call-site resolver. No new receiver ownership semantics are invented.

## Corrected frontier ownership

This census covers the **recorded 11-entry EXE suite's first terminals ->
known source/transport owners and downstream dependencies**. It includes the
six recorded red members; it excludes a new suite run, new dynamic terminal
ordering, production caller-zero, and all-repository candidate exhaustion.
No entry below is a newly measured runtime result.

| Member | Recorded first terminal | Next responsibility / boundary |
| --- | --- | --- |
| mimalloc_lite | `artifact-unowned-lifecycle-site` | Selected D0: receiver-entry demand and per-return Home flow for retained return-new claims. Argument representation, mixed return ABI and destruction remain distinct later dependencies. |
| boxtorrent_mini | `unsupported terminator Invoke` | Its smoke still uses generic JSON ingress. Physical-route acceptance migration is a separate harness decision; field-write `new`/DropPlan is downstream, not its observed first terminal. |
| binary_trees | `root-call-entry-unavailable` | Unreleasable root Home; owning-field destruction and untyped storage stay outside this D0. |
| allocator_stress | `NamedArray(TextSourceMissing)` | Named-array/call-result argument provenance. A separate owner is not by itself evidence of an external wait. |
| json_stream_aggregator | `route-not-front-selected` | Root `me` loop source coverage and existing locator consumer need a bounded connection decision. The selected-C arbitrary-UserBox park does not close canonical MIR design. |
| typed_object_untyped_field_min | `return_type_strategy` panic | Known compiler baseline defect reproduced at `31c4bc072e` in the recorded receipt; not an intentional rejection contract. |

### Re-observation 2026-09-30 (post-P3) — mimalloc-lite first terminal moved upstream

Re-run of `mimalloc_lite_exe.sh`'s `--emit-exe` physical route under the
current compiler: the first terminal is now
`[freeze:contract][ordinary-field-read/unconsumed-read]` (deterministic
across reruns; one run surfaced the sibling
`IncompleteOrdinaryNewCoverage` — same coverage family,
ordering-dependent), not the recorded `artifact-unowned-lifecycle-site`.

Root cause, no new census needed: `HakoAllocPage.allocate`'s retained
`return new HakoAllocHandle(me.page_id, block_id, requested_size)`
(`Body(13).Value.Argument(0)`). `7592ffc671` staged the `me.page_id`
argument as proven `I64Field` evidence, but the claim resolves
`RetainedUnavailable` — `allocate`'s body (`me.f = expr` writes at
`Body(2)`, bare `me.<field>.m(..)` statements, `me.<field>.get(..)`
initializers, branch `return null`) stays `PrefixNotCovered`, so the
selected emit never runs and the staged read has no consumer. The freeze
is the designed fail-closed boundary: a proven argument read must not
drain through the raw lane's unowned `FieldGet`. This is the same gap
the recorded `artifact-unowned-lifecycle-site` named, surfaced earlier
by argument-read staging — the named out-of-scope contracts from
`7592ffc671` (receiver field write, `me.`-receiver call, mixed/nullable
local use) are the coverage owners.

The selected-C park remains exactly
`MIR-CALL-ME-DECLARED-INSTANCE-SELECTED-C-ADMISSION-D0`, whose manifest keeps
canonical MIR routes. See
[manifest](mir-call-core-r6-d1b-method-none-manifest-2026-08-25.toml).
Existing canonical locator evidence is in
[S3](mirbuilder-exe-acceptance-declared-instance-loop-locator-s3-2026-09-26.md).
Neither that receipt nor the present static review proves current json-stream
EXE success. Do not change this app or select that sibling during this D0.

Receiver demand, CFG Home flow, destination semantics and terminal DropPlan
have distinct owners in
[Home task order](hakorune-home-ownership-task-2026-08-04.md):
`OWN-HOME-CALLABLE-ABI-D0`, `OWN-HOME-FLOW-CFG0-S0`,
`OWN-FIELD-CONTAINER-DEST-D0`, and terminal finalization.
The latter two are not reopened by diagnosing the former two. A downstream
field dependency does not remove an upstream internal design task.

## Selected source boundary

Target: the already-inventoried nine retained result-new sites in the
mimalloc-lite imported allocator bodies, including
`HakoAllocPage.allocate/1`'s `return new HakoAllocHandle(...)` and
`HakoAllocHandleResult` constructors. Reuse the existing inventory; do not
start another whole-suite or repository census to rediscover this target.
The finite source inventory in `lang/src/hako_alloc/memory/page_heap_box.hako`
is 1 + 3 + 5 = 9 Return-new sites (not nine HandleResult constructors):

| Callable | Source lines | Result-new sites |
| --- | --- | --- |
| HakoAllocPage.allocate/1 | 102 | one HakoAllocHandle; other returns are null |
| HakoAllocHeap.allocateResult/1 | 216, 221, 224 | three HakoAllocHandleResult |
| HakoAllocHeap.reallocResult/2 | 302, 306, 310, 315, 318 | five HakoAllocHandleResult |

The current implementation task admits declaration entry; it does not promise
available flow for all nine sites or erase unsupported field/call operations.

- Membership: `ordinary_new_coseal_issue.rs::issue_ordinary_source_cohort_v1` uses
  exact construction and parent `Return { value }` source membership.
  A trailing `Value` segment alone never establishes Return position.
- Source flow: `home_new_prefix.rs::scan_new_home_flow` and
  `home_new_prefix_scan.rs::scan_statement_flow` share the current issuer.
  A receiver/capture installs `EntryDemandMissing`; unsupported statements
  retain `PrefixNotCovered` (they do not override the earlier issue).
- Nested branch Returns are walked by `home_new_prefix_branch.rs` under a
  resolver-sealed `ResolvedIfRegionBundleV1`: a `return`-terminated branch
  leaves the join; fall-through branches join by ordered Home intersection;
  divergent Homes become `HomeFlowBranchDivergent`. All nine retained sites
  mint result claims with per-site `home_prefix` evidence.
- `emission_prepare.rs` turns a failed prefix into `prior_homes=None` and
  unavailable emission. Observing additional Return sites cannot by itself
  fix entry demands, branch state, cleanup or artifact admission.
- The common declaration parameter catalog owns parameter representation,
  not receiver or result meaning. Receiver classification must come from
  the canonical Home ABI owner, never an `if receiver { Handle }` patch in
  the scanner. Missing combined result authority must not become Unit.

Selected chain to finish designing:

```text
same parser/declaration/batch + common parameter contracts
  -> sole Home ABI owner: exact source entry demands
  -> scan_new_home_flow: path-specific prior Homes and Completion/Fault exits
  -> ordinary_new_coseal_issue: per-site result prefix + argument contract
  -> existing result claim ledger -> selected emission/return validation
```

Do not pair independent receipts later by names or owner keys. The entry and
flow evidence must belong to the same source invocation before co-seal.
A retained `Value(Construction)` describes a result; it is not evidence that
all paths have valid cleanup or that a mixed null/new callable returns Handle.

## Landed history — this card's lifecycle (tombstone)

Every design detail, rejected alternative and acceptance record for the
rows below lives in Git — `git show <commit>` or `git log -- <this
file>`; the pre-compaction card is at `339674c77b`.

| Commits | Landed row |
| --- | --- |
| `3b794d2ccd` | Gate-1 compaction; INSTANCE-ENTRY-HOME selected. |
| `77f6fceb72` | INSTANCE-ENTRY-HOME-S0 — exact instance entry Home loans. |
| `4d659f78a0`, `ebea445abe`, `3a9b98b75b`, `b567c5ad64` | PER-EXIT-HOME-FLOW D0+S0 + remediation — admitted subtree grammar, branch/join/terminated states, per-exit obligations, fault scopes, `home_new_prefix_scan` split. |
| `c2ee077fc9`, `7592ffc671` | CONSTRUCTOR-ARGUMENT-EVIDENCE D0+S0 — null + i64 field-read argument arms; one physical materialization owner. |
| `7488a334bb`, `ed43bb4cc0` | MIXED-NULL-NEW-RESULT-ABI D0+S0 — `NullableObject` claim arm; nullable never mints Handle. |
| `fdc4fcc2da`, `1830def7c4` | FORWARDED-RESULT-CLASS-COMPOSITION D0+S0 — two-pass draft + bounded fixpoint over F1..F5; `HakoAllocHeap.allocate` claims `NullableObject` on the real fixture. |
| `a932bec602`, `cf07ea1450`, `61faf07d0a` | RECEIVER-CALL-OBSERVATION D0 + deferred S0 + rework queue. |
| `436bbe44ad` | RESULT-CLASS-EXIT-EVIDENCE-S0 — verified-Completion exits via `verified_value_return_sites`; one exit authority for claim issuer + `construction_result_callee`/`map_result_callee`. |
| `d9bcf6108b`, `b870ec09c9` | CLAIM-FIRST-OBSERVATION-S0 + closeout — claims seal pre-walk; `me.m` mints site-keyed rows in-walk under the entry-loan proof with exact destination evidence; deferred pass deleted. |

## Rework queue — reviewer + worker design audit 2026-09-30

Both reviews converge — one verified-Completion exit authority.

P1 — claim correctness:
1. `RESULT-CLASS-EXIT-EVIDENCE-S0` — landed (`436bbe44ad`): exits from
   the verified Completion via `verified_value_return_sites`; claim
   issuer + `construction_result_callee`/`map_result_callee` unified.
2. `CLAIM-FIRST-OBSERVATION-S0` — landed (`d9bcf6108b`): claims seal
   pre-walk; `me.m` mints site-keyed rows in-walk under the entry-loan
   proof; deferred pass deleted; `LocalCallObservationV1` merge waits
   for the nullable ABI design.
3. `RECEIVER-CALL-OBSERVATION-S0` closeout — landed: exact destination
   local on every row (asserted per site); birth `me.m` rejected
   upstream (`ReceiverNonEscape` pin); non-`me` receivers, unclaimed
   callees, rebinding stay unobserved.

P2 — thinning: post-loops merged — `field_write`/`result_class`
drafts pre-scan since claim-first (`d9bcf6108b`); `birth_site_index`
collects in the same verified sweep (ctor rows join via the
program-source loan — births are not batch declarations); card
tombstoned (`b21e05ebe6`). NULLABLE-RESULT-ABI-S0 landed end-to-end
through the published wire. Reads of nullable locals (`h.f`, null
checks) stay out of scope on the `PrefixNotCovered` floor.

P3 — containers/types — landed (`9b95d3adc4`): `CompletionSeed` deleted;
the result-contract builder issues one row end to end
(`completion_seed.rs` folded into `result_contract.rs`), `map_read_facts`
uses one source loan, and the three sibling `Ready`/`Taken` enums are
aliases of one `DispositionSlotV1<T>`.

Result-claim census on the real fixture — pinned by
`page_heap_fixture_result_claim_census`, which fixes all nine claims'
sealed evidence states:

| Sites | prefix | construction | argument rows |
| --- | --- | --- | --- |
| `HakoAllocPage.allocate` Body(13) — 1 `HakoAllocHandle` | `PrefixNotCovered` at Body(2) `me.free_top = me.free_top - 1` (receiver field-write statement) | Ok — Birth stores are `me.f = param` | Ok(3); stages the `me.page_id` `I64Field` read |
| `allocateResult` ×3 + `reallocResult` nested ×3 — 6 `HakoAllocHandleResult` | **Ok** — the landed nullable lane already covers `local handle = me.allocate(size)`; branch conditions are outside the prefix walk | `BodyCoverageUnsupported` — normalized Birth stores `me.ok = 0`/`me.reason = 0` defaults, then user `me.ok = ok`/`me.reason = reason` re-store the same fields (replacement needs old-value release semantics); `me.handle = handle` object-typed param store waits behind it | Ok(3) |
| `reallocResult` tail ×2 | `PrefixNotCovered` at Body(3) `local replacement = me.realloc(..)` — `me.realloc` stays unclaimed (param-forwarding result class) | `BodyCoverageUnsupported` — same class | Ok(3) |

So the open "prefix ownership" question decomposes: prefix stays owned
by the existing per-function Home-flow scan, and the remaining gaps are
distinct bounded grammar admissions — receiver field-write statements,
`me.<field>.m(..)` field-receiver calls, and the `me.realloc`
param-forwarding result class — while eight of nine sites additionally
wait on the `HakoAllocHandleResult` Birth plan's duplicate
first-store/replacement bound and its object-typed param store
(owning-field family). Separately, the emission-eligibility
judgment (`compute_emission_prepare`, including dynamic prior-Home
availability) exists only at take time, so a staged argument read under
a claim judged `RetainedUnavailable` must be released there — it cannot
be withheld at issue.

`MIRBUILDER-GATE1-RETAINED-NEW-HOME-FLOW-D0` is accepted — see the
Decision section below. `RETAINED-NEW-HOME-FLOW-S0` landed: the
prepare-time `RetainedUnavailable` decline releases the claim's staged
argument reads (`Progress::Released`), keeping `unconsumed-read` armed
for emission-eligible claims.

Re-observation after S0 (3/3 deterministic): the fixture smoke's first
terminal moved upstream to
`[freeze:contract][ordinary-new/emission/nullable-argument-carrier]` —
the landed nullable lane's i64-carrier gate rejects `me.allocate(size)`/
`me.allocate(requested_size)` because the untyped `size`/`requested_size`
parameter carries no proven `MirType::Integer` carrier. This is the
designed fail-closed boundary (the wire carries proven i64 only, never a
guess), now surfaced ahead of the recorded `artifact-unowned-lifecycle-
site`: the S0b argument gate admits `Local` args at observation but the
parameter binding's carrier type is unproven at emit. The next design
question is whether untyped-parameter arguments gain a bounded carrier
evidence row (or the observation declines earlier). This is a distinct
family from the four coverage bounds in the census above.
`MIRBUILDER-GATE1-ARG-CARRIER-EVIDENCE-S0` landed: the D0 below
resolved the question in favour of the already-sealed wire contract —
`emit_receiver_nullable`'s `Local` arm now admits recorded `Integer`,
`Unknown`, and unrecorded carriers and still freezes concrete non-i64
carriers under the same token.

Re-observation after S0 (3/3 deterministic): the fixture smoke's first
terminal moved upstream to
`[freeze:contract][ordinary-new/local-commit/handle-release-shape-drift]`
— `HakoAllocHeap.allocateResult/1` emits **two** correctly-shaped
`HomeReleaseIfLive` invokes for the received nullable result
`ValueId(15)` (the `local handle = me.allocate(size)` binding), one per
reachable exit (`if handle == null { return new ...(0,4,null) }` and
the tail `return new ...(1,0,handle)`), while received-local validation
still demands exactly one release per function. Both releases pass the
shape check (`releases=2 matching=2`), so the drift is cardinality, not
vocabulary. The next design question decomposes into two sub-questions:
(a) whether per-exit `HomeReleaseIfLive` is the intended discharge —
one checked release on whichever exit runs — making the validator's
global `releases.len() == 1` bound a single-exit leftover, and (b)
whether the tail-exit `new HakoAllocHandleResult(1, 0, handle)` store
*moves* `handle`'s ownership into the result object (releasing at that
exit would free a handle the returned result still carries) — which
intersects the parked owning-field family. This is a distinct family
from the carrier gate. Next design row:
`MIRBUILDER-GATE1-RECEIVED-NULLABLE-EXIT-OWNERSHIP-D0` — pin the
ownership contract for a received-nullable local that reaches a
constructor argument on one exit path, then reconcile validation with
that contract before any emission or ownership change. The D0 below is
accepted; the next execution row is
`MIRBUILDER-GATE1-RECEIVED-NULLABLE-EXIT-OWNERSHIP-S0`.

## Decision — MIRBUILDER-GATE1-RECEIVED-NULLABLE-EXIT-OWNERSHIP-D0 (accepted)

Decision:
  A `new` positional argument that names a live owned binding — one
  present in the frame's `homes` (`Home`, `ReceivedHandle`,
  `ReceivedNullable`, `Map`) — moves the lease into the constructed
  object at the argument boundary, the same discharge `return <local>`
  already seals at `home_new_prefix_terminal.rs` ("a returned
  local/Home leaves with the caller"). The "prior Homes remain this
  frame's exit obligation" sentence there was sealed for scalar-only
  arguments: storing `handle` into `new HakoAllocHandleResult(1, 0,
  handle)` physically places the handle index in `result.handle`, and
  callers dereference it after return (`release(handle)` reads
  `handle.block_id`); `reclaim_checked_indexed` truly frees the slot,
  so a tail-exit release is a real use-after-reclaim, not a validator
  nit. The move completes on the invoke's Normal edge: `prior_homes`
  still names the arg binding so the Fault unwind discharges it when
  no store happened.
Source authority + canonical issuer:
  `scan_statement_flow` (`home_new_prefix_scan` /
  `home_new_prefix_terminal`) owns the per-exit `homes` obligation;
  `PrefixLocalFlow::consume_home` is the sole move operator, applied
  to the observed arg kind's root binding (`Local`/`Handle`/`BoundValue`
  — `Handle` resolves aliases to the owed root) after the result
  prefix is captured, on both the `local x = new` and `return new`
  paths. `validate_root_home_exit` already validates one origin per
  owed home per exit.
Non-authority:
  the physical instruction count is never the authority for "owed";
  `emission_validation`'s `releases.len() == 1` was a single-exit
  leftover and aligns to the exit-row homes count — the same sealed
  fact emission used. No Facts row, no arg-kind re-classification
  (`BoundValue` stays the nullable arg's kind), no ownership inference
  from MIR.
Fail-fast boundary:
  `I64Field` args never consume the receiver object (a field read
  borrows); literal/`Null` args move nothing; a binding absent from
  `homes` (parameters, `BorrowedMap`, generic `BoundValue`) is never
  consumed; post-move uses stay unobservable through `Consumed`;
  a release count disagreeing with the owed-exit count still freezes
  `handle-release-shape-drift`/`missing`.
Smallest next slice:
  `RECEIVED-NULLABLE-EXIT-OWNERSHIP-S0` — consume observed owned
  `new`-arg bindings in both scan paths and align the received-row
  release count to the owed-exit count; pin positive (moved arg drops
  the tail-exit `HomeReleaseIfLive`, `releases == 1`) and negative
  (a still-owed sibling exit keeps its checked release) coverage.
Non-claims:
  no field-store ownership contract is minted — the owning-field
  family (`FieldContractUnsupported` on `me.handle = handle`) stays
  parked; borrowed map/parameter handles stored into objects keep
  today's escape hole as a named residual; no claim that a nullable
  local owed on two exits emits correctly beyond the aligned count.

### S0 landed — RECEIVED-NULLABLE-EXIT-OWNERSHIP-S0

- `home_new_prefix_scan.rs` (`local x = new`) and
  `home_new_prefix_terminal.rs` (`return new`) now collect observed
  owned argument bindings (`Local`/`Handle`/`BoundValue` kinds whose
  root binding is in `homes`) into `moved_arguments` and consume them
  — `homes.retain` + `locals.consume_home` — *after* the result
  prefix records `prior_homes`. Normal edge moves the lease; Fault
  unwind still discharges it.
- `emission_validation.rs` `validate_call_received_emission` derives
  `owed` from recorded evidence: root-exit origins filtered to the
  current row owner plus every recorded release operation in emitted
  bindings (ordinary/result/call-received/map rows — the latter two
  via new `RootHomeReleaseEmissionV1::origin` /
  `MapLocalProgress::emitted_bindings` accessors) matching
  `row.end_operation()`. `releases.len() != owed` freezes missing or
  `handle-release-shape-drift`; every actual release must still match
  the sealed operation shape.
- Pin: `nullable_result_moved_into_new_releases_only_on_the_owed_exit`
  — `if h == null { return new OwMoveHolder(0) }` sibling keeps its
  `HomeReleaseIfLive`; the tail `return new OwMoveHolder(h)` owes
  none. Exactly one checked release. Fixture box names are unique
  (`OwMove*`): a process-global box-name registry made an earlier
  `Probe`-named fixture collide with
  `nullable_receiver_call_serializes_*`'s `Probe` under suite order
  (`ordinary-membership-drift`), detected by parent-vs-current
  failure-set diff (126 == 126 after rename).
- Serial lib suite: 8098/126, failure set identical to parent
  (zero regression delta; the earlier parallel-run 189 was
  env/global-flag leakage between tests, not this change).
- Fixture smoke re-observation (3/3 deterministic): first terminal
  moved past `handle-release-shape-drift` to
  `[freeze:contract][ordinary-new/local-commit/physical-boundary/phi-in-recorded-block]`
  — `PhysicalBoundary::capture` rejects a `Phi` inside a recorded
  emission block. Next design row:
  `MIRBUILDER-GATE1-EMISSION-BLOCK-PHI-D0` — pin whether a phi may
  share a recorded emission block at all (join-boundary lifecycle
  snapshot ownership) before any boundary relaxation.

## Decision — MIRBUILDER-GATE1-EMISSION-BLOCK-PHI-D0 (accepted)

Observed site: `HakoAllocHeap.realloc/2` — `local replacement =
me.allocate(requested_size)` emits its `Invoke{Call, NullableHandle}`
as the terminator of BasicBlockId(144), a genuine join block for the
`handle.page_id == 0` / `== 1` fallthrough merges (one real phi plus
three degenerate identical-input phis, then `Const Void`). The phis
are join semantics issued by the SSA value-join/phi_lifecycle
machinery, not by the lifecycle emission — the emission merely
records the invoke it finds there.

Decision:
  A recorded block may carry a leading `Phi` run only when it is a
  genuine join — at least two recorded incoming edges — because the
  sole-predecessor concatenation rule can never contract into it,
  and any later phi-input rewrite or incoming-edge change still
  fails `finished-sequence`/`incoming-drift`. `phi-in-recorded-block`
  narrows to phis in sole/zero-predecessor recorded blocks and to
  non-leading phis.
Source authority + canonical issuer:
  `PhysicalBoundary::capture` owns the recorded-block contract; the
  snapshot already pins every instruction verbatim, so admitting a
  join's phi head adds no second authority — finishing that rewrote
  or dropped a phi would still be caught by the exact sequence check.
Non-authority:
  block occupancy never implies emission ownership of the join phis;
  no phi rewriting, splitting, or emission-placement change is added;
  `validate_original`'s phi rejection in `root_cleanup_graph` (the
  cleanup-payload projection) stays untouched.
Fail-fast boundary:
  a phi in a sole/zero-predecessor recorded block — the only shape
  the concatenation rule can silently corrupt — still freezes
  `phi-in-recorded-block`; a phi after a non-phi instruction is not a
  join head and still freezes.
Smallest next slice:
  `MIRBUILDER-GATE1-EMISSION-BLOCK-PHI-S0` — admit leading phis in
  multi-incoming recorded blocks inside `capture`; pin positive
  (nullable call invoke recorded as terminator of an if-join block)
  and negative (phi in a sole-predecessor recorded block still
  freezes).
Non-claims:
  no claim `realloc` compiles beyond this boundary; owning-field
  release, `BodyCoverageUnsupported` for `HakoAllocHandleResult`, and
  emission block-splitting remain parked.

`MIRBUILDER-GATE1-EMISSION-BLOCK-PHI-S0` landed:
`PhysicalBoundary::capture` now permits a leading `Phi` run only on
recorded blocks with at least two incoming edges; sole/zero-predecessor
blocks and phis after any non-phi still freeze
`physical-boundary/phi-in-recorded-block`. Pinned by
`leading_phi_in_a_genuine_join_recorded_block_captures_and_validates`,
`phi_in_a_sole_predecessor_recorded_block_still_rejects`, and
`non_leading_phi_in_a_join_recorded_block_still_rejects` (4/4 focused
tests green; serial lib suite 8101 passed / 126 failed — the failure
set is byte-identical to the parent baseline). The mimalloc-lite
physical smoke advanced deterministically (3/3) past
`physical-boundary/phi-in-recorded-block` to
`[freeze:contract][ordinary-new/local-commit/artifact-unowned-lifecycle-site]`
— back to this lane's recorded frontier, which the selected lane's
receiver-entry/flow obligations describe.

## Decision — MIRBUILDER-GATE1-UNOWNED-LIFECYCLE-SITE-D0 (accepted)

Observed site: `HakoAllocPage.allocate/1` BasicBlockId(39) — a raw-lane
`Call{BirthConstructor{HakoAllocHandle.birth/3, receiver V131}}` for
the retained `return new HakoAllocHandle(me.page_id, block_id,
requested_size)` at `Body(13)`. `expected=0`: the claim resolved
`RetainedUnavailable{ExpressionCompleted}`, so no lifecycle bindings
were recorded and the raw lane's Birth Call has no owner.

Decision:
  The freeze is the designed terminal, not a boundary gap —
  `local_entry.rs`'s `is_complete` documents that a retained
  result-position row reaching `ExpressionCompleted` leaves "the
  unowned-lifecycle rejection" to downstream coverage. The raw lane
  legitimately emits `NewBox` + the Birth `Call`; the artifact gate
  rejects because no claim owns the footprint. The real blocker is
  `allocate`'s uncovered prefix: `return null` exits are already
  covered (`TerminalReturnedSourceV1::NullLiteral` exists and the
  terminal observer admits it), so the remaining named classes are
  `me.<field> = <rhs>` field-write statements (first
  `PrefixNotCovered`, `Body(2)`) and nested `me.<field>.m(..)`
  receiver calls (`.get` initializers and bare `.set` statements).
Source authority + canonical issuer:
  `ResolvedAssignmentTargetV1::FieldWrite{receiver}` is the
  resolver-sealed target classification; `scan_statement_flow` /
  `walk_branch` remain the sole prefix-coverage issuer — a
  receiver-rooted field write is a Home-neutral statement when its
  RHS subtree is provably free of `new` sites, call observations,
  map literals, and reads of owned bindings.
Non-authority:
  no physical emission change — the raw lane already emits the plain
  `FieldSet` instruction, which `requires_lifecycle_validation` does
  not cover; no new Facts row names the write itself; owning-field
  moves (`me.f = <owned binding>`) stay the parked owning-field
  family — this slice never classifies an owned-binding RHS as
  writable.
Fail-fast boundary:
  a field write whose receiver is not the self-rooted `me`, whose RHS
  subtree contains a `new`/call/map-literal site, or whose RHS names
  a live owned Home/Handle/Map/Nullable binding keeps
  `PrefixNotCovered` — the claim stays `RetainedUnavailable` and the
  artifact gate stays armed.
Smallest next slice:
  `MIRBUILDER-GATE1-RECEIVER-FIELD-WRITE-S0` — admit self-rooted
  `me.<field> = <rhs>` statements in `scan_statement_flow` (shared by
  the branch walk) when the RHS subtree is Home-neutral; pin
  positive (`me.free_top = me.free_top - 1` covered, claim prefix
  advances past Body(2)) and negative (`me.f = handle` with an owned
  `handle`, `x.f = v` non-self receiver, and `new`/call inside the
  RHS each keep `PrefixNotCovered`).
Non-claims:
  `allocate`'s claim does not emit after this slice — nested
  `me.<field>.m(..)` calls remain `PrefixNotCovered`, so the smoke
  terminal is expected to stay `artifact-unowned-lifecycle-site`;
  observable progress is the recorded prefix issue moving past the
  field-write sites. Nested receiver calls, `release/1`'s
  `handle.<field>` reads, and `Bool` return literals are distinct
  later rows.

Landed `MIRBUILDER-GATE1-RECEIVER-FIELD-WRITE-S0`:
`scan_statement_flow` now admits a `me.<field> = <rhs>` statement when
the target seals as `ResolvedAssignmentTargetV1::FieldWrite` rooted at
the lexical `me` receiver and every RHS subtree row is Home-neutral
(`new`, calls, map/array literals, and live owned Home/Handle/Map/
Nullable reads all keep `PrefixNotCovered`). `me.<field>` reads inside
the RHS are proven by a new receiver-side `scalar_field` predicate wired
through `scan_new_home_flow` — backed by
`numeric_substrate::is_numeric_integer_type_name` so `usize` fields
admit, while argument-position `i64` evidence keeps its own authority.
Pinned by `ordinary_new_receiver_field_write_is_home_neutral`,
`ordinary_new_receiver_field_write_stays_fail_closed` (owned-handle RHS,
non-self receiver, `new`/call inside RHS, compound `op=`), and the real
`page_heap_box` census asserting `allocate`'s prefix now stops at
`me.free_stack.get(..)` Body(3). The
`retained_unavailable_claim_releases_staged_argument_reads` fixture was
re-armed with an explicit `me.tick()` since `me.value = 3` is now
covered. Serial lib suite 8102 passed / 127 failed — the sole delta
versus the 8101/126 parent set is
`nullable_receiver_call_serializes_nullable_handle_and_checked_release`,
reproduced flaky on the parent commit standalone (per-process HashMap
nondeterminism) and classified as baseline debt. The mimalloc-lite
physical smoke stays 3/3 deterministic at
`[freeze:contract][ordinary-new/local-commit/artifact-unowned-lifecycle-site]`
— exactly the recorded frontier, since nested receiver calls still hold
the claim `RetainedUnavailable`.

Next: `MIRBUILDER-GATE1-NESTED-RECEIVER-CALL-D0` — design the nested
`me.<field>.m(..)` receiver-call family that now fronts `allocate`'s
prefix at Body(3).

## Decision — MIRBUILDER-GATE1-NESTED-RECEIVER-CALL-D0 (accepted)

Decision:
  a `me.<field>.m(..)` call whose receiver is a declared `ArrayBox`
  field of the self-rooted `me` is a covered prefix statement when the
  (selector, arity) pair resolves in the generated core-method
  contract manifest `CORE_METHOD_CONTRACT_ROWS_V2` via
  `lookup_core_method_result_row_v2` — the sealed result/effect
  authority (`ArrayBox.get/1` → `Dynamic`/`PureRead`, `set/2` →
  `NoValue`/`MutatesSlot`, `push/1` → `NoValue`, `length/0` →
  `I64Value`; `MapBox.set/2` is `Dynamic`, so no blanket "set is
  void") — and every argument subtree is Home-neutral. A
  `local x = ..` initializer installs the result by manifest class:
  `I64Value`/`BoolValue` → `Trivial(kind)`, `Dynamic`/`StringValue` →
  `BoundValue` (produced value, no obligation — the existing
  inventoried call-result vocabulary); a bare expression statement
  admits `result_kind == NoValue` only and mints no row. No ledger
  row, no claim change: the physical `Callee::Method{RuntimeData}` /
  `ArrayElementWrite` emission is already owned by the raw lane and is
  not lifecycle-required.
Source authority + canonical issuer:
  `VerifiedResolvedMethodCallSourceV1` rows keyed by call site in
  `method_calls()` supply `receiver_site`, ordered `arguments`,
  `selector`, `arity`. The receiver site must seal as
  `BodyExpressionShapeV1::FieldAccess { object: <me site>, field }`
  with `me` proven self-rooted by `PrefixLocalFlow::is_self_rooted_handle`;
  the field's declared type proves `ArrayBox` via
  `with_source_object_definition` on this declaration's own box
  source — a `container_field` predicate sibling to `scalar_field`,
  wired only into the verified-completion lane.
Non-authority:
  `ResolvedMethodCallReceiverSourceV1` — `Other` merely excludes the
  lexical/me lanes; the FieldAccess shape row is the receiver proof.
  MIR types, runtime layout, `ARRAY_SURFACE_METHODS`, and any
  selector matching outside the generated manifest decide nothing.
Fail-fast boundary:
  a non-`me` receiver, a non-`ArrayBox` field, a selector/arity absent
  from the manifest (`pop`/`slice`/`insert`/`clear`/`contains`/…), a
  non-`NoValue` call in statement position, an argument subtree
  containing a `new`/call/map/array/block node, or an argument leaf
  observing `Handle`/a live `homes` member (`BoundValue` results of
  nullable receiver calls are `homes` members and stay excluded —
  passing them into a builtin call is an untracked ownership
  transfer) keeps `PrefixNotCovered`.
Smallest next slice:
  `MIRBUILDER-GATE1-NESTED-RECEIVER-CALL-S0` — one sibling observer
  under `home_new_prefix_*.rs` admitting both statement shapes.
  Pins: `allocate`'s prefix advances past the `.get`/`.set` cluster
  and stops at `me.requested_sizes.set(block_id, requested_size)` —
  `requested_size` is an `OpaqueHandle` param observing `Handle`, so
  the boundary must fire there; negatives cover each fail-closed arm.
Non-claims:
  `allocate`'s claim still does not emit — the `OpaqueHandle` param in
  a store position, `release/1`'s `handle.<field>` reads, and `Bool`
  return literals stay distinct later rows, so the smoke terminal is
  expected to keep `artifact-unowned-lifecycle-site`.

Landed `MIRBUILDER-GATE1-NESTED-RECEIVER-CALL-S0`:
`home_new_prefix_field_call.rs` admits `me.<field>.m(..)` when the
sealed method-call row's receiver is a `FieldAccess` rooted at the
lexical `me`, a new `container_field` predicate proves the field's
declared type is exactly `ArrayBox` (source object definition — never
MIR type or runtime layout), the (selector, arity) pair resolves in
`CORE_METHOD_CONTRACT_ROWS_V2`, and every argument subtree is
Home-neutral (trivial locals/literals, `me` receiver-position reads
proven scalar, non-`homes` `BoundValue`s admitted; owned Handle/Home/
Nullable arguments and `new`/call/map/array/block nodes keep
`PrefixNotCovered`). Statement position admits `NoValue` manifest rows
only; `local x = ..` installs `I64Value`/`BoolValue` as `Trivial`
scalars (new `PrefixLocalFlow::install_scalar_call_result`) and
`Dynamic`/`StringValue` as `BoundValue` — no ledger row is minted, the
raw lane's `Callee::Method{RuntimeData}`/`ArrayElementWrite` emission
stays the sole physical owner. `container_field` is threaded through
`scan_new_home_flow`, `walk_branch`/`observe_if_statement`, both
`issue_new_home_prefixes_*` siblings, the compat verify wrapper, and
the coseal issuer's real `receiver_container_field` closure.
Pinned by `ordinary_new_receiver_field_call_is_home_neutral`
(`set`/`push`/`get`/`length`/`has` including a branch-local cluster)
and `ordinary_new_receiver_field_call_stays_fail_closed` (14 negative
shapes: discarded `get`, `NoValue`-in-local, `pop`/`insert`/`clear`/
`contains`, non-self receiver, non-`ArrayBox` field, `new`/nested-call/
array/map/owned-handle/`me.items` arguments). The real
`page_heap_box` census now pins `allocate`'s prefix at Body(8) —
`me.requested_sizes.set(block_id, requested_size)` — exactly the
`OpaqueHandle` boundary the D0 predicted.
Serial suite note: the wired deterministic baseline is
`cargo test --profile quick --lib -- --test-threads=1`
(`tools/checks/manifests/cargo_lib_red_baseline.toml`); an earlier
`--release` run of this slice reported five extra reds
(`outer_exit_after_nested_loop_targets_outer_region`, four
`runtime::plugin_loader_v2` env-fallback tests) that reproduce only in
release — `vm_compat_fallback_allowed` caches its env read in a
release-only atomic — and pass under the official quick profile.
Official-profile result: 8105/126, failure set byte-identical to the
wired manifest; zero current-change delta. The mimalloc-lite physical
smoke stays 3/3 deterministic at
`[freeze:contract][ordinary-new/local-commit/artifact-unowned-lifecycle-site]`
— `allocate` keeps `RetainedUnavailable` past `requested_size`, by
design.

Next: `MIRBUILDER-FIXED-EXE-SUITE-REMEASURE-I0` — re-run the fixed
11-entry `real-apps-exe-boundary` suite once to re-pin the current
pass/fail receipt (last record: 5 PASS / 6 FAIL on 2026-09-29); then
bundle one chosen app to full completion as the next task lane.

### I0 landed — FIXED-EXE-SUITE-REMEASURE (2026-09-30, `1eba1bcb11`+wt)

`tools/smokes/v2/run.sh --profile integration --owner-profile
integration --suite real-apps-exe-boundary` on the post-S0 binary:
**5 PASS / 6 FAIL** — the count matches the 2026-09-29 record; the
per-app first terminals are:

| app | result | first terminal |
| --- | --- | --- |
| typed_object_newbox_min | PASS | — |
| typed_object_birth_param_min | PASS | — |
| typed_object_method_min | PASS | — |
| typed_object_birth_min | PASS | — |
| real_apps_exe_boundary_probe | PASS | designed negative |
| typed_object_untyped_field_min | FAIL | `ordinary-new/local-commit/artifact-source-unavailable` |
| binary_trees | FAIL | `ordinary-new/local-commit/root-call-entry-unavailable` |
| boxtorrent_mini | FAIL | `ordinary-new/local-commit/emission-binding-drift` |
| allocator_stress | FAIL | `CoreMethodSource NamedArray(TextSourceMissing)` |
| json_stream_aggregator | FAIL | `callable-loop/route-not-front-selected` (`SourceCallOutsideSelectedFamily` in `JsonStreamAggregator.ingest/1`) |
| mimalloc_lite | FAIL | MIR-JSON leg now freezes `ordinary-new/local-commit/emission-binding-drift` — the 9/29 pin was `unsupported terminator Invoke`; coverage extension moved the app's first terminal to an earlier designed freeze (still fail-closed, no permissive Invoke admitted) |

Note: the mimalloc MIR-JSON negative pin drifted inside the fail-closed
ladder (compile now stops before the JSON emit leg ever sees the
program). The `--emit-exe` leg still reaches
`artifact-unowned-lifecycle-site` 3/3. Pin re-wording belongs to the app
bundle that closes this terminal, not to this slice.

Next: `MIRBUILDER-APP-BUNDLE-MIMALLOC-LITE` — see the app-bundle card
`mirbuilder-app-bundle-mimalloc-lite-d0-2026-09-30.md`.

## Decision — MIRBUILDER-GATE1-ARG-CARRIER-EVIDENCE-D0 (accepted, landed)

`Local` argument carriers follow the sealed scalar call-edge rule —
`check_call_edge` (`verification/invoke.rs`, `Sealed` policy) is the sole
carrier contract; no bounded carrier-evidence row is minted. Recorded
concrete non-i64 carriers still freeze `nullable-argument-carrier`.
`ARG-CARRIER-EVIDENCE-S0` executed; full record via git.

## Decision — MIRBUILDER-GATE1-RETAINED-NEW-HOME-FLOW-D0 (accepted, landed)

Prefix coverage stays owned by the existing per-function Home-flow scan;
`RetainedUnavailable` releases staged argument field reads at the decline
point inside `prepare_new_emission`/`prepare_result_new_emission`.
`RETAINED-NEW-HOME-FLOW-S0` executed; full record via git.

## Decision — MIRBUILDER-GATE1-NULLABLE-RESULT-ABI-D0 (accepted, landed)

Nullable receiver-call result ABI landed as `NULLABLE-RESULT-ABI-S0`
(S0a observation product + S0b emission/publication; S0c-e folded):
`ReceiverCallClassObservationV1` typed arguments, `Nullable` local-call
class + `StoredLocal::ReceivedNullable`, `CallReceivedNullable` with
`HomeReleaseIfLive` (never unconditional `HomeRelease`),
`OrdinaryNullableHandle` role + `"nullable_handle"` wire result +
`const_null` sentinel + `compiled_entry_contract` pairing + C shim.
The claim map stays the sole authority for the caller's ABI edge;
`handle-release-unproven` and `PrefixNotCovered` floors unchanged.
Full design, census anchors and contract set recoverable via git.

## Decision — MIRBUILDER-NONCOND-CARRIER-D0 (closed, superseded)

Premise falsified (full census via git): outside-cohort rebinds already
ride the armed LoopCond carrier path — `count += 1` and
`iterationCheck`'s exact loop shape compile end-to-end. The census
surfaced the real callee-graph wall (`make` multi-exit `return new`),
whose lineage landed below. `SourceItemsMissing` /
`SourceCallOutsideSelectedFamily` / `VariableAccumRecurrence*` remain
named fail-fast terminals. Next slice was
`MIRBUILDER-GATE1-MULTI-EXIT-RETURN-NEW-D0` (landed below).
Non-claims: no `--dump-mir`->EXE equivalence, no Gate-1 completion.

## Census — MIRBUILDER-GATE1-MULTI-EXIT-RETURN-NEW-D0 (accepted)

Design-stop census of the Handle-result singleton gate for multi-exit
`return new` callees, inside the `MULTI-RETURN-RESULT-NEW` D0 lineage.

```text
Decision:
  the singleton `Value(Construction)` terminal gate at
  `begin_handle_call_emission` is a deliberate fail-closed boundary,
  not an accidental gap — "several construction sites mint different
  canonical objects, and the caller cannot observe which exit ran".
  `make` is the production-relevant multi-exit case; admitting it needs
  an explicit per-exit result contract, never arbitrary-site borrowing.
Source authority + canonical issuer:
  callee side already carries N result claims —
  `result_claims` is a `Vec<OrdinaryNewResultClaimV1>` keyed per
  `OwnedExprSiteV1`, so `make`'s two exits co-seal independently.
  The sole rejected member is the CALLER-side
  `CallReceivedCommitV1.object` pin: one canonical object id feeds
  `end_children` (owned-field children -> teardown proof) at
  `begin_handle_call_emission`.
  Handle-class membership is minted by the CALLER's
  `home_new_prefix_scan`: `local x = <call>` whose callee is
  unannotated + construction-result seals
  `LocalCallResultClassV1::Handle` (`direct_call_lifecycle` /
  `home_new_prefix_scan` Handle arms). A `me.` receiver is not a
  binding -> never enters `local_calls`.
  Production Handle-candidate sites on `make` (exact inventory):
  `run()` `builder.make(stretch_depth,0)` + `builder.make(long_lived_depth,0)`
  via `local builder = me.builder`; `iterationCheck` loop-body
  `builder.make(depth,i)` + `builder.make(depth,0-i)` via param
  `builder`. All four are `local x = <construction-result call>` rows ->
  Handle minted in the caller's root-flow scan; emission then meets the
  singleton gate. `positive.itemCheck()`/`negative.itemCheck()` are
  i64-result calls — different class.
Non-authority:
  `Callee::SameModuleInstance` self-calls (`me.make` inside `make`)
  ride the generic same-module route — no local-commit singleton
  gate, observed compiled clean (`call_same_module_instance` rows +
  both `new TreeNode` exits + `ret` under `--dump-mir`). Dynamic
  `call_method` receivers owe no Handle commitment. `itemCheck`'s
  i64 return is a different result class entirely.
Fail-fast boundary:
  `handle-result-terminal-missing` (0 or >=2 Value relations) and
  `handle-result-terminal-mismatch` (non-Construction terminal) stay
  the exact reject tokens; no demotion, union-object, or exit-count
  fallback is admitted by this census.
Smallest next slice:
  `MIRBUILDER-GATE1-MULTI-EXIT-RESULT-ABI-D0` — pick the per-exit
  result contract among
  (a) uniform-exit admission (all exits same class + same children
      shape; `make` still fails: exit1 children={} vs exit2
      children={left,right} call-received handles),
  (b) exit-witness result ABI (Return edge carries the taken exit's
      object identity; caller children/release resolve dynamically),
  (c) caller-side multi-object commit (pre-commit every exit's
      children set, discharge on the observed exit's witness).
  `root-call-entry-unavailable` gates `run()` reachability upstream
  (separate family, already D0'd) but the contract choice itself
  does not need reachability — only the exact 4-site inventory,
  which this census fixed.
Non-claims:
  no production site is proven to reach the gate (Handle-class arming
  of `builder.make` field-receiver sites inside `run()` is
  unobservable at current reachability); no `--dump-mir`->EXE
  equivalence; no `run()` progress; no multi-exit callee-body coverage
  claim beyond `make` observed.
```

Census boundary: `handle_call_source` Handle-class local calls ->
`begin_handle_call_emission` -> `CallReceivedCommitV1.object` ->
`end_children`. Includes `make`'s two exits and the commit-row object
pin. Excludes SameModuleInstance/dynamic-call routes (no gate),
`itemCheck` i64 results, upstream `root-call-entry-unavailable`,
selected-C, Gates 2-4.

## Decision — MIRBUILDER-GATE1-MULTI-EXIT-RESULT-ABI-D0 (accepted)

The fork from the census above collapses once `CanonicalObjectIdV1`'s
level is read correctly: it encodes the module declaration index —
class-level, not per-site. `construction_result_callee` already
requires "every verified explicit `return` exit constructs `new` of
one agreed class" for the Handle class mint, so any Handle-minted
multi-exit callee is already class-uniform. `make`'s two exits both
construct `TreeNode` -> one shared canonical object -> identical
`end_children` (children are class field residences, not
construction-arg provenance).

```text
Decision:
  admit multi-exit `return new` at `begin_handle_call_emission` under
  the uniform-shared-object rule — every `Value` terminal relation of
  the callee must be `Construction(site)` owned by the callee, and
  every site must resolve to the SAME canonical object (verify all,
  never pick arbitrarily). The singleton `as_slice()` check was the
  pre-multi-exit shape of the same invariant.
Source authority + canonical issuer:
  `construction_result_callee` (uniform-class Handle mint) upstream +
  re-verified per-site `result_object` equality at the gate ->
  `CallReceivedCommitV1.object` shared object -> `end_children`.
Non-authority:
  construction-arg provenance (exit1 `null,null` vs exit2
  `left,right`) never decides children — field residences are
  class-level and the release ops (`OwnedObjectFieldRelease`,
  `OwnedFieldResidenceRelease`) are conditional on runtime liveness
  ("slots are zero-initialized, so an unwritten field is skipped").
  `HomeRelease { object }` is class-level too — exit-uniform.
Fail-fast boundary:
  zero relations -> `handle-result-terminal-missing` unchanged;
  any non-Value/non-Construction or foreign-owner relation and any
  object-id divergence -> `handle-result-terminal-mismatch` /
  `-missing` stay named. Mixed-class multi-exit (unreachable under
  the Handle mint but re-verified) never admits.
Smallest next slice:
  `MIRBUILDER-GATE1-MULTI-EXIT-UNIFORM-CLASS-S0` — LANDED: the gate
  now iterates all `Value(Construction)` relations and requires one
  shared canonical object (every site re-resolved, never picked).
  Pins: `handle_result_local_call_admits_uniform_class_multi_exit_
  callee` (2-exit `Point` callee lowers: Handle invoke + one
  HomeRelease + finalized artifact validates) and
  `handle_result_lane_rejects_mixed_class_multi_exit_callee`
  (Point/Other exits fail closed at issue). Suites: handle_result 9/9,
  direct_call_lifecycle 34/34, ordinary_new 273/273, lexical 232/232.
  No new receipt, ABI arm or exit witness minted.
Non-claims:
  no production `run()` reachability (upstream `root-call-entry-
  unavailable` still gates); no receiver-arming change; no mixed-class
  or non-Construction exit admission; no `itemCheck`/i64 result
  contract; no Gate-1 completion.
Next row: `MIRBUILDER-GATE1-ROOT-CALL-ENTRY-D0` — census which
  `RootHomeExitEntry` kind `run()`'s root Home needs; the multi-exit
  gate is now downstream-reachable once that entry admits.
```

## Landed — ROOT-CALL-ENTRY-D0 + UNRELEASED-ROOT-HOME-D0 + ROOT-INSTANCE-ENTRY-S0

Tombstone (full record via git): `root-call-entry-unavailable` was the
`take_finalized_root_call` non-`Emitted` take; untyped `init` fields
migrated to typed decls per UNTYPED-OBJECT-STORAGE-D0 branch (a);
issuer `unreleasable` widened to `end_available` parity (owned fields
+ sealed children). Baseline red `main_f1_rejects_..._before_lowering`
is parent-reproduced debt. Wall A2 (bisected, then landed below):
computed birth stores.

## Landed — MIRBUILDER-GATE1-BIRTH-STORE-RHS-D0 + BIRTH-COMPUTED-STORE-S0

Accepted (a): plan-level read-after-write — `me.<f>` reads in store
RHS resolve to prior `LiteralI64` stores; field-read BinOp folds
Add/Sub/Mul (checked ops, >=1 field read); `me_reads` transports `me`
sites observed at take. Declines: forward/unwritten/parameter/
object-field/pure-literal. Pins 6/6 + suites; `bench_min` reaches the
`Invoke` boundary.

## Landed — MIRBUILDER-GATE1-NOBIRTH-PROVIDER-CHILD-D0 + NOBIRTH-PROVIDER-S0

Tombstone (full record via git): arity-0 `new X()` on a birthless
fieldless class is the already-sealed `NoBirthZero` disposition;
`ProviderConstructionChildV1::{BirthIndexed, NoBirthZero}` sealed on
the plan; NoBirth arm emits NewBox+ObjectFieldSet+HomeRelease without
recipe/birth call/ABI record; accounting `3*birth + 1*nobirth`. Pins
8/8, suites green; real route clears `provider-birth-recipe-missing`
to the preserved `Invoke` boundary. Baseline red (parent repro):
`provider_owned_array_child` assert and `source_stringbox` parallel
env race (`emit_plain_program_and_mir_json` reads ambient env
unguarded; `host_providers::mir_builder` subset at >=4 threads).

## Census — MIRBUILDER-GATE1-STORED-CHILD-RECEIVER-D0 (closed)

Boundary: `local b = me.<obj-field>; b.m(...)` in `run()` on the
`--emit-exe` published route. Probe chain: `me.builder` `FieldGet`
projection admits only `slot_load_i64/u64`/ArrayBox-handle arms — a
`Box` slot load is `field-get-route-drift`; i64-result `b.m()` is
`admission-candidate-unavailable`; object-result shape is
`artifact-unowned-lifecycle-site` — the exact token the sibling card
records as its last unchanged-app observation.

Ownership: sibling lane `MIRBUILDER-STORED-CHILD-BORROWED-C-RECEIVER-S0`
(card `mirbuilder-stored-child-borrowed-call-receiver-d0-2026-10-05`)
owns the stored-child receiver family (direct `me.<field>.m(..)`,
canonical field read, WIP `instance_construction_child_call.rs`); the
local-bound variant is a boundary item inside the same family.

Decision: `ParkedSealed` on the sibling owner — no parallel receiver
authority. Reopen trigger: sibling S0 lands and the local-bound
spelling still rejects on the unchanged app -> new D0 for the
residual arm only. Non-authority: `.hako` rewrite, parallel field-load
arm. Fail-fast boundary: `field-get-route-drift`, `-lifecycle-site`
stay. Smallest next slice: none here — next census is
`MIRBUILDER-GATE1-NULLABLE-FIELD-D0`. Non-claims: sibling lane
completion, nullable fields, Gates 2-4.

## Census — MIRBUILDER-GATE1-NULLABLE-FIELD-D0 (accepted)

Boundary: `TreeNode` (`init {left,right,value}`) — `make` result
`new TreeNode(null|null|local,..)`; `itemCheck` `me.left == null` /
`me.left.itemCheck()`. Published `--emit-exe` probe map:
- `new T(null,null,v)` untyped `init` -> `artifact-source-unavailable`;
  retained row `construction=Err(SourceRelationMissing)`.
- Typed decls `left: T` -> `RetainedUnavailable`:
  `owned_field_children_of` declines self-referential children
  (`child != object`, ..._owned_children.rs:108).
- Non-self-ref `new Box2(null|local)` -> `Kind::Null`/`Handle` seal,
  then `actual-kind-unavailable` at `scalar_actual_kind`
  (physical_abi.rs:737).
- `me.<obj-field>` read/`== null`/`.itemCheck()` -> sibling family.

Decomposition: (1) typed-init migration (prerequisite only);
(2) self-ref children guard; (3) `Null`/`Handle` actual admission vs
field capability; (4) null-fed owned-field teardown — no issuer
(NULLABLE-RESULT-ABI covers result-position locals only); (5)
`scalar_actual_kind` tag arms — downstream of (3)/(4). No nullable
field surface exists in `.hako`; whether `left: T` means always-live
is the open question -> forwarded to NULLABLE-OWNED-FIELD-D0 below.

## Decision — MIRBUILDER-GATE1-NULLABLE-OWNED-FIELD-D0 (accepted)

Boundary: null-capable owned fields — `new T(null|local,..)` actuals,
`me.f = <possibly-null>` stores, object-field teardown. Excludes
field reads/`== null`/receivers (sibling family), Gates 2-4.

Authority map:
- `OwnedObjectFieldsNoHook` means every field is a live user-object
  residence; children census `owned_field_children_of` declines
  self-reference (:108) and nested kinds (:155 PlainI64/OwnedArray
  only); `end_plan` -> `OwnedObjectFieldRelease` walks child
  `owned_residences` one level — "deeper teardown stays unadmitted"
  (invoke.rs:117); `owned_object_residences` mark already published.
- Physical layer is already null-tolerant: slots zero-init, every
  field release is live-checked; `home_release_if_live` wire op and
  `Handle` `new`-actual `consume_home` move accounting are landed.
- Wire vocabulary complete: tag 0 = null pair, tag 3 = object +
  `object_view` runtime-type check (ordinary params); the birth
  prologue admits only kinds 1/2. `scalar_actual_kind` rejecting
  `Null`/`Handle` is the sole arm for non-self-ref `new T(null|local)`.

`.hako` surface resolution: object-typed fields are inherently
null-capable handle slots — `left: T` needs no marker; `init`->typed
migration is a prerequisite only.

Ordered slices:
S1 `NULLABLE-NEW-ACTUAL-S0` — capability gate (`Null`/`Handle` actual
must bind a formal provably stored into an object-typed declared
field via the construction relation; others freeze) +
`scalar_actual_kind` `Null`->0/`Handle`->3 arms + birth-prologue
object-param tag admission. Edge: `new Box2(null)`/`new Box2(local)`
published route.
S2 `NESTED-OBJECT-FIELD-TEARDOWN-S0` — children census admits
`OwnedObjectFields` incl. self-reference + recursive nested release
emit (flat unroll cannot express self-ref depth; a runtime
`home_release_owned`-class helper) + TreeNode `init`->typed migration.
Edge: `new TreeNode(null,null,v)` children seal.
S3+ — `me.left` reads/`== null`/`.itemCheck()`: sibling family.

Decision: `MIRBUILDER-GATE1-NULLABLE-NEW-ACTUAL-S0` accepted as the
smallest next slice.
Source authority: sealed `OrdinaryNewTrivialArgumentKindV1` + the
construction relation param->field map + declared field types.
Canonical issuer: coseal argument-capability gate; `physical_abi`
`scalar_actual_kind`; emit.inc birth-prologue object-param arm.
Non-authority: `.hako` surface invention, census/emit relaxation for
nested children (S2), unconditional release.
Fail-fast boundary: `actual-kind-unavailable`/`RetainedUnavailable`
stay for non-capable actuals and nested children.
Non-claims: nested teardown, field reads, Gates 2-4.

## Landed — MIRBUILDER-GATE1-NULLABLE-NEW-ACTUAL-S0 (S1 complete)

`Parameter{provided}` seals at plan issue -> `ObjectFieldStores` formal
capability -> `nullable_kind_payload_v1` param -> `object_field_set` +
`home_release_if_live` fault tail. `Null` spells (0,0); owned `Handle`
spells (3,payload) — `object_birth_actuals` keys (site,ordinal) and the
C flow consumes the live lease on the Normal edge only. C: `origin -3` param
class, k0/(3,nonzero) prologue; foreign handles reject. Evidence: pins
4/4; both nullable smokes PASS (v4-measure ok, EXE Result 0); mismatched
lanes reject; 4 baseline reds. Next row `MIRBUILDER-GATE1-NESTED-OBJECT-FIELD-TEARDOWN-D0`
(S2 census + emit authority); field reads stay sibling family.

## Preserved contract boundaries

- Generic `mir_json_emit` rejects lifecycle Invoke. Invoke, normal-result and
  Fault transport use `hako.published-lifecycle-physical-program.v2` through
  the existing published backend view and lifecycle V4 route. No generic
  reader receives a permissive Invoke arm to fix an app smoke.
- Artifact observation requires source completion; claimed-but-unavailable
  child construction rejects before lifecycle coverage, including NoBirthZero.
  Every lifecycle-required instruction still needs its exact ledger binding.
  `RetainedUnavailable` is never permission to publish an artifact.
- The current physical object profile requires `PlainI64NoHook` and I64 fields.
  Names, slot tags, absent hooks and process exit cannot prove releasability.
  Owning-field release and tagged dynamic storage remain separate dependencies.
- Result-new claims own exact Return membership, prior-Home Fault cleanup,
  Invoke-shaped construction and exactly one Return of the emitted object.
  Argument/field-position constructions do not inherit this authority.
- Direct and claim-local lexical Handle calls consume the callee's sealed
  Construction result plus class claim; received objects install as one
  owned caller Home, release accounted per normal/Fault path. Opaque,
  rebound, parameter/nested receivers and unsupported result contracts
  gain no path. Root-result ABI is separate.
- `BinaryTreesBench.run(): i64` (`5fd8f3fd17`) follows the `MiWorkload`
  explicit-contract precedent (`b16c3548ac`); not evidence for the
  unannotated input.

## Evidence retained from the previous card (tombstone)

Fixed-EXE suite `574d90ffc5`+`c82b7a415a`, reviewer remediation and the
organization closeout recover at `git show 339674c77b:<this-file>`.
