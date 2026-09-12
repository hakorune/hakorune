---
Status: Design stop — first owner-unit is already closed; next owner unresolved
Date: 2026-09-11
Decision: MIR-CALL-COMPATIBILITY-RETIRE-R7-D0
Parent: docs/development/current/main/investigations/mir-call-legacy-target-census-d0-2026-08-20.md
ProductionCaller: strict/dev selfhost and stage1 ingress already stopped; release compatibility remains
ReplacementCell: existing `MIR-CALL-LEGACY-READER-STOP-R0` terminal (landed)
---

# MIR-CALL-COMPATIBILITY-RETIRE-R7-D0

## Six-line brief

Decision: Design the R7 retirement as finite owner-unit Stop/Promote/Delete
rows. Do not open aggregate `LegacyCallV0` deletion until every selected
writer, reissuer, reader, re-entry, and environment route has an owner and a
caller-zero proof.

Source authority + canonical issuer: canonical `MirInstruction::Call(MirCall)`
and its typed `Callee` own native meaning. Existing `LegacyCallV0` producers,
reissuers, readers, and compatibility terminals are classified from the
resynchronized observation manifest and their real callers.

Non-authority: lexical counts, `func`, `ValueId::INVALID`, names, generic
reachability, test-only Loop-PHI files, and the observation manifest itself.
They classify evidence but never issue or repair a call target.

Fail-fast boundary: selected native admission rejects mixed legacy input before
artifact creation; explicit compatibility ingress keeps its existing terminal
until a replacement is switched. No writer or shared carrier is deleted while
an in-scope reader or re-entry remains reachable.

Smallest next slice: select another existing M7-S owner only if its caller set
and exclusive delete-set are independent of the shared release parser. The
strict/dev `boxcall` Stop itself is already closed; do not duplicate it.

Non-claims: no aggregate R7 deletion, `func`/`Option<Callee>` removal, JSON-v0
retirement, VM/WASM parity, environment global removal, Loop-PHI production,
OBJ/EXE completion, or performance result.

## Finite boundary and state table

```text
canonical/compatibility MIR ingress + compile profile
  -> LegacyCallV0 constructors/reissuer/readers/egress
  -> selected artifact reject or explicit compatibility terminal
```

Included: the current manifest's 243 production lexical rows across 127 files,
four compile-environment route anchors across four families, and the known
compatibility ingress families. Excluded: canonical typed `Call` definition,
ordinary tests, unrelated runtime/startup environment, runtime hook registry,
future VM/WASM parity, and test-only Loop-PHI residue except as inventory.

| State | Issuer/owner | Pre-effect terminal | Allowed action |
| --- | --- | --- | --- |
| `SelectedNative` | canonical typed call owner | named admission reject on legacy mix | retain typed route |
| `ExplicitCompatibility` | existing compatibility ingress | existing compatibility terminal | Stop or Promote only with caller proof |
| `MechanicalReissuer` | existing ID remapper | preserve IDs or named failure | retain until all callers close |
| `ReaderProjection` | existing serializer/backend projection | explicit compatibility output | replace/delete only at caller-zero |
| `EnvironmentRoute` | existing invocation/profile owner | restore or named failure | move to invocation-owned state in its own row |
| `TestOnlyInventory` | manifest observer | no production terminal | separate test cleanup decision |
| `UnclassifiedOrDrifted` | no authority | design stop | do not infer or delete |

## Acceptance for the next owner-unit row

Before implementation, the selected owner must have one source-backed issuer,
one real production caller set, one named terminal/consumer, and one exclusive
delete-set. The row must preserve the existing compatibility behavior or stop
it before effect, and the same series must remove its old edge after the
replacement or terminal is proven. Positive evidence covers the supported
caller; a one-point mutation negative identifies the owner-specific reject.

R7 aggregate closure requires all in-scope production writers, reissuers,
readers, re-entry paths, and environment routes to be caller-zero, with no
unconsumed replacement product. Until then, `LegacyCallV0` and shared
compatibility carriers remain live. Reopen this design if the manifest drifts,
a new non-test constructor/reissuer appears, a selected native reader consumes
legacy input, or an external caller cannot be assigned to an owner.

## Worker audit (2026-09-11)

The read-only owner audit found the R7 aggregate is not yet safe to open as an
implementation row. The current manifest must first be reconciled to the
selected admission source change (243/127 Legacy, four environment anchors,
four test-only Loop-PHI files, 251 rows). After that, select one owner-unit
Stop/Promote/Delete from the existing M7-S inventory; do not repeat the broad
census or delete the shared schema early.

### Candidate comparison and first bounded owner

The follow-up audit compared the two remaining concrete candidates without
reopening the census:

| Candidate | Real callers / terminal | Exclusive delete-set | Disposition |
| --- | --- | --- | --- |
| `src/runner/mir_json_v0/module.rs` `boxcall` arm | `json_artifact::mir_loader`, `selfhost::stage_a_route`, `stage_a_compat_bridge`, and `stage1_bridge::stub_emit::parse`; compatibility parser currently produces `LegacyCallV0`, while strict/dev has a named pre-effect stop | not yet exclusive: release/v0 compatibility and strict/dev ingress share the parser and the `LegacyCallV0` carrier | **Select first as an existing M7-S reader-stop owner.** Keep release compatibility; stop only the strict/dev outer ingress with its existing terminal. Promote is not allowed because JSON `receiver`/`box_name` is not a source-backed typed issuer. Delete waits for release caller-zero. |
| `src/mir/joinir_id_remapper.rs` Legacy arm | merge/rewriter and test/reference callers only; no production terminal | no production delete-set and no semantic consumer | **Park as test/reference cleanup.** It is not a safe first M7-S production owner. |

The first bounded owner is the existing generic
`MIR-CALL-LEGACY-READER-STOP-R0` row, scoped to the strict/dev outer ingress
around the JSON-v0 `boxcall` reader. Its exact boundary is
`stage_a_route::try_capture_stage_a_module` and
`stage_a_compat_bridge::resolve_program_payload_to_mir` through
`selfhost::json::parse_mir_json_v0_line`; the stage1 stub parser is an
additional caller of the same stop helper. The stable terminal is
`[freeze:contract][callsite-retire:legacy-boxcall]`, before
`mir_json_v0::parse_mir_v0_to_module` can reach `module.rs` and before a
`MirModule` is mutated. This Stop is already implemented at `4e1d6f92fb` and
is covered by the existing strict/dev named-reject and release compatibility
tests. The exclusive old edge for this row is the strict/dev call from that
helper into the v0 module parser; release/v0 callers keep their compatibility
edge until a later caller-zero row.

The existing acceptance is sufficient for this focused owner: a valid
compatibility base parses in release mode and still produces the existing
`LegacyCallV0`; the same base with exactly one `op: "boxcall"` mutation under
strict/dev returns the named terminal. No Program(JSON) or backend retry is
counted as proof. The row does not delete the shared `boxcall` parser arm,
promote JSON `box_name` or `receiver` into source identity, or remove
`LegacyCallV0` globally.

