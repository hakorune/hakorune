---
Status: Implementation complete — POSIX wrapper matrix strengthened; native Windows proof pending
Date: 2026-09-13
Decision: MIR-CALL-HARNESS-LOG-OWNERSHIP-D0
Parent: docs/development/current/main/investigations/mir-call-legacy-target-census-d0-2026-08-20.md
ProductionCaller: selected native ingress plus retained explicit compatibility
ReplacementCell: owner-local migration; aggregate legacy retirement remains open
---

# MIR-CALL-COMPATIBILITY-RETIRE-R7-D0

## Six-line brief

Decision: implement MIR-CALL-HARNESS-LOG-OWNERSHIP-I0 under the accepted D0 contract.
Source authority + canonical issuer: named C admission and existing PhysicalOptions own the request; exclusive file creation establishes temporary log ownership, not source semantics.
Non-authority: PID, dlsym wrapper, public reachability, and stderr do not issue request meaning.
Fail-fast boundary: complete compiler/options, path/command validation and log reservation/close before output removal or child launch.
Smallest next slice: MIR-CALL-HARNESS-LOG-OWNERSHIP-I0, switching the single C harness executor and deleting its PID-only log edge.
Non-claims: native Windows reservation/reopen, public ABI retirement, whole-compiler thread safety, same-output concurrency, or aggregate R7 completion.

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

The pause and selection statements in this section are historical. They are
superseded by MIR-CALL-TEMP-INPUT-OWNERSHIP-D0 below; they do not select the
next task or require retained callers to disappear before migration.

The FAST selector capture is closed below. The remaining C/AOT compatibility
callers share public entries, dlsym re-entry, child/provider paths, or the
retained compatibility terminals. This is not a Fast path: a caller-local
change would still have to name the owner, terminal, retained callers, and
exact old-edge deletion together.

The read-only audit recommendation to move the legacy TargetMachine selector
was compared with the current checkout and is already closed at `47e83a224e`:
`hako_llvmc_ffi_invocation.inc` owns the selector/opt-level capture,
`compile_doc_compat_pure` invokes it, and the legacy emitter consumes only the
invocation state. The current Windows child transport likewise builds a
CreateProcess environment block; its option values are not interpolated into
the `cmd.exe` text. Neither observation opens a new slice.

Decision: `NoSafeSlice__RetainedCapiOwnersNoExclusiveDeleteSet`.
Source authority + canonical issuer: each retained caller's accepted physical
request and its existing explicit entry; no profile label or ambient value
issues new semantics.
Non-authority: public symbol names, provider reachability, test-only emitters,
stale worker premises, and the closed FAST/published-row/TargetMachine/child-env
evidence.
Fail-fast boundary: preserve each existing contract, replay, tool, symbol and
input rejection before environment mutation, provider/child launch, or artifact
publication.
Smallest next slice: none is executable now; reopen this same frontier only
when a new source-backed caller/terminal relation supplies one finite
caller-specific old-edge delete-set. Do not repeat the closed census.
Non-claims: no code, route switch, fallback, public ABI deletion, LLVM parity,
Windows proof for later revisions, or aggregate R7 completion.

### Finite retained-boundary outcome table

| Outcome | Existing owner / authority | Pre-effect behavior | Allowed action |
| --- | --- | --- | --- |
| `ClosedExistingOwner` | invocation/options/child-env owner | existing admission and terminal | retain; no duplicate row |
| `RetainedCompatibility` | named C/AOT entry, shared provider, or public dlsym ABI | existing compatibility terminal | retain until its callers are co-sealed |
| `UnresolvedCutover` | no finite caller/terminal/delete tuple | design stop; no effect or artifact | do not implement or invent a receipt |
| `ParkedOutOfScope` | deferred backend, test-only, or separate owner card | its owner's existing policy | leave parked with its reopen trigger |

This is an explicit frontier pause, not a repository-wide blocker: the known
retained groups are classified, but none currently meets the Promote/Stop/Delete
entry condition without manufacturing a new authority or deleting a supported
caller. The next eligible change must reduce a production old edge in the same
bounded series.

