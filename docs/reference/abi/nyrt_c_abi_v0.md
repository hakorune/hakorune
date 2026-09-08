# NyRT Core C ABI v0 (Runtime Boundary)

Updated: 2026-02-13

This document defines the runtime-side C ABI lane.

Important boundary:
- Plugin method dispatch is not defined here.
- Plugin dispatch uses TypeBox ABI v2 (`docs/reference/plugin-abi/nyash_abi_v2.md`).

See also:
- `docs/reference/abi/ABI_BOUNDARY_MATRIX.md`
- `docs/reference/abi/nyrt_host_surface_v0.md`
- `docs/development/current/main/phases/phase-29y/10-ABI-SSOT.md`

## 1. Scope

Core C ABI covers:

1. Runtime route entrypoints (bootstrap/load/execute)
2. Runtime verifier/safety gate entrypoints
3. Plugin -> host reverse-call entrypoints
4. Handle lifecycle exports used by lifecycle contract (`borrowed args / owned return`)

## 2. Canonical Headers

### `include/nyrt.h`

Current v0 header provides minimal runtime scaffold:

- `nyrt_init()`
- `nyrt_teardown()`
- `nyrt_load_mir_json(const char* json_text)`
- `nyrt_exec_main(uint64_t module_handle)`
- `nyrt_verify_mir_json(const char* json_text)`
- `nyrt_safety_check_mir_json(const char* json_text)`
- `nyrt_hostcall(...)`

### `include/nyrt_host_api.h`

Host reverse-call ABI for plugins:

- `nyrt_host_call_name(...)`
- `nyrt_host_call_slot(...)` (preferred stable call path)

TLV values are used at this boundary.

### `include/nyrt_dynamic_v2_lease_v1.h`

The selected Boundary AOT lane uses this versioned C calling-convention
projection for the physical `CheckedCallOutEnd` cutpoint:

- `nyrt_dynamic_v2_lease_consume_end_authorized_v1(uint64_t) -> uint32_t`
- `0`: the existing Rust one-shot lease was consumed;
- `1`: zero/invalid token;
- `2`: unknown or already-consumed token;
- `3`: stale handle identity.

The header owns only fixed-width ABI/status vocabulary. The lease table,
generation check, and handle release remain solely in
`runtime::dynamic_v2_lease`. Boundary lowering treats a non-zero status as a
backend contract failure, not as a semantic Fault or fallback. This is a
projection for the static Boundary lane, not a second lifecycle authority.
Fresh End-authorized text publication obtains the handle and its reusable-slot
generation identity in one host-handle registry write-lock transition before
the existing lease table admits the token. Token collision/exhaustion rolls
back only that new identity; it never releases an existing generation.

### Target-compiled Fault ABI descriptor v1

Each selected `libnyash_kernel.a` contains exactly one
`.nyash.runtime_abi.v1` ELF object section. It is a fixed 200-byte
little-endian wire record, with magic `NYRTABI1`, record revision and length,
the Cargo target triple, endian and pointer-width facts, Fault/status ABI
revisions, and `NyrtFaultDiagnosticV1` / `NyrtFaultFrameV1` size, alignment,
and field offsets. `crates/nyash_kernel/src/exports/fault.rs` emits it from
the target-compiled Rust representation; it is not a host `sizeof` receipt.

The lifecycle host reads the descriptor only by this section name from the
explicit archive path. Missing, duplicate, malformed, unsupported, or
internally inconsistent records reject before user-object output. Selecting an
archive and proving equality to the backend target/LLVM layout are separate
invocation responsibilities; this descriptor does not authorize C lifecycle
execution or process-exit policy.

### Selected Birth scalar boundary

