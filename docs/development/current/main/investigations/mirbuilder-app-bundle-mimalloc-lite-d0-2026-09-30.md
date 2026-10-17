# MirBuilder app bundle — mimalloc-lite completion D0

Status: active; Required Object Forward S0 unaccepted; source-to-C connection landed.
Date: 2026-10-17
Scope: existing mimalloc-lite source-to-final-consumer series; no new lane.
Related: [CURRENT_STATE](../CURRENT_STATE.toml), [RULES](../../../RULES.md),
[final pipeline](../design/mirbuilder-final-pipeline-ssot.md),
[workstream](../workstreams/mirbuilder-inplace-replacement-current.md).

## Current position and history

Selected row: `MIRBUILDER-APP-MIMALLOC-LITE-REQUIRED-OBJECT-FORWARD-S0`.
Mode: design_stop for remaining required initialization; V837 Plan connection verified.
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
  Read-only V837 follow-up confirms fixed positive ownership in final-pipeline
  SSOT (fixed-suite1399+, historical i64 Pair EXE exit30 at2023+). Existing
  constructor-lifecycle SSOT498+ accepts fieldless NoBirthZero/fully initialized
  Birth, with no accepted partial-storage/late-LOCAL-initialization execution
  profile. Resolve that bounded owner contract before code: same declaration/
  New identity, exact original stores and Completion, read/escape/Fault boundary;
  never infer initial values from zeroed C storage or caller syntax guesses.
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

## Next required owner decision

Same S0 series remains open; Plan's selected generic read/Set edges have been
replaced, shared raw arms retained. Next mode is design_stop for the existing
required initialization/FieldWrite obligation, not a new feature or parked lane.
Current CLI176 app181 replay of unchanged typed-newbox remains exit1/no EXE,
.021610s, PrefixNotCovered Body1 (same prior stop). This is unaccepted evidence,
not a negative success or a whole-suite rerun.
Source authority + canonical issuer: existing instance_constructor_semantic
object_for/construction_for from the same Box declaration, original resolver
assignment inventory and Completion. Non-authority: C allocation zeroes,
caller syntax guesses, invented Birth or class-from-use.
Fail-fast boundary: current constructor contract lacks the required nonempty
no-Birth initialization execution profile; LOCAL writes also lack coverage.
Smallest next slice: resolve the profile against existing lifecycle/publication
rules in the constructor owner before code, including exact field readiness,
read/escape/control and outer storage Normal/Fault cleanup. Worker partial-slot
proposal is investigation input, not an accepted permission to publish incomplete
objects. Preserve fixed Pair positive/exit30 and all remaining S0/full goal duties.

## 2026-10-17 task organization and pending contract

Previous goal turn: PROGRESS (`c52e4f1231` production connection/native evidence).
User-requested roadmap task organization is integrated into the existing
workstream dependency table and existing structure queue; no new taskboard or
mandatory cleanup gate. Those two owner docs are in scope because they own
roadmap status and queued BoxShape selection. Fixed11 was measured10/17 on
CLI175 (4PASS/7FAIL), not unmeasured since9/30; app180 is separate evidence.
Gate1 includes R6/R7/Loop/final convergence; later Gates2–4 retain reopen rules.

Initialization review found no source rule giving scalar fields default zero;
local-binding UninitializedSlot grants no field permission. The worker's local
construction-continuation option would retain allocation/store source order,
keep only an unpublished storage obligation until the first all-ready Normal
cutpoint, and use existing failed-construction cleanup before publication.
This is **unaccepted**, not a completion or emission permission: it changes the
usable-result boundary beyond New and must be reconciled with the existing
no-new-language-semantics rule in the selfhost migration-order owner. The
normative lifecycle reference is unchanged; the tentative amendment was removed
from that entry and retained only as v838-local-initialization-unaccepted-proposal
scratch for comparison. No source/build/fixture/runtime activation occurred.
The next owner decision must distinguish restoring the already accepted Pair
profile from introducing new language meaning; keep construction eligibility,
field readiness, first Home publication and Normal/Fault cleanup explicit. Do
not promote partial storage to an existing complete ConstructionPlan. S0 and
all finite goal conditions remain open; no deadline or nearest-app claim.

Docs-only validation: worker roadmap/scope reviews, new relative links, pointer
and diff checks PASS. No Cargo/test rerun. Branch CI listing's newest available
run is35681768498 (2026-09-22,d4168473d5), historical macOS temporary-input
CAPI lifetime failure; provider step PASS, other listed skips are not evidence.
This is not a current-revision result or current CI green; parked platform
ownership remains unchanged. No workflow dispatch or cancellation performed.

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
