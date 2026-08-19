---
Status: SSOT
Date: 2026-08-19
Decision: accepted post-Recipe physical demand/session boundary and one final LLVM module closure
Scope: common Loop physical demand, one unpublished function session, DraftSeal handoff, and S6C TextEq physical corridor
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/joinir-loop-selfhost-recipe-pipeline-ssot.md
  - docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md
  - docs/development/current/main/design/archive/loop-common-physical-demand-and-session-history-2026-08-18.md
  - docs/reference/mir/loop-recipe-contract.md
  - docs/reference/mir/generic-loop-stage-matrix.md
  - src/mir/builder/resolved_lowering/README.md
---

# Loop Common Physical Demand and Session SSOT

## Current Capsule

- **Current decision:** the post-Recipe path has one semantic program, one
  JoinSig-bound physical layout, one canonical SSA/CFG/PHI session, one
  DraftSeal Return owner, and one whole-candidate rollback boundary.
- **Current implementation status:** the caller-zero S6C corridor reaches one
  real unpublished `MirFunction`. Its exact detached projection owns one
  source-issued Residence carrier and emits strict no-refresh MIR JSON with
  3 `PinnedTextOp`, 1 entry-owned Enter, 1 Trap, 2 Finish, and 2 value Return
  rows. This is not a production caller.
- **Current blocker:** the same parsed LLVM module now passes final lifecycle,
  target, and layout closure immediately before the sole trusted emit. The
  promotion evidence owner, comparable measurement strata, and production
  admission are not yet sealed.
- **Next ordered task:** `S6C-PINNED-CORRIDOR-PROMOTION-R0` design closure;
  its first executable cell will be link/run correctness only.
- **Production stop line:** promotion, production selection, fallback/retry,
  and `nyash.string.eq_hh` retirement remain closed.

The exact active row, mode, and blocker are selected only by
`CURRENT_STATE.toml`. This file owns the durable physical/session contract and
the compact task order. It is not an append-only execution log.

## One authority chain

```text
resolver source membership
  -> AST-free Facts
  -> one source/Facts/Core/Recipe/Join semantic program
  -> family-owned prepared semantic product
  -> physical-ID-free operation program and layout
  -> one prephysical admission
  -> one unpublished function transaction
  -> CanonicalSsaFunctionSessionV2
       sole CFG / Binding SSA / PHI owner
  -> common leaf dispatcher
  -> ReadyFunctionDraftSealV1
  -> detached DraftSeal projection
       sole Return writer
  -> collector / atomic publication
```

The physical layer consumes already-issued meaning. It does not derive
Recipe, Completion, JoinSig edge roles, TextEq semantics, or source binding
identity from MIR, numeric keys, runtime symbols, or backend success.

### Canonical issuers

| Responsibility | Sole authority / issuer |
| --- | --- |
| Loop membership and source identity | resolver source ledger and installed semantic package |
| operation meaning and ordering | Facts -> Recipe producer |
| continuation and edge role | JoinSig / Completion relation |
| physical placement | prepared physical layout issued from the owned operation program |
| entry binding and source index | canonical Binding SSA session |
| CFG blocks, edges, sealing | canonical CFG session |
| PHI lifecycle | canonical session `PhiTxn` |
| normal exit inventory | `PreparedFunctionExitSetV1` |
| Return emission | `FunctionDraftSealProjectionV1` |
| Residence lifecycle sites | canonical Enter/Trap writer plus detached Finish projection |
| whole-function rollback | outer unpublished function transaction |
| module publication | collector / module transaction |

Owner equality is validation, not construction authority. Products from
different issuers are never looked up by owner and re-paired after the fact.

## Durable product boundary

The semantic side may prepare family-specific products for Callable and
Generic G0, or a scoped V2 pre-session envelope. Those adapters must converge
before physical effect begins.

```text
VerifiedLoopSemanticProgramV1 | V2
  + source-bound input / item / carrier relations
  + JoinSig-derived continuation
  + callable ABI / Completion evidence
  -> PreparedLoopOperationProgramV1 | V2
  -> PreparedCallableLoopPhysicalizationV1
     OR PreparedGenericG0LoopPhysicalizationV1
     OR PreparedLoopV2PreSessionEnvelopeV1<'loan>
```

The common physical demand may contain:

- exact owner and installed-package cohort;
- complete operation inventory and exact coverage;
- input and result roles already bound to source relations;
- physical-ID-free layout and target capability;
- separate Loop After and callable Tail capabilities;
- borrowed Completion/expectation evidence with one consumer.

It must not contain:

- `MirBuilder`, `ValueId`, `BasicBlockId`, raw pointer, slot, generation, or
  runtime token;
- a second Recipe selector, CFG table, Binding SSA map, PHI owner, Completion
  owner, Return emitter, or publication transaction;
- inferred semantics from MIR adjacency, numeric ordinals, JSON, backend
  symbols, C shims, benchmarks, or legacy route labels;
