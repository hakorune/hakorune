---
Status: frontier_pause__NoReadyR7Owner__2026-09-23
Date: 2026-09-14
Decision: MIR-CALL-R7-STRINGBOX-SOURCE-ISSUER-D0
Parent: docs/development/current/main/investigations/mir-call-legacy-target-census-d0-2026-08-20.md
ProductionCaller: selected native ingress plus retained explicit compatibility
ReplacementCell: owner-local migration; aggregate legacy retirement remains open
---

# MIR-CALL-COMPATIBILITY-RETIRE-R7-D0

## Six-line brief

Decision: select MIR-CALL-R7-STRINGBOX-SOURCE-ISSUER-D0 under the source-contract design; preserve supported compatibility while designing the caller-local successor.
Source authority + canonical issuer: Rust AST/Program-v0 lowering is the structured source entrance, but no existing issuer binds StringBox membership to a contract. CoreMethodContractBox remains contract authority; this slice must co-seal the missing relation.
Non-authority: textual first-string/name matching, parity shape summaries, legacy MIR, and schema-spelling replacement.
Fail-fast boundary: producer disposition before MIR publication; declared-schema/direct-loader errors remain terminal without retry.
Smallest next slice: define the source-only product (owner/path, receiver, constructor argument, selector/arity) and one co-sealed consumer handoff for the existing StringBox cohort.
Non-claims: no implemented issuer/consumer, I0 permission, shared v0 reader deletion, whole-selfhost activation, or backend parity.

## Development queue (worker-audited 2026-09-13)

The earlier seven-task summary overstated direct-input and Stage-A closure.
`26e59acaef` stops declared-v1 -> v0 retry, `4e1d6f92fb` stops strict/dev
selfhost boxcall, and the bounded Stage-A I0 below closes rejection fallback.

Boundary for selected D0: `runner/mod.rs` --mir-json-file ->
`core_executor::execute_mir_json_text` -> json_artifact facade ->
`mir_loader::parse_direct_mir_json_text` -> module or named error. Includes
schema presence/absence, boxcall, malformed input and precedence; excludes
artifact umbrella intake, Stage-A, Program conversion, parser deletion and backend execution.

| Order | Task / entry | Observable finish and handoff |
| --- | --- | --- |
| 1 resolved | `MIR-CALL-DIRECT-INPUT-BOXCALL-D0` / design | Retain schema-absent MIR v0 compatibility, including ordinary boxcall, on the direct route until each real caller has its own migration/Stop. Declared schema errors remain terminal. No shared parser deletion or new flag is authorized. |
| 2 conditional | `MIR-CALL-DIRECT-INPUT-BOXCALL-I0` / only after D0 accepts Stop | Reject before module execution; delete this ingress's boxcall-to-legacy construction edge. Prove valid no-schema v0/v1, exact rejection, malformed/schema precedence and retained compatibility. Update loader README and MIR intake reference together. |
| 3a resolved | `MIR-CALL-STAGE-A-REJECTION-D0` / route, compat bridge and outer caller | Extracted `mir_line` parser errors are terminal. Preserve absent/unavailable/capture failure and no-MIR fallback behavior; do not add new capture classification. |
| 3b closed | `MIR-CALL-STAGE-A-REJECTION-I0` / accepted D0 | Propagate rejection through the outer caller and delete each accepted bypass in the same series; observed that prohibited Program/Rust/Python fallback never starts. |
| 4 resolved | `MIR-CALL-R7-JSON-V0-CALLER-DISPOSITION-D0` | Retain the phase14/17 StringBox compatibility cohort; shared v0 parser reuse is not treated as boxcall-arm dependence. |
| 4 resolved | `MIR-CALL-R7-COMPAT-ENTRYPOINT-D0` | Retain Global and dynamic Value compatibility; its old-edge delete-set is empty. |
| 4 resolved | `MIR-CALL-R7-RELEASE-SELFHOST-BOXCALL-D0` | Retain explicit release selfhost/Stage1 boxcall compatibility; its exact delete-set is empty. |
| 4 parent design | `MIR-CALL-R7-STRINGBOX-SOURCE-CONTRACT-D0` | Refined below into a source-issuer slice; compatibility remains supported and I0 stays conditional. |
| 4 selected design | `MIR-CALL-R7-STRINGBOX-SOURCE-ISSUER-D0` | Close exact source membership and the single source-to-contract issuer before consumer handoff. |
| 4 successive owner units | Remaining existing writer/reader/reissuer inventory | For each owner select Stop/Promote/Delete with finite callers, terminal, replacement and old-edge deletion. No broad recount or supported-caller deletion to manufacture zero. |
| 5 dependent | R7 schema retirement | Production writer/reader/reissuer/re-entry zero, then delete LegacyCallV0 and its exclusive repair/assets; retained compatibility must have an explicit completed disposition. |
| 6 dependent | Call/M8 physical thinning | Delete caller-zero Builder windows, wrappers and exclusive tests/guards, retaining equivalent evidence. |
| 7 dependent | Call/M9 backend retirement | Each backend's actual successor use, required evidence and caller-zero authorize its retirement. |