## MIR-CALL-COMPATIBILITY-RUST-LLVMLITE-REQUEST-D0 (accepted; I0 selected)

The previous frontier audit found no safe deletion of the retained provider or
public compatibility entry. A separate source audit found one behavior-neutral
transport boundary inside the existing Rust runner: the ordinary LLVM execution
entry reads the harness selector again in the harness executor, fallback
executor, and NyRT precheck helper. Those reads are not a new semantic owner,
but their different primary/alias/default contracts must not be collapsed.

Boundary: one `NyashRunner::execute_llvm_mode` ordinary compatibility attempt
from route decision through `HarnessExecutorBox`/`FallbackExecutorBox` to the
existing `ny_llvmc_emit_exe_lib` child terminal. The selected Dynamic Boundary
route, `backend=mir` emitter, host-provider `ExplicitHarnessCompat` route,
named C/AOT harness, and public ABI/dlsym paths are excluded.

Source authority + canonical issuer: `config::env::llvm_use_harness()` remains
the selector parser (primary, then deprecated alias, then its existing default).
At the existing `execute_via_harness_or_fallback` decision boundary, a private
`LlvmHarnessInvocationPolicyV1` snapshots that selector plus the existing raw
primary fallback gate and the literal-`1` child NyRT-precheck bypass. This is
one transport request owner with three named compatibility projections; it is
not a MIR, Recipe, ABI, or `Verified*`/`Prepared*` semantic product.

### Finite policy states

| Existing input state | Captured policy | Existing terminal behavior to preserve |
| --- | --- | --- |
| primary present and parsed true (`1`/`true`/`on`) | selector=true, primary-failfast=true; child bypass only for literal `1` | try harness; harness failure is not allowed to fall back |
| primary present and parsed false (including empty/other text) | selector=false, primary-failfast=false, child bypass=false | harness rejects as not requested; existing mock fallback remains eligible |
| primary unset, alias present and parsed true | selector=true, primary-failfast=false, child bypass=false | try harness; failure may use the existing fallback policy |
| primary unset, alias present and parsed false | selector=false, primary-failfast=false, child bypass=false | harness rejects; existing fallback remains eligible |
| both unset | selector=true from the existing keep-lane default, primary-failfast=false, child bypass=false | try harness; failure may use the existing fallback policy |

The primary key continues to win over the alias. The literal-`1` child
precheck rule remains distinct from boolean parsing; this preserves current
`true`/`on` behavior. Debug pipeline reporting remains observation-only and is
not a route authority.

Exact old-edge delete set for I0:

1. the direct primary `env_bool` fallback decision in `llvm/mod.rs`;
2. the ambient selector read in `harness_executor.rs` and its duplicate raw
   selector diagnostic (the captured policy is logged instead);
3. the direct primary `env_bool` fail-fast check in `fallback_executor.rs`;
4. the ambient `NYASH_LLVM_USE_HARNESS` read on this harness caller's NyRT
   precheck path in `common_util/exec.rs`, replaced by an explicit process
   policy. The ambient behavior for unrelated `backend=mir` and selected
   Boundary callers remains retained and is outside this delete set.

I0 must not delete `provider_keep`, `CodegenRouteRequestV1`, the public C/AOT
entry, fallback execution, `llvm_use_harness` itself, or the diagnostic-only
pipeline report field. No route switch, new fallback, retry, MIR/Recipe change,
or public ABI change is permitted.

Acceptance: the existing ordinary LLVM runner caller passes one policy through
harness, fallback, and the harness child emitter; positive and negative
selector states above retain their current result; the harness child keeps its
existing explicit arguments, artifact/error cleanup, and NyRT behavior; a
focused source guard proves the old runner reads are gone while unrelated
ambient callers remain; quick-profile compilation and the existing focused
LLVM compatibility smoke are green. This is WSL/Linux evidence unless a later
revision changes Windows-specific process code; no native Windows claim is
made by this I0.

## MIR-CALL-COMPATIBILITY-RUST-LLVMLITE-REQUEST-I0 closeout (2026-09-13)

