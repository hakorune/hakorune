---
Status: SSOT
Decision: provisional; callable ExactText two-lane boundary accepted
Date: 2026-08-16
Scope: deeper substrate capability workの前提として、runtime value representation と ABI manifest の正本を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-29ct/README.md
  - docs/development/current/main/design/substrate-capability-ladder-ssot.md
  - docs/development/current/main/design/abi-export-inventory.md
  - docs/development/current/main/design/abi-export-manifest-v0.toml
  - lang/src/vm/boxes/generated/abi_adapter_registry_defaults.hako
  - docs/development/current/main/design/hako-runtime-c-abi-cutover-order-ssot.md
  - crates/nyash_kernel/src/plugin/value_codec/borrowed_handle.rs
  - crates/nyash_kernel/src/plugin/value_codec/mod.rs
  - crates/nyash_kernel/src/plugin/value_codec/decode.rs
  - crates/nyash_kernel/src/plugin/value_codec/encode.rs
  - lang/src/vm/boxes/abi_adapter_registry.hako
  - docs/development/current/main/phases/phase-289x/289x-90-runtime-value-object-design-brief.md
  - docs/development/current/main/investigations/callable-text-formal-physical-signature-d0-2026-08-15.md
  - docs/development/current/main/investigations/callable-text-formal-call-residence-d0-2026-08-15.md
---

# Value Repr And ABI Manifest (SSOT)

## Current Capsule

- **Current decision:** one logical `ExactText` formal remains one
  `BindingRef` and expands to adjacent scalar `u64` lanes `[slot,generation]`;
  an InstanceBoxMethod has one separate leading receiver lane, and a by-value
  16-byte aggregate is not the callable ABI.
- **Current implementation status:** the pair validator and caller-zero atomic
  lease-set/pending-retirement substrate are landed; the package-owned total
  physical-signature mapping and every compiler consumer remain caller-zero.
- **Next ordered task:** close the design-only
  `TEXT-FORMAL-PINNED-RESIDENCE-BACKEND-FRAME-BINDER-D0`: one compile-time
  frame-contract projection must consume the accepted receiver-aware physical
  lane map and the Residence lifetime census, then reach the selected
  ny-llvmc consumer through one strict typed metadata handoff. Runtime frame
  state, raw pointers/lengths, direct backend lowering, lifecycle CFG, session
  wiring, and route admission remain separate later rows.
- **Production stop line:** no raw handle may recapture generation, no lane may
  become a second logical value, and no pointer/length residence may cross the
  callable boundary or detach from its lease set.
- **Retirement finish line:** all admitted Text calls use the same signature
  map, callee-entry lease-set, scoped root residence, and normal-exit finish;
  legacy recapture, per-iteration registry entry, retry, and fallback are zero.

## Goal

- current plugin export surface を手書き alias 群のまま増殖させず、manifest-first に寄せる。
- runtime value の classes と ownership を先に固定して、future `hako.abi` / `hako.value_repr` の土台にする。
- old compact call-shape codes を immediate truth にせず、manifest から生成される compatibility artifact に落とす。

## Four-layer compiler type convergence (parked)

General C-class lowering must not ask a backend to reconstruct one physical
type from unrelated optional facts. Keep four authorities distinct:

```text
semantic type
  -> physical representation
  -> ABI passing class
  -> storage layout
```

The parked `MIR-PHYSICAL-TYPE-INPUT-D0` may select one already-exact scalar
corridor and define a single verified backend input that consumes those four
rows. It must not widen language types, change an ABI, infer layout from a Box
name, or treat raw `MirType::Integer` as signedness/width/alignment authority.
Missing or conflicting rows reject before backend effects. General integer
widths, pointer-sized integers, overflow policy, aggregates, packing, and
address spaces remain separate BoxCount decisions and cannot be smuggled into
this BoxShape convergence row.

## Canonical Runtime Value Classes

current repo の実態に寄せた canonical classes は次。

1. `imm_i64`
   - plain integer immediate
2. `imm_bool`
   - bool semantic value
   - bridge/call surface では `0/1` i64 に lower されても class は区別する
3. `handle_owned`
   - ordinary live handle
   - return ownership を伴う canonical handle
4. `handle_borrowed_string`
   - borrowed string/string-view handle alias
   - fast path のための stable class
5. `boxed_local`
   - codec-local temporary only
   - public ABI manifest には直接出さない

