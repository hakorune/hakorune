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
- **Current blocker:** the selected C route emits a real caller-zero object,
  but its Enter/Finish no-unwind call contract is not yet explicit in the
  selected LLVM declarations. Post-codegen object inspection is promotion
  evidence, not a compiler acceptance authority.
- **Next ordered task:**
  `TEXT-FORMAL-PINNED-RESIDENCE-RUNTIME-CALL-CONTRACT-I0`.
- **Production stop line:** final-module closure, production selection,
  fallback/retry, performance promotion, and
  `nyash.string.eq_hh` retirement remain closed.

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

Row: `TEXT-FORMAL-PINNED-RESIDENCE-RUNTIME-CALL-CONTRACT-I0`
Kind: one bounded call-effect BoxCount; no new semantic receipt.

Change:
  Make the existing runtime-owned ABI explicit once: Enter returns an exhaustive
  status and does not unwind; Finish returns `void` on success, aborts on every
  nonzero internal status, and does not unwind. Project that contract into the
  selected LLVM declarations and delete their unannotated forms.

Contract:
  The Rust runtime implementation/export owns behavior and
  `include/nyrt_text_formal_residence_v1.h` is its C projection. The selected
  lowerer consumes the fixed signature/effect contract; it does not infer it
  from a symbol name. Finish is not globally `noreturn`, and neither call is
  `readonly`, `readnone`, `nofree`, or `speculatable`.

Done:
  The ABI guard and focused selected-lowering test prove exact signatures,
  `nounwind`, terminal nonzero Finish behavior, and absence of the forbidden
  attributes. All touched source stays below 760 lines and the real candidate
  remains caller-zero.

Stop:
  Any reachable unwind/catch path, signature ambiguity, new runtime token/frame,
  new semantic `Verified*`/`Prepared*` receipt, object reader, fallback, retry,
  production edge, or unrelated backend parity returns this row to design.

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

| order | bounded task | kind | exit condition |
| ---: | --- | --- | --- |
| 1 | `TEXT-FORMAL-PINNED-RESIDENCE-SELECTED-C-DISPATCH-SPLIT-R0` | BoxShape | **Landed.** Existing `call` / `mir_call` arms live once in a 114-line child; the parent remains below the 760-line split trigger and lifecycle opcode support remains 0. |
| 2 | `TEXT-FORMAL-PINNED-RESIDENCE-SELECTED-C-PREFLIGHT-I0` | BoxCount | **Landed.** One real module passes exact frame/carrier/parameter/root/plan/op/site parity before effect; drift rejects without `.ll`, object, replay, or fallback. |
| 3 | `TEXT-FORMAL-PINNED-RESIDENCE-SELECTED-C-TEXTUAL-LOWERING-I0` | BoxCount | **Landed.** One private draft lowers the three leaves plus Enter/Trap/Finish, passes its verifier, is discarded, and returns the stable target-closed tag. |
| 4 | `TEXT-FORMAL-PINNED-RESIDENCE-SELECTED-C-TARGET-MACHINE-I0` | bounded route | **Landed.** Verified owned bytes reach the retained LLVM18 target/layout session and one test-owned object; failures remove every temporary. |
| 5 | `TEXT-FORMAL-PINNED-RESIDENCE-BACKEND-OBJECT-OBSERVER-D0` | design stop | **Closed.** The probe failure was an ELF null-section false negative. LLVM18 Object/Disassembler works, but no compile-path observer is selected; machine evidence belongs to promotion. |
| 6 | `TEXT-FORMAL-PINNED-RESIDENCE-RUNTIME-CALL-CONTRACT-I0` | BoxCount | Existing Enter/Finish signatures and no-unwind/fail-stop effects become one runtime-owned ABI projection consumed by selected LLVM; no new receipt or runtime object. |
| 7 | `TEXT-FORMAL-PINNED-RESIDENCE-BACKEND-FINAL-MODULE-CLOSURE-I0` | bounded route | A private child checks the same parsed `LLVMModuleRef` with `LLVMVerifyModule`, exact selected lifecycle/call attributes, and target/layout immediately before the existing sole emit; no pass or module mutation may follow closure, and failure publishes no object. |
| 8 | `S6C-PINNED-CORRIDOR-PROMOTION-R0` | evidence gate | Unicode, alias, stale/foreign, exit/lifetime, link/run, IR/final-assembly structural-zero, exact/meso/whole-call, and C comparison gates pass; object tools are evidence only. |
| 9 | `S6C-PINNED-CORRIDOR-PRODUCTION-I0` | production cutover | One named production edge switches before effect; old S6C V9 CallOut fast edge retires atomically; fallback/retry stays zero. |
| 10 | `EQ-HH-RETIREMENT-R0` | independent cleanup | Generic C/Python `nyash.string.eq_hh` caller census reaches zero independently. |

Tasks 6 and 7 are separate: runtime call effects must be owned before the final
module can verify their projection. Module closure, promotion, and production
remain serial; none may infer semantics from object success.

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
| final LLVM module closure | call-effect contract then same-module verifier pending |
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
- selected LLVM18 memory ingress emits a nonempty object; invalid IR and
  unwritable-output negatives leave no temporary artifact;
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
