# MirBuilder app bundle — mimalloc-lite completion D0

Status: active; Required Object Forward S0 unaccepted; source-to-C connection landed.
Date: 2026-10-17
Scope: existing mimalloc-lite source-to-final-consumer series; no new lane.
Related: [CURRENT_STATE](../CURRENT_STATE.toml), [RULES](../../../RULES.md),
[final pipeline](../design/mirbuilder-final-pipeline-ssot.md),
[workstream](../workstreams/mirbuilder-inplace-replacement-current.md).

## Current position and history

Selected row: `MIRBUILDER-APP-MIMALLOC-LITE-REQUIRED-OBJECT-FORWARD-S0`.
Mode: fast for required constructor-owner preparation; allocation-only restoration Decision below.
Full prior card, decisions, original-source contracts, red classifications,
commands, pins and preparation evidence are preserved in this path at
**244c8d3f5e** and **fbc57efa14**. These are implementation commits, not just
receipt checkpoints. Older WIP/unlanded labels describe those historical points.
No archive, replacement taskboard or separate proof ledger is needed.

- **8a2d2974a8**: additive kernel/header/Array runtime boundary. Fault34 and
  typed-object33 PASS on primary and isolated HEAD+32; Array53 PASS. Existing
  DirectSlotV0 sign assertion repaired; production handle encoding unchanged.
- **1ac39278c6**: existing53-line Named Array owning-slot reference published
  unchanged. Earlier classification as unrelated was wrong; no user edits lost.
- **244c8d3f5e**: coordinated source/input/Completion/field support → finished
  outcomes → physical ABI/JSON → matching C index/flow/emitter connection.
  C-only publication was refused because old Rust lacked required numeric-check
  and Return-origin columns. Runtime and reference prerequisites are above.
- **fbc57efa14**: separate BoxShape; immutable scalar validation scan shares one
  lazy grounding fold, while EVERY loan rechecks original target, result row,
  Return census, dependencies and facts Rc. No persistent cache or new authority.

First goal milestone is verified on unchanged mimalloc-lite canonical EXE/run.
This does not complete S0 or `MIRBUILDER-FINAL-PIPELINE-v1`: fixed acceptance,
remaining integration/cutover/retirement, R6/R7, Loop, authority, parser and
nondelegating selfcompile conditions remain mandatory in the final-pipeline SSOT.
Parked lanes keep CURRENT_STATE's existing reopen conditions.

## Current Decision and live invariants

Source authority: original imported/resolved source and full original incoming/
parameter catalog, canonical result/constructor/field facts, Completion,
Home/Normal/Fault obligations and original affine Call/field packets.
Sole path: source → Facts/Recipe/Verify → existing physical writer → original
LocalSSA/FinishedBindings → BOTH independent final consumers → matching native C.
MIR type, opcode shape, class name, nonnullness, copied witness or an emptied
lowering pool never grants source, Home or execution permission.
No AST rewrite, .hako workaround, compatibility retry or second classifier.

Fresh moves only on Normal; Fault retains caller obligation. Forward keeps its
original full support and acquires/releases no owned Home. Nullable retains its
own acquired Home law. Exact original packet/result ordinal, child Rc and finished
producer remain mandatory through Birth actuals and conditional End.
All-writer residence proofs and per-hop formal/actual substitutions stay exact;
callee ordinals must be substituted, not equated to caller ordinals. Source-only
forward remains passive; no invented Main caller to an uncalled wrapper.
QualifiedObject priority, same original acquisition/current target/full root Rc,
per-hop nullability and Array-only exemption stay unchanged. Passive field
anchors grant no End. Runtime identity checks issue no source ownership right.

