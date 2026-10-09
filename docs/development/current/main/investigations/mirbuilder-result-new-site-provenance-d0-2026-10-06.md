# Result-new source-site provenance D0

Status: Checked Integer Return source/finishing/C execution closed; full S0 incomplete
Date: 2026-10-06
Scope: MIRBUILDER-GATE1-MIXED-RESULT-ORIGIN-D0; prior provenance/trace receipts
Related: CURRENT_STATE.toml; RULES.md;
  mirbuilder-artifact-lifecycle-site-diagnostic-s0-2026-10-06.md;
  mirbuilder-gate1-callable-loop-string-indexof-s0-2026-09-27.md.
## Current production frontier and historical receipts

Selected production caller: unchanged apps/mimalloc-lite/main.hako,
HakoAllocHeap.reallocResult/2. Its8 Result construction sites are3 allocateResult
and5 reallocResult. Retained claim census is9 including one HakoAllocHandle;
Result prefixes6covered/2uncovered (historical8/0 no longer authoritative).
302/306/310 early null-child Result exits prepare;315 Body4.IfThen0.Value and
318 Body5.Value still PrefixNotCovered(Body3 local replacement=me.realloc).
The latter source call has8 exact returns:5literalNull,2resize forwarded formal
and1allocate call local (with multiple source ancestry paths). It requires mixed
Null/Fresh/Forward input child handling, not fresh-only acquisition.

Historical pre-3388a8937c probe stopped at artifact-unowned-lifecycle-site, raw Result birth51
block368/instruction4 or57 block369/instruction3 (iteration-dependent samegap).
Latest unchanged-app probe at2e56 (receipt3388a8937c) refuses SourceOnly object
actuals at borrowed entry, with no EXE; exact evidence appears below.
MIR JSON unsupported Invoke lane remains a designed negative,
not alternate production acceptance. Source/app rewrites, AST fallback/retry,
unused-method pruning and class-only ownership upgrades were rejected.

Completed diagnostics and pre-anchored design history are compressed here;
Git retains exact contracts, red classifications, tests and logs:
- Closed diagnostics/Facts/composition: d877773df7,4ca8b7cd2d,8915833d44,
  08e11449ad,7c2242c277,dfad0a4980,3a5a96b299,a6b57ccebc,0383a32420,
  253ad3733a,b6657f6f87. Git owns evidence; sole fixpoint/body, ArrHolder refusal/
  RichHolder admission, no pending ownership-hole/input consumption and
  malformed-source Dead-before-Waiting remain current.

Input-law counterexamples remain current: ownership.md declaration-only demand;
apps/mimalloc-result-contract-proof/main.hako11–12 reads alloc.handle/same.handle,
then22–24 still uses those owner objects. They are not owning roots to move.
Full allocator_facade_box.reallocResult78–89 has borrowed unannotated input,
scalar result observation/counter effects and relay, no consuming destination.
Borrowing this support must preserve original input Home and aliases.
Source facts do not prove Home, acquired field, live anchor, runtime outcome,
constructor Normal/Fault cleanup or physical publication. Gate1 incomplete;
stored-child sibling and Gates2–4 parked; final selfcompile/acceptance still owed.
## PENDING-COMPOSITE-RESULT-D0 Decision (2026-10-07)

Accept completed fresh outer + exact Null/OwnedFresh/BorrowedFromFormal child
relation; reject pending ownership-hole completion as the unchanged-source path.
Evidence: apps/mimalloc-result-contract-proof/main.hako11-12 passes alloc.handle
and same.handle, then22-24 reads alloc/same/moved. Those reads are handles,
not owning roots; field move-out would require a separate parked law and alter
later reads. Facade78-89 supplies an ordinary unannotated formal and contains no
consuming destination. Delaying consumption until local result assignment would
still violate declaration-only parameter demand. Preserve input Home and infer
an anchored result lifetime, not a consuming parameter.

Source authority + canonical issuer: sealed source return outcomes, existing
constructor/field destination and Home-prefix/completion verification. The
language result law is clarified in ownership.md composite anchored results,
and exact construction residence teardown in lifecycle.md Fields and ordering.
These are target semantics only; current owning-only child schema is unchanged.
Non-authority: source origin sets, worker proposal, class-wide payload type,
pointer equality, policy release, local assignment or runtime representation.
Fail-fast boundary: acquired child requires verified Home transfer; borrowed
child requires exact original support and lifetime; neither can impersonate the
other. Complete outer publication and Fault/drop obligations must be verified.
Smallest next slice: MIRBUILDER-GATE1-RESULT-ORIGIN-WITNESS-S0 — retain immutable
source witness DAG in the SAME result Facts draft/fixpoint/product. Multiple
same-origin source paths and formal-derived null provenance must survive.
Non-claims: no input move, field store/cleanup expansion, runtime discriminator,
anchored residence receipt or production EXE from this observation-only slice.
## Closed result-child source prerequisites (historical)

`RESULT-ORIGIN-WITNESS-S0` closed at de7f6fbfb4: the SAME result Facts
retain branded Null/Fresh/Formal leaves and exact call ancestry.
`COMPOSITION-BOXSHAPE-S0` closed at b6657f6f87; `CHILD-SOURCE-RELATION-S0`
closed at b8cda4e125; source preparation closed at 0383a32420.
Those commits own exact tests and original decisions. The source relation
alone granted no Home, anchored lifetime, cleanup or EXE. The current
315/318 and facade obligations are stated above and in the outcome sections
below; stored-child sibling and Gates2–4 remain parked.

## OUTCOME-SUPPORT-D0 decision / MIRBUILDER-GATE1-CALLEE-RETURN-OUTCOME-D0 selected

Decision (2026-10-07): resolve destination authority as a per-construction
refinement issued by the existing instance_construction plan owner. Keep the
base owning-only plan, field_demands, ordinary parameter ABI and class-wide
owned_field_children inventory unchanged. The generic source rule is now
normative in ownership.md, mirrored by lifecycle.md: exact ClosedCallable
returned-New, no overrides, unique nonweak typed user-object field store,
original Birth Parameter and exact ObjectFieldStores/formal/class correspondence.
The refinement permits separately verified Null/OwnedFresh/BorrowedFromFormal;
it does not establish acquisition, support, installation or cleanup. Owning-only
permission must reject a borrowed actual. No name exception or silent inventory
reinterpretation is allowed.

Read-only worker reviewed the current source and confirmed the split. The new
finding changes the next action: Fresh acquisition cannot jump from Heap.allocate
source witnesses directly to replacement's Home. Heap.allocate directly returns
me.small_page.allocate and me.medium_page.allocate (source207/211), while realloc
returns a received replacement local. The current terminal returned-source
vocabulary has Construction/Home/Handle/Null but no compositional object-call
return contract. TerminalI64CallReturn is scalar. PrefixLocalFlow exposes direct
New acquisition only. begin_nullable_call_emission's same-class claim/commit
search determines canonical identity, not exact acquisition or returned-path
transfer; reusing it as acquisition proof would accept an unrelated sibling.

Source authority + canonical issuer: existing source call witness DAG and exact
selected targets; source Home-prefix/terminal verifier co-sealed with Completion
and exact result-position claim. ConstructionPlan's private refinement owns
field permission; the later same-plan physical owner discharges obligations.
Non-authority: class-wide claim lookup, NullableObject eligibility, pointer/tag
identity, ordinary assignment, emitted artifact success, or immutable provenance
alone. Mixed Facts get(None) must remain None.
Fail-fast boundary: an absent leaf lifecycle claim, missing/failed exact callee
completion, wrong returned exit, mismatched target/source batch, unverified call
Normal/Fault transfer, expired support or owning-only borrowed destination leaves
execution unavailable. Never retain only the convenient Fresh/Null subset.
Smallest next work: CALLEE-RETURN-OUTCOME-D0 fixes the object returned-call bridge
for BOTH direct returned receiver-field calls and returned received locals. Cover
pure fresh/null call results first; compose their exact original acquisition and
frame transfer/cleanup, without admitting the mixed borrowed realloc result.
Once its issuer, bounded mapping and acceptance are fixed, implement
CALLEE-RETURN-OUTCOME-S0; do not wait for downstream physical green to construct it.
Non-claims: no executable residence refinement, Home acquisition, local mixed
sum, anchor lifetime, new physical tag/ABI, EXE or final migration completion.