These classes are the canonical runtime/manifest vocabulary.
They do not imply that the current `native_driver` replay subset or the current
pure-first C subset already consumes every class directly.

## Current Mapping Rule

current `value_codec` から読むと、次が事実上の substrate rule だよ。

- `imm_i64`
  - `decode_array_fast_value(...)` は non-positive or integer-like route を immediate として扱う
- `imm_bool`
  - `runtime_i64_from_box_ref(...)` は bool を `0/1` へ encode する
- `handle_borrowed_string`
  - string / string-view is allowed to reuse borrowed handle alias route
- `handle_owned`
  - non-scalar / non-borrowed values are returned as owned handles
- `boxed_local`
  - only substrate helpers may use it as a temporary carrier

## Value Class Lock

current `value_codec` では、次の5 class を正本として固定する。

1. `imm_i64`
   - immediate integer
   - `decode_array_fast_value(...)` の `ImmediateI64`
   - `any_arg_to_index(...)` の integer-like fast path
2. `imm_bool`
   - semantic bool class
   - runtime surface では `0/1` i64 に lower されても class は `imm_bool`
3. `handle_owned`
   - ordinary owned handle
   - non-scalar / non-borrowed values の canonical public carrier
4. `handle_borrowed_string`
   - string/string-view だけに許可された borrowed alias handle class
   - concrete implementation shape is `BorrowedHandleBox`
5. `boxed_local`
   - internal codec carrier only
   - public ABI manifest row には出さない

`value_public` は V0 inventory の便宜上の umbrella であって、manifest の第一表現にはしない。

## Codec Profile Lock

`CodecProfile` は value class の派生 policy として次で固定する。

1. `Generic`
   - generic bridge/default decode
   - borrowed string alias metadata は付けない
2. `ArrayFastBorrowString`
   - array fast path
   - string/string-view は `handle_borrowed_string`
   - bool/int は scalar-prefer
   - non-scalar / non-string positive handle は conservative fallback
3. `ArrayBorrowStringOnly`
   - string/string-view だけを borrowed alias 化
   - それ以外の positive handle は immediate-style fallback
4. `MapKeyBorrowString`
   - map key decode path
   - string/string-view は `handle_borrowed_string`
   - bool/int は scalar-prefer
   - array profile の名前に依存せず、map-key policy として同じ scalar-prefer contract を読む
5. `MapValueBorrowString`
   - map value storage path
   - string/string-view は `handle_borrowed_string`
   - non-string positive handles keep object semantics instead of scalar-prefer fallback

`CodecProfile` は public ABI row には直接出さず、representation/decode helper の補助契約として扱う。

Future cleanup:

- `CodecProfile` should eventually split into internal demand vocabulary such as:
  - `ValueDemand`
  - `StorageDemand`
  - `PublishDemand`
- That split is tracked by `phase-289x` as successor planning/taskboard work.
- phase-289x also owns the docs-only mapping from current profile names to
  demand verbs such as read-ref, encode-immediate, encode-alias, cell-residence,
  stable-object, and generic-degrade.
- The cleanup is demand-vocabulary refactoring only:
  - it must not add a new public manifest class
  - it must not add a new public manifest row field
  - it must not change the stable public handle ABI truth owned by this document
- phase-137x remains the active implementation lane for proving any runtime-private
  string/result carrier that the cleanup later names more cleanly.

## Borrowed String Alias Invariants

`handle_borrowed_string` の invariants は次で固定する。

1. concrete boundary/cache carrier is `BorrowedHandleBox`
2. `source_handle > 0` でない alias は borrowed-handle fast path を使わない
3. `source_drop_epoch == handles::drop_epoch()` の間だけ source handle を再利用してよい
4. alias expiry 時は fail-open に source handle を信用せず、owned-handle re-materialize へ退避する
5. `inner` を non-string / non-string-view source へ retarget してはいけない
6. `try_retarget_borrowed_string_slot*` は `BorrowedHandleBox` でない slot を黙って変えない
7. borrowed alias production は `StringBox` / `StringViewBox` 系だけに許可する

Design lock:

- `BorrowedHandleBox` protects borrowed-alias encode and cached stable-handle reuse.
- It is not `TextRef` and must not become the semantic value-world carrier.
- Future value-world string reads should name `TextRef` / `AliasRef` explicitly,
  then use `BorrowedHandleBox` only at object/handle boundaries.

ここでは `fail-fast` と `conservative fallback` を分けて読む。