The follow-up candidate audit found no second safe M7-S production owner:
`joinir_id_remapper.rs` is test/reference-only, while the remaining release
`boxcall` callers share the parser and have no exclusive delete-set. Keep the
aggregate R7 row in design stop with
`NoSafeSlice__NoRemainingUnsharedM7SOwner`; reopen when a caller can be
isolated or a supported replacement is selected.

### Environment-route candidate reassessment (2026-09-11)

The two `hako_llvmc_ffi_route.inc` helpers are included in the R7 observation
manifest, but they are not an independent R7 owner-unit. They only force
`HAKO_AOT_USE_FFI=0` around the existing `hako_aot_link_obj` and
`hako_aot_link_obj_v2` consumers; they do not issue or reclassify a MIR call.

```text
Source authority + issuer:
  v2 = Boundary caller's explicit runtime archive;
  v1 = existing compatibility link contract.
Tracked callers:
  v1 = backend externals/global callers and the kernel compatibility surrogate
        through link_object_capi;
  v2 = published_mir_object plus boundary/static-artifact publication through
        link_object_to_exe_with_archive; focused C tests/guards also call it.
Terminal:
  existing VALIDATION / NOT_FOUND / LINK_FAILED and successful executable
  publication from hako_aot_link_obj(_v2).
Exclusive delete-set:
  none while those AOT exports still consult HAKO_AOT_USE_FFI. Removing only
  the two wrappers would leave the same process-global decision in the shared
  AOT implementation and would change the v1 compatibility boundary.
Next owner:
  existing lang/c-abi README task 3 (invocation-owned physical state/options),
  including Rust env overrides, the C save/set/restore pair, and the AOT
  direct-link consumer in one call-owned design.
```

The required acceptance for that existing Task 3 is overlapping distinct
rows/options, failure cleanup, environment preservation, and explicit v1/v2
link success without recursive FFI selection. It does not claim concurrent
compilation or LegacyCallV0 retirement. Until that owner is selected with its
full delete-set, the route pair remains `ParkedSealed__Task3Owner` and the
R7 aggregate stays at `NoSafeSlice__NoRemainingUnsharedM7SOwner`.

### Task 3 owner audit (2026-09-12)

A second read-only worker audit rechecked this candidate against the owner-unit
entry contract without reopening the census. The candidate remains a design
candidate, not an implementation row: v2 has an explicit archive authority,
but v1 still resolves `NYASH_EMIT_EXE_NYRT`, and both routes select through the
process-global `HAKO_AOT_USE_FFI`; those inputs are not yet one call-owned
issuer. The known v1 callers, v2 published-object caller, `dlsym` re-entry and
public C ABI must receive explicit owner/retain dispositions before a deletion
set can be named.

The audit also found that the current save/set/restore wrapper treats an empty
`HAKO_AOT_USE_FFI` value as unset. That is a concrete acceptance case, not
permission to patch the wrapper in this design-stop row. The missing proof set
is therefore bounded to call-owned authority, caller/retention assignment, and
success/failure plus unset/empty/present restoration, overlapping invocations,
and non-recursive v1/v2 link evidence. Keep
`NoSafeSlice__NoRemainingUnsharedM7SOwner`; no source, fixture, route switch,
or new receipt is authorized until that design is accepted.

### Co-sealed Task 3 design boundary (2026-09-12)

The consulted design review accepts the following boundary, while keeping the
implementation row unopened:

```text
Decision:
  Published v2 is a retained consumer, not an isolated deletion owner.
  Co-seal v1/v2 selection, dlsym re-entry, and public C ABI first.
Source authority + canonical issuer:
  One invocation-owned link contract issues route state and archive together;
  v2 borrows the published/Boundary explicit archive and v1 keeps its current
  compatibility contract. C exports and dlsym functions consume, not issue.
Non-authority:
  v2 never derives an archive from NYASH_EMIT_EXE_NYRT; ambient env values,
  symbol names, and runtime-dir strings cannot override explicit v2 input.
Fail-fast boundary:
  Reject invalid/conflicting input before env mutation, dlsym, or link effects;
  preserve unset/empty/present environment states and reject or isolate nesting.
Smallest next slice:
  Assign owner/retain/delete dispositions for both Rust v2 callers, the v1/v2
  C route pair, shared AOT dispatch, dlsym re-entry, and public C ABI.
Non-claims:
  No new MIR receipt, route switch, deletion, concurrency, or LegacyCallV0
  retirement is authorized by this design boundary.
```

The future acceptance must cover v1/v2 success, invalid and missing archive
rejection before effects, link-failure cleanup, unset/empty/present restoration,
overlapping or nested invocation isolation/rejection, one non-recursive dlsym
handoff, and retained public ABI behavior. The current exclusive delete-set is
still empty; keep `NoSafeSlice__NoRemainingUnsharedM7SOwner` until those owner
assignments are concrete.

### Task 3 caller disposition matrix (2026-09-12)

The current source-backed assignment is now explicit and is intentionally not a
deletion set:

| owner | current callers / terminal | disposition |
| --- | --- | --- |
| Selected published v2 | `published_mir_object.rs:150` and Boundary's `boundary_driver_ffi.rs:105` -> `capi_transport::link_via_capi_v2` -> `hako_llvmc_link_obj_v2` | retain as consumers; the explicit archive remains caller-owned and the route state must become invocation-owned |
| v1 compatibility callers | `handlers/externals.rs:305`, `handlers/calls/global.rs:296`, `compat_codegen_receiver.rs:168`, and the kernel surrogate `llvm_backend_surrogate.rs:91` -> `link_object_capi` -> `hako_llvmc_link_obj` | retain until each compatibility caller has a named replacement or stop terminal |
| Public AOT ABI | `hako_aot_link_obj` / `hako_aot_link_obj_v2` in `hako_aot_shared_impl.inc`, including external callers | retain; public ABI caller-zero is not observable from the repository |
| FFI dynamic re-entry | `hako_aot_try_ffi_link` / `_v2` dlsym the matching `hako_llvmc_link_obj` exports | retain as an explicit compatibility terminal until an internal direct-link boundary is proven |
| C FFI exports and route wrappers | `hako_llvmc_link_obj` / `_v2` in `hako_llvmc_ffi_pure_compile.inc` plus the two save/set/restore wrappers | retain the exports; the wrapper mutation is the future deletion candidate only after the shared direct-link boundary is co-sealed |

The remaining design decision is therefore narrow: define a private
invocation-owned direct-link boundary callable by the FFI translation unit,
while preserving the public v1/v2 ABI and its explicit compatibility behavior.
That boundary must carry the v2 archive and v1 compatibility resolution as one
link contract; it must not expose a new source or MIR authority. Until this
private boundary and its caller/retention proof are accepted, the matrix does
not authorize code, fixture, route, or deletion work.

### Private direct-link seam decision (2026-09-12)