Required next acceptance: generic pure-fresh and fresh/null direct-call relay,
local received result returned once, exact Null leaf without Home, same-class
unrelated claim cannot satisfy a missing leaf, wrong/foreign callee and wrong
source exit reject, unavailable completion rejects, transferred Home removed only
from Normal return obligation and retained on applicable pretransfer Fault,
no duplicate return/release and no forwarded borrowed parameter treated as Fresh.
Use exact terminal/completion/claim identity, not merely runtime result equality.
The unchanged Heap.allocate and page.allocate sources must exercise the true
returned-call ancestry. realloc's mixed relation and the real315/318 construction
stay fail-closed until outcome/anchor/destination and physical consumers complete.

Remaining required sequence: returned-call acquisition/completion bridge;
explicit constructor refinement consumer; mixed local outcome flow and original
formal support/lifetime; field Normal/Fault transfer and relay cleanup; sole
physical outcome transport/publication; unchanged production EXE and full goal
acceptance. These are prerequisites of the existing Gate1 task, not new optional
audits. Stored-child sibling and Gates2-4 stay parked.

Historical decision validation and terminal-value owner relocation are at
41f07cfa90; they granted no acquisition, publication, or EXE. The accepted
handoff below supersedes the historical interface brief.
## MIRBUILDER-GATE1-CALLEE-RETURN-OUTCOME-S0 accepted mapping

Decision: choose the immutable gate reviewed against current issuer/rootcleanup
source. Keep original RootHomeExit homes and shared Completion unchanged; add an
exact return-handoff obligation. After retain_completion_index retains all owners,
its private owner co-seals per-(owner,exit) NormalReturnDisposition from exact
source call/outcome ancestry, same leaf successful lifecycle claim and callee
Completion. One ledger normal_exit_projection(owner,exit) owns readiness and the
Normal cleanup delta; old no-obligation exits project identically.

Source authority + canonical issuer: existing result witness product (no new
scanner/fixpoint), canonical terminal source walker, existing source claim and
completed-index co-seal owner. Typed object-return source hook is distinct from
scalar terminal_call; received states retain exact acquisition CALL site. Direct
return has Return destination and no local omission. Local return obligation
names exact received binding/acquisition; only sealed projection omits it.
Non-authority: passive class/witness alone, raw RootHomeExit/all_exits_ready,
ordinary assignment, physical green or a different same-class claim. Do not
mutate/rebuild Completion Rc or omit a Home before successful co-seal.
Fail-fast boundary: absent or mismatched target, source argument/receiver, true
exit, lifecycle claim, completion or source Normal/Fault disposition records
unavailable and blocks preparation/publication. No borrowed/mixed outcome is
sealed in this pure-owned slice; mixed get(None) remains None. Unresolved
registered obligations reject artifact/SourceCompleteAtFinalization even where
older generic branches would otherwise skip root validation.
Smallest next slice: CALLEE-RETURN-OUTCOME-S0 implements both direct pure-owned/
nullable object-call returns and exact received-local return obligations, final
source co-seal and ONE Normal exit projection. Switch existing root_home_exit_is_complete,
prepare_root_home_exit and validate_root_home_exit readiness/expected-homes to
that projection; record/emission progress uses the same prepared result. Retain
new terminal obligation rows in child contracts. Fault uses original priorHomes.
Non-claims: mixed realloc support, anchored field installation/lifetime, new
physical runtime carrier, EXE or entire migration completion. New source claims
still require the independent physical owner to corroborate emitted call/Return,
acquisition and cleanup before publication.

Bounded paths: source terminal value owner/helper, received-local flow/scanner
and canonical terminal/Completion forwarders; existing result Facts typed loan
helper (no outcomes reanalysis); co-seal child terminal retention; private
completion-index/ledger handoff verifier and Normal exit projection; root_home,
root_call_entry if its preflight reads projected homes, emission_prepare and
root_validation readiness/finalization; focused new source/handoff tests and
required guard registration; relevant owner README, this card and pointer.
Near-cap issue797/issuer796 receive delegation only within hard800. If a needed
owner split is unavoidable, perform a separate bounded BoxShape before growing
it; no compressed lines or combined semantic/refactor implementation.

Focused acceptance: direct fresh and fresh/null relay; received local return
with unrelated sibling Home; no-receipt cannot omit, exact sealed local removes
ONLY returned binding, direct return identity, Fault priorHomes unchanged,
final validator uses same projection; same-class sibling/foreign target/wrong
exit/missing successful claim or completion reject; borrowed/mixed/unsupported
receiver/rebound source reject; source-complete cannot bypass unresolved handoff.
Unchanged Page.allocate->Heap.allocate207/211 true ancestry must be exercised;
realloc's returned replacement source follows allocate, while mixed me.realloc
and real315/318 stay outside this slice. Existing focused33 plus selected new
positive/negative pins, source-inclusive check, scope/pointer/diff/rustfmt/caps;
fresh CLI unchanged production frontier probe for semantic attachment. Compare
exact source/Normal/Fault evidence, not result equality alone. Build prerequisites
are the fixed mapping and acceptance above, not future green or emitted commits.
CALLEE-RETURN-OUTCOME-D0 closes this interface; select S0 fast and construct it.

