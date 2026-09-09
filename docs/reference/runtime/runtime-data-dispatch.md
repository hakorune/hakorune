# RuntimeData Dispatch Contract

Status: accepted  
Applies to: LLVM/AOT `RuntimeDataBox` method lowering

## Overview

`RuntimeDataBox` method calls in LLVM/AOT are routed to kernel exports:

- `nyash.runtime_data.get_hh(recv_h, key_any) -> i64`
- `nyash.runtime_data.set_hhh(recv_h, key_any, val_any) -> i64`
- `nyash.runtime_data.has_hh(recv_h, key_any) -> i64`
- `nyash.runtime_data.push_hh(recv_h, val_any) -> i64`

These symbols are implemented in `crates/nyash_kernel/src/plugin/runtime_data.rs`.

Array receiver が型確定している callsite では、array-specialized route
（AS-03）を使う場合がある:

- `push -> nyash.array.slot_append_hh(recv_h, val_any) -> i64`
- `get -> nyash.runtime_data.get_hh(recv_h, key_any) -> i64`
- `set -> nyash.runtime_data.set_hhh(recv_h, key_any, val_any) -> i64`
- `has -> nyash.runtime_data.has_hh(recv_h, key_any) -> i64`

さらに key が i64 と判定できる callsite では、
array-specialized route を整数キー版へ縮退できる（AS-03b）:

- `nyash.array.slot_load_hi(recv_h, idx_i64) -> i64`
- `nyash.array.slot_store_hih(recv_h, idx_i64, val_any) -> i64`
- `nyash.array.slot_store_hii(recv_h, idx_i64, val_i64) -> i64`（AS-03c）
- `nyash.runtime_data.has_hh(recv_h, idx_i64) -> i64`

`push` は integer-key 縮退を持たず、AS-03 でも `nyash.array.slot_append_hh`
を使う。`has` はこの wave では常に `nyash.runtime_data.has_hh` を使う。

これらは array/runtime_data plugin surface で実装され、
`runtime_data` の ArrayBox 契約と同一の戻り値意味を持つ。

## Receiver Dispatch

`recv_h` runtime type decides behavior:

- `ArrayBox`
  - `runtime_data.get_hh`: index read
  - `runtime_data.set_hhh`: set/append (`idx == len` append)
  - `runtime_data.has_hh`: bounds check (`0/1`)
  - `runtime_data.push_hh`: append
- `MapBox`
  - `runtime_data.get_hh`: key lookup (missing key returns `0`)
  - `runtime_data.set_hhh`: key set
  - `runtime_data.has_hh`: key existence check (`0/1`, missing key returns `0`)
  - `runtime_data.push_hh`: unsupported (`0`)
- other types: fail-fast return `0`

### Array index semantics (contract)

For `ArrayBox` receiver:

- negative index:
  - `runtime_data.get_hh` returns `0`
  - `runtime_data.set_hhh` returns `0`
  - `runtime_data.has_hh` returns `0`
- index in range:
  - `runtime_data.get_hh` returns element value (mixed i64/handle contract below)
  - `runtime_data.set_hhh` returns `1`
  - `runtime_data.has_hh` returns `1`
- index == len:
  - `runtime_data.set_hhh` appends and returns `1`
  - `runtime_data.has_hh` returns `0`
- index > len:
  - `runtime_data.set_hhh` returns `0`
  - `runtime_data.has_hh` returns `0`

Key decode contract for `runtime_data.get_hh/set_hhh/has_hh` on `ArrayBox`:

- `key_any <= 0`: treat as immediate index directly
- `key_any > 0`:
  - if `key_any` is a live handle to `IntegerBox`, use that integer value
  - if `key_any` is a live handle to `StringBox` and parseable as `i64`, use parsed value
  - otherwise, treat `key_any` itself as immediate index

This keeps positive immediate indices stable even when unrelated positive handles are live.

## Return Contract

`get_hh` return is intentionally mixed to match RuntimeData semantics:

- `IntegerBox` result: raw integer `i64`
- `BoolBox` result: raw `0/1`
- other boxed values (`StringBox`, `ArrayBox`, `MapBox`, etc.): host handle `i64`
- failure/not found/unsupported: `0`

`set_hhh` returns:
- `1` when mutation succeeds
- `0` on invalid receiver/invalid index/unsupported path

`has_hh` returns:
- `1` true
- `0` false or invalid receiver/path

`push_hh` returns:
- new array length for `ArrayBox`
- `0` otherwise

Related ABI note (legacy, non-runtime_data route):

- `nyash.array.set_h` and `nyash.map.set_{h,hh}` keep legacy completion code `0`
  (apply/no-op is not encoded in return code).

## Lowering Rule (SSOT)

For MIR method calls where `box_name == "RuntimeDataBox"` and method in `{get,push,set,has}`,
LLVM lowerers must use shared dispatch helper (`runtime_data_dispatch.py`) and follow:

- default: `nyash.runtime_data.*`
- AS-03 成立時（ArrayBox receiver + arity/key条件）:
  `push -> nyash.array.slot_append_hh`, `get/set/has -> nyash.runtime_data.*`
- AS-03b 成立時（AS-03 + key VID が i64/integerish）:
  `get -> slot_load_hi`, `set -> set_hih`, `has -> nyash.runtime_data.has_hh`
- AS-03c 成立時（AS-03b + value VID も i64/integerish）:
  `set -> set_hii`

Implemented in:
- `src/llvm_py/instructions/mir_call/method_call.py`
- `src/llvm_py/instructions/mir_call_legacy.py`
- shared helper: `src/llvm_py/instructions/mir_call/runtime_data_dispatch.py`

## E2E Contract Fixture

The LLVM/AOT route is pinned with a prebuilt MIR fixture that forces
`RuntimeDataBox` dispatch for `push/get/has/set`:

- fixture: `apps/tests/phase29x_runtime_data_dispatch_e2e_min_v1.mir.json`
- smoke: `tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh`

Smoke checks:
- IR contains the pinned active symbols for each method:
  - `push -> nyash.array.slot_append_hh`
  - `has -> nyash.runtime_data.has_hh`
  - `get -> nyash.runtime_data.get_hh` または `nyash.array.slot_load_hi`
  - `set -> nyash.runtime_data.set_hhh` または `nyash.array.slot_store_hih/slot_store_hii`
- compiled executable returns `rc=4`

### Primitive Array state-write outcomes

The native Array state owner retains structured outcomes for i64, Bool and F64
stores and appends before the existing raw compatibility projection. Invalid index,
unsupported storage and element-contract rejection are distinct internal errors;
contract failures retain the existing runtime-type-mismatch,
negative-to-unsigned or out-of-range reason. Returned validation failures do not
change storage or length. Validation and mutation use the same Array state lock,
so shared aliases observe the installed contract. Primitive append also determines
the end position and returns the committed length under this lock; concurrent
appends through shared aliases neither overwrite one another nor return duplicate
positions. Indexed store and append reuse the same primitive mutation owner.
This guarantee covers the raw integer append caller and the i64/Bool/F64 arms
of generic kernel append; boxed/string and alternate storage routes are separate.

Existing `slot_store_*_raw` booleans and kernel sentinel results keep their
compatibility meaning. This internal Result boundary introduces no C ABI or
source acceptance. It does not promise allocation-failure recovery; typed Script
C execution remains stopped pending its final input and physical consumer.

### Checked native Array ABI v1

The `nyash.array.checked_*` exports consume the existing Array state operations,
with caller-owned FaultFrame storage. They do not determine source annotations,
select an alternate storage backend or decode generic integer/handle carriers.

| Export (under `nyash.array.`) | Arguments after frame pointer | Result |
| --- | --- | --- |
| `checked_new_v1` | u64 site, i64* out | New native Array handle written only on Normal |
| `checked_claim_v1` | u64 site, i64 handle, u32 tag | Install/adopt the supplied element contract |
| `checked_append_i64_v1` | u64 site, i64 handle, i64 value | Integer append; even a value equal to a handle stays integer |
| `checked_append_bool_v1` | u64 site, i64 handle, u32 value | Exactly 0/1 Bool append |
| `checked_append_f64_v1` | u64 site, i64 handle, double value | F64 append with bit-preserving payload transport |