The source audit identifies `hako_aot_link_obj_with_archive` as the existing
single direct-link body. A future implementation may expose only a private,
invocation-owned seam from the FFI translation unit to that body; it must not
add a second physicalizer or a public third C ABI. The public
`hako_aot_link_obj(_v2)` dispatch and its dlsym compatibility behavior remain
retained, while the FFI route is the only candidate caller for the private
direct seam.

The design is not implementation-ready yet. The seam must co-seal the v2
explicit archive with the v1 compatibility archive resolution, and must state
how the Rust/C compile options currently carried through process environment
are represented in the same invocation-owned contract. Until those two
authority points and their failure/cleanup evidence are accepted, the current
R7 disposition remains `NoSafeSlice__NoRemainingUnsharedM7SOwner` and no code
or route change is authorized.

### Task 3 responsibility split and link seam admission (2026-09-12)

The source audit separates two mutable-state responsibilities that were
previously written as one Task 3 row:

| bounded owner | mutable state | current boundary | disposition |
| --- | --- | --- | --- |
| Link direct seam I0 | `HAKO_AOT_USE_FFI` route selection | `hako_llvmc_ffi_route.inc` -> the existing `hako_aot_link_obj_with_archive` body | selected for implementation |
| Compile options I1 | recipe, replay and opt-level values | Rust transport / C compile ingress / `HakoLlvmcInvocation` | design dependency; not opened here |

This is a responsibility split, not a concurrency claim. The link slice has
one existing physical link body, an explicit v2 archive, and a v1 compatibility
resolution that can be performed once at the private boundary. It does not
consume or reinterpret MIR/source facts. The compile-options row remains
separate because its Rust environment overrides and C environment reads have
different ingress owners and cannot be deleted by changing the link wrapper.

The accepted I0 boundary is:

```text
Decision:
  Replace both FFI link save/set/restore wrappers with one private
  invocation-owned direct seam; retain public v1/v2 ABI dispatch and dlsym.
Source authority + canonical issuer:
  v2 caller supplies the explicit runtime archive; v1 compatibility resolves
  its existing runtime archive once; the seam calls the existing link body.
Non-authority:
  HAKO_AOT_USE_FFI, dlsym re-entry, and public symbol names cannot select or
  mutate the private direct link after the request has been admitted.
Fail-fast boundary:
  Invalid object/executable/archive inputs reject before linker effects;
  v1/v2 mode is explicit; link failure keeps existing diagnostics and cleanup.
Smallest next slice:
  Add the hidden cross-translation-unit seam, route v1/v2 FFI forwarders to it,
  and prove valid/missing/failure plus unset/empty/present env preservation.
Non-claims:
  Compile-options ownership, concurrent compilation, public ABI retirement,
  LegacyCallV0 retirement, new MIR receipt, and whole R7 completion.
```

I0 exclusive delete-set: the two `HAKO_AOT_USE_FFI` save/set/restore helpers
in `hako_llvmc_ffi_route.inc` and their route calls to public AOT dispatch. The
public `hako_aot_link_obj(_v2)` functions, their dlsym compatibility behavior,
the v1 archive compatibility resolution, and the existing link body remain
owned and retained. I1 must later co-seal compile options before any Rust
transport environment override or C compile environment read is retired.

### Compile-options I1 audit (2026-09-12)

The next Task 3 row is not implementation-ready. A read-only owner audit found
multiple production option ingress points: Rust `Opts` and environment defaults,
`capi_transport.rs`, `static_invocation.rs`, Boundary FFI, compatibility
receivers/surrogates, subprocess environments, C FFI route/common readers, the
static-V2 ingress, AOT harnesses, and process-global published call rows. The
current `HakoLlvmcInvocation` is a physical-lifetime owner candidate, but it
does not carry these options; `HakoLlvmcAllocationConfig` and
`HakoLlvmcIngressProfile` are not substitutes.

The audit decision is
`NoSafeSlice__CompileOptionsNoSingleCrossBoundaryAuthority`. In particular,
the three Rust temporary environment mutation sites cannot be retired by a
static-only change while generic C, Boundary, static V2, AOT, compatibility,
and subprocess callers still read ambient state. The published call-row global
is a separate authority problem and is not silently included in this row.

```text
Decision: Compile-options I1 is a design stop until one cross-boundary owner is accepted.
Source authority + canonical issuer: one Rust admission adapter normalizes the existing options; HakoLlvmcInvocation owns the physical contract.
Non-authority: MIR/source/Recipe meaning, public C ABI, dlsym, and ambient getenv readers issue no new option meaning.
Fail-fast boundary: conflict, unsupported value, nested/re-entry, and unconsumed-row checks precede env mutation, dlsym, and compile.
Smallest next slice: co-seal the invocation-owned compile-options contract with every production caller and its delete/retain set.
Non-claims: no implementation, fallback, concurrent-compile guarantee, or published-row-global retirement is authorized here.
```

Required acceptance before opening I1 is explicit-options success for
pure-first/none, harness, static V2, and v1 compatibility; conflict,
unsupported-value, duplicate/unconsumed-row rejection before effects; complete
unset/empty/present restoration on success and failure; and overlap isolation
or pre-mutation rejection. This row is separate from the already-landed link
direct seam I0 and from the Builder-level duplicate snapshot cleanup recorded
in the G0 acceptance card.

### Compile-options contract consultation (2026-09-12)

The independent read-only consultation confirms the same stop and fixes the
candidate boundary precisely enough for the next authority decision. The
contract is physical invocation state, not a new MIR/Recipe receipt:

```c
#define HAKO_LLVMC_PHYSICAL_CONTRACT_REVISION 1u

typedef struct hako_llvmc_physical_contract_v1 {
  uint32_t revision;
  uint32_t byte_size;
  uint32_t ingress_profile;
  uint32_t flags;
  const char* compile_recipe;
  const char* compat_replay;
  const char* opt_level;
  const char* opt_tool_path;
  const char* llc_tool_path;
  const char* llc_flags;
  const char* llvmc_path;
} hako_llvmc_physical_contract_v1;
```

Rust keeps the existing `Opts` request and one admission adapter resolves it
once, including `HAKO_LLVM_OPT_LEVEL` versus `NYASH_LLVM_OPT_LEVEL`: one value
is accepted, equal aliases are accepted, conflicting aliases reject, and an
unset value normalizes to the existing default. The adapter owns the live
`CString`s for the synchronous call. C validates revision/size/profile/flags,
deep-copies strings into `HakoLlvmcInvocation`, and consumes only that copy;
Rust enums, `String`, `Vec`, and `Option` do not cross the ABI.

The explicit default-visible C entries are the only selected ingress for new
production callers:

```c
int hako_llvmc_compile_json_with_options_v1(
    const char* json_in, const char* obj_out,
    const hako_llvmc_physical_contract_v1* contract, char** err_out);
int hako_llvmc_static_open_v2_with_options(
    const char* bytes, size_t length,
    const hako_llvmc_physical_contract_v1* contract,
    hako_llvmc_static_invocation_v2** out, char** error);
```