- a retry, repair, route fallback, or partially published candidate.

The neutral layout owns placement only. Generic or S6C authority remains in a
family binding outside the layout.

## Exact consumption and fresh-session law

Before function/session effect:

1. resolver source context, source-bound Core, item/input/carrier relations,
   JoinSig continuation, ABI, expectation, and Completion are co-sealed;
2. the complete operation program and physical layout are issued;
3. target and entry signature/lane obligations are validated;
4. unsupported or foreign shapes reject as `NoSafeSlice`.

After the unpublished session opens:

```text
physical emission failure
  -> abort only still-pending provisional PHIs
  -> discard the complete unpublished function
  -> restore caller state once
  -> publish nothing
  -> no same-session repair, retry, or fallback
```

`PhiTxn` cleanup is local hygiene, not atomicity. Patched PHIs, other MIR, and
ID allocation are abandoned with the whole unpublished function. A new
attempt requires a fresh source-authorized candidate and fresh physical IDs.

### Entry and finish receipts

`ReadyLoopEntryV1` is a private temporal receipt proving that the exact
logical inputs required by the demand are installed in this function session.
It owns no source semantics and is consumed once.

`ReadyCanonicalProfileCloseV1` is a private temporal receipt proving the
profile-specific ledgers are closed. It is not a second Completion, CFG, PHI,
or Return authority.

`CanonicalSsaFunctionSessionV2::finish_for_draft_seal` is the sole V2 terminal
issuer of `ReadyFunctionDraftSealV1`. Its order is:

```text
Prelude / entry materialization
  -> recursive Loop physicalization
  -> profile edges and After seal
  -> Tail operand and Completion claim
  -> profile ledger close
  -> CFG / semantic / If / identity / Binding SSA close
  -> PhiTxn commit
  -> ReadyFunctionDraftSealV1
  -> detached DraftSeal checks
  -> operand -> required lifecycle Finish -> existing Return
  -> DraftSeal commit
```

The common physicalizer never writes Return, takes the function, publishes a
draft, or closes the module.

## Typed rejection boundary

Reject before Builder effect for at least:

- foreign owner, origin, source kind, frame, scope, region, package, target,
  ABI, plan stamp, or invocation cohort;
- missing, duplicate, foreign, stale, or unconsumed logical relation;
- Recipe item/block mismatch or JoinSig port/edge mismatch;
- input without an exact preheader producer;
- Loop After confused with callable Tail;
- missing/unsupported Completion or Return ABI;
- second Recipe/SSA/CFG/PHI/Completion/Return/publication owner;
- physical IDs or runtime authorities present in a semantic demand;
- unsupported recursive operation or exit shape.

After physical effect begins, every rejection is terminal for that unpublished
candidate. It is not reclassified as a pre-effect decline.

## One recursive algebra

The common physicalizer accepts one recursive Recipe algebra. V2 extends the
typed operation/value vocabulary; it is not converted to V1 and does not
create another physicalizer.

```text
source profiles/adapters: many bounded rows
portable Recipe algebra:  one, with exact V1/V2 projections
prepared profile shells:  bounded compatibility products
full operation demand:    one
common physicalizer:      one
```

The historical 19 route labels are coverage evidence only. They are not
physical variants or a scheduler. Unsupported shapes remain `NoSafeSlice`;
they do not mint a 20th Recipe kind or select a legacy lowerer.

## S6C TextEq physical corridor

### Source and representation law

S6C TextEq is the existing source binary operation:

```text
Equal(Text, Text) -> Bool
```

It is not a hidden `StringEquals/1` call and is non-faulting. The source chain
co-seals:

```text
Length(subject)
Substring(subject, i, i + 1) -> V9
TextEq(V9, needle) -> V10
If(V10)
Return canonical source index i
```

The fast route uses two immutable content roots already admitted by the exact
Text formal Residence:

```text
root 0 = Subject
root 1 = Needle
V9     = derived UTF-8 scalar slice(root 0, byte_offset, scalar_width)
```

V9 is not a root, runtime object, handle, pointer, or independently published
MIR value in this route.

### Canonical hot-loop shape

```text
preheader:
  subject_byte_len = ByteLen(root 0)
  byte_offset = 0
  canonical source i = 0

loop:
  V5    = byte_offset < subject_byte_len
  width = Utf8WidthAt(root 0, byte_offset)
  V10   = Utf8ScalarSliceEqWholeText(root 0, byte_offset, width, root 1)
  if V10 -> Return canonical i
  byte_offset += width
  canonical i += 1
```

Only `byte_offset` is fast-route-private state. Source `i` remains owned by
canonical Binding SSA and is the value read by the existing Return path.

The hot loop must contain zero:

- registry lock or generation validation;
- LeaseSet/Residence enter/finish work;
- allocation/deallocation or substring handle publication;
- callback, trait-object dispatch, indirect/extern call, fallback, or retry;
- `nyash.string.eq_hh` or `StringBox::equals`.

