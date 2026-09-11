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
five compile-environment route anchors across four families, and the known
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
selected admission source change (243/127 Legacy, five environment anchors,
four test-only Loop-PHI files, 252 rows). After that, select one owner-unit
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