The public three-argument compile exports, AOT compile exports, link v1/v2,
and the explicit harness export remain compatibility surfaces. They adapt into
the same invocation owner where applicable; they are not deleted or treated as
new option authorities. AOT link and `HAKO_AOT_USE_FFI` remain in the landed I0
boundary. Static V2 receives options at open, so its existing save/set/restore
RAII is not retained as a second authority. Subprocess and harness callers get
child-local explicit values from the adapter; parent-process environment is
never mutated.

The finite I1 caller disposition is: retain artifact/error handling and the
existing static query/frame/compile/close flow; delete only the three Rust
temporary environment overrides, the static settings RAII, Boundary FFI
`with_env_override`, duplicate compatibility option reads, and explicit C
`getenv` reads after their callers use the contract. Keep link-only environment
handling, public ABI symbols, external compatibility behavior, and published
call-row globals outside this delete-set. A truly hidden symbol is not a Rust
`dlsym` ingress; any Rust caller needs a default-visible versioned symbol in the
FFI library header.

The opening gate is now explicit but not yet passed: prove explicit
pure-first/none, harness, static V2, and v1 compatibility; fake tool/path and
flag propagation; C/Rust `revision`, `byte_size`, `sizeof`, and offset
agreement; success/failure preservation for unset/empty/present environment;
and pre-effect rejection for bad revision/size/profile/flags, alias conflict,
unsupported tool/value, replay/profile mismatch, duplicate/unconsumed rows,
and nested/published-row overlap. Until the finite production caller set is
co-sealed against that gate, `NoSafeSlice__CompileOptionsNoSingleCrossBoundaryAuthority`
and `work_mode = design_stop` remain authoritative.

### MIR-CALL-COMPATIBILITY-RETIRE-R7-I1A (selected implementation slice)

The contract shape is accepted for one bounded production edge. This opens the
explicit Boundary pure-first caller only; it does not claim that static V2,
explicit harness, AOT compatibility, or the public generic ABI have already
switched.

```text
Decision: add the versioned physical-options contract and switch the selected
  Boundary pure-first CAPI caller to it; preserve all other compatibility edges.
Source authority + canonical issuer: existing Rust Opts plus one admission
  adapter normalize request/env aliases once; HakoLlvmcInvocation owns the C copy.
Non-authority: MIR/Recipe meaning, public three-argument symbols, dlsym, and
  ambient C getenv do not issue a new option meaning for this edge.
Fail-fast boundary: contract revision/size/profile, recipe/replay, opt aliases,
  and configured tool values reject before C lowering or output publication.
Smallest next slice: header contract, invocation-owned copy, explicit compile
  entry, Boundary pure-first caller, and focused positive/negative/restore guard.
Non-claims: static-open adoption, harness/AOT propagation, public ABI removal,
  concurrent compile, published-row global retirement, or whole R7 closure.
```

I1A retains the existing C lowering body and only threads its options through
the selected generic/pattern physical terminal. The C entry is
default-visible because Rust `dlsym` cannot call a hidden symbol. Its contract
uses fixed-width integers and borrowed `const char*` inputs; C deep-copies the
strings into the invocation and clears them at destruction. `Opts` remains the
Rust request owner, with no new semantic receipt or settings layer.

The I1A delete-set is deliberately limited to the selected Boundary caller's
`set_var/remove_var` and restoration wrapper plus its old three-argument symbol
lookup. The public C compatibility export and all unselected environment
readers remain until their own explicit caller switch. Acceptance is one
explicit pure-first/none success, invalid revision/profile and conflicting
alias negatives before object effects, fake-tool/flag propagation, and
unset/empty/present environment preservation on both success and failure.

### R7-I1A closeout (2026-09-12)

Implementation landed at `454e49b755` and is pushed on
`codex/birth-definition-publication`. The selected Boundary pure-first CAPI
caller now resolves one Rust-owned request adapter, passes a versioned
default-visible contract to `hako_llvmc_compile_json_with_options_v1`, and
lets `HakoLlvmcInvocation` own the copied C strings through lowering. The
existing generic, exact-seed, and indexOf pattern terminals consume that same
invocation option pointer; explicit unset `llc_flags` is an empty value and
cannot inherit ambient flags. The old environment-mutating three-argument
path remains only for unselected compatibility callers.

Observed acceptance:

- `CC=cc bash tools/build_hako_llvmc_ffi.sh` passed.
- `bash tools/checks/llvm_compile_options_contract_smoke.sh` passed for the
  generic fixture and an exact-seed fixture, including fake opt/llc tool and
  flag propagation, ambient-state preservation, explicit unset-flags, and
  pre-effect revision rejection.
- `CARGO_BUILD_JOBS=1 cargo check -p nyash-rust --features plugins -j1`
  passed with the repository's existing warning baseline.
- `bash tools/checks/current_state_pointer_guard.sh` and `git diff --check`
  passed.
- `llvm_hako_aot_ffi_admission_smoke.sh` remains a named baseline red: the
  existing named direct compatibility call still returns only the host
  `Traceback` marker. It is not counted as I1A evidence or a compiler
  regression claim.

Scope is closed only for this Boundary pure-first compile-options edge.
Static V2, explicit harness, AOT compatibility, public ABI retirement,
published-row globals, concurrent compilation, and aggregate R7 caller-zero
remain open follow-up rows. The next selection must name one such row before
editing it; no whole-R7 completion is claimed.

### Static V2 open I0 closeout (2026-09-12)

The selected static owner landed at `0bae5fdd9c` and is pushed. The existing
published-MIR Rust caller now passes the accepted physical-options contract to
`hako_llvmc_static_open_v2_with_options`; the old Rust settings save/set/
restore seam is deleted. C copies the contract into the existing invocation
owner, while the public three-argument static entry remains compatibility-only.
The focused session test covers explicit open/query/compile/close with fake
tool/path/flag propagation, ambient recipe isolation, pre-effect revision/
size/profile/flags rejection, and the existing malformed/cleanup order. The
host's LLVM14 causes the legacy LLVM18 object portion to report a skip; this is
not runtime or executable evidence. `mir_call_static_v2_open_contract_guard.sh`,
the Boundary options smoke, C build, Rust check, and pointer guard pass.

### Post-static owner audit (2026-09-12)

One independent read-only audit rechecked the remaining production caller set
after static I0. It found no unshared next owner-unit: Rust/C explicit harness,
generic/v1 AOT compatibility, public AOT link/dlsym, and residual C/AOT ambient
option readers retain shared callers or externally observable compatibility.
The physical contract currently admits only the Boundary/Static profiles and
`compat_replay=none`, so switching one compatibility subset would leave the
other readers and public re-entry without an exclusive delete-set.