Implemented the accepted transport slice in the existing Rust runner. One
`LlvmHarnessInvocationPolicyV1` is captured at the ordinary harness/fallback
decision boundary and passed through harness execution, fallback admission,
and the child NyRT-precheck path. The primary/alias/default selector, the
primary-only fallback fail-fast gate, and the literal-`1` child bypass remain
separate projections. No provider, public ABI, MIR, Recipe, or fallback owner
was deleted or changed.

Evidence: default quick-profile test compilation passed; the exact policy
unit test passed; `CARGO_BUILD_JOBS=1 cargo check --profile quick
--features llvmlite-compat` passed; the focused LLVM options/ownership smoke,
`llvm_codegen_route_identity_guard.sh`,
`current_state_pointer_guard.sh`, and `git diff --check` passed. The changed
Rust sources remain below the 800-line limit. The first focused command used
an over-specific short filter and ran zero tests; it was corrected by listing
the binary and rerunning the full test name, which passed 1/1. The global
`cargo fmt --check` remains informational baseline debt because unrelated
repository formatting differences are already present; no formatter write was
performed. This is WSL/Linux evidence; native Windows remains covered only by
the separate AOT I1 result at `59e9a30b1f`.

Acceptance recheck (2026-09-13): `policy_preserves_distinct_legacy_projections`
now calls `LlvmHarnessInvocationPolicyV1::capture()` under the shared
environment-test lock. Its table covers both-unset, primary-empty with an
alias present, primary `true`, primary literal `1`, and alias-only input; it
asserts primary-over-alias precedence, the llvmlite default/alias selector
projection, the primary-only fail-fast projection, and the literal-`1`
child-precheck projection. The default build and the `llvmlite-compat`
feature build each pass the exact test; the deprecated-alias warning in the
feature run is expected. Environment state is restored by the lock-backed
test helper after every case.

## MIR-CALL-TEMP-INPUT-OWNERSHIP-D0 (accepted)

### Premise correction

The former D1 pause and its v0 crosswalk are superseded for task selection.
Their historical source checks remain in Git at a7ab5afcd3. Shared supported
callers do not prevent a bounded migration: caller-zero is required at physical
deletion, and may be produced by the same implementation series. The existing
current-docs update policy remains authoritative. Missing internal ownership
design is a design task, not an external wait.

The retained-v0 boxcall disposition remains an independent schema-retirement
obligation. It is not a prerequisite for repairing CAPI/provider input
ownership. No blanket rejection or parked-lane reopening follows.

### Finite owner and consumers

Boundary: the three Rust temporary-input preparation sites -> synchronous
child/CAPI return and existing artifact validation. Source inspection and
Singer's read-only audit agree on this finite set:

| Caller | Input authority / consumer | Current ownership gap |
| --- | --- | --- |
| provider_keep.rs::prepare_provider_io -> mir_json_to_object_llvmlite | validated JSON -> Python harness Command::status | input survives success, missing-tool, spawn and child failures |
| capi_transport.rs::compile_via_capi_keep | validated JSON -> compile_via_capi -> options-v1 C entry | input survives success and contract/library/symbol/compile failures |
| capi_transport.rs::compile_published_lifecycle_physical_v4 | serialized LifecycleInvocationInputV1 -> V4 C entry | manual removal targets the shared pathname; write failure precedes cleanup |

All paths currently call transport_io.rs::prepare_backend_input_json_file.
transport_paths.rs::build_backend_temp_input_path returns the process-independent
temporary pathname hako_llvm_in.json. Two live preparations can overwrite one
another, and V4 can remove another invocation's input. This is source evidence
of a possible collision, not a measured concurrent-compilation result.

### Resource and failure contract

Use the existing tempfile dependency. The existing transport_io owner returns
one private, non-Clone owned input containing a unique TempDir and its JSON path.
Create the file inside that directory; finish writing and close the File before
returning the owner, allowing native Windows child/CAPI reopening. Consumers
borrow its path and retain the owner until synchronous child/CAPI consumption
and the existing artifact check finish. No asynchronous consumer may retain it.