### Runtime lifetime boundary

The current caller-zero route reuses `TextFormalCallResidenceV1` and
`hako.pinned_text_backend_frame@2`. Subject and Needle are validated/pinned at
entry; Finish occurs on every explicit normal value Return and never on the
Trap path or inside the hot loop.

The exact concrete `StringBox` fast admission depends on the tracked
mutable-reachability census and guard. If a sanctioned mutation or pointer
escape to the same registry-held box is found, the route returns to
`NoSafeSlice` and an immutable-backing design is required. A new frame,
`Arc<str>` migration, V9 root, snapshot fallback, or second finish owner is not
part of the current route.

### Lifecycle and carrier law

The real caller-zero candidate has exactly:

```text
one entry-owned PinnedTextResidenceEnter
one successorless PinnedTextResidenceTrap
one PinnedTextResidenceFinish immediately before every explicit value Return
zero Finish on Trap or any other block
```

The source-issued carrier keeps signature, frame, plan, target, ABI, root map,
Enter/Trap sites, and exact exit inventory in one lineage. The direct carrier
constructor is test-only. A separate small projection seam validates the
carrier against the detached function and installs it once; MIR observation
there validates already-issued sites and does not issue meaning.

Strict MIR JSON transport is carrier-gated. General backend allowlists are not
changed to claim lifecycle support. Missing/foreign/duplicate carrier,
duplicate/non-entry Enter, duplicate Trap, Trap Finish, missing/extra Return,
Finish ordering drift, plan census drift, and refresh/synthetic promotion all
reject before JSON publication.

### Explicit non-authorities

None of the following owns S6C TextEq meaning, root identity, backing
lifetime, or fast-route admission:

- raw handle/slot/generation/pointer/token;
- `ValueId`, block number, MIR adjacency, JSON op count, or target layout;
- `SourceBoundV9RuntimeResultV1` and `EndAuthorizedTextV1` correctness canary;
- `StringBox::equals` or `nyash.string.eq_hh`;
- synthetic textual LLVM fixture or C symbol name;
- backend success, object emission, or benchmark victory.

## Current execution brief

Row: `TEXT-FORMAL-PINNED-RESIDENCE-BACKEND-FINAL-MODULE-CLOSURE-I0`
Kind: one bounded selected-backend route; no new semantic receipt.

Change:
  Add one private child that consumes the already-preflighted selected
  lifecycle expectation and checks the same parsed `LLVMModuleRef` after exact
  target/layout installation. Run `LLVMVerifyModule`, close exact lifecycle
  declarations/calls/Trap/Return attributes, then invoke the existing sole
  `LLVMTargetMachineEmitToFile` without an intervening pass or mutation.

Contract:
  Rust carrier/preflight owns expected sites; the versioned runtime header owns
  call effects; the final-module child only validates their physical projection.
  It does not rescan JSON, infer meaning from symbol counts, run a pass, emit a
  second object, or create a post-codegen receipt. File ingress remains unchanged.

Done:
  Positive real-candidate closure reaches one nonempty test object. Invalid
  module, target/layout, lifecycle count/site/order/attribute, EH construct, or
  late mutation rejects before emission with object/temporary count zero.
  Prescan is unchanged, generic owner remains below 760 lines, and the final
  closure lives in a private child below the split trigger.

Stop:
  If the preflighted expectation cannot reach the parsed module without a
  second ledger/JSON scan, or if any pass/module mutation must follow closure,
  return to design. Object readers, runtime changes, new semantic receipts,
  fallback/retry, production edges, and unrelated backend parity stay closed.

### Closed object-observer design

The failed probe did not expose an LLVM18 capability gap. It treated the
mandatory unnamed ELF `SHT_NULL` section as an empty iterator. The real object
has 11 sections; `.rela.text` exposes one Enter and two Finish relocations, and
the X86 disassembler consumes all 381 `.text` bytes. A modern reader would also
have to retain its borrowed MemoryBuffer until after `LLVMDisposeBinary`, use
`LLVMGetRelocationSymbol` rather than the always-empty LLVM18 relocation-value
string, and initialize the X86 disassembler explicitly.

That API is intentionally not added to the compiler route. Tail merging, block
rotation, shared epilogues, and cold splitting may legally change relocation
and `ret` counts. The real object also demonstrates that address-backward
branches do not identify natural loops and that ordinary `.eh_frame` presence
does not prove unwind. Treating those fingerprints as publication conditions
would create a second machine-shape authority and a target-specific decoder/CFG
layer. The selected boundary is therefore:

```text
carrier-bound preflight
  -> exact private LLVM projection
  -> parse one LLVMModuleRef
  -> target/layout + final module closure
  -> one trusted LLVM18 EmitToFile
  -> temporary exists
  -> rename
```