- fail-fast:
  - ownership mismatch
  - invalid retarget target
- conservative fallback:
  - expired borrowed alias
  - missing source handle
  - non-string positive handle

## ABI Manifest Row

future manifest row の最小 truth は次で固定する。

- `box_type`
- `method`
- `symbol`
- `args`
- `ret`
- `arg_ownership`
- `ret_ownership`
- `failure_contract`
- `compat_status`

example reading:

- `ArrayBox.get`
  - symbol: `nyash.array.slot_load_hi`
  - args: `handle_owned, imm_i64`
  - ret: `imm_i64`
  - arg_ownership: borrowed
  - ret_ownership: none
  - compat_status: active mainline

legacy `h` / `hh` / `hi` / `hii` 表記は compatibility artifact として残してよいが、SSOT の第一表現にはしない。

## Ownership Rule

- args are borrowed by default
- return is owned by default unless explicitly `none`
- borrowed string handle is a first-class manifest class
- ownership mismatch must fail-fast; no silent fallback

## Callable Text Formal Wire (V1)

The callable boundary for one logical `ExactText` formal is two adjacent
scalar lanes:

```text
logical ExactText / one BindingRef
  -> slot:u64
  -> generation:u64
```

The pair totals 16 bytes in the current fixed-width wire, but it is not one
by-value ABI aggregate. Logical `/N` remains explicit source-parameter arity.
The complete callable signature keeps four counts distinct:

```text
source_logical_arity    = explicit source formal count (/N)
receiver_lane_count     = 1 iff InstanceBoxMethod, otherwise 0
physical_formal_lane_count
  = sum(explicit-formal lane widths)
physical_callable_lane_count
  = receiver_lane_count + physical_formal_lane_count
```

Physical order is an optional leading `InstanceReceiver` lane followed by
explicit formals in logical ordinal order; ordinary formals use one lane and
ExactText formals use adjacent `[slot,generation]`. The receiver is the exact
source `Receiver` binding, not a formal ordinal, and cannot be inferred from a
function/parameter length difference. The sole physical-signature issuer
consumes the same-brand selected/batch identity, declaration mode, exact
receiver binding when present, and the complete explicit parameter-contract
cohort. It does not consume callable header, result, or Completion and contains
no `ValueId`, pointer, length, lease token, or route policy.

The lane map is also the only source for later physical parameter and call
argument projections. Within one callee function, physical parameter
`ValueId`s are pairwise distinct by lane. Within one caller function,
occurrence order is preserved but aliasing occurrences may repeat a
`ValueId`; caller and callee `ValueId` numbers belong to different scopes and
are never compared as identity.

### MIR carrier for the `u64` wire lanes

The callable wire remains explicitly `u64` for both `slot` and `generation`.
The current MIR/LLVM callable carrier is `i64`; the physical-signature row
records this checked bit-preserving carrier as `U64BitsOnI64`. This is a
mechanical transport fact, not a semantic `MirType::Integer` decision, a
source-level `u64` type, or a reason to add a new unsigned MIR type. The lane
role and signature row remain the sole authority for the unsigned-wire
meaning. In particular, the generation lane is not an ordinary arithmetic
value and may not be reconstructed from the `i64` carrier, a raw slot, a
`MirType`, a JSON length, or a parameter-name string.

Any consumer that cannot preserve the `U64BitsOnI64` contract must reject the
physical signature before effect. A spelling such as `"u64"` must not be fed
through source-type mapping (which would create an unrelated boxed type), and
the existing scalar `i64` carrier must not be exposed as a second semantic
authority.

At callee entry, the Rust runtime owns exact pair validation, Text-class
validation, generation identity, atomic invocation-wide pinning, pending
retirement, and move-only finish. One pin is held per admitted ExactText formal
occurrence; equal pairs in two formals count twice and each nested callee entry
acquires its own set. All pairs preflight before any pin or body effect.
Zero/out-of-range, missing, stale-generation, non-Text, retiring, and overflow
reject without fallback/retry.

The current `TextFormalBorrowV1` and `hako_text_formal_validate_v1` are
caller-zero validator/probe surfaces. Production entry consumes the already
published `[slot,generation]` lanes and must not recapture generation from a
raw handle. `Raw HostHandle`, `ObjectIdentity`, `HomeDemand::Handle`,
`MirType::String`, DynamicV2 lease tokens, `StringSpan`, `StringViewBox`, and
`TextReadSession` are not signature or residence issuers.