Decision: keep `NoSafeSlice__NoRemainingUnsharedM7SOwner` and
`work_mode = design_stop`. The next design slice is a co-sealed owner/retain/
delete matrix for those remaining callers, with pre-effect contract/replay/
tool rejection, environment preservation, and explicit non-claims. No code,
fixture, route switch, new receipt, or fallback is authorized by this audit.

### Explicit-harness follow-up audit (2026-09-12)

A second independent read-only audit confirms that the explicit harness is not
an isolated next I0. The Rust explicit-harness route reaches the existing
`mir_json_to_object_llvmlite` provider, which also serves the ambient llvmlite
compatibility route; selecting the harness therefore does not produce a
provider caller-zero/delete-set. The C/AOT harness, generic/v1 AOT entry, and
public AOT link/dlsym entries likewise have distinct external or public
callers, so they cannot be coalesced by extending the current physical profile
alone.

The current physical contract still admits only Boundary/Static profiles and
`compat_replay = none`. Before any I0, the design must co-seal the owner,
terminal, retained callers, exact delete-set, and pre-effect rejection for
contract/replay/tool/symbol/input errors while preserving the AOT FFI split and
ambient environment behavior. Explicit harnesses must not fall back to the
generic route. Decision remains
`NoSafeSlice__NoRemainingUnsharedM7SOwner`; no code, fixture, route switch,
new receipt, or fallback is authorized.

### R7 retained-caller matrix (2026-09-12)

The production census is now fixed to four retained boundary groups. This is a
design inventory, not an implementation permission; the paths below are the
caller/terminal evidence that prevents a profile-only cutover.

| group | current owner and terminal | named callers / re-entry | retain/delete disposition |
| --- | --- | --- | --- |
| Rust codegen ingress | `compat_codegen_receiver.rs:52,96` -> `route.rs:179,186` -> `provider_keep.rs:95,127`; ordinary Boundary and named harness both end at the existing provider/object terminal | backend extern dispatch, loader-cold, hostbridge, global/externals; the same llvmlite provider is also selected by ambient keep | retain shared provider; no provider delete-set until the named harness and ambient keep have separate owners or one explicit shared contract |
| C compile and AOT compile | `hako_llvmc_ffi_route.inc:440,459,488`; `hako_aot_shared_impl.inc:585,601`; generic, pure-first, and named harness terminals remain distinct | AOT dlsym at `hako_aot_shared_impl.inc:413,434`, public C callers, and direct harness callers | retain all public symbols and the AOT FFI split; delete no compile export while dlsym/public callers remain reachable |
| AOT link | `hako_aot_shared_impl.inc:725-768` dispatches compatibility v1 and explicit v2 into the existing link body | `hako_llvmc_ffi_route.inc:357,372`, public `hako_aot_link_obj`, public `hako_aot_link_obj_v2`, and FFI dlsym re-entry | retain v1/v2 ABI and shared dispatch; no wrapper/body deletion until archive authority and `HAKO_AOT_USE_FFI` callers are co-sealed |
| ambient physical selectors | C common/AOT readers (`hako_llvmc_ffi_common.inc:65,112`, `hako_aot_shared_impl.inc:119,529`); the Rust compatibility transport was retired at `7350421205` | generic compatibility, AOT, subprocess/tool selection, and legacy external entry points | delete only caller-specific reads/temporary mutation after explicit invocation state reaches every remaining caller; link-only env and public compatibility remain retained |

The matrix's six-line decision is:

```text
Decision: keep the four groups as separate owner candidates; do not promote a profile-only harness cutover.
Source authority + canonical issuer: each existing caller's accepted physical request; C exports/dlsym consume it.
Non-authority: symbol names, ambient env, provider reachability, and public ABI names do not issue new semantics.
Fail-fast boundary: reject profile/recipe/replay/tool/symbol/input conflicts before env mutation, dlsym, or artifact effect.
Smallest next slice: accept one group-specific owner/terminal/delete row only after its retained callers and exact delete-set are named.
Non-claims: no shared-provider retirement, public ABI removal, AOT route switch, concurrent isolation, or LegacyCallV0 deletion.
```

This closes the finite design census but leaves the implementation gate open:
the current matrix has no non-empty exclusive delete-set. `work_mode` therefore
remains `design_stop`; a future I0 must name one of these groups and its
caller-specific delete-set in the same card before editing code.

The owner-selection consultation also records that AOT link is already closed
by the prior direct-seam I0 and is not a next candidate. The single remaining
cross-group gap is an invocation-owned physical compatibility admission that
can be consumed by the harness, generic/v1 AOT, and ambient-selector callers;
the current contract has only Boundary/Static issuers. Until that authority is
co-sealed with retained public/external assets and a non-empty caller-specific
delete-set, the R7 decision remains unchanged.

### MIR-CALL-RUST-CAPI-AMBIENT-RETIRE-I0 (selected 2026-09-12)

The focused owner audit opens one behavior-neutral Delete row inside the Rust
transport only. `CodegenRouteRequestV1::LegacyAmbientKeep` has zero current
constructors; the only production constructors are Boundary or Explicit
Harness. Every CAPI route validates first, Boundary requires
`pure-first/none`, and Explicit Harness returns before the generic CAPI probe.
Therefore the non-explicit branch in `capi_transport.rs` is caller-zero in the
current repository.

```text
Decision: delete the caller-zero Rust non-explicit CAPI transport branch.
Source authority + canonical issuer: existing explicit Opts route request and its physical-options admission.
Non-authority: public C exports, AOT dlsym, llvmlite provider selection, and external ABI names are retained consumers.
Fail-fast boundary: route validation and explicit-options admission remain before dlsym, env mutation, and artifact effect.
Smallest next slice: remove the old Rust branch/selector and update its route census/guard in one I0.
Non-claims: no public C ABI removal, AOT or provider retirement, concurrent isolation, or aggregate R7 closure.
```

The exact old-edge delete-set is the three-argument dlsym plus environment
save/set/restore in `capi_transport.rs`, the `compile_symbol` plumbing used
only by that branch, `LegacyAmbientKeep`, the Rust-side
`compile_symbol_for_keep_recipe`/unused symbol constants and tests, and the
caller-zero Rust `ny-llvmc` provider helper. Retain
`compile_via_capi_with_options`, `hako_llvmc_compile_json_with_options_v1`,
the public generic/pure-first C exports, AOT dlsym, and the explicit llvmlite
provider helper. The separate `ny-llvmc` crate/driver remains the mainline
Boundary owner; this row removes only the old host-provider subprocess helper.
The existing route-identity guard and llvmlite production census must be
changed from expecting the old branch to proving its absence and classifying
the route as caller-zero.

I0 acceptance is: Boundary `pure-first/none` still reaches the explicit
options entry; Explicit Harness still bypasses CAPI and reaches the named
provider; invalid recipe/replay still rejects before effects; missing CAPI
still returns the existing unavailable terminal; the updated route guard,
census guard, source-size bound, `git diff --check`, and focused Rust route
tests pass. This row retains all C/AOT/public callers and does not claim R7
wide retirement.