Object/disassembly/link/run observations belong only to the later promotion
gate. They can stop promotion, but cannot reclassify compiler semantics.

### Accepted selected C consumer boundary

After the split lands, the next BoxCount stays on the existing selected route:

```text
hako_llvmc_compile_json_pure_first
  -> compile_json_via_pure_first_lane
  -> compile_json_compat_pure
  -> carrier-bound strict preflight
  -> generic lowering
  -> retained LLVM18 target-machine session
```

The preflight consumes one Rust-issued frame/carrier cohort before prescan,
session open, or IR-file publication. It validates the exact root mapping,
plan stamp, target/ABI, 3 `PinnedTextOp`, 1 Enter, 1 Trap, and 2
Finish-immediately-before-Return sites. C never re-issues Subject/Needle, V9,
cursor, source binding, or exit meaning. Missing, extra, foreign, stale, moved,
or unknown lifecycle rows reject without compat replay, synthetic-fixture
promotion, fallback, retry, `.ll`, or object publication.

## Ordered remaining tasks

Promotion has one verdict but six bounded evidence cells. The compiler never
reads that verdict: source/Facts/Recipe/Join and the final parsed LLVM module
remain the semantic and physical authorities. Object shape, disassembly, C
code, and benchmark wins may stop promotion but cannot issue or reclassify
meaning.

```text
Decision: Keep one promotion verdict and split evidence production into
correctness, structural-zero, exact, meso, whole-call, then aggregation only.
Source authority + canonical issuer: Existing source/Facts/Recipe/Join carrier
and final parsed LLVMModuleRef remain authoritative; the gate consumes evidence
from the exact candidate/commit only.
Non-authority: JSON/op counts, reloc/ret counts, object sections, disassembly,
benchmarks, C code, pointers, handles, and the report issue no compiler meaning.
Fail-fast boundary: One commit/target/LLVM18/config must pass the correctness
corpus, IR plus final-linked structural-zero, and fixed comparable performance
thresholds; any drift stops before production without fallback or retry.
Smallest next slice: S6C-PINNED-CORRIDOR-MESO-BENCH-R0 measures the S6C scan
body without Residence Enter/Finish across fixed lengths and match positions.
Non-claims: No production caller, old-V9 or eq_hh retirement, generic fastpath,
kernel syntax, new semantic receipt, fallback/retry, or C-speed claim.
```

| order | bounded task | kind | exit condition |
| ---: | --- | --- | --- |
| 1 | `TEXT-FORMAL-PINNED-RESIDENCE-SELECTED-C-DISPATCH-SPLIT-R0` | BoxShape | **Landed.** Existing `call` / `mir_call` arms live once in a 114-line child; the parent remains below the 760-line split trigger and lifecycle opcode support remains 0. |
| 2 | `TEXT-FORMAL-PINNED-RESIDENCE-SELECTED-C-PREFLIGHT-I0` | BoxCount | **Landed.** One real module passes exact frame/carrier/parameter/root/plan/op/site parity before effect; drift rejects without `.ll`, object, replay, or fallback. |
| 3 | `TEXT-FORMAL-PINNED-RESIDENCE-SELECTED-C-TEXTUAL-LOWERING-I0` | BoxCount | **Landed.** One private draft lowers the three leaves plus Enter/Trap/Finish, passes its verifier, is discarded, and returns the stable target-closed tag. |
| 4 | `TEXT-FORMAL-PINNED-RESIDENCE-SELECTED-C-TARGET-MACHINE-I0` | bounded route | **Landed.** Verified owned bytes reach the retained LLVM18 target/layout session and one test-owned object; failures remove every temporary. |
| 5 | `TEXT-FORMAL-PINNED-RESIDENCE-BACKEND-OBJECT-OBSERVER-D0` | design stop | **Closed.** The probe failure was an ELF null-section false negative. LLVM18 Object/Disassembler works, but no compile-path observer is selected; machine evidence belongs to promotion. |
| 6 | `TEXT-FORMAL-PINNED-RESIDENCE-RUNTIME-CALL-CONTRACT-I0` | BoxCount | **Landed.** Existing Enter/Finish signatures and no-unwind/fail-stop effects are one runtime-owned ABI projection consumed by selected LLVM; successful Finish stays returnable and no new receipt/runtime object exists. |
| 7 | `TEXT-FORMAL-PINNED-RESIDENCE-BACKEND-FINAL-MODULE-CLOSURE-I0` | bounded route | **Landed.** A private child checks the same parsed `LLVMModuleRef` with `LLVMVerifyModule`, exact selected lifecycle/call attributes, and target/layout immediately before the existing sole emit; no pass or module mutation follows closure, and failure publishes no object. |
| 8a | `S6C-PINNED-CORRIDOR-LINK-RUN-CORRECTNESS-R0` | evidence | **Landed.** The real linked candidate matches an independent code-point oracle for empty, ASCII, UTF-8 2/3/4-byte, mixed, combining, multi-scalar needle, alias, stale/foreign/non-Text/retirement-pending, match/miss, and lifecycle cases. Input generations come only from a default-off test issuer in the allocation transaction. |
| 8b | `S6C-PINNED-CORRIDOR-STRUCTURAL-ZERO-R0` | evidence | **Landed.** A compile-time-only hook borrows the same closure-verified ModuleRef before sole emit; final IR and linked assembly show only lifecycle calls, entry-only allocation/root projection, exact align-1 byte reads, and zero EH/noalias/wide read/indirect or helper call. Machine layout counts remain non-authority. |
| 8c | `S6C-PINNED-CORRIDOR-EXACT-BENCH-R0` | evidence | **Landed.** The exact real plan is projected by the sole production leaf emitter into a separate evidence callable. The first unchanged-threshold run exposed eager post-mismatch loads; `LLVM-PINNED-TEXT-SCALAR-EQ-SHORT-CIRCUIT-I0` made bytes 2..4 reachable only after prior equality. The unchanged 51-pair gate is green (ASCII max p50 1.061, mixed max p50 1.078, all-case max p95 1.113 versus 1.10/1.15/1.30). |
| 8d | `S6C-PINNED-CORRIDOR-MESO-BENCH-R0` | evidence | **Measured red; promotion remains closed.** The selected scan schedule reduced the uncontended max from 3.856 to 1.239, with the remaining worst case `width4/1MiB/first`. `PINNED-TEXT-SCALAR-EQ-DIRECT-BRANCH-BOXSHAPE-R0` is the sole bounded repair before rerunning the unchanged gate. |
| 8e | `S6C-PINNED-CORRIDOR-WHOLE-CALL-BENCH-R0` | evidence | ABI+Enter+projection+loop+Finish+Return reports short-input cost and break-even; 4KiB+ p50 <=1.20, p95 <=1.30, and current-route delta separately. |
| 8f | `S6C-PINNED-CORRIDOR-PROMOTION-R0` | verdict | One test/perf-only aggregate consumes exact commit/toolchain/environment/corpus evidence from 8a-e and measures nothing itself; any missing or red leaf means no promotion. |
| 9 | `S6C-PINNED-CORRIDOR-PRODUCTION-I0` | production cutover | One named production edge switches before effect; old S6C V9 CallOut fast edge retires atomically; fallback/retry stays zero. |
| 10 | `EQ-HH-RETIREMENT-R0` | independent cleanup | Generic C/Python `nyash.string.eq_hh` caller census reaches zero independently. |

