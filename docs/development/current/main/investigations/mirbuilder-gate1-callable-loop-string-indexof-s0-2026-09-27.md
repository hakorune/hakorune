# MirBuilder Gate 1 — current design and acceptance

Status: NULLABLE-RESULT-ABI-D0 accepted (dedicated nullable lane — never Handle reuse; six-contract spanning set named); card compressed to tombstone history; next NULLABLE-RESULT-ABI-S0
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
- Return observation uses `completion.explicit_site()`;
  `ExplicitReturns` yields no single site. Nested branch Returns are not
  traversed by the top-level walk. Result claims can therefore be retained
  with `SourceMismatch` even while exact Return membership exists.
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
tombstoned (`b21e05ebe6`). Remaining: the NULLABLE-RESULT-ABI-S0
slices — enum arms alone move no real call.

P3 — containers/types (~250-350 lines): seed→contract→header→index in
one owner (`CompletionSeed` ≡ `ResultContractRow`, seven identical
fields); `map_read_facts` five scans → one; unify duplicated types
(AdmissionClaim⇔ResultClaim, four sibling dispositions).

Next execution row: `MIRBUILDER-GATE1-NULLABLE-RESULT-ABI-S0`.

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
- S0b (emit owner): one lowering owner consumes the row and emits
  `Call { SameModuleInstance, result: nullable }` — the
  `emit_local_lexical` lane or a sibling; decided at S0b.
- S0c (physical): `InvokeCallResultKind`/`InvokeNormalResultKind`
  nullable arm + `MirType` carrier + verifier admission per class.
- S0d (state/cleanup): `StoredLocal`/`OrdinaryObservation` nullable
  arm + a checked-null conditional-release commit.
- S0e (publication): `OrdinaryNullableHandle` role, wire name, JSON
  spelling, contract pairing.

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