Verified comparison source prerequisites (2026-10-07): `34799a210e` links exact original Integer child source to SAME Const append; `d0b429e518` links exact original Binary/operator/ordered operand sites and original canonical static envelope references to SAME completed Compare, once per actual append including two borrowed operands. These are source/append observations, not final SSA lineage or execution permission. Source-loan preparation precedes both children; missing entry, operator/owner/contract drift, Arithmetic completion and duplicate append refuse. Final regression: 200 passed/0 failed, source check16.19s, qualified-route scope/pointer/diff PASS, CLI52.58s. Fresh unchanged mimalloc probe rc1/noEXE at Heap.allocate33 Body1.IfThen0.Value -> Page.allocate26 source-only-object-actuals; alias physical ABI still refuses borrowed-use/unproved-copy. Next Decision: selected checked Compare must consume its post-Normal Bool via SAME-function dominating identity/Copy, without operation replay or legacy failure recovery. Function-owned state capture/reset/restore and independent Fault-no-Cond/finished bindings remain required. Final Const/alias SSA lineage, LE source/physical admission, guarded outgoing actuals and Result315/318 remain owed; full S0/Gate1/goal incomplete. Checked Compare Bool reuse checkpoint (2026-10-07): SAME entry Weak and original Compare Rc own selected GT/direct-if identity or reachable dominating typed Copy, checked before cache/pin and without replay/recovery. Successful sealed Branch observations join mandatory existing lifecycle bindings; SAME FinishedBindings maps Compare/Copy/Branch, and independent final/source consumers check coverage, identity, Bool dataflow/order/dominance. Focused46/0, regression199/0, check18.53s, scope/pointer/diff PASS; fresh V4 source JSON/object/link/runtime7/7: Integer Normal, Null/Bool/Object Fault, once-only Compare. CLI56.67s; unchanged mimalloc rc1/noEXE at SAME Heap.allocate33 Body1.IfThen0.Value -> Page.allocate26 source-only-object-actuals. Required next: original Const/alias final operand correspondence and LE/guarded outgoing actuals; Result315/318, full S0/Gate1/goal incomplete. Existing uncommitted unrelated semantic/profile/config work preserved. Targeted Const correspondence checkpoint: SAME source-literal Rc shared by ledger/function reuse; original identity or dominating typed Integer Copy before cache/pin; Result-capable canonical compare finalizer, exact ordered binary/site/raw-child to final operand, mandatory existing FinishedBindings and independent final-source checks. Stage literal consumer map, validate entirely before publishing; failed-handoff leaves consumers unpublished. Focused10PASS/0, real-source literal2PASS/0, dominated-set once-only pin1PASS/0; current broad311PASS/7FAIL/4ignored. Reconstructed predecessor preserves all nonselected WIP (21 selected source differences, independent worker/hash audit), baseline306PASS/8FAIL/4ignored: SAME seven panic/diagnostics before/after, stale Comparecount2 pin repaired. Seven are preexisting unresolved S0 debt, not goal acceptance; comparison logs predecessor-borrowed/current-borrowed-comparison and manifest/baseline-comparison JSON in /tmp/hako-compare-literal-* and /tmp/hako-literal-predecessor-manifest.json. Fresh CAPI object/link/EXE7/7 Integer Normal, Null/Bool/Object Fault; freshCLI56.63s; unchangedappSHA33d3e8b9 first stop Heap.allocate33 Body1.IfThen0.Value -> Page.allocate26 incoming source-only-object-actuals, rc1/noEXE. Scope/pointer/diff/caps PASS. Selected Const checkpoint closed, no full S0/Gate1/goal claim; alias/formal final operand, LE, guarded actual, result315/318, seven predecessor reds and full acceptance remain owed. Carrier source loan49a65a6237 and production wiring3a7b4e58fa closed: SAME original Rc/site/side and FinishedBindings, direct source-rooted carrier Copy, exact ABI coordinate/side; source drift/alias/physical negatives and C12cases PASS. Complete evidence retained in these commits. Seven baseline nullable/field debts and two compare baselines remain owed; unchanged mimalloc incoming-row frontier remains.  Next incoming prerequisite Decision (read-only worker integrated): close original Heap.allocate borrowed use contract; do not promote SourceObject or create a receipt-only detour. Actuals early object-source branch is selected because caller is absent from prepared.definitions; SourceObject reason=None means ordered source arguments complete, not opaque executable actuals. requireExecutable remains correct at real entry adoption. Source authority/canonical issuer: original CompareOperand classifier + canonical LessEqual NormalInteger envelope, exact source condition/ordered children and original Normal-to-actual relation, SAME complete15-caller incoming inventory/source_seeds/fixpoint; existing QualifiedStaticArgumentSource and target input contract subsequently close Heap static use. Non-authority: object-result qualification, complete ordered args, empty opaque_actuals, runtime success, caller filtering, inferred declaredI64, or positive-number inference. Fail-fast: all15 callers/vetoes retained; wrong site/operator/side, preguard/unrelatedbranch actual, rebind/capture/annotated alias and missing Normal relation refuse. Smallest next slice: Le source+physical Normal correspondence through SAME checked Compare/carrier/Bool/literal/final/ABI owners, with guard-following outgoing actual consuming its original source receipt in existing incoming classification. Fixture unannotated bridge(p){if p<=0{return null} return page.allocate(p)} with integer callers (including negative), plus Bool/Null/Object Fault and root/site/order/guard/veto negatives. Need canonical source-flow issuer audit before coding Normal-to-actual join; no second scan/solver or new semantic authority. Then exact static Integer payload projection/closure uses existing Executable actual producer; result315/318/Home/result publication remain separate owed. Non-claims: Le source receipt alone does not close actual entry or move production probe; full goal remains active. Le family closeout: original CompareOperand and canonical LessEqual NormalInteger issuer connected to SAME final source loan and ABI occurrence/side closure; JSON sle and C kind-1 checks retained. Real-source both orders/shared carrier/two-stage alias and Le-to-Gt drift PASS. Original-source 22-input C execution PASS (direct/alias, negative/edge integers, Bool/Null/Object Fault); real-source Le Normal Add/set/new positive and nested bypass negative 2/2PASS (/tmp/hako-compare-le-normal-consumers-fixed.log). Initial fixture unrelated unused provider produced check-uncovered-function; corrected to existing per-consumer fixtures, no production workaround. Final borrowed323PASS/7baseline/4ignored and compare86PASS/2baseline, exact normalized diagnoses match previous carrier runs; no new unclassified or THISCHANGE red. Scope/pointer/COPY-UNKNOWN/diff PASS; scope stale old occurrence-test name corrected to actual source-side coverage child. Fresh CLI29.64s PASS; unchanged mimalloc SHA33d3e8b9 still Heap.allocate33 Body1.IfThen0.Value -> Page.allocate26 source-only-object-actuals/incoming-row, rc1/noEXE (/tmp/hako-compare-le-mimalloc-probe.log). Selected Le family closed; guarded actual source proof, static payload, executable entry,315/318, seven old S0 debts and full acceptance remain owed. Guard-to-outgoing-actual Decision (read-only reviewed; supersedes sequence-later alone): source authority is SAME CompareOperand canonical envelope plus resolver-sealed If bundle, control-parent region and exact lexical scope. Require guard and actual in the same original ordinary statement sequence: exact guard prefix through its final ordinal position, equal segment kind and strictly later actual ordinal, same original region/scope; inner guard to outer actual, sibling arms, foreign container and preguard actual reject. No truth/positivity inference. SAME use-draft producer collects immutable exact call/ordinal/site/binding/root receipts after existing alias/rebind/capture/annotation validation; UnsupportedUse may retain first error while collecting source-only receipts, structural errors invalidate all. SAME source_seeds exact incoming rows/classifier consumes this fact per actual, retaining all15 callers/vetoes and one domain fixpoint; transport closure never resurrects rejected whole caller profiles. Smallest next slice original Normal-to-actual source proof and incoming classification; acceptance same-sequence direct/alias/post-guard (including dropped UnsupportedUse caller) and nested bypass/sibling/pre-guard/owner/root/ordinal/rebind/capture negatives, imported15-row census. Final checked Fault/SSA/ABI transport proof remains separate before Executable activation; source receipt alone grants no physical payload, Home, result or production acceptance.
## Closed source and physical prerequisites (historical compression)

The detailed decisions, counterexamples, validation counts and logs formerly
between this heading and Verified source prerequisites are retained in Git at
`a22d0a6487` (this card before compression). Their implementation checkpoints
include signed-immediate query `8907e44db4`, passive lexical source `989ca86f58`,
checked Compare/Bool and original literal/carrier correspondence, Static
incoming full-caller closure, shared packet/physical source, and the Result
source/ABI chain summarized above. These closed checkpoints do not prove
production EXE or the finite product goal.

Current boundaries carried forward: the original Static Rc, call site, formal
ordinal and all incoming caller/veto rows remain one source authority. A
SourceOnly or complete argument list does not grant executable actual, Home,
or physical publication. No whole-caller Integer inference, name-based target
reconstruction, legacy retry, or dropped caller is allowed. Both original
Result sites 315/318, anchored cleanup, final source coverage, unchanged-app
EXE, Gates 2–4, old-edge retirement and nondelegating selfcompile remain open.
The selected 2026-10-09 Static Loop decisions and present stop are below.

## Verified source prerequisites (2026-10-09)

CLOSED7f6a6efa83/a26502ff22/9bad8736b8: checked Return/exact Integer/Compare child;
corrected PASS receipts/source/guard/finished/C/USIZE and caller/veto laws: Git/README.
Original workdisk receipts remain; no SourceOnly/range/entry permission.
## Guarded Mul next Decision (2026-10-09)

Read-only review_cleanup_path: sole dynamic_operator_contract/borrowed arithmetic
owner gains explicit Mul; Add unchanged. NormalInteger x NormalInteger yields
fresh NormalInteger. SAME guard/ordered formal/alias/literal or call-child loan survives finishing.
Original bin<=8 then-arm requires checked Normal branch-region lending, not
old Add's later-sibling-only scope check; physical CFG dominance remains owed. Both scanners verify
exact operation/coordinates/ordinals/guard dominance, not counts/Add spelling.
JSON/C validator/index/admission/flow/emit spell mul; Tagged kind1 required
before result, wrong kinds Fault without mutation. Acceptance: original literal/
call sibling, orders/alias/optimization, source->JSON->C; Bool/null Fault and
wrong/missing guard/operation/operand/call/FinishedBindings refuse.
Current dynamic I64 arithmetic policy stays; exact-width checked overflow is
separate. CurrentOwner entry/context and all caller/veto laws remain unchanged.
Both315/318, cleanup, unchanged EXE, Gates2-4, old-edge retirement and final
selfcompile remain open; Mul alone cannot close bin_size.

Arithmetic append CLOSED62bf32f0e4: contracts/PASS receipts in Git/README and
workdisk hako-arithmetic-original-append-evidence.json; Mul remains open.
## CurrentOwner zeroarg result/packet Decision (2026-10-09)

Source CLOSED12d022a270: contracts/exclusions in Git/resolved README;
workdisk hako-static-i64-source-evidence.json retains PASS.
Packet closure OPEN: ALL incoming/caller/veto, original signature/Completion,
SAME Static Rc and unconsumed affine publication handoff remain mandatory.
Existing lexical_i64 Invoke owns NormalResult/group/FinishedBindings; selected
CurrentOwner generic bridge caller must switch without retry. Generic Call birth
JSON grants no Static permission; no fake borrowed entry or generic append detour.
Read-only review_cleanup_path ordering: ALL raw callee incoming/veto/staged inputs
plus callee own signature/Completion precede per-site packet/priorHomes; final
collector requires ALL caller finished proof. No word_size<->Mul start-gate cycle.
Zeroarg input/Invoke and guarded Mul original call-child join one semantic closure.
Original nested/loop/short-circuit and runtime acceptance obligations remain.
Opaque CurrentOwner input/context, Both315/318, cleanup, unchanged EXE, Gates2-4,
selected old-edge retirement and nondelegating selfcompile remain required.

Input CLOSED e3fb0c71e1: contracts/PASS in Git/README and
/mnt/workdisk/hako-static-zero-input-evidence.json; no fake entry/packet grant.
## Static zero packet part VERIFIED (2026-10-09)