D0 implementation-entry checklist: name the direct loader edge and retained
`load_json_artifact_to_module`/`load_mir_json_to_module`, selfhost, Stage1 and
emitter consumers; decide invalid JSON/schema/malformed/genuine-boxcall order
without substring dispatch; reuse existing parsed admission and tests/guards.
D0 itself runs no Cargo or CI; a future I0 deletes only its caller-local edge.

Stage-A task boundary covered three live transitions: route MIR Err -> Program,
compat bridge MIR Err -> Rust bridge, and selfhost unresolved Option -> Python/
default Rust. The D0 named the error carrier and unavailable policy; acceptance
observed outer callers and forbidden fallback absence. Helper-only tests were insufficient.

The Windows temporary-input receipt below is evidence only; it does not gate
these design tasks or select lifecycle support as new work.

## MIR-CALL-DIRECT-INPUT-BOXCALL-D0 (resolved 2026-09-13)

Tombstone: resolved 2026-09-13 — schema-absent MIR v0 compatibility
(including ordinary `boxcall`) is retained on `--mir-json-file` until
each real caller reaches its own Promote/Stop row; declared v1 stays
terminal. Body retired to git history.

## MIR-CALL-STAGE-A-REJECTION-D0 (resolved 2026-09-13)

Tombstone: resolved 2026-09-13 — an extracted MIR parser error is
terminal for that mode-A invocation in every mode; no Program/Rust/
Python re-entry. Body retired to git history.

## MIR-CALL-STAGE-A-REJECTION-I0 closeout (2026-09-14)

Tombstone: closed 2026-09-14 — both mode-A helpers carry the parser
`Err` to the outer terminal (30 focused tests green; production
fixtures exit 1 with the named rejection, no fallback markers).
Body retired to git history.

## MIR-CALL-R7-JSON-V0-CALLER-DISPOSITION-D0 (resolved 2026-09-14)

Tombstone: resolved 2026-09-14 — the JSON-v0 caller cohort is retained
as an explicit supported compatibility product; the textual producer
has no replacement tuple yet. Body retired to git history.

## MIR-CALL-R7-RELEASE-SELFHOST-BOXCALL-D0 (resolved Retain; R7 frontier pause, 2026-09-14)

Tombstone: resolved Retain 2026-09-14 — release selfhost boxcall stays
supported compatibility; no bounded delete-set exists. Body retired
to git history.

## Worker-audited retained rows (2026-09-14)

Two remaining-looking rows were checked as bounded design candidates before
selecting another R7 task. Both are retained compatibility, not executable work:

| Candidate | Finite boundary and evidence | Disposition / reopen trigger |
| --- | --- | --- |
| `method.rs:517` interpreter singleton fallback | `execute_method_callee` has one production caller (`array_write.rs`) and it always supplies `Some(receiver)`; the canonical static path is `Callee::Global(StaticBoxMethod)`. The `None` branch has no independent production caller or pre-effect terminal. | Retain as `NoSafeSlice`; no exclusive delete-set. Reopen only with a caller-local Stop/migration that names the terminal and removes this edge without deleting the live receiver-present path. |
| `array_element_write.rs:251` llvmlite projection | `llvmlite_emit_obj_lib -> project_for_legacy_backend -> project_module_to_legacy_calls` is one explicit `llvmlite-compat` object route. Typed-array metadata/instruction drift is rejected before cloning/emission; the consumer has not accepted the V1 operation. | Retain as `NoSafeSlice`; old-edge delete-set is empty. Reopen only after a V1 consumer or explicit caller-local Stop supplies a successor and exact deletion. |

