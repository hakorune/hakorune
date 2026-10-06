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

## Decision — MIRBUILDER-GATE1-ARG-CARRIER-EVIDENCE-D0 (accepted)

Decision:
  `Local` argument carriers follow the sealed scalar call-edge rule —
  a recorded `Integer`, `Unknown`, or unrecorded wire type admits;
  any recorded concrete non-i64 carrier rejects. No bounded
  carrier-evidence row is minted: `check_call_edge`
  (`verification/invoke.rs`, `Sealed` policy) already is the sole
  carrier contract for cataloged same-module Call edges, and a Facts
  row would only re-assert the same recorded `value_types` fact it
  corroborates. Unannotated parameters (`size`, `requested_size`)
  ride under the untyped admission the callee's own `Unknown` formal
  and the legacy call path already implement — the wire carries i64
  slots and borrowed arguments transfer no ownership.
Source authority + canonical issuer:
  `check_call_edge` is the canonical carrier rule ("a recorded
  argument type must prove `Integer` — or stay unrecorded, matching
  the physical lane's i64 spelling"); `emit_receiver_nullable` stays
  the sole emission owner and `classify_argument` in
  `ordinary_new_receiver_call_observation` stays the sole argument
  issuer — neither gains a new authority.
Non-authority:
  `value_types` is the corroborated physical fact, not a semantic
  classification source — no `param_decls` re-classification, no
  callee-side param-ABI catalog inference; an unsealed instance-method
  formal means untyped admission, never a guessed i64 demand.
Fail-fast boundary:
  a recorded concrete non-i64 argument type (`Box(_)`, `String`,
  `Bool`, `Void`, `Float`, `Array`, `Future`, `WeakRef`) still freezes
  `nullable-argument-carrier` — including `Box("MapBox")`, whose
  borrowed-storage pair is a separate map-argument contract this lane
  does not serve; `Bool`/`Null` literal and non-`Local` argument kinds
  stay unadmitted exactly as issued today.
Smallest next slice:
  `ARG-CARRIER-EVIDENCE-S0` — align the `Local` arm's carrier check in
  `emit_receiver_nullable` to the sealed edge rule under the same
  freeze token; pin positive (untyped parameter arg emits) and
  negative (concrete `Box`-typed local arg still freezes) coverage.
  Expected fixture first terminal returns to
  `artifact-unowned-lifecycle-site`.
Non-claims:
  no handle/map/object-carried argument family is admitted; no
  parameter-ABI catalog is wired; no source type-annotation
  requirement is added; the four census coverage families stay parked;
  this is not a production-success claim.

## Decision — MIRBUILDER-GATE1-RETAINED-NEW-HOME-FLOW-D0 (accepted)

Decision:
  Prefix coverage stays owned by the existing per-function Home-flow
  scan — there is no new "nine-site prefix" admission; the remaining
  gaps decompose into bounded grammar forms (receiver field-write
  statements, `me.<field>.m(..)` field-receiver calls, the `me.realloc`
  param-forwarding class) plus the `HakoAllocHandleResult` Birth
  replacement-store/object-typed-param-store bound. A claim judged `RetainedUnavailable`
  releases its staged argument field reads at the decline point;
  `unconsumed-read` remains the fail-closed terminal only for
  emission-eligible claims.
Source authority + canonical issuer:
  `home_new_prefix` scan and the ordinary-new claim ledger (existing
  owners); the release is issued inside `prepare_new_emission` /
  `prepare_result_new_emission` at the `RetainedUnavailable` store —
  the single point where eligibility, including dynamic prior-Home
  availability, is fully known.
Non-authority:
  The raw lane's compatibility `FieldGet` never discharges staged exact
  evidence; `RetainedUnavailable` never authorizes emission or
  publication; no site/class/destination shape infers ownership.
Fail-fast boundary:
  A staged read left neither `Emitted` nor `Released` still freezes
  `ordinary-field-read/unconsumed-read`; `Taken`-without-`Emitted`
  faults through `emission-mismatch`/`root-exit-phase` unchanged.
Smallest next slice:
  `RETAINED-NEW-HOME-FLOW-S0` — `Progress::Released` disposition plus
  decline-time release for ordinary and result claims; the fixture
  smoke's first terminal returns to `artifact-unowned-lifecycle-site`.
Non-claims:
  No retained site emits; no new grammar is admitted; the four coverage
  families above and the recorded lifecycle-artifact boundary are
  separate later rows; this is not a whole-gate claim.

## Decision — MIRBUILDER-GATE1-NULLABLE-RESULT-ABI-D0 (accepted)

Decision: a dedicated nullable-result lane — never a reuse of Handle.

Source authority + canonical issuer:
  `OrdinaryNewResultClassV1::NullableObject(C)` (sealed claim) +
  `ReceiverCallClassObservationV1` (site-keyed, entry-loan-gated,
  exact destination) — both already issued inside one co-seal sweep.
Non-authority:
  `callable_result_class` stays `Object`-only; `BoundValue` stays the
  unclassified floor; no consumer may infer ownership from the
  observation row alone.
Fail-fast boundary:
  emit-time, not verify-time — missing observation, destination drift,
  unsupported carrier, or claim↔kind disagreement all freeze.
Smallest next slice:
  one new semantic observation product + one physical result arm + one
  lowering owner for exactly `local x = me.m(<args>)` whose callee is
  `NullableObject` and whose destination is provably live-stored.
Non-claims:
  no backend execution contract exists today; enum arms alone move no
  call; no implementation has been done under this row.

### Census evidence anchors (worker, read-only)

- Executable result chain is `I64 | Map | Handle` (+`Unit`) everywhere:
  `LocalCallResultClassV1` (`home_local_call_flow.rs:14-25`),
  `InvokeCallResultKind`/`InvokeNormalResultKind`
  (`instruction/invoke.rs:34-45`, `invoke_map.rs:4-17`), `StoredLocal`
  (`home_prefix_local_flow.rs:22-48`), published roles
  (`physical_program.rs:34-73`, JSON `"i64"|"map"|"handle"` at
  `physical_program_json.rs:501-581`), verifier matrix
  (`verification/invoke.rs:164-216`). No nullable arm anywhere.
- `local handle = me.allocate(size)` (`page_heap_box.hako:219`) is
  doubly excluded: non-literal argument + non-definite class →
  `install_inventoried_call_result` floors it to `BoundValue`
  (`home_prefix_local_flow.rs:390-409`, prefix `PrefixNotCovered`).
- `LocalCallObservationV1.arguments: Box<[i64]>` is literal-only
  (`home_local_call_flow.rs:35`, gates :131-143/:183-195) — a
  `Receiver`-kind `me` receiver and parameter arguments are both
  ineligible; the nullable lane needs typed arguments, i.e. a *new*
  observation product, not an arm on this row.
- `CallReceivedCommitV1` emits unconditional `HomeRelease`
  (`call_received.rs:87-91`) — structurally incompatible with null
  paths; conditional release needs a checked-null operation that does
  not exist yet.
- Claim↔terminal divergence to resolve at design time: `allocate`
  ends in `return <call>` which `call_result_kind` classifies `I64`
  while the claim says `NullableObject` — the D0 names the claim map
  the sole authority for the caller's ABI edge.
- No `Option`/`MirType` nullability, no tag+payload carrier, no
  null-check instruction (`physical_abi.rs:429-444` has null only as
  a non-scalar argument tag); null representation (sentinel vs tagged
  pair) is an open decision for the S0 slice.
- Reads of a nullable local (`handle.f`, null checks) are out of
  scope: unsupported downstream use keeps `PrefixNotCovered`.

### Named contracts (spanning set, ordered)

1. Semantic: a new receiver-call observation consumption product (typed
   arguments — binding/literal kinds like `SelectedNewArgumentKindV1`,
   not `Box<[i64]>`), or widening `LocalCallObservationV1`; decide one.
2. Physical: `InvokeCallResultKind` + `InvokeNormalResultKind` nullable
   arm with a `MirType` carrier; `verification/invoke.rs:164-216`
   admits it per callee class; exactly one `InvokeNormalResult`.
3. Lowering owner: extend the `emit_local_lexical` lane
   (`terminal_call.rs:307-396`) to consume the observation row +
   a disposition whose `row.result()` admits nullable (`:316`,
   port `:87-92`), emitting `Call { SameModuleInstance, result:
   <nullable> }`.
4. Local state: `StoredLocal` + `OrdinaryObservation` nullable arm;
   `stored_local_same`/`join_branch` stay fail-closed.
5. Conditional cleanup: a new commit variant with a checked-null
   release — never reuse `CallReceivedCommitV1`'s unconditional
   `HomeRelease`.
6. Publication: `OrdinaryNullableHandle` role + wire name +
   `"nullable_handle"` JSON + `compiled_entry_contract` pairing.

### Split plan — NULLABLE-RESULT-ABI-S0

- S0a (semantic) — landed: `ReceiverCallClassObservationV1` carries
  typed arguments (`SelectedNewArgumentV1` — Integer/Bool/Null/Local
  only; anything else leaves the site unobserved). Consumption evidence
  complete, still no emission change.
- S0b (emit owner) — landed: `emit_receiver_nullable` inside the
  `handle_call` emission owner consumes the sealed row and emits
  `Invoke { Call { SameModuleInstance, result: NullableHandle } }` +
  `InvokeNormalResult { NullableHandle }` with `MirType::Box(C)` on the
  live arm. `LocalCallResultClassV1::Nullable` +
  `StoredLocal::ReceivedNullable` carry the flow; `CallReceivedNullable`
  commits with `HomeReleaseIfLive` (never unconditional `HomeRelease`).
  Publication: `OrdinaryNullableHandle` role, `"nullable_handle"` wire
  result, `const_null` sentinel (Void → i64 0), `home_release_if_live`
  cleanup, `compiled_entry_contract` pair enforcement, C shim
  (`const_null` seeds a handle-lane slot with negative origin;
  release-if-live discharges the lease and calls the kernel only for
  live handles). Return-position `new` birth actuals now mint from
  `Result` commit rows (`destination` is `Option` — the site owner is
  the only owner authority there). End-to-end JSON:
  `nullable_receiver_call_serializes_nullable_handle_and_checked_release`.
- S0c/S0d/S0e — folded into S0b (single owner + one wire vocabulary);
  no separate rows remain.

## Decision — MIRBUILDER-NONCOND-CARRIER-D0 (census, accepted)

Recorded premise corrected. Workstream row H named the next wall
"`sum` carrier lacks ConditionRead". Probe census falsifies it: an
outside-cohort rebind already rides the armed LoopCond carrier path —
`count += 1` (BodyRebind, no ConditionRead) compiles with a real phi,
and a source fixture replicating `iterationCheck`'s exact loop shape
compiles end-to-end under `--dump-mir`.

Scope: callable-lane coverage for `BinaryTreesBench.iterationCheck`'s
`sum` — `local sum = 0; loop(i <= iterations) { local positive =
builder.make(depth,i); local negative = builder.make(depth,0-i);
sum += positive.itemCheck(); sum += negative.itemCheck(); i += 1 }
return sum`. Census covers binding classification -> `ReadyWithBodyOnly`
-> route match -> route-token coverage -> physical carrier. Excludes
`run()`'s upstream `root-call-entry-unavailable` (still first),
production EXE acceptance, and `itemCheck`'s own body.

### Layered census (probe-observed)

| Layer | Shape | Disposition |
| --- | --- | --- |
| Binding | `sum`: BodyRead+BodyRebind, no ConditionRead | `outside_bindings` -> `ReadyWithBodyOnly` — by design (outside-observed-class D0); not a defect |
| Outside carrier | `count += 1`, `sum += <call>` | rides a phi — carrier mint derives from the recipe, not the binding schedule |
| Facts condition | `i <= <binding>` | `VarCompareBound{Le,Var}` observed — `<=` is not a facts wall |
| Route | `i <= n` + 5-stmt body | LoopCond front-selects |
| Recipe items | `Local`/`Assignment`/`MethodCall` | admitted `Stmt` vocabulary, incl. decl-init and call-valued rebinds |
| Call coverage | `builder.make` param receiver, `positive.itemCheck()` claim-local | arms only via caller-edge/initializer claim provenance — unprovable in isolated probes (`SourceCallOutsideSelectedFamily`), closes in the real call graph (recorded at CALLRESULT-RECEIVER-RESULTCLASS-S0) |
| CallFree arm | `local` decls, `<=`, non-scalar exprs | rejected by design (`<` + rebind-only + scalar) -> `SourceItemsMissing` for no-call variants — irrelevant on the armed path |
| Accum deferral | 2-stmt `x = f(x, induction)` | `claims_variable_accum_family` defers exclusively to fixture-pinned VariableAccum (S10/M10b-I0-R0-VAR); binary-trees never reaches it |

### Armed-path evidence

`bt_full` probe (caller `local b = new Builder()` -> `me.iter(b,1,3)`;
`iter` carries the exact loop): `iter` lowers with `sum`/`i`/`iterations`
phis (`icmp Le`, two `call_same_module_instance`, `local.contract.write`
Reassigns) — full compile, no freeze. `pow2`'s `out = out * 2`
multiplicative outside carrier also compiles clean.

The first real wall inside the callee graph surfaced at `make`: a
`local left = me.make(...)` handle-result call whose callee has **two**
`return new` exits (`if depth == 0 { return new TreeNode(..) }` /
`return new TreeNode(left, right, value)`) freezes at
`ordinary-new/local-commit/handle-result-terminal-missing` —
`begin_handle_call_emission` admits only a single `Value(Construction)`
terminal relation per callee, so a mixed multi-exit return-new callee
stays unadmitted. That is the `MULTI-RETURN-RESULT-NEW` D0 lineage
already decomposed above, not a carrier gap.

Decision:
  retire the recorded `sum`-carrier wall as stale; outside-cohort rebinds
  are already carried by the armed LoopCond path.
Source authority + canonical issuer:
  binding classification -> `ReadyWithBodyOnly` -> selected LoopCond
  route token + recipe carrier mint; outside rows stay verification
  evidence in `body_only` via `consume_pre_effect`.
Non-authority:
  Carrier-class reclassify of body-only rebinds; VariableAccum fixture
  widening; `claims_variable_accum_family` relaxation; generic Outside
  consumers.
Fail-fast boundary:
  `SourceItemsMissing` / `SourceCallOutsideSelectedFamily` /
  `facts-absent` / `VariableAccumRecurrence*` stay named terminals.
Smallest next slice:
  none for `sum` itself — the observed next wall inside the callee graph
  is `make`'s multi-exit `return new` (`handle-result-terminal-missing`);
  the per-exit membership census is
  `MIRBUILDER-GATE1-MULTI-EXIT-RETURN-NEW-D0` under the
  `MULTI-RETURN-RESULT-NEW` D0 lineage. `run()`'s
  `root-call-entry-unavailable` still gates reachability upstream.
Non-claims:
  no `--dump-mir`->EXE equivalence, no `run()` progress, no callee-body
  coverage claim beyond `iter`'s observed loop, no Gate-1 completion.

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

## Evidence retained from the previous card

### Fixed EXE suite — recorded 2026-09-29, not rerun here

Receipt: `574d90ffc5` + `c82b7a415a`; full record at
`339674c77b:this-file` — 5 PASS / 6 FAIL, retained-result completion and
explicit-LLVM-18 fallback repaired; suite failure is not "six designed
stops".

### Reviewer remediation — recorded at 339674c77b

Stale pins corrected, six oversized parents split under 800 lines,
focused suites recorded green; one root-catalog failure reproduced on
parent `053b659637`. Full record at `339674c77b:this-file`.

### Landed history tombstone

Full designs, rejected alternatives and old terminal order are in Git;
`git show 339674c77b:docs/development/current/main/investigations/mirbuilder-gate1-callable-loop-string-indexof-s0-2026-09-27.md`
recovers the complete prior card. No archive copy or new card is created.

| Commit | Landed responsibility / evidence |
| --- | --- |
| `d3a259a01c`..`43b706e07e` | Early slices: StringIndexOf/1, flat call-free LoopCond, MiWorkload contract, instance receivers, StringLen, field-write provenance — full record via git. |
| `179c6c67c7` | Uniform return-new result-class claims and call-result receivers. |
| `5fd8f3fd17` | Non-condition carrier coverage, root-call unavailable finishing/seal distinction, binary-trees result annotation. |
| `4e89bbb551` | Generic Invoke rejection preserved; untyped-field smoke uses physical route. |
| `e054920b05` | Unreleasable artifact boundary; mimalloc smoke physical route. |
| `c8c930d8e6` | Child retained-unavailable rejection before lifecycle coverage. |
| `30ffce464c` | Exact Return-position new claims and artifact bindings. |
| `1dad229932` | Direct Handle result ABI; lifecycle focused 29/29. |
| `493e55de4e` | Claim-local lexical Handle result; lifecycle focused 32/32. |
| `574d90ffc5`, `c82b7a415a` | Recorded suite and retained-result / explicit-tool fixes. |
| `339674c77b` | Guard pins and six source splits; focused results above. |

## Organization closeout

Scope: this card, CURRENT_STATE, the canonical Home ABI D0, the existing
pointer guard (1,000-line budget; verified at `339674c77b`, pre-compaction
record recoverable there). Worker findings integrated; no new observer
census or semantic receipt landed.