CLOSED6d7ac8f04e: original Rc/cohort/Completion/affine shared Invoke contracts
and focused/runtime/guard evidence are retained in Git and
/mnt/workdisk/hako-static-zero-packet-evidence.json. Publication baseline
root-exit-source-missing reproduces at e3fb and remains unwaived;
shared generic bridge and all full-goal obligations remain open.
## Guarded Mul source construction (2026-10-09)

Normal branch lender CLOSED95fab122f2: exact SAME guard/branch/source laws and
corrected focused3+31+10 PASS/build8m40 receipts in Git/owner README and
/mnt/workdisk/hako-guarded-mul-normal-reach-evidence.json.
Decision integrated from read-only review_static_zero_packet: one immutable Mul
product per original binary, ordered per-operand guard/origin/literal/Static child.
Each source use keeps SAME product and exact side; whole view/Compare membership
is revalidated. Missing guard/origin/sibling proof stays UnsupportedUse.
Existing Static inventory receives retained Compare/Mul child candidates from
original drafts, rejects conflicting Rc and corroborates SAME canonical loan
before reuse; no new global source map or Add count-proof admission.
Source part CLOSED3b6cabeec1: canonical Mul source/consult/entry getter and
Static graft contracts, corrected failures and focused/reused95 PASS are in
Git/owner README and /mnt/workdisk/hako-guarded-mul-source-final-evidence.json.
Original in-branch call child and unchanged bin_size source partition are
verified; physical Mul execution remains unverified. Source-only collection
precedes qualified-target eligibility without weakening incoming/entry/
Completion/transport laws; Bool/Text gain no Integer agreement or entry.
Next Decision (read-only worker): typed Compare/Mul dispatch in SAME binary driver;
Mul retains SAME RootLocalCallBindingGroup lexical packet Rc/NormalResult + sole append.
Finish/project exact guards/ordered operands/packet with both scans and JSON/C; no Add counts.
All full-goal obligations above remain unwaived; no app source rewrite.
## Completed Mul and tagged-entry prerequisites (2026-10-09)

The source/append, SourceInstance, driver, physical Mul, finished-binding,
checked Copy, and tagged-entry slices are retained at 430198d21a,
5351063d91, 999b3ffc17, 40a73b0ee1, 57da98d9c0, 31c2caf637,
and 3607535f38;
the tagged-entry evidence is in `/mnt/workdisk/hako-tagged-entry-*` and
its owner README. These preserve the original source/Copy/FinishedBindings,
Normal/Fault and source-only refusal boundaries. No public `i64` input
contract is inferred from all callers. `me.word_size()` non-view proof,
whole Static entry/packet, unchanged app, Both315/318, cleanup, Gates2-4,
old-edge retirement and selfcompile remain open.

Closed Static Loop source census (2026-10-09; historical): the original
`SizeClassBox.size_to_bin` Loop body contains `If n <= me.bin_size(bin)`, while
the header calls `me.max_regular_bin()` and the backedge writes `bin + 1`.
The arity-bearing CurrentOwner local/direct-return source and original
Completion were proved at `10bd3ff07d`; shared exact Static site claims and
source negatives are at `deb7eba579`/`e54660e07f`. The source-bound V2
Recipe/JoinSig, Home-neutral receipt and cataloged entry followed at
`7b9f345d7f`, `b69906ee8a` and `2069578825`. The full historical decision
and counterexamples remain in those commits. No V1 Call operation, G0/S6C/
Dynamic rebranding, generic Static transport expansion or public i64 contract
was granted. Whole-caller Integer agreement is false for `size_to_bin` and
its forwarding chain; uncalled `LayoutBox` forwards cannot be pruned. The
current selected `bin_size` caller census and executable obligations are
recorded below; source-only evidence never grants production publication.

S1 checked-input prerequisite Decision (read-only review integrated):
Source authority is the original `UnresolvedArgument`/checked `CompareOperand`
body-use graph, ExactI64 conditional required ordinals, and retained Static Rc;
the existing tagged entry and checked numeric operation own Normal/Fault.
All-caller candidate disagreement is not proof of a different class, and body
use is a conditional requirement, never unconditional Integer agreement.
`ExactBool` has no result-owned required-ordinal slot, so a general input
condition cannot live only in result disposition. D0 must choose one canonical
input-requirement issuer and checked-use handoff for I64/Bool wrappers, with
wrong-tag Fault before numeric use and no silent retry. No uncalled method may
be excluded without sealed non-public reachability. Smallest next work: audit
that issuer/physical handoff and fix the exact source-only requirement contract;
then close `n`/`bin` Home, V2 loop, SSA, and S2 packet in that order. The
diagnostic failures are observed frontier, not accepted test regressions.

Checked-input source design closed at `02d6ee4044`: the borrowed-formal
package joins original use/forward sites, Static target/formal and complete
incoming source. A tagged input may reach a checked numeric use; wrong tags
Fault before payload use. The result catalog corroborates conditional I64
Normal output, not input classification. Whole-caller Integer agreement and
source-only calls remain non-authorities. The original `normalize_size` ->
`size_to_bin` -> `good_size`/`accepts` chain passed source 1/1, domain 10/10
and actual 13/13; the commit owns detailed decisions and negatives. This
source proof grants no executable packet or EXE.

S1 callable-path correction and source-only loan landed at `264791096c`:
the selected Loop enters `raw_loop_child_port`, not standalone `route_entry`;
package one-take rows join original Static Rc/site/target/ordered args with
the result claim. Pre-loop impersonation rejects. Focused evidence is in that
commit; no executable actual, Home or V2 physical permission was granted.

S1 V2 Decision (read-only worker, 2026-10-09): issue a source-bound V2
semantic product at `raw_loop_child_entry`, before its V1 Facts issuer.
Authority is the resolver Loop/If/assignment/return topology, original
Completion, and both one-take Static loans (header zeroarg, body arity one).
The V2 producer owns keys; common verifier and JoinSig own structural and
logical joins. Each CallSlot must co-seal its original Rc/site/target/args.
The selected old responsibility is the V1 `SourceItemsMissing` stop for this
source; other callable families and non-callable routing stay untouched.
Reject missing/foreign/duplicate loans, changed statement order, wrong
ordinal/class, absent return or `bin` backedge before any MIR effect.
Acceptance is unchanged source with both calls, inner return and one I64
backedge; negatives cover missing/swapped loan, missing return and Bool update.
The post-loop `return me.huge_bin()` is a separate continuation obligation.
Semantic V2 grants no actual value, Home, physical SSA/packet or EXE.

S1 producer prerequisite correction: the two in-Loop loans do not prove the
I64 class of `n`. The same Static claim issuer must co-seal the pre-loop
`local n = me.normalize_size(size)` initializer, ExactI64 Normal result and
original Rc. The checked-input condition remains conditional; wrong tags
Fault before that result. No raw tagged formal is treated as I64.

S1 pre-loop loan closed at `0fc9cdaf03`: original `normalize_size` Rc and
`local n` initializer co-seal ExactI64 in one-take row; source 1/1, claim
9/9, actual 13/13, domain 10/10 PASS. No live entry `n` or production switch.

S1 source-bound V2 semantic landed at `7b9f345d7f`: original resolver
Loop/If/return/rebind, three Static loans and Completion issue common V2
Recipe/JoinSig with checked source-to-key roles. Swapped loans, missing inner
return, Bool backedge and same-site reentry reject. Focused source/route,
raw-entry 11/11, schema 23/23, quick check and CLI build PASS. The route
stops at `static-i64-v2/physical-unavailable`; whole-app probe still stops
earlier at `borrowed-entry/source-only-object-actuals`. No live Home/SSA/EXE.

S1 Static source/Home history (closed): the source-bound V2 Recipe/JoinSig
landed at `7b9f345d7f`; exact post-Loop Static return source continuity at
`a78e9637b6`; ClosedCallable Home-neutral source proof at `b69906ee8a`.
The original same-module target/actual inventory and full transitive body
check own the effect boundary. Annotated ExactI64 and absent `new` are not
Home evidence. These source products issue no executable actual, ValueId,
Fault/Normal packet or EXE; Git owns their focused tests and decisions.

Issuer census I0 (read-only): `physical_entry_session.rs` requires an empty
Builder and opens the sole unpublished function transaction. The selected
raw Loop port already runs inside earlier lowering and has no canonical
session parameter; `callable_canary.rs` demonstrates the direct Static
prelude only inside an existing session. Select a function-level handoff
before physical effects, or prove a same-session loan from the enclosing
owner. Until that seam is fixed, `n`/`bin` ValueIds remain unavailable.

