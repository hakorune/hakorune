# MirBuilder app bundle — mimalloc-lite completion D0

Status: active; Required Object Forward S0 unaccepted; source-to-C connection landed.
Date: 2026-10-17
Scope: existing mimalloc-lite source-to-final-consumer series; no new lane.
Related: [CURRENT_STATE](../CURRENT_STATE.toml), [RULES](../../../RULES.md),
[final pipeline](../design/mirbuilder-final-pipeline-ssot.md),
[workstream](../workstreams/mirbuilder-inplace-replacement-current.md).

## Current position and history

Selected row: `MIRBUILDER-APP-MIMALLOC-LITE-REQUIRED-OBJECT-FORWARD-S0`.
Mode: fast for required LOCAL initialization/Return readiness connection; Decision below.
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
  keeping complete execution at InitializationContractMissing. V843 now covers
  both LOCAL pair writes; Return field-value readiness is the next source stop. This is a compiler profile gap, not invalid source. Keep the
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
CLI176 app181 was exit1/noEXE .021610s PrefixNotCovered Body1; V843/CLI177
below supersedes that selected source frontier, not the historical suite result.

Required BoxShape is complete: test-owner split `da029c5c36` and
existing-definition-owner move `ea70a42bd2`; V839 evidence below. No semantic admission changed.
Smallest next slice: retain allocation-only completion/readiness in the SAME source row, keeping
complete execution unavailable until the owned-LOCAL store/read and finishing/C
consumers connect in this series. No isolated preparation green closes S0.
Required remaining initialization, fixed acceptance, retirement and full goal
conditions remain open. No deadline or nearest-app claim.

## Landed initialization preparation

- `da029c5c36` / `ea70a42bd2`: constructor tests and existing definition lookup
  ownership split. Exact bodies/visibility/callers preserved; V839 constructor25
  PASS, root9213 unchanged. No semantic activation.
- `1a29de4ce4`: explicit allocation-only completion in original construction
  source, retained in the same no-Birth row. Both complete-plan fences stay
  InitializationContractMissing; source-discarding wrapper physically deleted.
  V840 build/focused27 PASS, no new tests or adapter.
- `93129de754`: same Rc moves through New resolution/candidate -> common
  local/result claim core -> existing Home consult -> Normal local Home.
  Overrides unavailable; join checks original source/acquisition, consume/rebind
  invalidates with Home state. Missing-Birth diagnostic order unchanged.
  V841b build78.750103s, focused69PASS/0FAIL, root9213 unchanged and guards PASS.
  V841a module-path compile error corrected; no waived red. Full source mapping,
  commands, pins and bounded evidence remain in that implementation commit.

Current preparation pin V841b SHA5b463236a67703580dd1e7bcec8e4d813d8ebb795c3e6558fc6cde43e841af25.
Keep V839/V840/V841 source/build/focused receipts while the following connection
needs comparison. Existing CLI176/app180/app181 are pre-change native/frontier
provenance, not current-source EXE evidence. No stage/full-S0 completion claim.

## LOCAL writer/RHS preparation — V842

Previous turn PROGRESS: source carry `93129de754`, audit task reconciliation
`eee537fda3`; audit candidates remain unselected. Same accepted restoration
Decision and required full initialization series, not a new lane.
Current scope: original construction completion retains declared names with
canonical fieldrefs; source consult retains original-writer inventory loan
instead of projecting only an expression; existing field-write qualifier can
require whole RHS I64 (plain/reference/Add as well as Sub/Div). Old me effect
caller retains its existing demand. Existing constructor and installation tests
are extended; no new test/fixture/physical writer or admission permission.
Owner README/current card describe these contracts. Worker diff review PASS.
Build PASS73.822772s: RUST_MIN_STACK=16777216 cargo quick-test-fast --no-run
--message-format=json, jobs4. Focused constructor25 + existing whole-expression
installation1 (11 RHS x batch success/failure) + original-writer identity/foreign
inventory1 + ordinary-New23: **50PASS/0FAIL**. Root inventory9213 unchanged.
Wrong string/Bool/null/unknown-reference RHS refuse before field staging; checked
positive and final-batch rollback retain the same original plan. Existing effect
caller remains unchanged. Root-route/layer/thin-shim/pointer/diff PASS.
Source drift zero; root SHA286de9c9242f609e9237bf2530d6933568f13f7294773992c0cd7fd6de015d11.
`v842a-root` source/build/focused receipts and pin retained until the frame/readiness
connection no longer needs comparison. No new adapter or obsolete physical owner
introduced; no fabricated deletion. Completed preparation history compressed to
its real source commits above, retaining current contracts/reds and required pins.
Latest available CI35681768498 remains historical d4168473d5 failure, not evidence
for current source. Existing CLI176/native/frontier provenance is unchanged.