Tasks 6 and 7 are separate: runtime call effects must be owned before the final
module can verify their projection. Module closure, promotion, and production
remain serial; none may infer semantics from object success.

### Accepted `S6C-PINNED-CORRIDOR-MESO-PROJECTION-D0`

```text
Decision: Use a promotion-only final-IR outline. Delete only the exact
Residence/lane shell; do not add a canonical scan-body issuer for evidence.
Source authority + canonical issuer: The source/Facts/Recipe/Join chain and
closure-verified final ModuleRef are the only current scan issuers.
Non-authority: Leaf plans, ValueIds, labels, C code, timings, outlined helper,
assembly, and evidence reports cannot issue CFG or compiler meaning.
Fail-fast boundary: Remove exactly the old signature, 13-instruction Enter
block, 2-instruction Trap block, 8 root-projection instructions, and two
Finish calls. The current selected schedule preserves 21 blocks, 59
instructions, 31 edges, 6 PHIs, and two Returns by normalized graph digest;
any synthesis or drift is NoSafeSlice.
Smallest next slice: S6C-PINNED-CORRIDOR-FINAL-IR-SHELL-OUTLINE-I0 implements
only the offline projector, digest parity, mutation negatives, and cleanup.
Non-claims: No optimizer, receipt, runtime ABI, production, fallback/retry,
whole-call result, generic fusion, or promotion verdict.
```

The helper entry is the existing normal landing. Its four test-only formals
use the existing root SSA names, so retained scan operands are not rewritten.
The outlined object remains production caller zero. If shell/body interleave,
a retained value depends on lane/frame/status state, a removed block supplies
a retained PHI incoming, or any LLVM cleanup pass is required, stop rather
than rebuilding from the three leaf plans. A canonical physical scan-body
issuer is reserved for a future genuine production consumer, not introduced
for one evidence row.

The remaining 8d order is deliberately split:

1. **Landed** `S6C-PINNED-CORRIDOR-MESO-OUTLINE-PARITY-R0`: link the unchanged real
   candidate and outlined helper, lend both the same Residence-acquired roots,
   and require identical first-code-point index or `-1` for empty, widths 1–4,
   mixed, first/middle/last/miss, and alias cases.
2. **Measured red** `S6C-PINNED-CORRIDOR-MESO-BENCH-R0`: the fixed corpus and
   unchanged oracle first produced max 3.856. The selected scan schedule
   reduced the uncontended result to 1.239 at `width4/1MiB/first`; 8e stays closed.