Function-entry handoff Decision: the selected cataloged Static method is
currently `CanonicalCallableRouteV1::Outside` and reaches the raw Loop child.
Move only its one-take V2 source/Home product consumption to the cataloged
function entry, before legacy draft or Builder effects. The package ledger
and original resolver Loop site remain authority; the Recipe issuer stays
unique. Missing/duplicate/foreign receipts reject there with no raw retry.
Keep the physical-unavailable stop until exact executable input/packet and
canonical session admission are issued; this slice grants no ValueId.
Acceptance: original four-call source selects once at function entry; a
second take and changed source reject, while unrelated methods keep routes.

Function-entry I0 verification: original selected cataloged method reaches
the V2 physical stop before Builder function/block effects; source mutation
and second take reject, unselected Loop returns no product. Focused 4/4,
candidate 6/6, raw Loop 16/16, Static claim 9/9, quick check and pointer
guard PASS. The executable Static actual/packet and n/bin session input are
still unissued; no production MIR or EXE was published.

Tagged-entry source and physical-signature prerequisites were closed at
`fc69461bfd` and `90e7dafaf9` (focused tests, candidate veto, signature
family and quick library check PASS there). The source receipt binds the
original pre-loop `normalize_size(size)` Rc to checked formal ordinal 0;
the package signature joins one Static `OrdinaryScalar` lane, no receiver,
to that exact binding. The joined carrier is `BorrowedTaggedValue`; the
coarse lane role alone never proves it. The original unannotated result
requires a separate I64 result proof. Current entry/session behavior is
defined below and at `2dd41b595f`; these earlier checkpoints granted no
executable actual, packet, or production MIR.

Canonical-entry result Decision (read-only worker integrated): unannotated
`size_to_bin` has `result_contract.result=None` and no generic physical
header. The instance-only `BorrowedI64ResultSourceV1` is not its authority.
The existing same-module Static result solver issues exact-key `ExactI64`,
but the qualified-claim index currently drops the function rows after
building call-site claims. Preserve that issued row in the same package and
lend only the selected key/owner result to the Static V2 entry; corroborate
the retained Completion's complete value-return set against the V2 inner and
tail return sites. An absent, foreign or non-I64 row, Void declaration or
extra return rejects before Builder effects. Do not manufacture `: i64` or
reuse the absent physical header. Then one unpublished canonical session may
install physical I64 result and the joined tagged formal, discarding its
draft at the missing executable packet. Packet proof gates publication, not
opening a disposable draft. No new result solver or fallback is authorized.

Static result-source handoff closed: the original solver's function rows now
survive call-site claim construction in the same package. The selected Loop
takes one exact-key I64 row joined to its unannotated Completion; V2 checks
that its inner and tail returns exhaust the retained exit set. Original
mimalloc focused 2/2 and cataloged entry 1/1, candidate 6/6, Static claim
family 9/9, physical header family 5/5, quick lib check PASS. Non-I64 Static
rows remain non-I64; unannotated generic physical headers remain absent.
No Builder session or production MIR was published. Next: one unpublished
canonical session installs the tagged formal and physical I64 result, then
discards on the still-missing executable Static packet.

Static canonical-entry Decision (read-only worker integrated): the exact
checked tagged-formal/source-signature join and retained I64-result receipt
are the only representation authority. Borrow the package's existing
Completion by owner and selected BlockExpr expectation; the existing
owned-Loop If partition closes outer If control. The sole physical owner
installs one detached shell with `BorrowedTaggedValue` at `%0` and I64 result,
preserving the original parameter declaration and absent result annotation
in metadata. Do not use the declared-signature setter: it rewrites an opaque
source annotation into a physical Box lane. `EffectMask::ALL` is a
conservative unpublished-shell upper bound only; executable effect/packet
proof is required before publication. A late packet absence discards the
canonical session, leaving no function/header or retry. Positive acceptance
is the unchanged selected method reaching that late stop after entry
adoption; negative acceptance retains foreign/missing/cohort refusals before
Builder effects and empty Builder state after discard. This is an entry
responsibility, not executable actual/packet or production MIR.

Static canonical-entry I0: original selected mimalloc method reaches the
late `executable-packet-missing` stop after tagged `%0`/I64 entry adoption;
its unpublished session restores empty function/block/header state. Focused
1/1, original V2 2/2 and candidate veto 6/6, quick lib check PASS. The
original foreign signature and source/result refusal pins remain green.
Next: original Static executable pre-loop `normalize_size(size)` actual and
packet, followed by canonical `n`/`bin` inputs. No production MIR or EXE.

Executable Static packet Decision (read-only worker integrated): the selected
pre-loop loan already keeps the original CurrentOwner `Rc`, exact call site,
ordered argument and I64 target claim. The tagged-entry proof binds that
argument to the checked caller formal. The existing `SourceStatic` actual
phase refuses executable admission, and the general Static packet path admits
qualified or zero-input CurrentOwner calls only. Extend neither globally.
First source slice: in the same package, retain one-take selected-Loop packet
authority from that original Rc, checked caller input, target formal contract,
target Completion and same catalog's ExactI64 result row; preserve scoped
errors and issue no ValueId. Then the sole canonical physical owner may use
that packet to emit Invoke with Fault/Normal and I64 NormalResult. The declared
`: i64` direct-call emitter is unavailable for the unchanged source. Fault
cleanup, result binding and packet consumption must be verified before any
publication; no generic retry or whole-caller Integer assumption.

Static packet-source S0: the original selected `normalize_size(size)` Rc now
retains a one-take package packet source, corroborated at the disposable
canonical entry. It reuses the callee's checked OpaqueHandle formal,
Completion and ExactI64 result. Initial focused red was a THISCHANGE
`bin_declaration`/entry declaration key mix-up, fixed by taking the original
pre-loop entry declaration. Final focused 1/1, candidate veto 6/6, existing
Static packet family 6/6 and quick library check PASS. The endpoint still
discards at `executable-packet-missing`; no physical Invoke or production MIR.
Next: sole physical owner maps this source packet to tagged `%0`, Invoke,
Fault/Normal and I64 NormalResult with no fallback.

Selected Static Invoke Decision (read-only worker integrated): the package
projects the original catalog key to the Global call and lends its one
argument site. The canonical identity owner claims that exact site and reads
tagged `%0`; the canonical CFG owner places one I64 Invoke, separate Fault
and Normal landings, and ReturnFault. Normal projects the I64 result first.
The draft must still discard: `SourceStatic` does not yet issue executable
BorrowedActual coverage, and final source-aware verification requires the
original call coordinate and actual slot. This slice neither promotes the
generic Static path nor publishes MIR. Next: bind the pre-loop `n` result,
then issue the bounded actual/packet and close final source coverage before
production publication. The declared-`: i64` direct Call remains inapplicable.

Unpublished Invoke I0: unchanged mimalloc-lite selected entry reaches the
post-Invoke `actual-coverage-missing` stop and leaves no Builder/header state.
Focused 1/1, candidate veto 6/6, Static packet 6/6, independent Invoke Call
verifier 12/12, quick library check and pointer guard PASS. The first wrong
Invoke verifier filter selected 0 tests; the corrected family above is the
recorded result. Production MIR/EXE and executable actual remain open.

Pre-loop `n` result S0: the same canonical identity owner publishes the
Normal-only I64 projection at the resolver's original entry declaration and
exact `n_binding`, then reads it in the Normal block to corroborate SSA.
The draft still discards before `bin` input and executable actual coverage;
this adds no second result authority or production publication.

`n` result focused 1/1 and existing candidate veto 6/6 PASS; quick library
check, pointer guard and diff check PASS. Builder/header remains empty after
the new post-`n` stop; no production MIR or EXE was published.

Bin-input Decision (read-only worker integrated): the existing Static V2
semantic producer already proves the unique original `bin` initializer,
literal Integer(1), declaration/binding and Recipe input/carrier relation.
Retain that checked literal in the same product. After the `n` Invoke Normal
projection, the sole canonical session emits its constant and publishes the
exact original `bin` declaration through canonical identity. The shared
literal-only initialized-input materializer cannot own this mixed pair because
`n` is a call result. Changed/missing/reordered bin source rejects upstream;
foreign binding, input-key or carrier drift rejects before physical effects.
Keep the draft unpublished pending full actual/source coverage and body.

Bin-input S0: the Static V2 semantic retains its already-checked source
Integer(1). The same unpublished Normal block emits that value via the
existing Const owner, then canonical identity publishes the exact original
`bin` declaration/binding and reads the same SSA value. Fault has no bin
value. The post-input stop is executable actual/source coverage; Recipe
segment allocation, body, publication and EXE remain open.