Function-internal residence is a later, separate product:

```text
atomic lease-set
  -> non-splittable TextFormalCallResidenceSetV1
       lease token + immutable UTF-8 root descriptors
  -> session-branded TextSliceRef / backend-local TextPlan
  -> scoped backend ptr/len projection
```

The root lifetime never belongs to raw `ptr,len`, and slices/plans never detach
from the residence set or cross the stable callable ABI. TextEq, Substring,
Builder, MIR/CFG/SSA, and production selection remain outside the current
caller-zero runtime substrate.

## Demand Verb Reading

The public ABI manifest owns value classes and ownership.
It does not make helper names the source of semantic legality.

For runtime-private work:

- `get` is a demand verb:
  - read ref
  - encode immediate
  - encode borrowed alias
  - publish stable object
- `set` is a demand verb:
  - store immediate
  - consume owned payload
  - write cell residence
  - degrade to generic object storage
  - invalidate aliases/caches
- `call` is a demand verb:
  - thin internal value entry
  - public handle/object entry

`phase-289x` may refine these internal demand names, but public ABI rows remain
owned by this manifest.

## Internal Direct-Kernel Result Manifest

The public manifest above is not enough for phase-137x.
Birth / Placement already admits backend-private carriers such as
`OwnedBytes`, but the manifest currently has no first-class internal result
vocabulary that lets direct-kernel lanes return unpublished outcome without
pretending it is already a public handle.

Reading slogan:

- value-first means unpublished outcome first
- boxes are publication artifacts, not semantic truth

Lock that split here.

### Public manifest classes

- `imm_i64`
- `imm_bool`
- `handle_owned`
- `handle_borrowed_string`
- `boxed_local`

These remain the public/runtime manifest vocabulary.

### Internal direct-kernel result classes

These are runtime-private only and must not widen the public ABI surface.

1. `PublishedHandle`
   - already in handle/object world
   - cold/public boundary only
2. `OwnedBytes`
   - phase-137x minimal unpublished text carrier
   - owned payload, not yet `StableBoxNow`, not yet `FreshRegistryHandle`
3. `SourceKeepToken`
   - future opaque token for source-preserving lanes
   - not part of the phase-137x minimal keeper

Reading rule:

- Birth / Placement may choose `MaterializeOwned` without choosing
  `StableBoxNow`
- internal direct-kernel ABI may carry `OwnedBytes`
- public ABI still speaks only in public manifest classes

Do not treat `TextReadSession` itself as a result class.
It is runtime mechanics, not result ABI vocabulary.

### Lifecycle mapping

The runtime-private result classes map back to the lifecycle vocabulary in
`lifecycle-typed-value-language-ssot.md`.

| Current/internal class | Lifecycle reading | Public ABI status |
| --- | --- | --- |
| `TextReadSession` | `Ref` mechanics | not a result class |
| `OwnedBytes` | `Owned` unpublished payload | internal only |
| `KernelTextSlot` | `Cell` residence | internal only |
| `BorrowedHandleBox` | borrowed alias / stable-cache bridge | public class remains `handle_borrowed_string` |
| `PublishedHandle` | `Stable` / handle world | public handle carrier |
| `boxed_local` | codec-local temporary | never a manifest row |

## Public ABI vs Internal Direct-Kernel ABI

Keep the public handle-based surface stable.
Do not add a new public ABI class on the phase-137x lane.

The split is:

- public ABI / facade
  - existing handle-based exports remain the stable surface
- internal direct-kernel ABI
  - may carry runtime-private internal result classes
  - phase-137x minimal target is `OwnedBytes`

Preferred transport shape for internal direct-kernel results is a caller-owned
slot plus a tag, not secret reuse of the public `i64 -> handle` return shape.

Illustrative shape:

```c
struct KernelResultSlot {
    uint32_t tag;
    uint64_t a0;
    uint64_t a1;
    uint64_t a2;
    uint64_t a3;
};
```

Reading:

- `PublishedHandle`
  - `a0 = handle`
- `OwnedBytes`
  - `a0/a1/a2 = ptr/len/cap`
- `SourceKeepToken`
  - opaque token payload if/when that lane is proven

This SSOT owns the ABI split itself.
Lane-specific legality for when a corridor may stay unpublished belongs to the
string corridor SSOT, not here.

### Successor planning boundary