C retains separate seed/cycle refusal, all original Return/Provided/store/input
relations and complete flow. Field loans receive no Home: nullable scalar readers
use checked Normal/Fault paths; raw bases/method receivers/numeric moves remain
strict. The shared reverse interval distinguishes strict reads from exact
one-child-End cleanup on every suffix. Cleanup mode cannot authorize ordinary
reads or moves. Parent lifetime and complete typed End order remain mandatory.
Provider Birth Fault reclaims storage only; post-Birth store Fault uses full
shared typed discharge. No depth gate or unwinding refusal was weakened.

## Verified bounded connections

- Mixed received Birth actual → physical ABI/JSON/C: original M190 replacement
  becomes its real HandleResult Birth argument and paired residence.
- Forward field reuse, nested scalar reads and conditional child End: M190 and
  source-wire wrong-formal/missing-End/raw-nullable/early-parent-End refusals.
- Mixed/Borrowed Call and ObjectEndIfOwned: M189 native0/1/2 and borrowed0/2
  cohorts, M190 wrapper native and original source/final End mutations.
- LOCAL owning object Array: SAME source recipe/read Rc and whole holder
  construction/append/read/exit-End finishing census → array_owned_* → runtime.
  App179 executes13 original appends and4 reads. Field I64 Array residence uses
  its separate Write/Read/layout and array_append/get/set physical owners.
  Native app179 validates these two bounded cohorts. Independent physical
  Normal/Fault/negative and runtime tests complement, never replace, source proof.
- Raw FieldIndexSet V739 and guarded usize FieldWrite V734 stay verified premises;
  they do not close all LOCAL initialization; Plan Set connection is below.

No all-Array-family or every-Mixed-runtime-Fault-injection claim is made.
Straight-line generic Push, share and unsupported container mutations are not
silently activated or automatically added to this finite goal.

Selected retirements are physical: emit_receiver_nullable,
nullable_call_prior_home_unwind and Array-only residence_chain have zero exact
code references. Common receiver packet, OwnedFieldChildV1 and typed
emit_discharge/field_discharge_chain own those responsibilities. Shared lexical
begin/record, nonselected callers and prior_home_unwind_for remain live.

## Current evidence and provenance

Evidence base: `/tmp/mirbuilder-required-forward-resume` (workdisk-backed).
The commits above preserve executable owner code, contracts and validation;
temporary paths/hashes alone are not the completion evidence.

- V832a root build66.417804s; aggregate Print1, constructor/provider/Fault25,
  physical JSON33, terminal22, observation/refusal cases and merged facade PASS.
  Original three-hop provider native n=7 plus all3 allocation/all3 store Faults
  check exact-once cleanup. Original actual both-opts PASS3339.060297s.
- V834b `RUST_MIN_STACK=16777216 cargo quick-test-fast --no-run
  --message-format=json` build PASS95.460835s; existing scalar identity,
  qualified retained-loss and composed dependency tests3 PASS. Actual same
  unchanged app/both-opts source/compiled-entry/ABI/backend/mutation test
  PASS594.146438s with16MiB test stack. No check or fixture removed.
  Prior3339s and current594s are elapsed observations with different concurrent
  builds, not a controlled speed benchmark. V834a E0425 corrected before this
  build; default-stack run exit-6/5.885892s retained as non-PASS configuration run.
- CLI175 `cargo build --profile quick --bin hakorune --message-format=json`
  PASS200.620625s. CLI175/C831e/runtime808b, pure-first/no compat replay:
  canonical `--grammar-profile canonical --backend mir --emit-exe <out>
  --emit-exe-nyrt <runtime> <unchanged source>`:
  app179 mimalloc-lite compile153.819668s/native.001858s/exact existing7/exit0;
  app178 M190 compile404.371506s/native.001769s/exact existing11/exit0.
  Only the known plugin-disabled warning. M190 C resolves694 values and checks/
  emits all6554 blocks/29functions; no partial flow or publication-only claim.
- C owned16, field-return6 and stored-child96 PASS; source-wire negatives6
  refuse before artifact. Fresh interval unit build7.650422s/run.001259s PASS
  independently checks strict reads, exact per-path Normal/Fault End and refusal.