Executable actual Decision (read-only worker integrated): retain the same
original `StaticIncomingSourceV1` Rc. The selected CurrentOwner arity-one,
ordinal-zero forwarded Opaque site may enter the existing executable ingress
only after the same source inventory closes every incoming caller of that
callee. `SourceStatic` and `source_only_definitions` currently refuse that
handoff; simply promoting one actual would fail final `incoming == original`
coverage. Reuse the existing CallPacketSource, lexical projection and finished
coordinate owner; do not create a second call-position ledger. Canonical `%0`
corroborates the physical actual but does not classify its meaning. The next
bounded series must prove exact Rc/site/formal/target, all incoming coverage,
and existing packet/result projection before body and root-source publication.
Wrong site, ordinal, callee, or unproved forwarding stays fail-closed.

Bin-input focused 1/1, candidate veto family 6/6 including original
`bin=1 -> 2` mutation, quick library check and pointer guard PASS. The
unpublished draft still leaves no function/header; production remains open.

Complete Static incoming source S0: the existing inventory's whole-callee
projection now supplies the selected packet with every original Static Rc.
The packet checks exact callee/target/batch, unique sites and inclusion of
its own original Rc. An inventory veto, foreign target or missing selected
source refuses before Builder effects. This is source census only: none of
those callers gains executable actual, call coordinate or publication.
The unchanged original mimalloc focused case passed 1/1; existing candidate
veto 6/6 and Static packet 6/6 passed against the same quick-profile build.
Quick library check and current-state pointer guard passed. A separate
`borrowed_formal_incoming` filter selected 0 tests and is not acceptance.
Next: issue the selected CurrentOwner executable actual while preserving
the complete incoming cohort for final source-aware verification.

Selected forwarded-actual Decision: the existing pending `SourceStatic` row
and `StaticArgumentSourceV1` fact share the original incoming Rc. For this
packet's single CurrentOwner, ordinal-zero Opaque forward, corroborate the
staged candidate, source-only draft origin, caller/target checked-input
receipts and the canonical entry formal. The selected packet carries the
resulting source actual witness into its disposable tagged `%0` Invoke. Do
not promote the generic `SourceStatic` phase or infer Integer from `%0`.
An absent/mutated candidate, wrong site/formal or unchecked forward rejects
before Builder effects. All other incoming edges, executable packet
coordinates, body, root-source publication and production EXE remain open.
Selected actual witness focused positive/mutated-candidate negative 1/1,
unchanged mimalloc entry 1/1, candidate veto 6/6, Static packet 6/6 and
existing forward-identity negative 1/1 passed in the same quick-profile test
binary. Quick library check, pointer guard and diff check passed. The
post-Invoke `actual-coverage-missing` stop remains intentional because the
other incoming edges and finished coordinates are not yet proved.

Full incoming execution Decision (read-only worker integrated): original
`SizeClassBox.normalize_size/1` has two source callers: CurrentOwner
`size_to_bin` and qualified Return `LayoutBox.normalize_size`. The source
cohort already retains both original Rcs, but the generic `SourceStatic`
phase, final borrowed entry and physical packet still reject the mixed
cohort. First extend the selected packet's source-actual witness over every
original incoming Rc using the same pending SourceStatic candidate,
`StaticArgumentSourceV1`, caller/target checked inputs and Opaque formal;
keep the draft's post-Invoke stop. Next, separately close the selected
CurrentOwner packet/coordinate, the qualified Return packet/coordinate,
and final entry/source-aware verification before publication. The Return
caller must use the existing Return-context finished producer, not a local
binding substitute. A source census alone, `%0` or a single finished call
cannot stand for all incoming coverage. `SourceStatic` remains generally
non-executable and the original result/publication handoff remains a
separate gate.

Full original Static source-actual cohort S0: the selected packet now issues
the same forwarded-actual witness for every retained incoming Rc. The real
`normalize_size/1` source has both CurrentOwner and qualified Return rows;
the bounded test proves both, then mutates only the qualified staged candidate
and observes fail-closed rejection. Original mimalloc entry 1/1, bounded
positive/negative 1/1, candidate veto 6/6, Static packet 6/6 and forward
identity 1/1 passed with the same quick-profile binary; quick library check,
pointer guard and diff check passed. No generic `SourceStatic` activation or
physical packet/finished-coordinate claim was added. The draft still discards
at `actual-coverage-missing`. Next: exact CurrentOwner publication handoff
and packet/coordinate, then qualified Return packet/coordinate and final
entry/source-aware verification. Neither one-row publication nor source
cohort alone completes the callee.

CurrentOwner result handoff Decision (read-only worker integrated): the
existing `VerifiedStaticCallResultPublicationOwnerV1` already issues an
ExactI64 handoff for arity-one CurrentOwner source targets. It is the sole
result-publication authority. The general original-source corroboration and
Static packet admission intentionally cover qualified or zero-input callers
only. Add a selected Loop-packet corroboration for the same original Rc and
ExactI64 handoff identity; leave the general rule unchanged. The disposable
canonical entry borrows the collector's exact caller/site handoff without
consuming it. A handoff take or new result issuer before publication would
lose affine authority. Positive acceptance is the unchanged mimalloc entry
reaching its existing post-Invoke stop after the peek; negative acceptance
is foreign catalog/identity refusal and continued general CurrentOwner
rejection. Physical packet/finished-coordinate and both callers' entry
coverage remain later work.

CurrentOwner handoff peek S0: the disposable selected entry checks the
existing publication owner's exact caller/site handoff against its original
source Rc and forwarded-actual proof without consuming the handoff. The
unchanged mimalloc entry and bounded same/foreign-catalog test passed 1/1
each; full source cohort 1/1, candidate veto 6/6 and Static packet 6/6
also passed in the same quick-profile build. Quick library check passed.
The first focused red was a test-fixture omission: the fixture had not
installed the publication owner that production installs. The corrected
fixture passed; no implementation red remains. The draft still discards at
`actual-coverage-missing`. Physical packets and finished coordinates for
both incoming callers, final entry coverage and production EXE remain open.

Selected physical-packet Decision (read-only worker integrated): the existing
lexical packet/finalized visitor is the physical owner, but its generic Static
admission deliberately rejects arity-one CurrentOwner `SourceStatic`. Keep that
law. The first bounded step observes the selected original Rc, checked
forwarded actual, unconsumed ExactI64 handoff, canonical SSA read, and the
actual disposable Invoke/NormalResult in one function; wrong source, actual,
result kind, landing or target refuses. This observation neither promotes
`SourceStatic` globally nor records a root binding group. `discard_unpublished`
rolls back MirBuilder state only: it does not roll back the shared claim ledger
or collector publication owner. Therefore take the affine handoff and record
source-ordered bindings only at the later whole-function commit boundary,
after both incoming callers and final entry coverage can publish. The
qualified Return caller keeps its distinct terminal finished producer. No
single CurrentOwner packet claims whole-cohort or production completion.

Selected unpublished physical observation S0: the packet now compares its
original Global target and canonical actual against the actual draft Invoke,
I64 result kind, separate landings and unique NormalResult producer. It
borrows the existing ExactI64 handoff and mutates neither shared ledger nor
collector. Unchanged mimalloc focused entry 1/1, physical shape positive and
wrong-result/wrong-landing negative 1/1, staged actual negative 1/1 and
handoff catalog negative 1/1 passed in the same quick-profile test binary.
Candidate veto 6/6 and generic Static packet 3/3 passed before the final
shape helper extraction; that extraction did not change their path. Quick
library check, pointer guard and diff check passed. The draft still stops at
`actual-coverage-missing`; no finished call coordinate or production switch.

Selected forwarded-actual loan Decision (read-only worker integrated): the
existing `SourceStatic` candidate issuer already proves the original Rc,
ordinal/site, candidate binding, caller root/formal, target formal and source
draft origin for both original `normalize_size/1` callers. Retain that checked
binding in a derived immutable `Forwarded` actual row, then lend exactly one
row per original caller from the selected complete cohort. The canonical
CurrentOwner read must use this row's binding, not infer the value from its
formal or manufacture `ExactLexicalReadV1`. Wrong Rc/site/formal/candidate
refuses. This is source loan only: generic `SourceStatic` admission stays
closed, and no ledger binding group or affine handoff is taken. The eventual
final verifier needs a bounded selected entry bridge because its generic
`source.incoming`/qualified path omits this CurrentOwner source-only caller;
both callers' finished coordinates and callee entry remain required.