These retained rows remain unchanged. Their lack of an executable delete-set
does not prevent the selected StringBox source-contract design below.

## Worker-audited native Windows lifecycle gap (2026-09-14)

| state | authority / issuer | pre-effect boundary | terminal / continuation | fallback |
| --- | --- | --- | --- | --- |
| NoCandidate | lifecycle ABI/archive and `LifecycleRuntimeSessionV1::select` / `hako_lts_open` (production chain: `published_mir_object.rs` -> `compile_published_lifecycle_physical_v4` -> `hako_llvmc_compile_published_lifecycle_physical_v4`) | Windows lifecycle session admission before V4 effect/artifact | retain existing platform/session terminal; no R7 execution card | no new fallback |

```text
Decision: retain native Windows lifecycle close/reopen as BackendCapabilityMissing.
Source authority + canonical issuer: target-built lifecycle ABI/archive and LifecycleRuntimeSessionV1::select / hako_lts_open.
Non-authority: generic Windows CAPI observer, fake DLL, Linux archive, or hand-built Windows session fixture.
Fail-fast boundary: the Rust selector rejects non-Linux lifecycle triples; C V4 keeps its existing Windows session/platform terminals.
Smallest next slice: `MIR-CALL-WINDOWS-LIFECYCLE-CAPABILITY-D0` evaluates MSVC `.lib`/COFF descriptor tools, `LoadLibraryA`/`GetProcAddress`, and `_open`/`_close` V4 paths for `x86_64-pc-windows-msvc`; then `MIR-CALL-WINDOWS-LIFECYCLE-OWNERSHIP-I0`.
Non-claims: existing TempDir, provider/CAPI Windows, and generic CAPI receipts do not prove lifecycle Windows completion.
```

## Finite scope and retained owners

Boundary: canonical/compatibility ingress -> Call writers/reissuers/readers
and physical compile/link admission -> typed rejection or artifact terminal.
Includes the previously inventoried C/AOT/Static/harness callers and their
public re-entry. Excludes new source families, unrelated runtime environment,
allocator work, and unselected backend parity.

The original manifest observed 243 production lexical rows in 127 files and
four environment anchors. These are historical observation counts, not fresh
caller-zero evidence. Its known anchor drift is listed under evidence debt.

| Responsibility | Current owner / terminal | Disposition and reopen condition |
| --- | --- | --- |
| Selected typed Call | existing Facts/Recipe -> selected physical publication | retain; a live legacy bypass reopens the exact cutover |
| JSON v0 release compatibility | existing parser `boxcall` -> compatibility terminal | retain; strict/dev outer ingress already stops before parsing/effects |
| Mechanical reissuers and shared readers | existing remapper/serializer/backend owners | delete only after their affected callers switch or stop |
| Rust Boundary and Static options | versioned options request -> C invocation | closed transport; retain supported public adapters |
| AOT and public C Generic options | profile-0 request -> existing options copy/lowering | closed transport; preserve compatibility admission |
| Named C/AOT harness | explicit named entry -> child/provider/error | retained; next change needs its own caller-local deletion |
| Explicit Rust harness | explicit route -> llvmlite provider | retained shared provider; public/provider presence is not caller-zero |
| AOT link v1/v2 and dlsym | existing archive request -> shared link terminal | direct seam closed; retain public link ABI and FFI split |
| Legacy TargetMachine probe | invocation-captured compatibility choice -> probe/opt/llc | selector capture closed; probe remains explicit compatibility |
| Published call rows | Static V2 -> invocation ledger -> peek/take/finish/end | closed at `8fe3f00a6c`; preserve borrowed lifetime and typed validation |

A shared owner is permitted. Its other callers prevent deleting the whole
owner, but do not prevent switching a finite caller and removing that caller's
old edge. Historical `NoRemainingUnsharedM7SOwner` wording must not be used as
a requirement to manufacture a new owner or stop all development.

Aggregate retirement still requires affected production writers, reissuers,
readers and re-entry paths to reach caller-zero, with no unconsumed replacement
product. Keep `LegacyCallV0` and shared compatibility assets until then.