3. **Landed**
   `LLVM-PINNED-TEXT-SCAN-SHORT-CIRCUIT-SCHEDULE-I0`: ordered ASCII/2-byte
   WidthAt, branchless 3/4 tail, cached lead, and direct equality ladder are
   correctness/exact/structural/parity green.
4. **Landed** `PINNED-TEXT-SCALAR-EQ-DIRECT-BRANCH-BOXSHAPE-R0`: the strict
   scalar equality consumes its sole adjacent Branch directly. The retained
   graph is 20 blocks/57 instructions/29 edges/5 PHIs/2 Returns, digest
   `ea07b0aa8b57`; the unchanged meso worst improved to 1.174594 but remains red.
5. **Measured red** `S6C-PINNED-CORRIDOR-MESO-ALIGNMENT-CONTROL-R0`: both
   functions are 64-byte aligned and body hashes recorded, but the unchanged
   gate remains red at `mixed/4KiB/first = 1.170296`.
6. **Design stop** `S6C-MESO-HWCOUNTER-SEPARATE-ARM-D0`: the current WSL2
   virtual PMU and mixed-arm process cannot issue owner evidence. A native
   Linux separate-arm protocol is required before any further lowering change.

### Accepted scalar-equality schedule repair

```text
Decision: Replace the selected scalar-equality width switch and duplicated
byte-0 arms with one cached WidthAt lead-byte compare and a direct 1-to-4
short-circuit ladder.
Source authority + canonical issuer: Existing selected preflight already
co-seals equality.byte_offset == WidthAt.byte_offset and equality.width ==
WidthAt.dst; the sole selected leaf emitter consumes that relation.
Non-authority: Assembly counts, C scheduling, benchmark ratios, SSA spelling,
block labels, and the outlined helper issue no source or compiler meaning.
Fail-fast boundary: Accept only that preflighted same-cohort plan relation;
width N reads exactly N bytes, later reads require prior equality, and switch,
indirect branch, helper, overread, noalias, and extra call stay zero.
Smallest next slice: LLVM-PINNED-TEXT-SCAN-SHORT-CIRCUIT-SCHEDULE-I0 changes
only the selected leaf schedule and focused structural evidence, then reruns
exact, link/run, structural-zero, and the unchanged meso benchmark.
Non-claims: No LLVM pass pipeline, MIR/source/Facts/Recipe/ABI/frame change,
production switch, fallback/retry, threshold relaxation, or C-oracle rewrite.
```

The real same-session object has no loop-carried stack spill. The landed-shape
candidate now has no indirect width dispatch or duplicate lead load; the
uncontended remaining red is the one-scalar success path, where true/false are
materialized through a Bool PHI and immediately branched again.

### Accepted direct-Branch projection

```text
Decision: Project the selected scalar equality's true/false terminals directly
to its sole adjacent Branch targets; suppress exactly that generic Branch.
Source authority + canonical issuer: Source/Facts/Recipe/Join already co-seal
V9 -> TextEq(V10) -> If(V10), and the canonical cursor CFG remains the sole MIR
Branch issuer. Selected preflight consumes only that strict physical projection.
Non-authority: JSON adjacency, ValueIds, plan index, assembly, timings, and the
outlined helper cannot issue source, CFG, or exit meaning.
Fail-fast boundary: Before LLVM effect, census every function use of equality
dst; require exactly one same-block terminal Branch condition, exact targets,
existing blocks, plan relation, and lifecycle/exit parity. Otherwise reject.
Smallest next slice: PINNED-TEXT-SCALAR-EQ-DIRECT-BRANCH-BOXSHAPE-R0 adds the
preflight census and one exactly-once selected Branch consumer, then reruns all
focused gates and the unchanged meso benchmark.
Non-claims: No MIR vocabulary, source/Recipe, DraftSeal, Residence, generic
peephole, production selector, C oracle, threshold, fallback, or retry change.
```

Generic `emit_branch` remains the normal owner. Only the preflighted selected
pair may bypass Bool materialization. Extra/copy/PHI/Return/call use,
non-adjacent or non-terminal Branch, foreign targets, and duplicate/missing
consumption reject before publication. If exhaustive use census cannot be
closed without inferred semantics, this row is `NoSafeSlice`.

### Accepted meso alignment control

```text
Decision: Do not add another lowering optimization from the width3/4KiB/first
red; it returns after one scalar and current assembly has no identified deficit.
Source authority + canonical issuer: The unchanged real candidate, independent
C oracle, and existing paired meso runner remain the sole evidence issuers.
Non-authority: One p50, input allocation size, function address, assembly size,
and benchmark placement cannot issue compiler or semantic meaning.
Fail-fast boundary: Give both linked functions the same 64-byte alignment and
record address modulo 64 plus disassembly hashes; keep CPU/toolchain/51 pairs,
30ms arms, corpus, oracle, and p50 <= 1.15 unchanged.
Smallest next slice: S6C-PINNED-CORRIDOR-MESO-ALIGNMENT-CONTROL-R0 changes only
the evidence harness and reruns the existing gate.
Non-claims: No threshold relaxation, first-case exclusion, C rewrite, lowering,
MIR/source/production change, fallback, or promotion verdict.
```