`phase-289x` may reorganize internal/runtime-private demand vocabulary around
this manifest, but public ABI truth stays here.

Reading rule:

- this file owns public value classes, manifest rows, and the ABI split
- `phase-289x` may clean up demand names such as `ValueDemand`,
  `StorageDemand`, and `PublishDemand`
- `phase-289x` must not widen public ABI truth while doing that cleanup
- runtime-wide implementation must wait until phase-137x reaches keeper/reject
  on the active read-side lane

### Phase-137x Minimal Slot Shape

Phase-137x does not need a fully generic slot family yet.
The minimal direct-kernel-local carrier may stay string-first as long as it
remains runtime-private.

Illustrative minimal shape:

```c
struct KernelTextSlot {
    uint8_t state;   // empty | owned_bytes | published
    void*   ptr;
    size_t  len;
    size_t  cap;
};
```

State-machine reading:

- `empty -> owned_bytes`
  - corridor-local executor freezes newly materialized text into the caller-owned slot
- `owned_bytes -> published`
  - cold publish adapter converts the slot payload into `StringBox -> Arc -> handle`
- `published -> empty`
  - slot owner clears or reuses the slot; the slot itself is never the registry carrier

Ownership rule:

- the slot is caller-owned
- overwrite must clear the prior `owned_bytes` payload first
- drop/early-return paths must also clear the prior `owned_bytes` payload first
- runtime-private slot payload must not escape into the public manifest
- registry remains publication/storage for `PublishedHandle` only

Phase-137x guard:

- do not treat the local slot seam as proof that loop-carried slot transport is
  already solved
- the first landing may keep `KernelTextSlot` corridor-local
- same-corridor transport is a separate follow-on card

## Fixed Order

### V0. Inventory current symbols

- `nyash.array.*`
- `nyash.map.*`
- `nyash.runtime_data.*`
- `nyash.string.*`
- docs-side inventory truth is [`abi-export-inventory.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/abi-export-inventory.md)
- [`AbiAdapterRegistryBox`](/home/tomoaki/git/hakorune-selfhost/lang/src/vm/boxes/abi_adapter_registry.hako) is a runtime consumer of that inventory, not the manifest truth

current hand-written exports are allowed only as inventory input.

### V1. Freeze value classes

- `imm_i64`
- `imm_bool`
- `handle_owned`
- `handle_borrowed_string`
- `boxed_local`
- `value_public` is umbrella-only, not the first manifest vocabulary
- `BorrowedHandleBox` is the current concrete shape for `handle_borrowed_string`
- `CodecProfile` is fixed as helper policy, not a public ABI row field
- current boundary exports may still remain handle/i64-shaped while this
  manifest rolls out; that temporary transport shape does not override the
  canonical value-class truth above

### V2. Freeze manifest schema

- row shape above becomes the only truth
- current adapter-default slice is materialized in
  `docs/development/current/main/design/abi-export-manifest-v0.toml`
- `AbiAdapterRegistryBox` defaults are generated from that manifest slice
  - generated Hako defaults live in
    `lang/src/vm/boxes/generated/abi_adapter_registry_defaults.hako`

### V3. Generate compatibility shims

- `array.rs`
- `map.rs`
- `runtime_data.rs`

these should shrink to thin exports over substrate helpers and generated alias data.

### V4. Shrink manual codec glue

- `value_codec/*` remains substrate owner
- but public ABI semantics must be readable from the manifest, not inferred from scattered exports

## Immediate Task Pack

1. manifest inventory doc for current array/map/runtime_data/string symbols
   - [`abi-export-inventory.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/abi-export-inventory.md)
2. value class / borrowed alias lock in `value_codec/*`
   - `decode.rs`
   - `encode.rs`
   - `borrowed_handle.rs`
3. `AbiAdapterRegistryBox` default rows mapped into manifest vocabulary
4. `array.rs` / `map.rs` / `runtime_data.rs` export groups tagged as:
   - mainline substrate
   - compat-only
   - pure/historical compat
5. `handle_cache.rs` metal helper contract lock

## Non-Goals

- broad ABI redesign in one wave
- changing current borrowed/owned return contract
- removing all compact export names immediately
- turning `boxed_local` into a public ABI type
- turning internal direct-kernel result classes into public ABI classes on the
  phase-137x lane
- using `TextReadSession` itself as a result ABI payload
- using the host-handle registry as the unpublished carrier