## MIR-CALL-COMPATIBILITY-ADMISSION-D0 (historical frontier pause)

The pause and selection statements here are historical and are superseded by
the owner-specific rows above and below. FAST capture, TargetMachine capture,
published rows, child environment, and the C/AOT compatibility frontier were
closed or retained at their recorded commits; their shared callers are not a
new task. The old outcome was
`NoSafeSlice__RetainedCapiOwnersNoExclusiveDeleteSet`: preserve existing
entry/terminal contracts and reopen only with a finite caller/terminal/delete
tuple. Closed detail remains in Git; do not repeat that census or promote a
parked row.

## MIR-CALL-COMPATIBILITY-RUST-LLVMLITE-REQUEST-D0 (accepted; I0 selected)

Tombstone: accepted, I0 landed — one invocation-scoped request policy
with three existing consumers; ambient reads deleted. Body retired
to git history.

## MIR-CALL-COMPATIBILITY-RUST-LLVMLITE-REQUEST-I0 closeout (2026-09-13)

Tombstone: closed 2026-09-13 — focused route and C contract evidence
green; ordered queue row 3 records the receipt. Body retired to git
history.

## MIR-CALL-TEMP-INPUT-OWNERSHIP-D0 (accepted)

Tombstone: accepted, I0 landed — the three Rust preparation sites hold
one scoped `BackendInputJsonFile` (private TempDir) through child/CAPI
consumption; the shared `hako_llvm_in.json` edge is deleted. Body
retired to git history.

## MIR-CALL-FAST-INVOCATION-CAPTURE-D0 (accepted)

Tombstone: accepted, I0 landed — FAST is captured once per compile
invocation in `hako_llvmc_ffi_invocation.inc`; the three GNU-helper
readers and the ambient zero-argument helper were switched/deleted
atomically. Body retired to git history.

## MIR-CALL-FAST-INVOCATION-CAPTURE-I0 closeout (2026-09-13)

Tombstone: closed 2026-09-13 — invocation-scoped FAST capture landed;
all three readers migrated and the ambient helper deleted; focused C
and contract-smoke evidence green. Body retired to git history.

### R7 frontier status (live)

R7 frontier pause — 2026-09-23: no remaining owner has a ready Promote/Stop/Delete tuple (live consumer, successor/terminal, same-series production deletion).
Remaining candidates are sealed as `NoSafeSlice`, supported Retain, closed, or deferred to their owning lanes.
Do not repeat the inventory or reopen retained lanes; resume only after a changed caller/contract supplies the complete tuple.
Aggregate R7 retirement remains open.

R7 frontier refresh — 2026-09-25 (S4–S9 sequence): all production
`LegacyCallV0` ingress minters retired (S4 Value, S5 boxcall), so the
caller-zero evidence changed and four bounded deletions landed:
S6 canonicalize Method arm + helpers, S7 `canonicalize_legacy_array_write_calls`
+ refresh call site, S8 canonicalize Closure arm, S9 canonicalize Global
no-op arm. The D4 worker census (recorded in
`mir-call-r7-caller-zero-d4-2026-09-25.md`) confirms the remaining
`LegacyCallV0` surface is: the interpreter `reject_legacy_call` terminal
(sole VM-lane deny boundary — `compile_normal` bypasses admission and the
verifier classifies the variant `Kept`), published-view `func`-slot named
errors, selected admission gates, the llvmlite ExplicitCompatibility
projection + v0 compat emission owners, the variant/`func` slot itself
(variant-lifetime contract), and ~140 shared structural readers. No
further bounded deletion set exists; this lane is `NoSafeSlice` until a
changed caller/contract supplies a complete tuple. Observable reopen
triggers: v0 wire retirement, llvmlite-compat lane disposition (note:
post-S7 the projection's minted rows self-reject at its own egress
`reject_residual_calls` — the lane is functionally dead on those
inputs), or any new production `LegacyCallV0` mint.

## Ordered queue