- Root-route, builder-layer, thin-shim, pointer and diff checks PASS. Ancillary
  canonical route guard previously ABORTED its redundant debug build, never PASS.

CLI175 SHA79e9ae5273009d1c59a819132f934f6837ee61a8d18ceb85344aabdbab985fe9;
C831e SHAee5d244562c8104dd080bcf20aff008f854babce17496817346644f546d350d7;
runtime808b archive SHA2b6083aac7e2d168fe8f76eb821c079900d15e69c8da0bc5eb843a05eba9222c.
Root834 SHA b3c1817d0ab518f2def34f9c044e6b8982edafaa6c6e204247a9c10cacc684eb.
Source manifests v835-cli175/v834b-root; native app178/179 build/run receipts;
v834c-actual and v835-field-interval receipts retained with commands/output.
Three trailing empty EOF lines removed at source landing; exact byte-only delta
recorded. No token/source authority change. V836 pure module moves follow below.

## Completed Plan connection — c52e4f1231

`c52e4f1231` owns the V837 Decision, source/physical connection, common raw/Plan
operand completion, replaced selected edges, focused27 PASS and guard evidence.
Original local read destinations are the Plan values; no Copy algebra or new
semantic classifier. One independently missing test reuses the original source
and actual declaration/entry preparation. Earlier test-setup/compile reds and
superseded binary disposition are preserved in that implementation commit.
Root D SHA5baa244b3449c25f219d051094b466de6824ba0fbc28ed20940d29941557a2f4;
CLI176 SHA4c6b1d87182b2279d35077b118e30229e9d39bd8d5cd75cf9dbf02f1f4643f16.
Current native/source proof: v837d-root.focused, v837-cli176.source/build and
app-v180-mimalloc-lite build/run receipts. CLI build172.282976s; unchanged
canonical app180 EXE128.194057s/native.001438s/exact existing7/exit0 PASS.
C831e/runtime808b remain matched; no compat replay. Source manifest drift zero.
No full-suite, M190-on-CLI176 or full S0 completion claim. Two superseded B/C
root binaries retired after D/native PASS:514690040 bytes, current pins/logs/
manifests retained (v837-cleanup). No new adapter was introduced.

## Remaining required same-series work

- [x] PlanSourcePublication Set connection (V837): retain exact original FieldGet locator before
  Plan descent; connect the SAME affine read/LocalSSA owner to the sole recorded
  Set writer/census at the source-aware Loop/Plan emission port. Full S0 remains
  unaccepted; next required construction/FieldWrite profile is below.
- [ ] Required field Birth/full support and remaining actual FieldWrite/
  initialization pins; reuse V734/V739, do not invent fields or zero-fill proof.
  Typed-newbox Pair has no Birth: current NoBirthZero requires fieldless class,
  causing InitializationContractMissing, and LOCAL pair writes lack selected
  prefix coverage. This is a compiler profile gap, not invalid source. Keep the
  unchanged positive, no fabricated Birth or conversion into a negative.
  The bounded restoration Decision is resolved in `cac87926f6` and below:
  the existing allocation-only typed-i64 positive retains its original New/Home
  publication point. The remaining task is source/readiness -> owned LOCAL
  stores/reads -> both final consumers -> C, not another language decision.
  Keep the same declaration/New identity, exact original stores and Completion,
  read/escape/Fault boundary; never infer initial values from zeroed C storage.
- [ ] Required fixed acceptance; retarget a negative only at its truthful
  rejected boundary. First app/native success does not replace this suite.
- [ ] Remaining named production cutover and exclusive-edge/adapter retirement;
  delete in the same series after actual caller switch/zero and required checks.

V835 fixed11 exit1/333.949828s: same4PASS/7FAIL as pinned V832 source. Three EXE
positives are Birth, Birth-param and method; fourth PASS is an empty marker.
Failures remain unaccepted:

| case | current stop |
| --- | --- |
| typed-newbox | PrefixNotCovered Body1; direct canonical app177 also reproduced |
| json-stream | borrowed-static/source-actual-unavailable |
| boxtorrent | stored-child/receiver-unavailable |
| binary-trees | nullable-input/source-missing in original Loop Body2 |
| allocator-stress | NamedArray(TextSourceMissing) |
| mimalloc-lite generic negative | required-integer-source-unproved before expected Invoke |
| untyped-field negative | before expected Invoke boundary |

The negative harness failures do not contradict canonical direct app179 PASS.
These are unchanged V832→V835 stops, not a proven clean-HEAD baseline or a full
suite success. Source errors cannot be dismissed as merely old harness behavior.

## V836 bounded hygiene follow-up

Final explicit size audit caught four800+ source files; broad route guard had
not established the full candidate size claim. All four are now repaired.
Decision (worker read-only review): raw syntax receipt/classifier moves to its
private child boundary; field receiver fixture/tests and original Instance
result-law test move to child owners; local field-read installation moves into
its existing field-read owner. Same source/diagnostics/API visibility, no new
classifier, case, fixture, callbacks or expanded semantic admission.
Field install early return combines with the same caller continue: no fallback.
New sizes754/672/572/771; all hard stops resolved, scan remains in warning range.
Worker compared original test bodies (only module-relative namespaces changed),
receipt/classifier bodies and field result control flow. Two empty EOF lines
trimmed after compile; no tokens changed. Root build PASS67.691311s;
focused raw-loop16 + moved Instance1 + local-field10 PASS. Three REDs (two moved
field receiver tests source-not-i64 and parameter receiver source-actual-unavailable)
reproduce with identical issue strings on pinned pre-move V834b. These are baseline
failures, not PASS or a new profile expansion. Total root inventory9212 unchanged.
Route/layer/thin-shim/pointer/diff PASS; two existing guard owner paths updated
without changing predicates. Ancillary headerport guard still stops at missing
port_aware_function_draft.rs (already absent at fbc57efa14); qualified-route guard
still stops at its old Handle pattern in byte-identical ordinary_new_arguments.rs.
Both are confirmed pre-move baseline failures, not claimed PASS. Current proof:
v836a-root.focused.json, v836-baseline.receipt.json and v836-red-classification.

New focused build is pinned; preserve current native/source evidence
as pre-move provenance. Pure moves use existing raw-loop/moved-test/field-read
regressions; do not rerun expensive native tests without a new concern.

## Required initialization restoration Decision — 2026-10-17

Previous goal turn: PROGRESS, task/roadmap reconciliation landed `ac2dff91ce`.
That commit retains the unaccepted delayed-publication option and CI/doc review;
its cleanup queue does not select a new implementation lane. Workstream remains
navigation, CURRENT_STATE the sole selector.

Decision: restore the already accepted allocation-only typed-i64 profile, keeping
New/Local identity and first Home publication at their original Normal cutpoint.
Read-only historical audit resolves the prior ambiguity: Pair allocation-only
source `09d2c20370`; first-profile narrowing `dc80865f01`/`7acee9c860` was explicitly
execution eligibility, not source invalidity; later `052c54518b` accepted i64
migration preserving both fixtures, implemented `89199dbc3c`. This is existing
positive recovery, not permission for new language semantics. Lifecycle reference
and constructor SSOT clarify this distinction before source changes.
The earlier delayed-New/publication proposal stays rejected/unactivated; retain
its scratch only for this comparison, never revive it as source authority.