The normalized assignment frame and source Normal readiness now land in V843
below. Its local source progress does not activate physical/publication support.

## LOCAL source frame / Normal readiness — V843

Previous goal turn PROGRESS: `2da3f845ef` supplies declared field lookup, retained
original writer loans and whole-I64 RHS demand. Worker reviewed this mapping and
current diff from original source to mandatory final consumer boundary. One
module/reexport mismatch found before build and corrected. V843a build PASS
104.486474s/focused50PASS. Added two actual branch-join refusal cases to the SAME
existing test, then V843b build PASS56.118552s/focused50PASS (root9213 unchanged).
Command remains16MiB-stack cargo quick-test-fast --no-run --message-format=json,
jobs4. Constructor25/ordinary-New23/expression transaction1/original writer1;
Normal store/overwrite, type/read/compound refusal and unequal branch frames
PASS. Worker review and root-route/layer/thin-shim/diff PASS, no waived red.
Root B SHA07f707a03585cf4743f6651444d7ce004aa086b4f30f1361549b238ff8b1e977;
v843b source/build/focused receipts retained. Superseded A test binary retired
only after B PASS and exact source comparison (only test table differs), no live
process; reclaimed258155432 bytes. A logs/manifests remain, B/current baseline
and native pins kept (v843-root-disposition). No temporary adapter introduced.

Decision: extend the existing assignment frame/slot into CheckedFieldWriteFrame,
with common original source/prior Homes and explicit FormalBorrowed vs OwnedLocal.
No parallel optional frame or compatibility type alias. A sealed LOCAL source
matches exact candidate acquisition/destination/SAME constructor Rc and source
field, original writer inventory, same resolver assignment/target/receiver and
LOCAL binding. Existing me-only receiver attribution/global field veto remains.

Scope: existing writer/candidate consult, frame/whole-RHS qualifier, expression
slot/consistency/refinement/lifecycle formal consumer and reexports; two cohesive
children for LOCAL observation and branch-local readiness; old scanner/read gate
and existing brand claim regression, owner README/card. No fixture or new root
test. Attach frame before approve; validate old-readiness RHS and final field
batch, commit, then record latest Normal field writer. Every store remains in
expression inventory. Exact frame joins, overwrite order and consume/rebind state
loss are required. Tests cover renamed stores/overwrite/source identity/formal
refusal and wrong type/missing field/read-before-write/unwritten field/compound
refusal; positive read-after-write and physical Normal dominance still owed.

The common lifecycle collector retains ALL frame occurrences and explicitly
rejects LOCAL with owned-local-field-write-consumer-pending. Its owner is the
existing lifecycle/selected FieldSet connection; retire this bounded pending
refusal only when source->sole writer->BOTH final consumers->C closes. Constructor
complete eligibility and publication remain unavailable; no broad formal bypass,
New/Home delay or ReclaimUnpublished after Local commit. Keep fixed acceptance
and unchanged Pair EXE/OBJ30, physical Fault/cleanup and required retirement open.

## Current source frontier — CLI177/app182

CLI177 `cargo build --profile quick --bin hakorune --message-format=json`
PASS191.121111s. Same WIP source fingerprint as root B; source drift zero.
Binary SHAa0985b4e58cf23152a142b44df58e3886c79e63dcead4f4f30fea51984958fc7.
Unchanged `apps/typed-object-newbox-min/main.hako` SHA80b07fe145e5fe61b2d326620fd5b1929c452124e3fe5c5528d4aeb96be54a7b,
canonical direct `--backend mir --emit-exe`, pure-first/no replay, C831e/runtime808b
pins unchanged. App182 exit1/noEXE .092705s:
`ordinary-new/object-return/normal-projection-exit: ReturnValueNotCovered Body3`.
Prior CLI176/app181 stopped PrefixNotCovered Body1. Original LOCAL stores Body1/2
now pass the source walk; Return's value proof is still unavailable. This is a
verified first-stop advance, not EXE/native success, fixed-suite success or S0
completion. Receipts app-v182/v843-cli177 retain argv/environment/pins/timing.