| Order | Task | Entry / finish condition |
| --- | --- | --- |
| 1 | FAST capture D0 | accepted above; design and task organization delivered |
| 2 | FAST capture I0 | closed at the current implementation commit; three readers and the old helper are aligned |
| 3 | Rust llvmlite runner request I0 | closed in this revision; one invocation policy, three existing consumers, and the exact ambient-read delete set above |
| 4 | Temporary input ownership D0 | accepted above; shared-owner premise corrected and three consumers co-scoped |
| 5 | Temporary input ownership I0 | implemented below; scoped input, all three caller switches, fixed input-path edge deleted, focused lifetime evidence |
| 6 | Named C harness log ownership I0 | implemented below; invocation-owned diagnostic log and PID-only edge retired |
| Deferred | non-Loop snapshot reacquisition | existing perf owner; prove duplicate acquisition and compatible lifetime before reuse |
| Deferred | Read/Write/Carrier unused information | owning Rust metadata paths; prove zero consumers before behavior-neutral deletion |
| Deferred | ordinary-new unclaimed writer | existing Birth/ordinary-new owner; retain direct-local, foreign/transferred and uncovered cases |

The deferred cleanup items are not silently part of the C/AOT slice.
The existing compile-time performance card owns measurement and snapshot
investigation. Metadata cleanup must preserve source identity and publication
ownership; it does not authorize a new semantic receipt.

## MIR-CALL-TEMP-INPUT-OWNERSHIP-I0 implementation

Tombstone: implemented — focused lifetime tests pass; no remaining
`build_backend_temp_input_path` / `hako_llvm_in.json` call sites.
Body retired to git history.

## MIR-CALL-COMPATIBILITY-RETIRE-R7-D1 (resolved)

Tombstone: resolved — admission is singular; the only concrete shared
edge was `hako_llvmc_build_harness_log_path`, migrated by the
HARNESS-LOG-OWNERSHIP I0 below. Body retired to git history.

## MIR-CALL-HARNESS-LOG-OWNERSHIP-D0 (accepted)

Tombstone: accepted, I0 landed — one invocation owns one diagnostic
log through child completion; `hako_llvmc_build_harness_log_path` and
the three manual remove sites are deleted. Body retired to git
history.

## MIR-CALL-HARNESS-LOG-OWNERSHIP-I0 (POSIX and native Windows direct-C/helper proof complete)

Tombstone: POSIX implementation + focused overlap/failure evidence and
the native Windows direct-C/helper proofs are complete (CI run
`34751337118` at `a274ac03b3`); lifecycle wrapper boundary stays open.
Design/detail retired to git history. The CI receipt below is kept as
irreproducible evidence.

### Collected CI receipt and independent work

Run `34759102231` at `8e7179bb7e` closes the Windows provider/generic CAPI
receipt; exact timestamps and durations are recorded above. It supersedes the
observer red from `34753858020`; it does not establish native lifecycle support,
native LLVM execution, or mixed-CRT error deallocation. The merged provider/CAPI
job shares the test build. Manual dispatch defaults to `scope=windows`; `full`
adds macOS/plugin checks. PR path filtering uses the complete PR changed-file
set, so docs-only commits do not necessarily skip Windows. Closed workflow
repair and independent A-4/G0 audit detail remain in Git at `9eee6e153d`.
No foreground polling or redispatch is required before the selected design.

## MIR-CALL-R7-STRINGBOX-SOURCE-ISSUER-D0 — selected (2026-09-14)

Tombstone: selected 2026-09-14 — bounded length/size artifact design
agreed (exact membership, co-sealed source-to-contract issuer in
`program_json_v0/authority.rs` with the module handoff type as sole
consumer); implementation delegated to the bounded parser/contract I0
successor card. Body retired to git history.

## Closed evidence and contracts

Closed detail is available with
`git show 8fe3f00a6c:docs/development/current/main/investigations/mir-call-compatibility-retire-r7-d0-2026-09-11.md`.
This compact card replaces the accumulated historical selection/closeout prose;
it does not create a second archive or change the older evidence.