Create the TempDir owner before writing so partial-write errors also drop the
owned directory. Scope exit releases it on success and every early return.
Cleanup is best-effort through the resource destructor; it does not mask the
primary error or turn successful compilation into a new error contract. Tests
require removal under normal filesystem conditions; OS-level removal failures
remain outside that guarantee.

Preserve validation order, JSON bytes, route selection, options, public ABI,
and output-path behavior. Do not remove caller-supplied inputs or outputs.
The fixed default object output is outside this slice; overlap tests use
different explicit outputs and prove input isolation only.

### Ordered implementation tasks and finish line

1. In transport_io.rs, replace the unowned PathBuf preparation result with
   the scoped input resource using TempDir. Reuse the existing write-error
   category and keep write handles closed before returning the path.
2. Migrate all three callers above in one I0. Keep each owner alive through
   its actual terminal and replace V4's manual input removal with scoped
   cleanup. Preserve feature-disabled and error-order behavior.
3. Delete build_backend_temp_input_path, the fixed input filename, and the
   old unowned input return contract after all three sites switch. Retain
   remove_backend_temp_file: ll_emit_compare_driver still uses it.
4. Add focused tests in the existing transport/provider test ownership:
   two live inputs have different paths and exact bytes; dropping either
   preserves the other; preparation write failure and consumer early returns
   release their own files; success retains input until consumption completes.
   Exercise real provider/CAPI wrappers with controlled child/library failure
   seams, including missing tools, invalid contract, missing symbol, child/C
   failure, and successful object validation. An owner-only unit test cannot
   substitute for the three caller lifetime checks.
5. Check the relevant plugins and llvmlite-compat feature combinations with
   one quick-profile Cargo process, jobs 1-2; run route/pointer guards and
   diff check. Update src/host_providers/llvm_codegen/README.md and the affected
   harness/ABI reference with the input-lifetime contract, then commit/push.

I0 closes only with all three production preparation sites switched, the
fixed-name input edge removed, and positive/negative lifetime evidence. Native
Windows close/reopen evidence must be reported separately from Linux evidence.
No new semantic receipt, route dispatcher, dependency, or per-task guard is
needed. Current target source files are 28-510 lines; split at 760 and prohibit
800-line source growth.

This user-requested design delivery stops before code and tests. The bounded
next implementation is MIR-CALL-TEMP-INPUT-OWNERSHIP-I0 in this same card.

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

Implemented at the FAST capture closeout revision: capture FAST in the existing
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

The three Rust preparation sites now receive a private `BackendInputJsonFile`
owner from `transport_io`. It contains a unique `TempDir` and a generated JSON
`TempPath`; the file handle is closed before the path is passed to a child or
CAPI. Provider, CAPI options, and lifecycle V4 retain that owner through their
synchronous consumer and artifact check, so success and every early return
drop only that invocation's input. The V4 manual removal is gone, while the
compare driver's unrelated text cleanup helper remains.

The fixed global input-path builder and `hako_llvm_in.json` pathname are deleted.
JSON bytes, validation/error ordering, route selection, output paths, and public
ABI remain unchanged. No asynchronous consumer receives the borrowed path.

Focused evidence:
`host_providers::llvm_codegen::transport_io::tests::invocation_inputs_are_unique_and_drop_independently`
and
`host_providers::llvm_codegen::transport_io::tests::input_survives_consumer_error_until_owner_drop`
pass with jobs 3/4; they prove distinct paths and exact bytes, independent drop
of one live input, cleanup after the final owner drops, and retention through an
artifact-consumer error. `CARGO_BUILD_JOBS=4 cargo check --profile quick
--features llvmlite-compat` passes, covering the provider feature path; the
default plugin test also passes. Source search shows no remaining call to
`build_backend_temp_input_path` or global `hako_llvm_in.json`. The full C
library/provider failure matrix and native Windows close/reopen run remain
environment evidence for a later acceptance pass; this I0 does not claim
whole-compiler concurrency safety.