Decision (2026-09-07): the lifecycle-private unannotated Birth boundary uses
separate kind and payload lanes, preserving Integer versus Bool. The receiver
is a separate object handle; it never consumes a source argument ordinal.
Kind 1 denotes Integer with any signed i64 payload; kind 2 denotes Bool with
payload exactly 0 or 1. Kind 0 and other tags are invalid in this bounded
protocol. These tags do not reuse Dynamic-V2/CallSlot or heap-handle encoding.
The existing source argument issuer supplies the kind; MIR types, constants,
first-call values and field names cannot recover missing source authority.

The exact i64 FieldSet consumer checks the kind before calling the existing
raw-i64 checked store. Valid Bool produces `FieldTypeMismatch`, reserved reason
103, at the already issued FieldSet diagnostic site, with details
`{expected_kind=1, actual_kind=2}`. It calls `nyash.fault.record_static_v1` and
follows that FieldSet's existing Fault successor only on Fault. Recording
failure, unknown kind or malformed Bool payload follows InvalidContract, never
source Fault. Propagation adds no second diagnostic; root reclaim precedes
final report/disposal. The failed field is not mutated. Reasons 100/101/102
retain their existing meanings. No boxing or additional runtime wrapper is
introduced; FaultFrame layout and the runtime descriptor revision are unchanged.

The v2 compiler input and V4 consumer implement this bounded wire and
diagnostic interpretation. Unresolved formal/actual relations still reject
before input issuance; selected host cutover remains separately gated. Local has no selected initializer-kind relation and remains unavailable;
Text/handle arguments are outside this bounded protocol. Source declarations
remain unannotated and each definition has one unspecialized body.

### Selected Map literal store v1

Decision (2026-09-08): the selected Map literal consumer uses one explicit-kind
runtime call, independent of the permissive legacy Any encoding:

```c
uint32_t nyash.map.literal_store_v1(
    int64_t map, int64_t key, uint32_t kind, uint64_t payload);
int64_t nyash.box.from_i8_string_const_len_v1(const uint8_t *bytes, uint64_t len);
```

The dotted names are linker export names, not C source identifiers. Kernel
exports are implemented in `plugin/map_literal.rs` and `exports/box_helpers.rs`.
Private V2 compiler consumers execute allocation, ExactBits, selected physical
operations and direct String/Map OriginalHandle writes through both C walkers,
with linked-kernel readback and nonzero-status trap evidence. Bool operation
results are widened at their producer; Integer comparison uses its selected
physical operation without runtime kind inference.
String keys retain their byte length and zero handles trap at materialization.
The complete projection/formal consumer and public host/source switch remain
open; the physical fixture proof does not establish source-to-EXE cutover.

Map kind constants are scoped to this protocol: `NYRT_MAP_LITERAL_I64=1`,
`BOOL=2`, `F64=3`, `VOID=4`, `HANDLE=5` (each with the same prefix). Zero and
unknown kinds reject. I64 retains its signed two's-complement bits; Bool accepts
only0/1; F64 retains IEEE754 binary64 bits; Void accepts only0. Null is runtime
Void. Handle requires a live registered value and preserves the existing Map
String/StringView borrow/materialization and other-value clone policy. No
numeric payload is tested against the handle registry to infer its kind.
Typed-store indices/direct-slot pointers and direct-array pointers are not
Handle5 values. Their object identity, lifetime and thread-transfer escape
contract remains a compiler cutover blocker; scalar bit round-tripping through
legacy collection storage is not a boxed-object representation proof.

Map and key are borrowed live handles; key must be a StringBox. The call retains
no borrowed input pointer. Validate and materialize key/value outside the Map
write lock, then commit once through `MapBox::insert_key_str`, preserving current
key normalization and duplicate replacement. No temporary scalar handle is
registered. Status `NYRT_MAP_LITERAL_OK=0` means committed;
`NYRT_MAP_LITERAL_INVALID_CONTRACT=2` means no insertion. Status1 is reserved and
must not be interpreted as source Fault. Any nonzero status takes the selected
consumer's contract-failure terminal, without retry or continuation. Existing
fatal allocator/lock termination is outside returned-status guarantees; there
is no added FaultFrame or all-OOM recovery promise.