If the equal-alignment run stays red, preserve the red result and return to a
hardware-counter/linked-layout design audit. Do not guess another LLVM schedule.

### Accepted clean-pair acquisition D0

```text
Decision: Adopt bounded complete-pair eligibility sampling; only scheduler
contamination may refill one predeclared accepted slot, never a fatal failure.
Source authority + canonical issuer: One hashed acquisition plan fixes binary,
corpus, CPU, events, 3x51 accepted slots, 68 attempts per block, ordering,
eligibility, interval, and classifier; the outer collector is the sole issuer
of AcceptedPair or SchedulerRejectedPair from two complete arm observations.
Non-authority: One arm, a partial pair, rejected PMU counts, attempt count,
stderr, old JSON, timing wins, and PC profiles issue no ratio or owner.
Fail-fast boundary: Scheduler metadata alone may reject a whole completed pair;
fatal identity/corpus/oracle/PMU/output drift stops the invocation immediately.
Rejected IDs cannot enter ratios; 18 rejects, attempt 69, order drift, or an
incomplete ledger closes a non-evidence NoSafeSlice terminal receipt.
Smallest next slice: S6C-MESO-HWCOUNTER-CLEAN-PAIR-ACQUISITION-I2 changes only
the C raw observation, private acquisition ledger, terminal receipt, and gates.
Non-claims: No causal PC owner, backend BoxShape, threshold/corpus/oracle change,
production switch, fallback, promotion verdict, or generic profiler authority.
```

The first native report (`601f45fe...afc59c`) found stable Hako/C ratios near
cycles 1.143, instructions 0.960, and branches 1.125 across 3x51 pairs. It is
provisional: its scheduling software events were user-filtered, manifest
identity was incomplete, and both arms could share a drifted corpus. The I1
repair replaces those premises with `getrusage`, exact affinity, a private
frozen binary, full manifest/commit binding, fixed 4096-byte/1642-scalar corpus
and classifier matrices. The old JSON cannot authorize A0. Integrity R0 at
`915e62f1fb22` then stopped before publication on a C-arm context-switch or
affinity drift (binary `fda6e59a...c269db`, build-id `0547a55c...4cec0`); this
is the accepted current result, not permission for an unbounded retry.

I2 landed the fixed accepted order: block 0/2 start AB,
block 1 starts BA, for total AB=77 and BA=76. A contaminated first arm never
skips the second; both epochs and an arm-envelope `getrusage`/affinity census are
recorded. The same slot/order is retried, and arms from different attempts can
never pair. One chronological attempt ledger is the SSOT; block summaries hold
only accepted/rejected attempt IDs. Ratios consume exactly 51 accepted IDs per
block, preserve df=50 log-ratio t95, and never pool all 153 observations.

The terminal artifact is issued only after commit/binary/corpus/oracle preflight
has produced the acquisition plan. Accepted and bounded NoSafeSlice outcomes
use one schema with `evidence_eligible`; failures before plan issuance publish
nothing rather than fabricating a receipt. The 243-line private acquisition
owner keeps the collector and C observer below 760 lines; self-tests cover clean,
17-rejection, 18th-rejection, fatal, missing-arm, interval/classifier, identity,
and atomic-publication negatives. Native R0 remains one explicit invocation.

Ordered task ladder:

1. `S6C-MESO-HWCOUNTER-SEPARATE-ARM-D0/I0`: landed evidence foundation.
2. `S6C-MESO-HWCOUNTER-EVIDENCE-INTEGRITY-I1`: landed repair; no compiler path.
3. `S6C-MESO-HWCOUNTER-EVIDENCE-INTEGRITY-R0`: accepted NoSafeSlice; no JSON.
4. `S6C-MESO-HWCOUNTER-CLEAN-PAIR-ACQUISITION-D0`: accepted as above.
5. `S6C-MESO-HWCOUNTER-CLEAN-PAIR-ACQUISITION-I2`: landed evidence-only.
6. `S6C-MESO-HWCOUNTER-CLEAN-PAIR-ACQUISITION-R0`: run once on native Linux.
7. `S6C-MESO-HWCOUNTER-PC-ATTRIBUTION-A0`: if R0 reproduces one driver, collect
   7+ AB/BA arm-separated branch profiles for the exact build-id and normalize
   raw IPs to symbol-relative offsets. A single branch region must own >=70% of
   the Hako excess in >=5/7 pairs; otherwise return `NoSafeSlice`.