### Rust ambient transport I0 closeout (2026-09-12)

The caller-zero Rust non-explicit CAPI branch is retired. The typed
`BoundaryPureFirst` request now reaches only
`compile_via_capi_with_options`; `ExplicitHarnessCompat` reaches the existing
llvmlite provider directly, and no Rust route selects the removed
`ny-llvmc` subprocess helper or mutates compile-recipe/replay environment
variables. Public C exports, AOT dlsym, the separate `ny-llvmc` crate/driver,
and the explicit llvmlite compatibility provider remain retained consumers.

Observed acceptance:

- focused Rust route tests: 5 passed, 0 failed;
- `cargo check -p nyash-rust --features plugins --profile quick -j1`: passed
  with the repository warning baseline and no new `provider_keep` warning;
- `llvm_codegen_route_identity_guard.sh`: passed;
- `llvm_llvmlite_production_census_guard.py`: passed (`rows=37`,
  `automatic_python_ingress=0`, `native_retry=0`, `keep_roots=26`);
- `current_state_pointer_guard.sh`, JSON validation, and `git diff --check`:
  passed.

The host preflight still reports Ubuntu 22.04 with LLVM14.0.0 only, so no
LLVM18 object/EXE runtime evidence is claimed. The follow-up environment task
`LLVM18-TOOLCHAIN-INSTALL-I0` is registered at
`docs/development/current/main/investigations/llvm18-toolchain-installation-task-2026-09-12.md`.
It uses the existing CI `apt.llvm.org` recipe and keeps LLVM14 side-by-side.

This closes only the Rust caller-zero transport row. The remaining C/AOT,
public, harness, provider, and aggregate R7 rows remain open by design.

### Post-Rust-I0 owner consultation (2026-09-12)

The read-only follow-up audit rechecked the remaining production callers after
the Rust transport closeout. It found no new standalone owner: C/AOT compile
and public dlsym callers share public compatibility surfaces, while explicit
harness and llvmlite/provider callers share retained provider/entry behavior.
Their exclusive delete-sets are empty under the current physical contract.

Decision remains `NoSafeSlice__NoRemainingUnsharedM7SOwner`; keep
`work_mode = design_stop`. The next design slice must co-seal one remaining
owner, terminal, retained callers, exact delete-set, and pre-effect rejection
for contract/replay/tool/symbol/input errors before any new implementation.
This audit did not edit code, add fixtures, add receipts, or run Cargo.

### Shared invocation-owned compatibility admission D0 (selected 2026-09-12)

The second independent read-only design consultation accepts one next design
slice: define a shared invocation-owned physical compatibility contract for
the remaining callers. This is a design stop, not an implementation grant.

```text
Decision: co-seal one invocation-owned physical compatibility contract before selecting another R7 implementation row.
Source authority + canonical issuer: each existing explicit physical entry request, normalized once by one admission adapter; C consumes a deep-copied HakoLlvmcInvocation.
Non-authority: MIR/Recipe meaning, profile names alone, ambient environment, provider reachability, public ABI names, and test-only emitters issue no new semantics.
Fail-fast boundary: entry/profile -> revision/size/flags -> recipe/replay/provider/tool/alias -> input/path/re-entry -> contract copy -> child/provider/lowering -> artifact publication.
Smallest next slice: fix a finite issuer -> terminal -> retained-caller/delete-set matrix and its pre-effect rejection obligations for every remaining compatibility group.
Non-claims: no code, route switch, fallback, public ABI deletion, concurrent guarantee, LLVM18 runtime evidence, semantic receipt, or R7 completion.
```

The design boundary is finite and keeps the existing terminals:

| issuer / caller group | current terminal | retained surface | design obligation |
| --- | --- | --- | --- |
| Rust `compat_codegen_receiver` and stage1/boundary compatibility | C options entry or explicit llvmlite provider | Rust route, `boundary_driver_ffi`, llvmlite keep | distinguish Boundary, explicit harness, and provider ownership without ambient retry |
| C generic/pure-first and named harness | `hako_llvmc_ffi_route.inc` terminals | public C compile exports and existing C lowering | carry explicit invocation options while retaining public symbols |
| generic/v1 AOT and AOT dlsym | `hako_aot_shared_impl.inc` compile/link terminals | AOT v1/v2, dlsym, runtime archive/link split | preserve public re-entry and classify link-only environment separately |
| C common/AOT ambient selectors | existing tool/provider child selection | compatibility subprocess/tool behavior | isolate invocation-owned options without parent-process mutation or fallback |

The canonical issuer is the existing physical request at each explicit entry;
the shared adapter may normalize its transport fields once, but it must not
issue a new MIR or Recipe product. C-side `HakoLlvmcInvocation` remains the
copy/lifetime owner. All public C ABI, AOT v1/v2, dlsym, `ny-llvmc --driver
harness`, llvmlite/provider keep, and existing C lowering remain retained until
a later row names a real caller-specific delete-set.

The counterexample blocking profile-only routing is fixed: Rust explicit
harness reaches the llvmlite provider, while
`hako_llvmc_compile_json_compat_harness` reaches `ny-llvmc --driver harness`
and AOT dlsym can re-enter a third owner. The shared `harness` label therefore
does not identify a physical terminal.

Design acceptance is a source-backed finite table covering every listed issuer
and terminal with `fallback=none`, explicit retained/delete disposition, and
pre-effect negatives for profile/replay/provider/tool/alias conflicts,
unset/empty/present environment preservation, nested invocation, and dlsym
re-entry. Only after that table contains a non-empty caller-specific delete-set
may `work_mode` leave `design_stop`. Until then the decision is
`NoSafeSlice__NoRemainingUnsharedM7SOwner`.

### MIR-CALL-NY-LLVMC-BOUNDARY-COMPILE-OPTIONS-I0 (selected 2026-09-12)

The focused source audit found one real production caller with an exclusive
delete-set: `ny-llvmc`'s default `DriverKind::Boundary` reaches
`boundary_driver_ffi.rs` through `boundary_driver.rs`, and no other crate
caller enters that Rust helper. The existing C
`hako_llvmc_compile_json_with_options_v1` ABI already owns the physical
options copy/lifetime, so this is an in-place transport replacement rather
than a new semantic or settings authority.

```text
Decision: switch the ny-llvmc Boundary compile caller to the existing versioned physical-options C entry.
Source authority + canonical issuer: the Boundary CLI's one captured physical request, encoded by a private repr(C) ABI mirror of the tracked C contract.
Non-authority: MIR/Recipe meaning, public C symbol names, AOT dlsym, explicit harness/native drivers, and environment mutation do not issue new semantics.
Fail-fast boundary: capture/validate recipe, replay, alias, level, and C layout before dlsym or lowering; C repeats contract, tool, input, and artifact checks.
Smallest next slice: remove Boundary Rust three-argument symbol selection/env override and pass the existing pure-first/none contract with explicit tool values.
Non-claims: no public C ABI removal, AOT/provider retirement, concurrent guarantee, LLVM18 runtime evidence, new receipt/settings layer, or R7 completion.
```