All return u32 Normal=0/Fault=1/InvalidContract=2. Explicit element tags are
`1=i8, 2=i16, 3=i32, 4=i64, 5=u8, 6=u16, 7=u32`; source enum discriminants
are not wire values. Null/malformed frame, null allocation out, unknown tag,
Bool outside0/1, invalid/non-Array handle and primitive append UnsupportedStorage
are InvalidContract, without recording a source Fault. A live nonnull frame
must be aligned, exclusively borrowed and initialized until disposal; output
storage must be writable and nonoverlapping. Header checks cannot validate
arbitrary foreign pointers.

Returned claim/write rejections leave Array storage, length and installed
contract unchanged. Success remains Normal when the frame already retains a
Fault. Only returned Fault records diagnostics; existing first/suppressed/overflow
behavior and final-owner reporting/disposal apply.

| Diagnostic reason | details[0] | details[1] |
| --- | --- | --- |
| ARRAY_CLAIM_CONFLICT=200 | requested wire tag | 0 |
| ARRAY_EXISTING_ELEMENT_MISMATCH=201 | checked i64 existing index | subtype |
| ARRAY_APPEND_ELEMENT_MISMATCH=202 | subtype | 0 |

Subtypes are `1=runtime-type-mismatch, 2=negative-to-unsigned, 3=out-of-range`.
Unknown internal reasons or an unrepresentable index are InvalidContract before
recording, without a generic fallback diagnostic. Claim's noninteger-storage
adoption error, including InlineRecord, remains ExistingElementMismatch
(index0,type mismatch) and maps to Fault201/[0,1]. This differs from primitive
append's UnsupportedStorage capability error; no storage precheck/reclassification
is performed by the kernel.

New constructs ArrayBox/Arc and publishes through the existing host registry.
An unrepresentable/nonpositive internal handle is withdrawn before out publication
and yields InvalidContract. Fatal allocator termination and OS kill are outside
returned-Fault recovery under the policy below. Native residence cleanup uses
the existing void `nyrt_handle_release_h` exactly once; there is no new release
status or Array-specific registry. These exports alone do not activate the
selected Script C backend.

### Lifecycle physical input and runtime session binding

The host invocation owns one issued physical input and borrows one selected
runtime session. Binding checks the selected target/Fault/status layout and
physical requirements before temporary JSON or output creation. The C wrapper
serializes that input once, supplies its session's target row and releases the
temporary input after success or error. The EXE linker receives the archive
from the same bound input; independent JSON/session Rust arguments are retired.

NativeArray requirements additionally demand exactly one defined external
function for each of the five versioned checked Array entries above and
`nyrt_handle_release_h`. The runtime archive inspection owner reads an explicit
`nm` symbol inventory; absent, undefined-only, local/data and ambiguous definitions
are rejected. A name prefix or string in object data is not availability proof.
This check establishes physical ABI availability, not source admission or
implementation correctness. Typed-object requirements preserve their own
profile/layout contract and do not demand Array symbols. The existing runtime
and entry descriptors remain authoritative; no additional Array descriptor is
introduced. Retained Script inputs use this same binding for I64 and Unit roots;
their native requirement carries ABI version1, seven explicit claim tags and
distinct i64/Bool/f64 append representations. Float transport preserves u64 bits.
No object layout/profile is synthesized. Issued cleanup/Normal/Fault coordinates
remain physical input, not runtime inference. Input issuance and symbol-backed
binding do not activate selected Script host execution. The C V4 consumer now
lowers native checked calls directly with the declared i64/i32/double lanes,
uses the supplied Normal/Fault targets and releases supplied residences before
report/dispose. Root Unit uses no value lane and returns0 after successful
dispose; root I64 keeps the existing0..255/range-Fault policy. C validates live
identity/claim/operand consistency without selecting source cleanup. The actual
Rust OBJ/EXE production switch remains a separate acceptance obligation.