## MIR-CALL-COMPATIBILITY-RETIRE-R7-D1 (resolved)

Worker premise: this was not Fast path because the named AOT-to-C harness
branch had no settled responsibility/delete-set mapping. One read-only worker
examined admission, options, lifetime, execution and failure; the primary
checked existing README/reference and acceptance contracts. The worker's
report is integrated into the Decision below, not treated as implementation.

Admission is already singular: the AOT FFI wrapper validates input, loads the
named symbol and retains the library during its synchronous call. The C export
captures/deep-copies the compiler path once through existing PhysicalOptions.
There is no reason to add an options layer or delete a supported public entry.

A concrete old shared edge remains inside that consumer:
`hako_llvmc_build_harness_log_path` in `hako_llvmc_ffi_common.inc` produces
`hako_llvmc_harness_compile_<pid>.log`. Its sole caller is
`compile_json_compat_harness_execute` in `hako_llvmc_ffi_route.inc`, which
removes that path before the child and on its success/failure paths. Two live
calls in one process can therefore truncate, read or remove each other's log,
even with different object paths. This is a source-backed ownership defect;
no runtime overlap reproduction is claimed in this design delivery.

## MIR-CALL-HARNESS-LOG-OWNERSHIP-D0 (accepted)

Boundary: named C export (direct or via AOT FFI) -> existing compiler/options
capture -> C harness executor -> synchronous child -> object/error projection
-> log cleanup. Includes both callers sharing this executor. Excludes direct
AOT child execution, Generic/pure-first/Static compilation, llc/link logs,
Rust input storage, external-reader inventory and child-internal semantics.

Reuse the existing executor and request owners. A private log storage helper
owns a bounded path and an ownership flag; it issues no semantic receipt.
POSIX uses `mkstemp` then `close`. Windows uses `_mktemp_s` for a candidate
and `_open` with `_O_CREAT | _O_EXCL | _O_WRONLY | _O_BINARY`, then `_close`.
Candidate generation alone is not ownership: only exclusive create succeeds.
Windows collisions retry finitely on EEXIST; other failures terminate. Neither
platform falls back to the PID-only name. Keep the reserved empty file until
the child reopens it for stderr, so no unlink/recreate gap is introduced.

Preserve existing TMPDIR selection and 1024-byte path / 4096-byte command
budgets. Preflight the template and command length before reservation; the
substituted suffix has fixed length. Reformat using the reserved path before
launch. New create/close failures are explicit FAILED storage errors before
object removal or child effects. Mark ownership immediately after creation so
close/format failures clean up the reserved path. Cleanup is idempotent and
best effort; its failure never overwrites the primary diagnostic. Copy the
existing first-line message before cleanup, preserving null err_out handling.

| Stage/outcome | Required behavior / terminal |
| --- | --- |
| compiler/options rejection | existing error; no log reservation, child or object removal |
| path/command overflow | existing length error; existing object sentinel remains |
| exclusive create/close failure | explicit log-storage error; no child or object removal; release any owned path |
| reserved log, final command rejection | cleanup owned log; retain existing object |
| child zero exit with object | success; cleanup only this log |
| child nonzero or zero exit without object | existing first-line/fallback diagnostic; cleanup only this log |

This repairs invocation isolation; it does not claim resistance to hostile
filesystem mutation, full environment capture, global thread safety, or safe
concurrent use of the same obj_out. Native Windows reservation/reopen requires
its own evidence; Linux green cannot substitute. AOT FFI on Windows retains
its existing unsupported terminal; the direct C export is the Windows witness.

## MIR-CALL-HARNESS-LOG-OWNERSHIP-I0 (complete on POSIX; Windows proof pending)

Change:
1. Add private temporary-log storage in a focused include, wired before route
   consumption; keep common=650 and route=548 current lines below the source
   budget instead of expanding common past the 760-line design threshold.
2. Switch `compile_json_compat_harness_execute` to that lifetime. Both direct C
   and AOT FFI callers then use it automatically, without another dispatcher.