Exact delete-set: `CompileFn`, the three-argument `hako_llvmc_compile_json{,_pure_first}`
lookup, `boundary_compile_symbol` and its constants/tests, the Rust
`with_env_override` helper and its test, and the Boundary caller's temporary
recipe/replay environment mutation. Retain the existing public C generic,
pure-first, and harness exports; C/AOT dlsym; the C lowering; `link_obj_v2`;
and explicit `--driver harness/native` routes. The private Rust `repr(C)` row
must match the tracked `hako_llvmc_physical_contract_v1` layout exactly and
must not become a second authority.

Acceptance is: Boundary normal and dummy object emission pass through the
options symbol; pure-first/none is the only admitted Boundary pair; conflicting
recipe/replay or `HAKO_CAPI_PURE` rejects before library/symbol/lowering
effects; revision/size/profile/flags/tool/input failures retain the existing C
diagnostics; environment values remain unchanged; and public C/AOT/dlsym
callers remain source-covered. The implementation is one Boundary caller
switch plus old-edge retirement, with no fallback or retry.

### Boundary compile-options I0 closeout (2026-09-12)

Implementation landed in commit `36d7fc784f`. The production `ny-llvmc` Boundary caller now reaches the existing versioned
options entry with one invocation-owned Rust row. The old Rust three-argument
lookup, symbol selector, and process-environment save/set/restore path are
absent; public C generic/pure-first/harness exports, AOT dlsym, explicit
harness/native drivers, and `link_obj_v2` remain.

Observed acceptance:

- targeted Rust Boundary tests: 3 passed, 0 failed;
- `cargo check -p nyash-llvm-compiler --profile quick -j1`: passed;
- production Boundary normal and `--dummy` object emission passed with
  explicit fake opt/llc tools; the trace named
  `hako_llvmc_compile_json_with_options_v1` and both artifacts were written;
- Boundary recipe, replay, `HAKO_CAPI_PURE`, and opt-level alias conflicts
  rejected with no artifact;
- `llvm_compile_options_contract_smoke.sh`,
  `llvm_codegen_route_identity_guard.sh`,
  `llvm_llvmlite_production_census_guard.py`,
  `mir_r7_legacy_census_manifest.py`, and the current-state pointer guard:
  passed;
- the broader crate run was `50 passed, 2 failed`: one is the pre-existing
  LLVM14 opaque-pointer failure, and the other is the existing direct public
  pure-first fixture's `CheckedCallOut` metadata mismatch; neither enters the
  changed Rust Boundary caller and neither is claimed as an I0 regression;
- the workspace-wide format check still reports unrelated baseline drift;
  the two changed Rust sources pass targeted rustfmt checks.

This closes only `MIR-CALL-NY-LLVMC-BOUNDARY-COMPILE-OPTIONS-I0`. The next
R7 decision stop must select the remaining shared compatibility owner and a
non-empty caller-specific delete-set. LLVM18 remains an environment-only task
blocked by external sudo permission; no LLVM18 object/EXE runtime evidence is
claimed.

### LLVM18 environment follow-up (blocked 2026-09-12)

The host check found Ubuntu 22.04 with only LLVM14.0.0 and no Jammy archive
candidate for LLVM18. The environment-only installation task is
`LLVM18-TOOLCHAIN-INSTALL-I0` at
`docs/development/current/main/investigations/llvm18-toolchain-installation-task-2026-09-12.md`.
It uses the existing CI `apt.llvm.org` recipe, keeps LLVM14 installed
side-by-side, and requires versioned tool/header/prefix verification before
rerunning the named G0 object/EXE witness. The session cannot run the recipe:
UID 1000 has no non-interactive sudo permission (`sudo: a password is
required`). This task does not authorize a backend switch, a fallback, or a
semantic MirBuilder change; resume it after the external permission is granted.

### MIR-CALL-AOT-GENERIC-COMPILE-OPTIONS-I0 (selected design 2026-09-12)

Two independent read-only worker audits selected the remaining AOT Generic
caller as the next bounded owner. The source-backed exclusive edge is
`hako_aot_try_ffi_compile` in `lang/c-abi/shims/hako_aot_shared_impl.inc`:
it alone dlsyms the old three-argument `hako_llvmc_compile_json`. The existing
versioned `hako_llvmc_compile_json_with_options_v1` contract and the
invocation-owned C options copy are the physical owner; no new ABI, receipt,
settings layer, or semantic product is introduced.

```text
Decision: route AOT Generic FFI compile through the existing versioned physical-options C entry.
Source authority + canonical issuer: the AOT Generic physical request, encoded once as the tracked C contract with ingress profile GENERIC_COMPAT(0).
Non-authority: MIR/Recipe meaning, profile labels alone, ambient environment, public ABI names, named harness, link dlsym, and tests issue no new semantics.
Fail-fast boundary: AOT args/FFI admission -> recipe/replay/pure/level/tool validation -> options symbol -> revision/size/profile/flags/tool copy -> JSON/invocation -> lowering -> artifact.
Smallest next slice: accept profile 0 in the shared options contract, pass that profile into the existing invocation, and delete only AOT Generic's old compile dlsym/type/call edge.
Non-claims: no public old-symbol removal, no named-harness or link change, no fallback/retry, no provider retirement, no LLVM18 evidence, and no R7/MIRBuilder completion.
```

Finite outcome and ownership boundary:

| issuer / input | terminal | retained / deleted | pre-effect obligation |
| --- | --- | --- | --- |
| AOT Generic + FFI + `pure-first/none` | existing generic C lowering or its typed error | accept; delete only old AOT compile dlsym edge | reject invalid args, FFI mode, recipe, replay, `HAKO_CAPI_PURE`, level, tool, and alias before dlsym |
| AOT Generic without FFI | existing `aot-compat-admission-required` terminal | retain; no fallback | do not invoke child or old public compile symbol |
| AOT Generic invalid contract/profile 3/unknown | existing compile-options contract reject | reject; retain profile 1/2 strict behavior | revision, size, profile, flags, recipe, replay, and tool checks precede JSON/lowering |
| named AOT harness | existing named harness provider/export | retain; no shared-profile change | keep direct harness route and its explicit compatibility admission |
| public C generic/pure-first/harness and link v1/v2 | existing public C/link owners | retain; out of scope | source coverage remains; no AOT compile/link re-entry is added |

The exact implementation obligations are: `hako_llvmc_physical_options_copy`
accepts physical profiles 0/1/2 and rejects 3/unknown; the options compile
entry maps the validated contract profile to the existing invocation profile
(0 Generic, 1 Boundary; Static remains a separate retained profile owner);
and AOT Generic constructs revision 1, `sizeof`-sized, flags-zero,
`GENERIC_COMPAT(0)`, `pure-first/none` contract data without mutating the
parent environment. AOT must carry the effective opt level, opt/llc paths,
and llc flags; a null llc-flags value is an explicit empty value in this
contract and must not silently restore the old ambient default. The AOT-side
`HAKO_CAPI_PURE` rejection remains before `dlopen`/`dlsym` because the old
generic export previously owned that admission.

