---
Status: Design stop — remaining shared compatibility admission
Date: 2026-09-13
Decision: MIR-CALL-COMPATIBILITY-ADMISSION-D0
Parent: docs/development/current/main/investigations/mir-call-legacy-target-census-d0-2026-08-20.md
ProductionCaller: selected native ingress plus retained explicit compatibility
ReplacementCell: owner-local migration; aggregate legacy retirement remains open
---

# MIR-CALL-COMPATIBILITY-RETIRE-R7-D0

## Six-line brief

Decision: define the next shared C/AOT compatibility owner after FAST capture I0.
Source authority + canonical issuer: each retained physical request and its existing invocation or process boundary.
Non-authority: historical task prose, profile names, public symbol presence, and adjacent smoke results.
Fail-fast boundary: preserve admission, typed row finish, owner cleanup and pre-artifact failure.
Smallest next slice: MIR-CALL-COMPATIBILITY-ADMISSION-D0; one caller/terminal/delete-set matrix.
Non-claims: no aggregate R7 completion, new MIR/Recipe authority, ABI retirement, or general concurrency proof.

The accepted Decision closes the technical mapping. Changing capture time
creates an execution-state ownership boundary; Hypatia's read-only audit
identified the three consumers and the primary agent accepted the Decision.

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

## MIR-CALL-COMPATIBILITY-ADMISSION-D0 (design stop)

The FAST selector capture is closed below. The remaining C/AOT compatibility
callers share public entries, dlsym re-entry, child/provider paths, or the
retained TargetMachine probe. This is not Fast path because the next edge may
change the owner, terminal, or caller-specific retirement boundary. One
read-only worker audits the existing retained-caller matrix; the primary
agent will accept one Decision and one bounded slice.

Decision: pending source-backed owner/terminal/delete-set review.
Source authority + canonical issuer: retained physical request at each
explicit entry; no profile label or ambient value issues new semantics.
Non-authority: public symbol names, provider reachability, test-only emitters,
and the closed FAST/published-row evidence.
Fail-fast boundary: reject contract, replay, tool, symbol and input conflicts
before environment mutation, provider/child launch, or artifact publication.
Smallest next slice: choose exactly one retained caller group and its old edge.
Non-claims: no code, route switch, fallback, public ABI deletion, LLVM parity,
or aggregate R7 completion before the Decision.

## MIR-CALL-FAST-INVOCATION-CAPTURE-D0 (accepted)

Boundary: C compile execution -> generic string lowering and its route
diagnostic -> existing residual checks -> object/error terminal. AOT Generic
enters through `hako_aot_generic_ffi_compile.inc` and the versioned options
entry. Public Generic, Boundary, Static V2 and direct core tests share
`compile_doc_compat_pure`; each uses that execution's invocation.

Add one private scalar to the existing invocation, zeroed at initialization.
At each `compile_doc_compat_pure` entry, capture FAST before any lowering
consumer runs. Capture at compile time, not Static open/query time: successive
compiles using the same open invocation must sample fresh values. An
environment read has no new error terminal; existing validation order stays.
Do not reuse the legacy TargetMachine eligibility mask: FAST's consumer set
includes paths where the legacy probe is bypassed.

| Input / state | Required behavior |
| --- | --- |
| exactly `1`, `on`, `true`, `yes` | captured true, case-sensitive |
| unset, empty, `0`, uppercase, suffix or other text | captured false |
| environment changes after capture | this execution keeps its captured value |
| next compile on same or different invocation | capture fresh value |
| Static V2 | retain its existing separate hoist suppression |
| malformed or residual typed rows | existing rejection and cleanup, no object |

The three consumers are GNU nested helpers within the same compile core:

| Consumer source under `lang/c-abi/shims/` | Existing responsibility | Replacement |
| --- | --- | --- |
| `hako_llvmc_ffi_const_string_hoist.inc` | emit hoisted string definitions unless Static V2 | invocation scalar plus existing Static condition |
| `hako_llvmc_ffi_string_concat_emit_helpers.inc` | choose constant-result emission behavior | same scalar |
| `hako_llvmc_ffi_string_chain_policy.inc` | label folded-string route when tracing enabled | same scalar; trace gate retained |

Exact old-edge delete set: these three calls and the zero-argument
`hako_llvmc_fast_enabled` helper in `hako_llvmc_ffi_common.inc`.
The single capture read belongs in `hako_llvmc_ffi_invocation.inc`, invoked
from the compile core. Do not add a physical-contract ABI field or route
fallback. This is a consistency change for mid-execution environment mutation;
stable-environment output must remain equivalent. It makes no speed claim.

The Static hoist guard and constant-result helper currently have different
conditions. Preserve that asymmetry; if a live fixture exposes invalid IR,
classify it against the parent revision and resolve its owner separately
before claiming that fixture as acceptance.

## MIR-CALL-FAST-INVOCATION-CAPTURE-I0 closeout (2026-09-13)

Implemented at the current closeout commit: capture FAST in the existing
invocation at compile entry, migrate all three readers atomically, and remove
the caller-zero ambient helper.

Contract: one compile execution owns the value. Preserve exact parsing,
Static suppression, trace gating, typed residual checks, public ABI, existing
fallback and source/Recipe meaning.

Evidence: `fast_invocation_capture_test.c` passes accepted values
`1/on/true/yes`, rejected spellings, mutation-after-capture, and fresh
recapture. `llvm_compile_options_contract_smoke.sh` passes its existing
contract cases plus real LLVM18 `opt-18`/`llc-18` object emission for the
string fixture with FAST=0 and FAST=1. The C build used Ubuntu GCC 13.3.0;
the focused route and C tests pass. `llvm_codegen_route_identity_guard.sh`,
`current_state_pointer_guard.sh`, `git diff --check`, and source-size checks
pass; the largest changed C unit is 660 lines.

The Static V2 suppression, typed residual cleanup, public ABI, source/Recipe
meaning and existing fallback remain unchanged. This is WSL/Linux evidence;
the native Windows AOT I1 proof remains the separate revision-scoped result
at `59e9a30b1f`.

Next blocker: choose the remaining shared C/AOT compatibility owner through a
new design stop with a finite caller/terminal/delete-set. Deferred non-Loop
snapshot reacquisition, Read/Write/Carrier cleanup, and ordinary-new writer
work remain in their existing owners.

## Ordered queue

| Order | Task | Entry / finish condition |
| --- | --- | --- |
| 1 | FAST capture D0 | accepted above; design and task organization delivered |
| 2 | FAST capture I0 | closed at the current implementation commit; three readers and the old helper are aligned |
| 3 | Next shared compatibility D0 | active design stop; choose one finite caller/terminal/delete-set |
| Deferred | non-Loop snapshot reacquisition | existing perf owner; prove duplicate acquisition and compatible lifetime before reuse |
| Deferred | Read/Write/Carrier unused information | owning Rust metadata paths; prove zero consumers before behavior-neutral deletion |
| Deferred | ordinary-new unclaimed writer | existing Birth/ordinary-new owner; retain direct-local, foreign/transferred and uncovered cases |

The deferred cleanup items are not silently part of the C/AOT slice.
The existing compile-time performance card owns measurement and snapshot
investigation. Metadata cleanup must preserve source identity and publication
ownership; it does not authorize a new semantic receipt.

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
| RAM replacement | host task; one Cargo process, jobs 1–2 until RAM validation |

Current documentation checks: pointer guard, diff check and manual active-card
line budget. No Cargo build is needed for this design/organization change.