8. `S6C-MESO-RESIDUAL-OWNER-D0`: open one backend BoxShape only when A0 names
   one owner; otherwise retain the performance `NoSafeSlice`.

The StringBox admission premise remains a release gate: every `as_any_mut` caller, `Arc` uniqueness/recovery path, sanctioned
extern/C provider, nowait/task sharing path is part of the
proof obligation for the theorem that no in-scope path can mutate pinned bytes.
The current census finds no sanctioned path; an unclassified external unsafe provider remains
a stop, never permission to assume immutability.

## Compact landed ledger

Exact prose and superseded stops are historical. The durable result is:

| boundary | current disposition |
| --- | --- |
| source/Facts/Core/Recipe/Join co-seal | landed source authority |
| common V2 pre-session and installed Port HRTB | landed caller-zero |
| physical-ID-free layout and common operation dispatcher | landed caller-zero |
| source-backed If/Return-read and shared segment scope | landed caller-zero |
| S6C Subject/Needle base-root admission | landed caller-zero |
| UTF-8 cursor/preheader and scalar equality leaf plans | landed caller-zero |
| canonical Binding SSA index plus private byte-offset CFG | landed caller-zero |
| function-owned pinned Text plan table/frame timing | landed caller-zero |
| explicit Enter/Trap physical vocabulary | landed caller-zero |
| exact-two Finish -> existing Return detached projection | landed caller-zero |
| real candidate source-issued carrier lineage | landed caller-zero |
| carrier-gated no-refresh strict MIR JSON | landed caller-zero at `44e4df38a0` |
| selected C generic dispatch call-family split | landed behavior-neutral BoxShape |
| selected C carrier-bound preflight | landed effect-free BoxCount |
| selected C textual lifecycle lowerer | landed caller-zero BoxCount |
| selected C TargetMachine handoff | landed caller-zero bounded route |
| post-emit temporary-object observer | probe false negative corrected; compile-path observer intentionally absent |
| runtime Enter/Finish call-effect contract | landed caller-zero BoxCount |
| final LLVM module closure | landed caller-zero immediately before sole trusted emit |
| real candidate link/run correctness | landed offline evidence; production ABI unchanged |
| final IR / linked assembly structural zero | landed offline evidence; compiler never consumes it |
| production | closed |

The current production selector remains selected-Dynamic. Generic G0 and S6C
lifecycle candidates are caller-zero evidence until an explicit cutover row.

## Evidence and reusable guards

The latest landed real-candidate JSON slice has:

- `pinned_text_real_candidate_json`: 2 passed;
- `residence_backend_carrier`: 2 passed;
- pinned Text plan-census JSON test: 1 passed;
- `cargo check --profile quick --lib` green;
- fmt and `git diff --check` green;
- `current_state_pointer_guard.sh` green;
- `common_v2_s6c_structure_guard.sh` green;
- `text_formal_residence_finish_or_abort_abi_guard.sh` green;
- `pinned_text_residence_carrier_lowering_smoke.sh` green.
- `pure_compile_generic_dispatch_split_guard.sh` green at parent 714 / child 114;
- selected carrier-bound preflight guard and runtime-generated JSON smoke green;
- selected private textual lowerer verifier and ordering negative green;
- selected runtime-call `nounwind` positive and forbidden-effect negative green;
- `text_formal_residence_finish_or_abort_abi_guard.sh` and selected preflight
  guard cover the versioned header-to-LLVM projection;
- selected LLVM18 memory ingress emits a nonempty object; invalid IR and
  unwritable-output negatives leave no temporary artifact;
- same-final-ModuleRef and final-linked structural-zero smoke rejects unknown
  calls, `noalias`, wide reads, indirect calls, and evidence-publication failure;
- byte-for-byte extraction comparison against the pre-split parent green;
- checked-callout physicalizer and pure-first route preflight green;
- same-module static-helper row guard is baseline-red because its tracked
  proof-app script lacks the executable bit both before and after this split.

Local green proves only the named caller-zero slice. It does not prove the
post-transform observer, production selection, or C-speed performance.

## Documentation and history boundary

This stable path remains the current authority. It now contains only durable
contract, live execution brief, bounded queue, and compact landed status.

The former append-only D0/I0 consultation and closeout body is indexed by the
[historical ledger](archive/loop-common-physical-demand-and-session-history-2026-08-18.md).
The exact 5,108-line pre-compaction snapshot is commit `44e4df38a0`; use
`git show 44e4df38a0:docs/development/current/main/design/loop-common-physical-demand-and-session-ssot.md`
when exact historical prose is required.

Archived decisions cannot select a task, mint a receipt, or change the current
blocker. Do not restore landed detail to this file. New progress updates only:

1. the Current Capsule;
2. the one active six-line design/execution brief;
3. the ordered task table if the dependency graph changes;
4. one compact landed-ledger row after a real boundary lands.

Code, focused tests, reference pages, and git history remain the evidence
owners for implementation detail.