The length-aware String entry accepts a nonnull pointer to `len` readable bytes
within one valid allocation, without concurrent mutation during the synchronous
call, with `len <= isize::MAX` on the runtime target. Even
an empty string uses a nonnull pointer. Null, an out-of-range length or invalid
UTF-8 returns0; arbitrary dangling pointers cannot be validated by this ABI and
violate caller memory preconditions. Embedded NUL is valid text. Successful
conversion delegates to existing `string_literal_handle_from_text`, preserving
its content-keyed cache/handle policy; no pointer-keyed or second cache. The
consumer treats0 as a contract failure. Exact byte length must survive JSON,
C constant storage and LLVM byte emission before this entry is called.

Compiler input revision/layout and formal-domain propagation are still gated by
the [Map projection owner](../../development/current/main/design/collection-literal-construction-ssot.md#published-operand-projection-inventory).
Do not reinterpret the existing static C row layout or widen call signatures
silently to implement this planned runtime contract.

Decision (2026-09-08): the unpublished static compiler frame v2 distinguishes
result representation from physical operation selection. Its value row has an
Operation action with a finite selection for I64 binary, I64/Bool/String
comparison, String concat and I64/Bool Not; all other actions zero that field.
The selection fixes the result kind/encoding and requires the original producer.
The planner must validate operand domains and opcode; C checks the same body
instruction without inferring a String subtype from an arbitrary Handle.
This schema declaration is not producer admission or executable C support, and
does not change runtime Map tags or activate static compiler v2.

Decision (2026-09-08): private V2 Copy/NamedAlias transfers forward Map kind and
payload independently of the optional original lane. Their wire kind, encoding,
payload and operation fields are zero; operands come from the exact retained body.
NamedAlias requires the captured selected walker's AliasOperandZero outcome.
With original-required clear the old producer is omitted; with it set the
existing producer and its original input demand remain mandatory. CopyOwned is
not erased by this rule. Phi/Select/Formal and public cutover remain unimplemented.

### Map-demanded callable linkage (accepted design, not implemented)

Decision (2026-09-08): the new selected Map consumer's expanded same-module
formal ABI is compiler-private. Its physical projection assigns one internal
LLVM function symbol per demanded canonical definition and projects every exact
selected call through that same mapping. Semantic definition keys, logical
arity and the public zero-argument root entry remain unchanged. Neither the
external linkage of the old emitter nor `alwaysinline` proves a closed caller
set. The new target must not retain an externally callable old-name alias or
an adapter that guesses kinds from old raw arguments. Any separately admitted
export ABI must be accounted for explicitly before switching that definition.
Compatibility plan/direct-symbol ingress to the changed target rejects before
emission. This planned compiler ABI does not change the two canonical
runtime/plugin ABIs or the still-live static compiler transport v1.

### Selected lifecycle physical program v2

Decision (2026-09-07): replace the untagged v1 document, retaining one parser
and one V4 physical consumer. Schema is exactly
`hako.published-lifecycle-physical-program.v2`. Function keys are exactly
`name, role, receiver, params, entry, blocks`. Root uses `receiver:null` and
`params:[]`; Birth uses an explicit receiver ValueId (u32) and source-ordered
`params:[{"value":11,"representation":"kind_payload_v1"}, ...]`. Params omit
receiver. They share the existing SSA namespace; receiver, params and other
value definitions must not collide. No redundant ordinal/arity/lane fields.

Birth call keeps `target, receiver, args, dst` with `dst:null` for Unit;
args become `[{"kind":1,"value":21}, {"kind":2,"value":22}]`. Kind is an
unsigned JSON integer; value is the existing payload SSA ValueId (u32), not
another literal copy. Explicit Bool is
`{"op":"const_bool","dst":22,"value":true}` with a JSON boolean only.
Integer(1) and Bool(true) remain different definitions. Call kind must agree
with payload representation; malformed/unknown keys, values, references,
dominance, arity, target or kind disagreement reject before LLVM/artifact.
Other root/layout/operation/edge keys keep their existing closed contracts.

Object field `storage_kind` remains the unsigned wire value1 for I64, named
`HAKO_LLVMC_LIFECYCLE_STORAGE_I64` in Rust physical input and the shared C
header. It is independent of Birth actual kind and source type. Unknown storage
tags reject before artifact output; no value, layout or ABI revision changes.

MIR keeps receiver + N logical parameters. LLVM projects these once into
`ptr frame, i64 receiver, (i32 kind, i64 payload) x N`; existing physical_ordinal
remains the MIR parameter index. Birth formal has TAGGED representation, root
ConstBool has BOOL, object identities have HANDLE, and scalar arithmetic/result
has I64. Tagged Copy preserves both lanes. Add and I64 return cannot read tagged
payload or HANDLE as an integer. Every tagged FieldSet consumes the kind check
and its existing Fault/Normal edges according to the scalar decision above.

The v2 validator replaces the v1 validator/export and their host/internal V4
callers in the same schema implementation. V4 compile keeps its pointer ABI
and symbol, but becomes v2-document-only. Old v1 schema and numeric-array
params/args reject without default tags before target-session/temp-output work.
FaultFrame/status ABI and runtime descriptor stay v1: the changed revision is
the compiler physical document, not the runtime frame. V2 replaces v1 in the compiler serializer, validator export and V4 consumer.
The selected host now passes the same completed v2 input through exact numeric
capability and V4 compile, using an explicit runtime session. Capability binds
runtime obligations to physical borrow identity, exact FieldSet/value, canonical
field/layout and retained tagged formal coverage. Generic OBJ admission is
unchanged. Default source CLI acceptance and old transport retirement remain
separate acceptance requirements; host-test execution alone does not close them.

### `include/nyrt_dynamic_call_slot_v2.h` and `include/nyrt_dynamic_text_scan_v1.h`

The selected Boundary AOT CheckedCallOut lane uses the versioned CallSlot
wire and TextScan entry declarations. The headers own fixed-width transport,
entry, ABI, and wire vocabulary only:

- `hako.text.scan.substring.v1` has logical arguments `(receiver, start, end)`
  and produces an EndAuthorized host-handle result;
- `hako.text.scan.index_of.v1` has logical arguments `(receiver, needle)` and
  produces an ImmediateI64 result with no lease;
- both entries write semantic Normal/Fault and payload/disposition/lease data
  to `HakoDynamicV2CallOutV1`; the `uint32_t` return is transport status.

The canonical MIR `CheckedCallOut` plan/census owns site identity, CFG
successors, Normal projection, and End chronology. The C physicalizer consumes
that site-id projection and emits direct calls plus local trap paths for
malformed transport, wire, or lease status. It does not choose a provider,
reconstruct a site from block coordinates, or turn a backend contract failure
into semantic Fault/fallback. Link, artifact validation, and live publication
remain later W6 transactions.

The AOT JSON handoff also carries the one closed census view for each selected
site: source block, receiver/arguments, Normal/Fault landing, Normal-result
block/index, effect, and the three End cutpoints. Boundary C1 compares those
facts with the emitted MIR JSON topology (including exact landing incoming
edges) before object emission; JSON is transport and does not become a second
CFG or cleanup authority.

When C1 splits one MIR edge into validation/trap blocks, its physical CFG
projection also issues the exact LLVM predecessor label keyed by the original
MIR predecessor and successor. PHI lowering consumes that edge-keyed row; it
does not infer labels from naming conventions or reuse one tail label for all
successors of a block. The projection is closed before LLVM output and any
missing, duplicate, dangling, or emission-drift row rejects the candidate.

Wire failures are backend contract violations, not semantic Faults: C1 traps
on nonzero transport status, unknown status/tag/disposition, Suspended,
malformed reserved/lease fields, an I6 EndAuthorized host-handle payload of
zero, or a Fault code outside the header's fixed `1..=8` range. I7's
ImmediateI64 zero is valid. Only a known semantic Fault code reaches the MIR
Fault landing.

### W6 explicit static link boundary

The selected Boundary W6 route uses the versioned link symbol:

```c
int hako_llvmc_link_obj_v2(
    const char* object_path,
    const char* temporary_executable_path,
    const char* runtime_archive_path,
    const char* extra_link_flags,
    char** error_out);
```

`runtime_archive_path` is resolved once by the Rust `--nyrt` caller and is
passed as an exact archive path. The selected route does not temporarily set
or rediscover `NYASH_EMIT_EXE_NYRT`; the unversioned four-argument symbol is
compatibility-only. This ABI fixes link-input authority; it does not publish an
executable or issue runtime addresses.

### `include/hako_dynamic_v2_artifact_descriptor_v1.h`

Selected Dynamic Boundary objects and linked executables contain exactly one
fixed `.hako_dynamic_v2_descriptor` section and the global descriptor symbol
named by this header. The layout carries the two canonical CheckedCallOut site
IDs, entry IDs/symbols/arities, contract/profile, ABI/wire revisions, registry
generation, and PlanStamp. It is a physical projection of final candidate
metadata, not another semantic or provider authority.

The Rust W6-D transaction observes the actual object, explicit runtime archive,
and temporary executable; it checks descriptor parity and exact symbol state,
records all three artifact digests, and issues one move-only static link
receipt. Failure removes the temporary candidate and preserves the previous
final path. Final rename and production activation remain exclusively W6-E;
`RuntimeExecutablePlan`, `dlopen`/`dlsym`, provider reselection, fallback, and
retry are not part of this static direct-symbol lane.

## 3. Lifecycle Extension Symbols

Lifecycle-specific handle operations are currently exported from NyRT kernel FFI:

- `nyrt_handle_retain_h(i64) -> i64`
- `nyrt_handle_release_h(i64) -> void`

Implementation reference:
- `crates/nyash_kernel/src/ffi/lifecycle.rs`

Semantic contract reference:
- `docs/development/current/main/phases/phase-29y/10-ABI-SSOT.md`

## 4. Runtime V0 Helper Slice (execution-path-zero)

execution-path-zero cutover では、以下 4 語彙を固定する。

1. `string_len`
2. `array_get_i64`
3. `array_set_i64`
4. `map_size_i64`

Ownership contract:

1. `args borrowed / return owned` を維持する。
2. 失敗は strict/dev で fail-fast とし、silent fallback を許可しない。

Entry lock:

1. `lang/src/runtime/collections/string_core_box.hako` (`string_len`)
2. `lang/src/runtime/collections/array_core_box.hako` (`array_get_i64`, `array_set_i64`)
3. `lang/src/runtime/collections/map_core_box.hako` (`map_size_i64`)

Detailed SSOT:

- `docs/development/current/main/phases/phase-29cc/29cc-216-runtime-v0-abi-slice-lock-ssot.md`
- `docs/development/current/main/design/hako-runtime-c-abi-cutover-order-ssot.md`

## 4.1 Host Surface Lock (Step-1)

Core C ABI host-facing symbols are fixed by category in:
- `docs/reference/abi/nyrt_host_surface_v0.md`

Rule:
1. Host layer only provides bridge/bootstrap/lifecycle primitives.
2. Runtime/plugin semantic policy must stay in `.hako` side.

## 5. Compatibility Policy

1. Keep C ABI signatures stable in v0 lane.
2. Breaking changes require new symbol versioning (`*_v1`, etc.).
3. Never move plugin method semantics into Core C ABI; keep them in TypeBox ABI.

## 6. Non-goals

- Defining TypeBox plugin dispatch wire protocol (belongs to TypeBox ABI v2).
- Defining GC algorithm details (only lifecycle boundary contracts are fixed here).