Source authority + canonical issuer: same parser Box declaration/New identity,
instance_constructor_semantic construction source, canonical i64 fields and
hook-free destruction; exact resolver writer inventory/Local binding and
Completion own caller store/read facts. The SAME constructor source structure
must distinguish allocation-only completion from initialized-field completion.
Non-authority: allTrivial, zeroed C storage, copied witness, class/field spelling,
physical MIR shape or caller stores as constructor/destruction facts.
Fail-fast boundary: allocation-only source grants no field initial value.
Original LOCAL FieldWrite Normal alone refines exact field readiness; pre-store
RHS uses prior readiness. Missing writer/read/source/Normal dominance, unsupported
escape/control or wrong root is unavailable. Do not relabel LOCAL as formal or
turn missing initialization into an empty all-ready ConstructionPlan.

After New/Local commit, Fault releases the published PlainI64NoHook Home and
prior Homes once; it never re-enters ReclaimUnpublished. Keep existing New-to-
Local failure accounting, original store order/overwrites, no fake Birth,
no AST/.hako rewrite and no retry. Source-to-physical-to-BOTH-final-scans-to-C
readiness and complete occurrence coverage remain mandatory before activation.
Unchanged no-Birth Pair EXE/OBJ30 is required; Birth-Pair does not substitute.
CLI176 app181 remains exit1/noEXE .021610s PrefixNotCovered Body1 until repaired.

Smallest next slice: required BoxShape before semantic growth: constructor tests
are800 lines, parent770. Move the six cohesive final construction tests to a
sibling owner and source-definition lookup methods into existing object_definition.
Preserve visibility/callers/bodies, no new tests or permissions; use two bounded
commits. Pinned V837d baseline constructor25 PASS .075454s; compare after moves.
Then retain allocation-only completion/readiness in the SAME source row, keeping
complete execution unavailable until the owned-LOCAL store/read and finishing/C
consumers connect in this series. No isolated preparation green closes S0.
Required remaining initialization, fixed acceptance, retirement and full goal
conditions remain open. No deadline or nearest-app claim.

## Constructor BoxShape validation — V839

Six unchanged construction regressions now have a sibling `construction_tests`
owner; existing test file800->539 lines. The selected two-move series also moves
six source-definition lookup methods into existing `object_definition`, keeping
bodies, effective visibility and callers unchanged. Birth/plan selection remains
in the parent. Worker exact-body review PASS; no new tests or semantic admission.

One final-series `RUST_MIN_STACK=16777216 cargo quick-test-fast --no-run
--message-format=json` build PASS66.160698s. Pinned constructor filter
`instance_constructor_semantic:: --test-threads=1` PASS25/0 .073404s; pre-move
same25 PASS .075454s, names identical after module-prefix normalization. Root
inventory9213 unchanged. This validates the final two-move series, not a separately
compiled intermediate commit. Root-route/layer/thin-shim/pointer/diff PASS.
Final source sizes parent684, tests539, construction_tests264, object_definition450.
Root pin SHA cb045e990f8babc9f8daf840207079b344ef0a2eebed1ff5b56758838a9269c3;
source/build/focused receipts `v839a-root` retained for next source comparison.
Current CLI176/app180/app181 remain pre-move native/frontier evidence; no new
EXE or S0 completion claim. No adapter introduced. Definition move is the second
commit of this behavior-neutral series; initialization connection follows.

## Temporary artifact disposition

Completed history stays in244c8d3f5e/fbc57efa14. Retain current CLI175, C831e,
runtime808b, latest root test pin, original source-issued wire and necessary
manifests/logs until remaining series acceptance/source comparison no longer
needs them. Source landing removed the need for temporary staged source copies;
retired two exact staged source copies after verifying244c8d3f5e bytes, plus two
identical V832 root copies after terminal checks. Reclaimed514570191 bytes;
original baseline, current pins and all evidence retained (v836-cleanup receipt).
The owned isolated runtime checkout/two superseded copies were already removed;
shared Cargo target and actual evidence retained. `v751-unlanded-received-birth-source.rs`
is historical rejected synthetic scratch, not an authority or live adapter;
keep only while its failed-cohort comparison is required, never revive its gate.
No selected old edge is kept solely to await whole-app success. Shared owners and
parked lanes retain their existing conditions. Full goal remains active.