| Closed boundary | Commit / existing receipt | Result |
| --- | --- | --- |
| strict/dev JSON legacy reader stop | `4e1d6f92fb` | outer ingress rejects before parser/mutation; release compatibility retained |
| direct AOT link seam | `fc19313028` | invocation archive reaches shared link body without FFI environment save/restore |
| Boundary physical options | `454e49b755` | versioned request -> copied C invocation options; focused contract smoke |
| Static V2 open options | `0bae5fdd9c` | explicit open/query/compile/close and pre-effect invalid-contract checks |
| Rust ambient transport deletion | `7350421205` | caller-zero non-explicit CAPI branch removed; five route tests and check recorded |
| ny-llvmc Boundary options | `36d7fc784f` | options entry replaces private three-argument lookup/environment override |
| AOT Generic options | existing options smoke / historical receipt | Generic profile 0 replaces private old-symbol lookup; fake-tool object evidence |
| public C Generic options | `3d3b118ccf` | public admission retained; private Generic wrapper removed |
| named compiler-path / replay | `6e5b59c901`, `34d9e409a6`, `16329a0da2` | compiler path owned; automatic replay and unused private replay storage removed |
| legacy TargetMachine selectors | `47e83a224e` | invocation capture replaces emitter environment reads; probe/fallback retained |
| AOT child environment I1 | `59e9a30b1f` | native Windows and Linux child-environment proof; details below |
| published-row owner I0 | `8fe3f00a6c` | global ledger removed; invocation-local production consumers and cleanup |

### Published-row invocation ownership I0

Durable Decision:
[Published-row ownership D0](../design/mir-call-published-rows-invocation-ownership-d0.md).
Static V2 uses one `HakoLlvmcInvocation` ledger through prepass, emission,
residual validation and cleanup. Same-ledger re-entry rejects; distinct
ledgers are isolated. Borrowed Rust rows, exact typed coordinates, duplicate
take rejection, zero-row V2 and unfinished-row rejection are preserved.

Reported implementation evidence at `8fe3f00a6c`: WSL2/Linux, Ubuntu
GCC 13.3.0, ELF `target/release/libhako_llvmc_ffi.so`; focused published-row
ASan tests, 22 document-lifetime cases, 16 named-query cases, Static V2 formal
cases, no artifact after residual failure, and jobs1 Cargo check passed.
Existing warning baseline: 1806. Maximum changed C source: 683 lines.
Route/pointer guards and diff check passed and were independently rerun on
the receiving checkout. These are Linux I0 witnesses.

### AOT direct-harness child environment I1

At `59e9a30b1f`, native Windows Git Bash, w64devkit GCC14.2/MSVCRT and
Python3.11.8 produced a Windows x64 PE DLL and passed the standalone
`child-env` smoke. Both-unset, HAKO-only, NYASH-only, both-present and
both-empty observe `(0,0)`, `(3,0)`, `(0,1)`, `(3,1)`, `("","")`.
The native C probe distinguishes unset/null from empty; parent Win32/MSVCRT/
UCRT option environments remain unchanged. A 3000-character overflow rejects
with `command too long` and no child record/object. Linux also passed.

This Windows result covers that commit's AOT child transport. It does not
establish native Windows execution of the later published-row I0. Windows
verification for a future changed boundary must use the resulting revision.

### Related source/physical completion

Source-called G0 reaches LLVM18/EXE3 without skip at `16f87f5eeb`, rerun at
`16329a0da2`. Empty checked-site admission, source declarations and existing
V4 Compare/PHI/Branch consumers retain their own ABI/CFG validation.
The owning G0 card and `docs/reference/abi/nyrt_c_abi_v0.md` hold the contract.

Scalar local+terminal Call recovery at `ad5730b48d` and the subsequent
empty-Home Plain cleanup repair have recorded source, publication, missing-
binding mutation, normal/fault, and G0 regression evidence. The former Plain
`root-cleanup-graph/residual-node` debt was resolved by the cleanup repair.
Do not revive its earlier baseline as a current unresolved failure.

## Evidence debt and operating limits

| Item | Classification / handling |
| --- | --- |
| named llvmlite admission smoke | known environment baseline at the recorded host; standalone child-env remains independently runnable |
| legacy census `capi_transport.rs:247` anchor | informational census drift recorded before I1; repair from actual source anchors when that observer is selected |
| pre-install LLVM14 failures/skips | historical environment evidence; no LLVM18 execution inferred from fake tools or skip |
| compiler warnings | existing nonzero baseline; no zero-warning claim |
| RAM replacement | reboot validation shows about 28GiB total / 20GiB available during jobs-4 check; keep one Cargo process and raise jobs only while this margin holds |

Current checks: pointer guard, diff check, feature check, focused ownership test,
and manual active-card line budget. Existing compiler warning output remains a
known baseline; no zero-warning claim.