3. Delete `hako_llvmc_build_harness_log_path` and the executor's three manual
   `remove(log_path)` sites. Keep shared tmp selection/first-line reader and
   unrelated log builders. AOT wrapper and public headers need no change.

Contract:
One invocation owns one diagnostic log through child completion and diagnostic
copy. Preserve opaque JSON forwarding, exact child args/settings, public
profile3 rejection, compiler/options ownership, error precedence and output
policy. Only temporary-log allocation adds a new pre-effect failure boundary.

Done:
- A focused private helper test holds two logs alive in one process, verifies
  independent contents/cleanup and exclusive-create collision behavior; inject
  create/close failures and verify cleanup after ownership acquisition.
- Extend `tools/checks/llvm_compile_options_contract_smoke.sh` to run that test
  and a synchronized fake-child overlap through direct C plus AOT FFI in the
  same process, with distinct outputs/err_out and fixed parent env. The
  synchronization must confirm both children have started before release;
  after both calls finish, assert that invocation-owned logs leave no stale
  residuals and that each first-line diagnostic came from its own child.
- Cover success, child failure, zero-exit/no-object, null err_out, path/command
  overflow, storage failure with output sentinel intact and no child record.
  Keep existing named-harness args/env, opaque-input and failure-order cases.
- Update the existing `llvm_codegen_route_identity_guard.sh` for selected PID
  edge retirement; no new guard entry. Run pointer guard and diff check.
- In the implementation slice update `lang/c-abi/shims/README.md` and
  `docs/reference/abi/nyrt_c_abi_v0.md#named-harness-physical-options-ownership`.
  Record platform-scoped results; native Windows helper/C-export close/reopen
  is required before claiming cross-platform completion.

Stop:
Return to design if exclusive-create/closed-handle ownership cannot be kept
through the child, existing error precedence changes beyond the named new
storage failure, or implementation requires new admission/ABI/fallback.
Shared callers and public reachability are not stop conditions. POSIX
implementation and focused evidence are complete; native Windows remains a
separate proof boundary.

Implementation evidence (2026-09-13): `build_hako_llvmc_ffi.sh`, the private
`harness_log_ownership_test.c`, and
`llvm_compile_options_contract_smoke.sh` pass. The smoke covers direct C plus
AOT FFI calls in one process. Its overlap case now waits for both fake-child
marker files before releasing either child, then checks each first-line
diagnostic and verifies that the dedicated temporary-log directory is empty
after both calls finish. Serial execution cannot satisfy that barrier. The
smoke also covers distinct first-line failures, success, zero-exit/no-object,
null `err_out`, path/command overflow, blocked-TMPDIR storage failure with an
output sentinel and unchanged child record, and the existing
argument/environment contract. `llvm_codegen_route_identity_guard.sh`,
`current_state_pointer_guard.sh`, and `git diff --check` pass. The compiler
build emits only the existing AOT path-format warnings. Native Windows
reservation/close/reopen is not claimed from this Linux run.

Integration acceptance gap (2026-09-13): the temporary-input I0 owner tests
and Rust caller compilation do not by themselves close the three real consumer
lifetimes. The resulting revision now has controlled Linux wrapper evidence:
`provider_wrapper_exercises_missing_tool_invalid_input_child_failure_and_success`
passes the real provider executor for missing Python, invalid input, child
failure, and successful object output; and
`capi_production_wrapper_exercises_contract_loader_child_and_success_paths`
passes the real CAPI loader for invalid contract, missing tool, missing symbol,
C child failure, and successful object output. The ignored
`lifecycle_capi_wrapper_keeps_unique_input_through_success_and_failure` test
also passes through the lifecycle V4 production wrapper, covering success and
C failure while checking the serialized input path and bytes during the call
and cleanup after return. Exact quick-profile results are 1/1, 1/1, and 1/1
respectively; the lifecycle result ran with `-- --ignored` on a 32 MiB stack
test thread. These are controlled POSIX/WSL witnesses; native
Windows close/reopen evidence remains a separate requirement. Until that
platform result runs on the resulting revision, the temporary-input row is
POSIX wrapper evidence and must not be reported as cross-platform complete.

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