Selected forwarded-actual loan S0: the witness now retains one derived
`PreparedBorrowedFormalActualV1::Forwarded` row from the already-checked
source candidate, and the selected complete-cohort packet lends it by exact
caller/site. The disposable CurrentOwner canonical read takes the row's
binding; its existing original MIR call still stops at
`actual-coverage-missing`. Both original callers' witness rows and the staged
candidate mutation passed 1/1. Unchanged mimalloc entry 1/1, handoff negative
1/1, physical-shape negative 1/1, candidate veto 6/6 and generic Static packet
3/3 passed in the same quick-profile binary. Quick library check, pointer
guard and diff check passed. Source-only phase, shared ledger, affine handoff,
finished coordinates and production path remain untouched.

Selected packet publication Decision (read-only review integrated): the
original `StaticIncomingSourceV1` and checked forwarded actual remain source
authority; `VerifiedStaticLoopPacketSourceV1` is their selected cohort loan.
The existing lexical binding group and invocation collector are the physical
packet and publication owners. Generic `CallPacketSourceV1::static_i64` must
keep its qualified/zero-input and executable-actual law: the CurrentOwner
source-only caller cannot enter through it. A selected packet arm must bind
the original Rc, local-call observation, canonical read and actual Invoke/
NormalResult without constructing an `ExactLexicalReadV1` from a `ValueId`.
Its site must be selected in source order by the same lifecycle group owner.
`record_root_lexical_call_bindings` currently mutates the shared ledger
immediately, while draft discard restores only MirBuilder. Therefore hold the
selected packet/group in function-local pending state, preflight its group
order and physical shape, and commit it with the affine handoff only at the
successful existing collector admission terminal. A late draft/admission
failure must leave both ledger and handoff untouched. Finished visitor and
callee entry must consume that real packet; scanning final MIR for source
identity would create a second authority. The qualified Return sibling uses
its separate terminal finished producer. Next bounded implementation is the
CurrentOwner caller's selected packet plus transactional group publication;
accept one real collected caller packet and reject late admission without
group/take. Both-caller completion, final entry coverage, full body and EXE
remain open. No general `SourceStatic` activation or publication by the
disposable probe is permitted.

Completed selected packet prerequisites (commit evidence):
- `e6319b1a26`: root lexical group preparation preflights source order and
  packet prefix before shared-ledger mutation; ordinary callers commit at
  their existing terminal.
- `455123f3dc`: the selected CurrentOwner route uses the original I64 local
  observation and ordinal-zero BorrowedActual; generic qualified/zero-input
  Static packet admission remains closed.
- `ae7d7fdd68`: the lexical packet owner validates original Rc, checked
  forwarding actual, canonical read and Invoke/NormalResult in an unpublished
  selected prepacket. It cannot enter a group or take the affine handoff.
Focused positive/negative evidence and late-failure checks are in those
commits. The current obligation remains to preflight finished packet/group
and collector admission, then consume the handoff and commit the group only
at a terminal with no remaining fallible step. Draft discard alone does not
restore the shared ledger or affine handoff. The qualified Return caller,
final independent incoming/entry coverage and EXE remain open.

Selected Static I64 Loop physical history: header `d7692b5599`, body
predicate `8c58c7e3d9`, Return/backedge `13d8b5f404`, and post-loop tail
`0fe5c082d7` advanced the same unpublished canonical session. The source-
bound Recipe/JoinSig, original Static loans, canonical CFG/SSA, two Completion
claims and one Fault frame remain current. These commits contain focused
positive/negative evidence. The exact `me.bin_size(bin)` carrier, finished
packets/coordinates, group/handoff terminal, qualified Return sibling, final
entry verifier, collector and EXE remain open. No production MIR was published.

Selected Loop body actual and construction history: `5c236fdfd8`,
`cc1697fc29`, `49646f8bdb`, `635f834fe9`, `8cf6fadebf`,
`731b05b824`, `61933a40fd`. These commits own exact source Rc,
signature/Integer ordinal-zero correspondence, canonical SSA read,
Invoke/NormalResult, detached `prepare_exact_two` and positive/negative
evidence. The ValueId still comes from SSA after the backedge; source-only
Static is never independently executable. Later packet and publication
obligations are stated below.

Decision (2026-10-10, selected `bin_size` caller census): the current
`mimalloc-lite` import closure contains three CurrentOwner `bin_size` calls
in `size_class_box.hako` and two qualified integer-literal calls in
`layout_box.hako`. Four `bin_size(max_regular_bin())` sites found by a
repository-wide census belong to separate huge apps and are absent from this
batch. The read-only review confirmed the five-site selected inventory; no
nested-actual source expansion is justified by this active cohort. Continue
the selected packet/finished-coordinate work for these five original sites.

Decision (2026-10-10, selected body prepacket): `me.bin_size(bin)` is a
nested Loop predicate call and has no Home `LocalCallObservation`; do not
fabricate one or route it through the entry-only lexical packet arm. Move
the existing body scalar source receipt into an unpublished prepacket with
the canonical SSA read receipt and exact Invoke/NormalResult coordinates.
Recheck it after whole-function close. This preserves source-to-physical
identity while final visitor, executable tagged actual, callee entry and
collector publication remain separate obligations.
Body prepacket S0: quick-profile `static_loop` 5/5 pass, including original
source/Home mutation and physical landing/result negatives; the executable
packet stop remains. Pointer guard and diff check pass.

Decision (2026-10-10, next body coordinate): the final lexical visitor only
enumerates local and return packets. The body predicate has neither Home local
observation nor finished caller while DraftSeal still aborts at packet-missing.
Next prove a read-only detached finished projection for this retained body
prepacket, then carry it through selected publication and add an explicit
LoopBody visitor context. Do not fabricate a local row. Scalar tagged actual
and all-incoming acceptance remain separate. Reject wrong source/read/Invoke,
missing or duplicate finished producer and coordinate.

Decision (2026-10-10, detached body projection): lend the verified DraftSeal
image read-only after `prepare_exact_two`, then recheck the retained body
source/read/Invoke/NormalResult prepacket. The selected disposable path must
still abort and restore before reporting either shape rejection or the
existing packet-missing stop. No finalized coordinate, tagged actual, group
or collector admission is issued. Reuse the static-loop positive and source/
Home/physical-shape negatives; no duplicate guard is needed.
Detached projection S0: quick-profile `static_loop` 5/5 pass. The original
selected test reaches the unchanged packet-missing stop and verifies parent
restoration; source/Home mutation and wrong landing/result still reject.

Detached body actual `d307d68744`: the original Rc/candidate and canonical
read issue one Scalar(Integer) ordinal-zero actual at the unique DraftSeal
Invoke coordinate. Generic SourceStatic stays source-only and no Home local
row is fabricated. Quick-profile Static Loop 5/5 covered source, shape and
duplicate-coordinate negatives; later collector work is recorded below.

Decision (2026-10-10, selected body packet publication seam): the original
source Rc, scalar receipt and DraftSeal-checked detached Invoke are the
packet authority; canonical finished-child validation owns the completed
physical coordinate. Retain this selected LoopBody packet in the existing
`OrdinaryNewClaimLedgerV1`, keyed by owner and source site, separately from
Home local-call groups. Finished-child validation must first lend the exact
unpublished body binding to its existing `PhysicalBoundary`; the current
`lifecycle_bindings` omits that block, so a later finished lookup would fail
as `unrecorded-binding`. This loan is read-only and cannot publish the packet.
After successful exact collector collection, commit the preflighted packet
infallibly to the existing
owner ledger, then move it with the finalized source handoff and project its
original Invoke onto the finished function. Extend the existing final lexical
visitor with an explicit LoopBody context; its consumer must match the
finished coordinate and Scalar Integer actual against the physical Invoke.
Do not install shared retention before a failed collector collection or add
a fallible check after collection. Do not infer a packet from final MIR or
open generic SourceStatic transport. Wrong owner/site/read/Invoke, missing
or duplicate finished producer, and absent collector row reject. First
bounded construction slice: finished-child validation of the selected packet.
At `498e036e20`, transactional retention, visitor/consumer and full incoming
coverage remained separate responsibilities; the selected path still stopped
at `executable-packet-missing`. See the newer collector row below.

Selected child boundary `498e036e20`: detached entry/body Invoke bindings
pass PhysicalBoundary; the single-predecessor PHI exception is body-only.
Selected collector/retention `2602442eec`: exact entry/body packets preflight
before collection and commit only on success. Quick-profile Static Loop 6/6,
PhysicalBoundary 7/7, finished copy 5/5, terminal visitor 6/6; duplicate
collector and missing route leave both packets absent. This is a collector row,
not a finished program or production EXE.