### Selected native Array failure policy

Decision: accepted by the user on 2026-09-08. Keep stable Rust and the existing
std Arc, Array state and host registry. Distinguish returned operation failures
from fatal host termination; allocator-OOM-to-Fault recovery is not a requirement
of the selected Array runtime contract.

A returned checked contract/resource failure follows the source-issued Fault
successor, preserves the first Fault and executes the prescribed cleanup.
Acquired native resources remain owned until successful wrapping/registration
and publication; any returned failure before that transfer must retain cleanup
responsibility. A successful output is published only on Normal. Malformed ABI
inputs remain explicit InvalidContract, never zero-success or compatibility retry.

Rust allocator fatal failure and OS kill are separate process outcomes. They provide no guarantee of language cleanup, final report,
disposal or exit70, and must not be counted as successful Fault propagation.
This policy does not permit converting a returned failure into an abort to avoid
cleanup, or turning an abort into a fabricated Fault result.

Each checked entry must document which failures it returns and which underlying
allocations retain host-fatal behavior. Test returned failures and their cleanup
through real operation paths; do not require a destructive host-OOM test as a
language-Fault witness. Stronger allocator recovery needs a separate Decision.
This policy alone does not activate the selected Array C consumer.

## Map replacement native teardown

The shared `MapBox::insert_key_str` commits the new value and detaches any old
value while holding the Map write lock, then drops the old native Box after
unlocking. Teardown may re-enter and observe the committed replacement. A
missing-key insert drops no displaced value. Key equality and insertion return
behavior are unchanged.

Decision: `remove_key_str` and `clear_entries` use the same lock boundary:
detach under lock, then drop after unlock. Remove retains its presence Bool;
clear retains table capacity and commits an empty table before any child Drop.
Reentrant mutation after that commit is a later operation and is not drained
again by the outer clear. Missing removal and empty clear drop no values.