Next: keep exact prior-ready frame in LocalFieldReadRequest and staged descriptor
instead of reducing it to bool. Existing ExpressionFieldRequirement carries that
request; source value provenance must distinguish existing all-writer proof from
LOCAL original-ready frame before global field_candidates/formal lookup. Keep
me-only global veto. Return observe_terminal_scalar_source currently uses the
old field_is_integer bool / initialized_integer_field complete-construction path;
thread the SAME request/ready frame into terminal field staging, not a true bypass
or Return-specific inference rule. Then sole checked FieldSet/Get writer, physical
Normal/Fault/cleanup coverage, both final scans and C must consume these exact
sources before constructor/publication profile activates. No second classifier.

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

## V844 selected source-ready Return connection (source proof only)

Same S0 responsibility: retain the exact branch-local successful LOCAL write
frame through direct field requests, terminal source relations and staged field
value provenance. Terminal Declared/Numeric/OwnedLocal sources stay distinct;
OwnedLocal never inherits the borrowed Numeric installed-Home exemption.
Declared local-read descriptors retain the same frame. Nested paths and whole
expression numeric loans cannot discard LOCAL readiness into old formal proofs.
Scope: existing home prefix/terminal source owners, ordinary-New field issuer,
read ledger and existing brand/terminal regression owners; no new lane.
Both final physical consumers/C and constructor activation remain owed. The
existing lifecycle and new owned-ready physical-consumer pending fences stay
closed until the original store/read Normal dominance and Home/Fault proof are
connected. Focused positive/negative regressions precede fresh CLI/source probe.

Read-only worker confirmed the next common owner mapping: retain separate
construction source eligibility through NewLocalCommit (execution eligibility
cannot substitute); normalize the existing selected checked FieldSet owner around
common frames, with FormalBorrowed guard operands and OwnedLocal source-qualified
RHS kept distinct. Use lifecycle prefix -> binding group -> finished projection;
mandatory membership must include all field_write_frame occurrences. BOTH final
scans must corroborate store Normal landing dominance of each exact-frame read,
plus original Home and Fault cleanup. Existing FieldSet C representation is the
candidate consumer; no Return-only store validator or new schema is selected.
This review is a connection map, not physical/native acceptance.

V844 live source evidence: rootC build110.187822s; focused76 PASS (23 brand,
25 constructor,22 terminal,2 field-batch,2 physical field projection,2 existing
installation/opaque-boundary). An initial wrong filter selected0 and was rejected,
then corrected using the pinned binary list; zero tests is not PASS.
CLI178 build191.334185s, unchanged Pair app183 .089015s/exit1/noEXE now stops
`[ordinary-terminal-result/raw-field-read-reentry]`, previously CLI177/app182
`ReturnValueNotCovered Body3`. This proves source-ready Return advances to
lowering, not completed initialization/physical execution.
CLI178 SHA ce773ab2adc4d347a14c1c73097d9a730f44ea562119b29b2529b562ef0e7372;
source manifest v844-cli178.source.json, same pinned C831e/runtime808b and unchanged
Pair sourceSHA above. RootD adds only method visibility narrowing (new private
interface warning) and source-negative Return regression cases; no execution
behavior change. This CLI evidence describes its exact pre-RootD source manifest.
No fresh mimalloc/full-suite or current CI-green claim. Next: common LOCAL
field-write/read physical path and both final consumers, with construction source
Rc retained through affine commit; keep activation fences closed until connected.

RootD final build56.770264s, same76 focused PASS including uninitialized-field,
unwritten-other-field and unequal-branch Return refusals in the existing case.
Test count remains9213; no new standalone test/guard/fixture. New warning delta
against V843b is zero. A/B builds101 were this implementation's API/closure
integration errors, corrected before C/D PASS; not baseline waivers.
RootD pin v844d-root.test-binary SHA
200d5b36dc5f59cfff51be2614d6e2549fe079e61c8ce16fe8716f64ee6e01b0.
Root-route/layer/thin-shim/pointer/diff guards PASS. Source issuer normalization
physically replaces the former live inline terminal staging; read-only probe
uses the same issuer with temporary staging, no second source classifier.
Temporary disposition: identical incomplete-filter rootC pin retired after same
SHA and no live executable check; full rootC `.extra` pin, rootD, CLI178 and
matching C/runtime retained for pending physical connection and regression.
All commands/source manifests/receipts remain under the evidence base; commit
owns code/contracts/results, not temporary SHA alone. Shared constructor and
borrowed-field physical owners remain live; no old-edge-zero or S0 completion
claim is made. This source slice lands while its enclosing series stays open.