Selected result-publication S0: entry, loop header, loop body and tail share
one source cohort and one branded publication owner. The four handoffs first
match source/target/I64/required arguments; detached header/tail zero-arg
Invoke+NormalResult and existing entry/body physical proofs precede one
atomic batch take after collector admission. No generic Call is emitted.
Quick-profile selected caller and owner missing/duplicate tests pass; Static
Loop 6/6, owner 11/11, PhysicalBoundary 7/7 (`47df4b14d9`).

Selected finished-child projection S0 landed at `0aa3e8aec4`: the original
LoopBody Invoke resolves one checked finished coordinate. The 44 pass/3 known
baseline reds are recorded there. Publication remains open.

Next Decision — LoopBody publication: preserve the lexical visitor and add a
sibling read-only LoopBody visitor. Lend retained source Rc, Scalar(Integer)
actual and finished coordinate to both existing final verifiers; match exact
physical Invoke and reject cross-vocabulary duplicate site/coordinate. No
Home local-call row, generic packet or second authority. Production mimalloc
still stops earlier at `borrowed-entry/source-only-object-actuals`; a minimal
real SizeClassBox import stops at `borrowed-static/local-source-missing`.
Full consumer acceptance, EXE and retirement remain open.

Decision (2026-10-10, real zero-input source-context census): the unchanged
`SizeClassBox` source has four CurrentOwner zero-arg Static sites selected by
the source inventory but absent from Completion Home `local_calls()`:
`bin_size` Body2/IfCondition/Rhs `max_regular_bin`, `bin_size`
Body11/Value/Rhs `word_size`, `size_to_bin` Body2/LoopCondition/Rhs
`max_regular_bin`, and `accepts` Body0/IfCondition/Rhs `huge_bin`.
An existing-owner one-shot exact-source census produced these four rows;
its deliberate failing assertion was removed after observation. This is
diagnostic evidence, not a passing acceptance or a new persistent test.

Source authority + canonical issuer: the retained `StaticIncomingSourceV1`
and exact CurrentOwner claim own call identity; for the first `bin_size`
IfCondition site, the existing verified If-region/scalar-expression/Home
issuer owns `ExpressionValue` with prior Homes. The zero-input closure owns
callee signature, Completion and result. Non-authority: the mere source
inventory, empty argument list, sibling call's Home row and final MIR alone.
Fail-fast boundary: keep `borrowed-static/local-source-missing` until the
same exact site has its own verified Home row; never fabricate a local row
or route the nonlocal site through a lexical local packet.
Smallest next slice: inspect this If-region bundle, `contains_source_request`,
scalar preflight (including formal `bin` kind), and Completion root flow to
find why the existing issuer did not retain the first row. Then admit only
the proved condition context and check source Rc/site/target, zero args and
Home statement/region. Wrong site/owner/target, unproved operand or region,
and missing Home row must reject. The later return, Loop and `accepts`
contexts are separate bounded followups. This Decision grants no executable
packet, tagged entry, production EXE or old-edge retirement.

Selected If-condition Home source S0 landed at `1fc58fc9e6`: original `bin_size`
Body2 now borrows exact checked `CompareOperand` for its full Bool condition,
then retains the zero-input call Home row; unsupported `>=` stays closed. The
unchanged SizeClassBox import still stops at `borrowed-static/local-source-missing`.
No final Invoke, all-incoming publication, EXE or production cutover follows.

Return `word_size` prerequisite audit (`938f1f4466`): an exact Static source
call is insufficient until its parent expression has Integer Home. Checked
Add, positive-literal Divide and nested Mul local proofs landed at
`e76b7a3b66`, `017de06ed0`, `2d7fa3ceb5`; Loop After, `words`, non-view
returned Mul and the exact `word_size` Home row remain open. Home neutrality
proves ownership effects, not scalar class or Loop assignment/merge. The
borrowed-formal `IntegerMulReturn` cannot prove ordinary local `words`.

Selected checked-Add local Home S0 landed at `e76b7a3b66`: exact dominated
`AddOperand` proves `bin + 3`, advancing the Home uncovered prefix from Body3
to Body4. Wrong binary identity rejects; Divide and the later chain remain
open. Focused positive/negative and required guards passed at that commit.

Decision (2026-10-10, positive-literal Divide Home): the original resolved
binary source owns operator and ordered child sites. The preceding checked
Add Home row proves `x` Integer; the original RHS `4` is a positive Integer
literal. Home may install `bit_group` Integer only after the complete local
initializer proves those facts. Zero/variable divisors, unproved lhs, wrong
operator/site, conditions and returns remain closed. Positive divisor also
avoids signed minimum/-1 overflow; no generic dynamic Divide envelope exists.
The smallest slice advances original `bin_size` Body4 to the next nested Mul
at Body5. This does not issue an executable Divide Recipe, physical call,
production EXE or old-edge retirement.
Validation: original source advances Body4 to Body5; zero/variable RHS and
Multiply replacement remain at Body4 (one focused test, four cases). The prior
Add test now asserts progress past Body3 without pinning the next frontier.
Add, If and scalar-expression family regressions passed (1/1, 1/1, 11/11),
as did the scope/pointer guards, rustfmt and diff check.

Decision (2026-10-10, nested Mul local Home): Body5 `top = x -
(bit_group * 4)` has original ordered binary source, and prior Home rows prove
`x` and `bit_group` Integer. The sealed NormalInteger Mul envelope supplies
the nested operation class; the local initializer preflight may borrow it
only for a root Subtract's exact right Multiply child with a proven Integer
local left and original Integer literal right. Root Mul, variable/Bool/call
child, nested Divide and return/condition contexts stay closed.
Whole-expression admission installs `top` after both children prove. This
source/Home slice cannot publish Sub/Div/Mul physical execution or an EXE.
Validation: original source advances Body5 to the Loop at Body9; root Mul,
variable RHS, nested Divide and Bool RHS retain Body5 (one focused test, five
cases). The prior Divide test now asserts progress past Body4; Add, If and
scalar-expression family regressions passed (1/1, 1/1, 11/11).

Decision (2026-10-10, `bin_size` Loop carrier prerequisite): resolved Loop
region seals placement, not post-loop scalar values. The production seam is
`raw_loop_child_entry` → `issue_callable_variable_accum_recurrence` → compiler
projection → AST-free Facts → Recipe. Its projector fixes a five-statement
root, literal bound and Add recurrence; GenericG0 is nested-loop/caller-zero.
Reuse resolver Loop membership/source identity, lexical `BindingRef` and
pre-loop Home Integer classes as authority; do not infer class from spelling.
The selected Home scanner currently rejects Loop/assignment. Build a bounded
single-loop Facts→Recipe→JoinSig/After for distinct I64 `scale`, `i`, and
variable I64 `shift_count`, condition `i < shift_count`, ordered `scale * 2`
and `i + 1`, with complete coverage and no other effects. Home borrows
verified After only; Facts alone are an interim prerequisite, not production
advance. Reject wrong owner/site/region/class, incomplete or reordered body,
changed update, nested call/control, or missing/duplicate After. V1/V2
LoopBinaryI64Op and physicalizer lack Mul; separate physical slice is owed.
No EXE or old-edge retirement is claimed.

Decision (2026-10-10, Loop proof handoff order): the selected package runs
`scan_new_home_flow` while co-sealing Completion, before `raw_loop_child_entry`
can issue its Recipe. The scanner stores pre-loop scalar classes only in
`PrefixLocalFlow` and currently marks Loop `PrefixNotCovered`; its returned
`RootHomeFlow` has exits/maps/local calls, no Loop After loan. Therefore a
builder-only Facts product cannot unblock the selected Home walk. The first
construction series must seal one source/membership/binding-based Loop
product with pre-loop Integer input proofs, lend its verified After classes
to this Home scanner, then carry the same source identity into the existing
raw Loop Recipe/JoinSig path. A second independent Home loop solver or
reclassifying the Loop from its syntax is rejected. Acceptance must show the
unchanged `bin_size` prefix advances past Body9 and wrong bound class,
owner/site, update order or missing After keeps Body9 closed. Physical Mul,
the later return expression and EXE remain separate obligations.

BoxShape prerequisite: return-position `new` membership moved from the near-cap
package issuer into its own source-membership module, retaining exact site,
candidate exclusion and builtin refusal. This changes no Loop admission.
The issuer now has room for the Loop consult; Body9 remains closed.
Validation: quick `return_position_new` 5/5, pointer/diff PASS; new module
rustfmt PASS (parent's unrelated existing tail layout differs from rustfmt).