This closes native Drop under the shared insertion/removal/clear locks. It does
not activate source Home transfer, user finalization, ordered Map end, or certify
read/clone. Those remain governed by the
[Map lifecycle target](../language/lifecycle.md#intrinsic-map-construction-and-end).

## Native JSON observation

`JSONBox::set` returns `Result<Box<dyn NyashBox>, JsonSetError>`. Success
retains the StringBox `ok`; finite errors are `SourceMapUnavailable`,
`DestinationUnavailable` and `DestinationNotObject`. This changes the public
Rust signature; source-language dispatch and external client compatibility
are not established by this native observer boundary.

The setter converts the key, borrows native Array/Map children into an
independent JSON Value, disposes its consumed input, then locks and validates
the destination object before committing. Nested Map failure propagates before
destination validation; failure performs no direct destination mutation.
Observation and input Drop run outside the destination lock. Arbitrary input
callback side effects are not rolled back.

Serialization invokes no child `clone_box` or `share_box`. Scalar/key and
fallback string conversion retain their behavior. Map owns scoped borrowed
iteration; its raw-table `get_data` accessor is removed. GC uses the same access
with its distinct native child-clone semantics. This is not source Home
acquisition or a checked projection for indexed owned Map slots. Collection
traversal locking, cycles, native panics and unsupported owned residences remain
separate concerns.

## Checked Map storage and indexed residence

The checked intrinsic facade is not NyashBox and cannot be cloned or put in native
host handles. It and native MapBox use the common MapTable implementation, with
one payload table per Map. Native MapBox remains permanently Native-only.
The initial checked runtime residence is an authorized SafeMutex indexed object;
this implementation does not add source candidate families or Dynamic carriers.

Checked storage starts Unissued; acquisition changes it to Live once. Install
accepts an already prepared canonical key and validates phase/order/capacity
before membership changes. Failure returns the same candidate for prior-owner
cleanup. Successful install gives the new entry the latest order and returns
its detached predecessor without ending it. No-old is also a consumed outcome.

Terminal end marks Ending, moves all entries out, and releases the lock before
end callbacks. End-buffer capacity is prepared at install, so sorting/draining
needs no end-time allocation. End attempts run in reverse live-install order;
failures retain first plus eight suppressed facts and a total suppressed count.
The FaultFrame consumer bounds indexing by stored capacity. Returned
failures do not skip later entries, and the Map reaches Ended on either result.
Poisoned storage rejects normal operations; terminal end recovers the valid
storage and records unavailability while attempting cleanup. Native panic or
allocator abort recovery is not supplied by this returned-error contract.

The kernel residence validates SafeMutex/profile, exact indexed identity and type
without taking the payload. Preparation does not issue a Home or transfer it.
Dropping a rejected wrapper does not reclaim the prior owner's payload; consuming
end uses the existing exact indexed reclaim. Profile mismatch, storage failure
and invalid identity are finite physical error facts. No kernel dependency enters
root, and no registration callback or second payload registry is introduced.

A present owned entry has no authorized native Box projection and returns
ProjectionUnavailable; a missing key returns None. Native keys/clone/iteration
semantics remain unchanged. CheckedMap and its detached outcomes require explicit
end by the placement caller. Map destruction is legal only Unissued/Ended;
opaque detached storage rejects disposal before consumption. Rust Drop is
not a fallback source finalizer. Opaque ABI and descriptor/session are implemented.
Selected C Map emission uses the existing V4 physical input and exact-origin
storage validation. Mixed-origin finishing and physical-consumer execution do
not by themselves establish source-to-EXE activation; that cutover remains open.

## Checked Map opaque ABI contract

The runtime ABI uses three caller-owned opaque regions: Map, prepared key
and detached outcome. All belong to one invocation's physical storage owner;
none is a source Home token or native host handle. The target archive issues all
three size/alignment/contract-revision triples in the same descriptor revision.
Keep the existing Fault ABI/status values: Normal0, Fault1, InvalidContract2.

The compiler C consumer tracks exact physical storage liveness and consumption.
Successful indexed allocation does not itself establish source Home completion
or NoBirth. Source-required constructor calls and destruction permission remain
the published Rust caller's responsibility; neither absent calls nor empty
layouts establish those facts at the C boundary. This does not waive C input
checks for roles, identities, SSA, transfer, cleanup or temporary consumption.

| Entry family | State / result contract |
| --- | --- |
| Map storage init | fresh unique aligned bytes -> Unissued native bookkeeping |
| checked Map new | validate frame and SafeMutex profile; Normal -> Live; Fault stays Unissued |
| key init | fresh native storage -> Empty |
| key prepare UTF-8 | bytes plus length; Empty -> Ready native MapKeyDomain before child evaluation |
| checked indexed install | validate frame/Map/profile/Ready key/Unissued outcome and nonoverlap before consumption; move key, prepare indexed residence and install |
| checked value install | validate kind and Bool bits before key consumption, then the same install state machine; I64/Bool occupy inline payloads, never indexed identities |
| detached end | move Ready payload and mark Consumed before child end; ReadyNoOld also consumes |
| Map end | require Live; consume through Ending to Ended with first-Fault/best-effort cleanup |
| key dispose | cancel Ready natively, or release Empty/Consumed bookkeeping |
| outcome dispose | only Unissued/Consumed; Ready is rejected unchanged |
| Map storage dispose | only Unissued/Ended; Live/Ending is rejected unchanged |

Implemented export spellings are `nyash.map.storage_init_v1`,
`nyash.map.checked_new_v1`, `nyash.map.key_init_v1`,
`nyash.map.key_prepare_utf8_v1`, `nyash.map.key_dispose_v1`,
`nyash.map.outcome_init_v1`, `nyash.map.checked_install_indexed_v1`,
`nyash.map.checked_install_value_v1`,
`nyash.map.outcome_end_v1`, `nyash.map.outcome_dispose_v1`,
`nyash.map.checked_end_v1` and `nyash.map.storage_dispose_v1`.

The value entry uses `(frame, profile:u32, site:u64, map, key, kind:u32,
payload:i64, outcome)->u32`; pointer regions keep the same opaque contract.
`NYRT_MAP_VALUE_I64=1` accepts the full signed i64 range;
`NYRT_MAP_VALUE_BOOL=2` accepts only0/1. Other kinds/bits return InvalidContract
without consuming Key or publishing Outcome. No Handle/String/Float inference.
Storage retains I64/Bool inline alongside Residence in one non-Clone payload
sum; rejection returns the exact payload, and detached/end attempts invoke real
Home end only for Residence. Native projection of present Values still refuses.
The value export does not by itself activate source or C consumer coverage.

Key preparation consumes exact UTF-8 bytes, including embedded NUL. Preserve
canonical i64 versus noncanonical numeric text through the existing MapKeyDomain
owner. No strlen, String-handle/cache lookup or post-child conversion is allowed.
A null bytes pointer is allowed at zero length (empty key); other regions must
be non-null. Returned allocation failure records reason100 and leaves Empty; malformed UTF-8
or pointer/length contract returns InvalidContract without installing a key.
Allocator fatal termination is not represented as a returned source Fault.
Child Fault cancels the Ready key with key dispose; this is native cleanup and
must not synthesize a source String Home or finalizer.

Install preflight contract rejection leaves Map/key/outcome unchanged. It checks
profile/lifecycle and storage contracts before moving the Ready key. After that
move the key is Consumed on both Normal and returned Fault. Exact indexed
residence preparation is then part of the attempt: unavailable/dead identity or
returned storage failure records the existing physical diagnostic (reason100 for
storage/capacity, reason101 for identity), never transfers the candidate and
leaves the outcome Unissued. Root install failure likewise drops only the native
key/wrapper, leaving the prior source owner responsible for its indexed payload.
Normal transfers to Map and publishes ReadyNoOld/ReadyOwned before returning.
A new candidate wrapper is prepared before commit; no allocation, hook or fallible
operation is allowed between membership commit and outcome publication.

Unknown/profile mismatch, invalid lifecycle, malformed/null/overlapping region
arguments and incorrect key/outcome state are preflight InvalidContract. Order
capacity exhaustion is a returned capacity Fault after key consumption. These
status rules do not weaken the unsafe valid-pointer contract. No ordinary Box
or missing-value zero is a checked Map projection.

Frame validation uses a short shared borrow. Do not retain mutable Map/outcome/
FaultFrame borrows or locks across any callback-capable end. Consume the detached
outcome first, run end, then reborrow the frame and record the returned physical
failure. Map end records its finite report only after callbacks. Previously
recorded primary Fault remains primary. Merge the actual stored diagnostics and
mark omission for additional failures without inventing a diagnostic or indexing
beyond eight slots. End consumes its obligation even when it reports a Fault;
there is no retry or rollback after committed install.

Init requires fresh unique storage and may not read uninitialized bytes to detect
prior use. Compiler lifetime validation must reject live reinitialization/copy;
this is not a promise that runtime init detects arbitrary misuse. Prepare rejects
Ready overwrite, end rejects Unissued/double consumption, and dispose rejects
live obligations at runtime. Disposed storage has no live Rust value; a second
call needs a new valid initialized lifetime, not a fabricated header.

Descriptor/transport uses the 236-byte V2 record/ELF section/export, carries all
three opaque layout triples through the Rust C row and C session revision2, and
rejects older revisions before extended-field reads. Required symbols, driver
initialization and launcher symbol assertion are updated. Selected C allocation,
opaque projections and mixed cleanup consume these layouts; actual callable
source cases reach direct EXE and linked OBJ. No unused region is allocated
just to claim consumption of a layout field. Existing native birth_h
compatibility is separate. Common install selects only proved source readiness;
unsupported source and Local annotations retain pre-effect rejection. Selected
source execution is not a claim of whole-Map compatibility retirement.