Exact delete-set: the private AOT generic three-argument typedef, its
`dlsym(h, "hako_llvmc_compile_json")`, the private call, and the associated
missing-symbol diagnostic. Retain public
`hako_llvmc_compile_json{,_pure_first,_compat_harness}`, AOT v1/v2 and named
harness exports, Boundary/Static options, link dlsym, and direct harness
environment handling. No fallback or retry is permitted.

Design evidence: Rawls audited the finite AOT caller/delete-set matrix;
Gibbs independently audited profile-0 plumbing and confirmed that merely
allowing profile 0 is unsafe unless the contract profile reaches invocation
initialization and AOT preserves the pure/replay admission. Both audits were
read-only with no Cargo, fixture, or worktree changes. The implementation may
start in `fast` mode only after this Decision is recorded.

### AOT Generic compile-options I0 closeout (2026-09-12)

Implementation is complete in the bounded C owner set. AOT Generic FFI now
constructs the existing revisioned contract with Generic profile 0 and calls
`hako_llvmc_compile_json_with_options_v1`. The shared options copy admits
profiles 0/1/2 for their existing owners; the JSON compile entry maps only
Generic 0 and Boundary 1 into invocation profiles, while Static 2 remains on
its dedicated V2 entry and Explicit Harness 3 rejects. The old private AOT
three-argument compile typedef, dlsym, call, and missing-symbol diagnostic are
deleted. Public C compile exports, AOT named harness, AOT v1/v2, Boundary and
Static options, and link dlsym remain.

Observed acceptance:

- `bash tools/build_hako_llvmc_ffi.sh`: passed;
- `llvm_compile_options_contract_smoke.sh`: passed; direct Boundary and
  Generic profile paths consumed explicit fake opt/llc settings, AOT Generic
  produced an object through the options symbol, and recipe/replay,
  `HAKO_CAPI_PURE`, revision, and Explicit Harness profile rejects produced no
  artifact;
- `llvm_codegen_route_identity_guard.sh`,
  `llvm_llvmlite_production_census_guard.py`,
  `mir_r7_legacy_census_manifest.py`, `current_state_pointer_guard.sh`, and
  `git diff --check`: passed;
- source-size bound: `hako_aot_shared_impl.inc` is 742 lines and the new
  `hako_aot_generic_ffi_compile.inc` is 105 lines, both below the 800-line
  hard stop. The new include is part of the shared AOT source truth.

The object evidence uses fake tools and therefore claims route/contract
reachability only; LLVM18 is still unavailable on this host, so no LLVM18
object/EXE runtime claim is made. This closes only
`MIR-CALL-AOT-GENERIC-COMPILE-OPTIONS-I0`; public C, named compatibility,
provider, aggregate R7, backend parity, and whole-MIRBuilder completion remain
open by design. The next R7 decision stop must select another finite owner
with a non-empty delete-set.

### Public C Generic compile ingress disposition (design stop 2026-09-12)

The read-only public-ingress audit selected Stop / retain, not an immediate
options-ABI cutover. The public three-argument
hako_llvmc_compile_json is an externally consumable compatibility ABI; its
current Generic profile intentionally keeps compile_doc_compat_pure and
ambient tool fallback semantics. The existing options contract is stricter
and cannot be substituted until its compatibility conversion rules are
explicitly owned.

    Decision: stop the public C Generic ingress at its existing typed admission and retain its external ABI and Generic lowering.
    Source authority + canonical issuer: the public three-argument C request plus its existing recipe/alias/replay admission in hako_llvmc_ffi_route.inc.
    Non-authority: profile labels alone, options defaults, external symbol presence, tests, and a guessed environment snapshot do not redefine public compatibility.
    Fail-fast boundary: public args -> alias/recipe/replay admission -> existing Generic JSON reader/core -> existing tool/lowering terminals; no fallback from Generic to named harness.
    Smallest next slice: design the explicit environment-capture/compatibility matrix for recipe, replay (including unknown values), tool fallback, opt level, empty flags, alias, and nested invocation before any public route edit.
    Non-claims: no public ABI deletion, no Generic options cutover, no shared-core retirement, no fallback/retry change, no new receipt, no LLVM18 evidence, and no R7/MIRBuilder completion.

Census boundary: public hako_llvmc_compile_json -> its existing Generic
JSON/core terminals; includes the public pure-first wrapper, Generic profile 0
options caller (already closed through AOT), Boundary profile 1, Static profile
2, named C/AOT harness, and link v1/v2 as retained neighboring surfaces;
excludes external callers not visible in the repository, the dedicated Static
V2 owner, and provider/llvmlite execution.

| issuer / condition | terminal | authority and disposition | fallback |
| --- | --- | --- | --- |
| public Generic, pure-first, replay unset/none | existing Generic profile 0 lowering or typed error | retain public ABI, legacy reader, and compile_doc_compat_pure | none to named harness |
| HAKO_CAPI_PURE=1 | env/hako_capi_pure_retired | retain existing alias reject/diagnostic | none |
| recipe unset/non-pure-first | generic-capi-recipe-required | retain existing recipe gate | none |
| replay=harness | generic-capi-compat-admission-required | retain explicit named C harness owner | no implicit replay |
| unknown replay / empty and present tool or flag values | current ambient compatibility behavior, not yet fully normalized | CutoverBlockerOpen; design conversion rule first | no guessed options mapping |
| Generic profile 0 options (AOT) | existing options Generic lowering | retain already-closed AOT owner; no public re-entry | none |
| Boundary 1 / Static 2 / Explicit Harness 3 | existing strict, Static V2, or reject terminals | retain separate physical owners | no profile coercion |
| public link v1/v2 and AOT link dlsym | existing link terminals | retain; outside compile row | none |

The public route's exact current chain is
hako_llvmc_compile_json -> recipe/alias/replay admission ->
compile_json_via_pure_first_lane -> compile_json_compat_pure(profile=0) ->
compile_doc_compat_pure. The apparent delete candidates
compile_json_via_pure_first_lane and its call edge are not a safe production
delete-set: the public symbol cannot reach caller-zero from repository evidence,
and the shared core remains consumed by options and Static V2/test lifetimes.
The next design slice must resolve, in one table, whether each unset/empty/
present value is preserved, captured, rejected, or mapped to an existing
terminal, including unknown replay and tool-resolution failure. Until that
table and a non-empty caller-specific delete-set exist, keep
work_mode = design_stop and retain all public C/AOT/harness/link surfaces.

Design evidence: Erdos independently audited the public C route, shared-core
consumers, and environment mismatch without editing, Cargo, fixture, or receipt
creation. This finding supersedes the earlier broad remaining C/public
candidate wording only for this public Generic ingress; it does not reopen the
closed AOT Generic slice.
