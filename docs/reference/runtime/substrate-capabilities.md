# Runtime Substrate Capabilities Manual

Status: provisional reference

This manual is the user-facing/reference entry for low-level substrate
capabilities used by future allocator, collection, and runtime internals.

The design SSOT is:

- `docs/development/current/main/design/substrate-capability-ladder-ssot.md`
- `docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md`
- `docs/development/current/main/design/inline-plan-ssot.md`
- `docs/development/current/main/design/rune-profile-effect-capability-plan-ssot.md`

## Core Rule

Hakorune does not add a broad C-style `unsafe` language surface for allocator
work.

Low-level operations are exposed through explicit capability modules:

- `hako.mem`
- `hako.buf`
- `hako.ptr`
- `hako.atomic`
- `hako.tls`
- `hako.gc`
- `hako.osvm`
- `hako.intrin`

Optimization and safety obligations are expressed with `@rune Contract(...)`
and must be verified before a backend may use them.

## Current Live Surface

The current live surface is intentionally narrow.

| Capability | Current live reading |
| --- | --- |
| numeric substrate | fixed-width and pointer-sized type names are classified by MIR metadata for storage planning; runtime values still use the current `Integer(i64)` lane; current `>>` is signed i64 arithmetic shift |
| raw layout | MIR-owned `repr_c_v0` vocabulary can plan fixed-width numeric field offsets/size; no source syntax or backend-active native allocation yet |
| `hako.mem` | allocation facade rows exist under `MemCoreBox`; exact public surface is still substrate-internal |
| `hako.buf` | `len/cap/reserve/grow` facade rows exist under `BufCoreBox`; capacity routes through `PtrCoreBox.slot_cap_i64` |
| `hako.ptr` | typed pointer/span facade is staged for current raw collection routes and owns direct array-slot backend route names for the live row |
| verifier | bounds, initialized-range, ownership, and rune contract gates exist for current rows; RawArray remove/insert are verifier-gated before pointer-substrate calls; `Contract(no_alloc/no_safepoint)` is MIR-verifier checked |
| `RawArray` | first raw-array path exists for slot load/store/len/cap/append/reserve/grow |
| `RawBuf` | first allocation facade exists over `MemCoreBox` |
| `hako.atomic` | helper-shaped `fence_i64`, memory-order vocabulary, and `fence_order_i64(order)` rows exist; generic load/store/CAS/fetch_add are not live |
| `hako.tls` | helper-shaped diagnostics TLS rows exist: `last_error_text_h`, `last_error_is_ok_i64`, and `last_error_code_i64`; generic thread/task-local slots are not live |
| `hako.gc` | helper-shaped `write_barrier_i64` row exists |
| `hako.osvm` | page-size plus reserve/commit/decommit rows exist |
| `hako.intrin` | current-lane non-negative i64 bit-count rows exist: `clz_i64`, `ctz_i64`, `popcnt_i64`; backend optimization use is not live |
| backend export attrs | consistency guard is live; only current weak attrs are allowed, runtime-decl `readonly` rows must carry `memory = "read"`, while `noalias`/`nonnull`/`dereferenceable`/alignment export remain blocked |
| static readonly data | backend-private static-data manifest can emit a u16 size-class fixture; source `static const NAME: u16[] = [...]` declarations lower to MIR `static_data_plans`; `NAME[index]` reads lower to MIR `StaticDataLoad` and current-lane `i64` values; narrow integer const expressions in u16 table initializers are live |
| inline planning | `@rune Hint(inline/noinline/hot/cold)` and substrate-only `@rune Lowering(inline_required)` preserve MIR InlinePlan metadata; `Hint(inline)` has a narrow best-effort same-module MIR leaf inline row; required inline verifier/backend use is not live |
| profile/effect/capability planning | `@rune Profile(...)`, EffectPlan, and CapabilityPlan are design-reserved only; Profile is future sugar over MIR facts and is not backend-readable |

## Reserved Surface

These names are reserved but not fully live as user-facing allocator substrate:

- exact-width integer runtime semantics beyond the current `Integer(i64)` lane
- numeric literal suffixes such as `1u64` / `64usize`
- wrapping and checked arithmetic syntax
- logical right-shift surface distinct from current `>>`
- raw layout source syntax / repr-like structs
- backend-active `sizeof`, `offsetof`, explicit alignment
- pointer-sized, pointer, handle, or Box fields inside raw layouts
- `MaybeInit`
- unrestricted raw pointer arithmetic
- atomic load/store/CAS/fetch_add operations with memory order
- language-level TLS cells
- raw numeric TLS slot APIs
- TLS cache-slot primitives
- `@rune Contract(no_alloc)` backend optimization/export use
- `@rune Contract(no_safepoint)` backend optimization/export use
- `prefetch`, `assume`, `unreachable`
- full unsigned-width runtime semantics for intrinsic rows
- `noalias`, `nonnull`, `dereferenceable`, stronger alignment export
- const fn table generation and references to other const declarations
- backend-active required inline from `@rune Lowering(inline_required)`
- verifier-backed required inline acceptance
- `@rune Profile(...)` parser acceptance and expansion
- backend-readable profile names
- MIR EffectPlan / CapabilityPlan backend use
- unrestricted `unsafe(...)` blocks

## Pointer/Handle Return Proof Vocabulary

Decision: M10c-pre vocabulary is live as a proof/ownership vocabulary lock.
It is still not an LLVM attr export row.

Locked return classes:

- `imm_i64`
- `handle_existing_borrowed`
- `handle_existing_owned_ref`
- `handle_fresh_owned`
- `native_ptr_nonnull`
- `native_ptr_nullable`
- `native_ptr_dereferenceable(len, align)`

Locked proof vocabulary:

- `fresh`
- `nonnull`
- `dereferenceable_bytes`
- `alignment`
- `noalias_scope`
- `no_refcount_mutation`
- `no_registry_write`

Current rule:

- `handle_*` return classes are runtime values, not LLVM pointer attr targets.
- `native_ptr_*` return classes are the only future target for LLVM pointer
  attrs such as `nonnull`, `dereferenceable`, alignment, and `noalias`.
- M10c strong attrs remain blocked until verifier-owned proof and exporter
  gates consume this vocabulary.

## Atomic Ordered Fence Row

Owner module:

- `lang/src/runtime/substrate/atomic/atomic_core_box.hako`

Current memory-order vocabulary:

| Method | Value |
| --- | --- |
| `order_relaxed_i64()` | `0` |
| `order_acquire_i64()` | `1` |
| `order_release_i64()` | `2` |
| `order_acq_rel_i64()` | `3` |
| `order_seq_cst_i64()` | `4` |

Live operations:

- `is_valid_order_i64(order)` returns `1` for the five vocabulary values and
  `0` otherwise.
- `fence_i64()` remains the compatibility fence row over
  `hako_barrier_touch_i64(0)`.
- `fence_order_i64(order)` validates the order vocabulary and routes to
  `hako_barrier_touch_i64(order)`.

Unsupported operations:

- `load`
- `store`
- `CAS`
- `fetch_add`
- `pause/yield`

VM-hako subset behavior:

- `boxcall(AtomicCoreBox.fence_order_i64)` is accepted with one register
  argument.
- invalid order values fail-fast with
  `[vm-hako/contract][boxcall-fence_order_i64-invalid-order]`.

## TLS Diagnostics Row

Owner module:

- `lang/src/runtime/substrate/tls/tls_core_box.hako`

Live operations:

- `last_error_text_h()` reads diagnostics TLS through `hako_last_error` and
  returns a string handle.
- `last_error_is_ok_i64()` returns `1` when the diagnostics TLS value is
  `OK`, otherwise `0`.
- `last_error_code_i64()` maps the current diagnostics TLS text to a narrow
  code:

| Text | Code |
| --- | --- |
| `OK` | `0` |
| `OOM` | `1` |
| `VALIDATION` | `2` |
| `UNSUPPORTED` | `3` |
| `NOT_FOUND` | `4` |
| other | `-1` |

Unsupported operations:

- generic thread/task-local slots
- `TlsCell<T>`
- cache-slot primitives
- allocator-local cache policy

VM-hako subset behavior:

- `boxcall(TlsCoreBox.last_error_text_h)` is accepted with no arguments.
- `boxcall(TlsCoreBox.last_error_is_ok_i64)` is accepted with no arguments.
- `boxcall(TlsCoreBox.last_error_code_i64)` is accepted with no arguments.
- accidental arguments are rejected by the subset checker.

## OS VM Page Row

Owner module:

- `lang/src/runtime/substrate/osvm/osvm_core_box.hako`

Live operations:

- `page_size_i64()` returns the current platform page size through
  `hako_osvm_page_size_i64`.
- `reserve_bytes_i64(len_bytes)` reserves address space.
- `commit_bytes_i64(base, len_bytes)` commits a reserved range.
- `decommit_bytes_i64(base, len_bytes)` decommits a committed range.

VM-hako subset behavior:

- `boxcall(OsVmCoreBox.page_size_i64)` is accepted with no arguments.
- `externcall(hako_osvm_page_size_i64/0)` is accepted with no arguments.
- VM-hako returns deterministic `4096` for page size.

## Intrin Bit-Count Row

Owner module:

- `lang/src/runtime/substrate/intrin/intrin_core_box.hako`

Live operations:

- `clz_i64(value)` returns the count of leading zero bits for a current-lane
  non-negative `i64` value.
- `ctz_i64(value)` returns the count of trailing zero bits for a current-lane
  non-negative `i64` value.
- `popcnt_i64(value)` returns the number of set bits for a current-lane
  non-negative `i64` value.
- `clz_i64(0)` and `ctz_i64(0)` return `64`.

Unsupported operations:

- negative current-lane values
- full `u64` runtime values
- `prefetch`
- `assume`
- `unreachable`
- backend optimization/export use

VM-hako subset behavior:

- `boxcall(IntrinCoreBox.clz_i64)` is accepted with one register argument.
- `boxcall(IntrinCoreBox.ctz_i64)` is accepted with one register argument.
- `boxcall(IntrinCoreBox.popcnt_i64)` is accepted with one register argument.
- `externcall(hako_intrin_*_i64/1)` is accepted for the three live rows.
- unknown `hako_intrin_*` extern rows are rejected.

Native behavior:

- `hako_intrin_clz_i64`, `hako_intrin_ctz_i64`, and
  `hako_intrin_popcnt_i64` validate current-lane non-negative `i64` values.
- negative values set diagnostics TLS to `VALIDATION` and return `-1`.
- `clz(0)` and `ctz(0)` return `64`.

## LLVM Export Attrs Consistency Gate Row

Decision: accepted for the M10a guard only. This row does not widen backend
facts.

Current live export attrs:

- `llvm_py` runtime-helper policy may emit function `readonly`.
- `llvm_py` runtime-helper policy may emit pointer-argument `nocapture`.
- `.hako` runtime-decl manifest rows may emit only `nounwind`, `readonly`,
  and `willreturn`.

Owner modules:

- `src/llvm_py/instructions/llvm_attrs.py`
- `docs/development/current/main/design/runtime-decl-manifest-v0.toml`
- `lang/src/shared/backend/ll_emit/generated/runtime_decl_defaults.hako`
- `tools/checks/k2_wide_export_attrs_consistency_guard.sh`

Unsupported behavior:

- `noalias`
- `nonnull`
- `dereferenceable`
- backend alignment export
- stronger `nocapture`
- `readnone`

Safety/verifier contract:

- Strong attrs remain blocked until MIR contract facts and boundary export
  facts have a verifier-owned consistency proof.
- Backends must not infer strong attrs from helper names, app names, method
  names, or manifest spelling alone.

Fixture/gate:

- `bash tools/checks/k2_wide_export_attrs_consistency_guard.sh`

## Runtime-Decl Readonly Fact Guard Row

Decision: accepted for the M10b runtime-decl weak-attr verifier.

New rule:

- A runtime-decl manifest row with `readonly` must declare `memory = "read"`.
- Rows with `memory = "read"` are not required to add `readonly`; conservative
  omission remains allowed.

Owner modules:

- `docs/development/current/main/design/runtime-decl-manifest-v0.toml`
- `tools/checks/k2_wide_export_attrs_consistency_guard.sh`

Unsupported behavior:

- This row does not prove helper semantics from implementation bodies.
- This row does not make strong attrs backend-active.
- This row does not infer attrs from symbol names.

Safety/verifier contract:

- Manifest facts and emitted attrs must not contradict each other.
- If a future row wants `readonly` on a `memory = "readwrite"` helper, the
  manifest fact must be fixed first or the guard fails.

Fixture/gate:

- `bash tools/checks/k2_wide_export_attrs_consistency_guard.sh`

## Static Readonly Data Segment Row

Decision: accepted for M11a backend-private static readonly data only.

New surface:

- `docs/development/current/main/design/static-data-manifest-v0.toml` owns
  backend-private static data rows.
- `lang/src/shared/backend/ll_emit/generated/static_data_defaults.hako` is
  generated from the manifest.
- `StaticDataRegistryBox` emits LLVM readonly global data rows for the
  `.hako ll emitter`.

Live row:

- `.hako_size_class_u16_v0`
  - element: `u16`
  - count: `64`
  - align: `2`
  - purpose: mimalloc size-class fixture

Unsupported behavior:

- No source-level `static const` syntax.
- No `const fn` or const eval.
- No runtime `ArrayBox` / `MapBox` table construction in the target program.
- No semantic size-class policy owner is implied by this manifest row.

Safety/verifier contract:

- The static-data manifest is backend-private declare/data truth.
- The `.hako ll emitter` reads manifest rows and emits data; it does not
  rediscover table meaning.
- Public ABI semantics remain separate from this manifest.

Fixture/gate:

- `bash tools/checks/k2_wide_static_data_first_row_guard.sh`
- `bash tools/checks/k2_wide_static_const_table_decl_guard.sh`

## Static Const Table Source Row

Decision: M11b-decl is live for the first narrow source declaration shape. The
design SSOT is:

- `docs/development/current/main/design/static-const-table-syntax-ssot.md`

M11b is split into:

- `M11b-decl`: source declaration to MIR `static_data_plans`
- `M11b-load`: read route from static data
- `M11b-eval`: narrow integer const expressions; const fn remains future

Live first source shape:

```hako
static const SIZE_CLASS: u16[] = [
  8 + 8, 3 * 8, 1 << 5, (40 - 8) | 1,
]
```

Implemented M11b-decl flow:

```text
source static const
-> AST/Program metadata
-> MIR module metadata static_data_plans
-> backend data row reader
-> LLVM readonly global
```

Implemented M11b-eval surface:

- expressions are evaluated before MIR `static_data_plans` are produced
- accepted operators are integer-only: `+`, `-`, `*`, `/`, `%`, `<<`, `>>`,
  `&`, `|`, `^`, unary `-`, and parentheses
- final values must fit `0..65535`
- bitwise and shift operands must be non-negative in this narrow row

Current unsupported behavior:

- Const fn is not accepted yet.
- References to other const declarations are not accepted yet.
- Function calls, method calls, variable reads, allocation, and runtime code
  execution are not accepted during const evaluation.
- Source static const tables must not be lowered into runtime `ArrayBox` /
  `MapBox` construction.

Safety/verifier contract:

- `M11b-decl` must handle Rust parser and `.hako` parser fronts explicitly.
- MIR-owned `static_data_plans` must be the truth after parsing.
- Backends must consume static data rows; they must not infer allocator table
  meaning from symbol names or app names.
- Unsupported element types, initializers, and out-of-range values must fail
  fast at the declaration boundary.

## Manual Update Rule

Every implementation row that widens this surface must update this manual in
the same commit.

The update must state:

- new surface
- owner module
- accepted backends
- unsupported backend fail-fast behavior
- safety/verifier contract
- fixture or smoke gate

If a new row adds syntax, update these manuals too:

- `docs/reference/language/EBNF.md`
- `docs/reference/language/types.md`

If a new row changes ABI or exported backend facts, update these manuals too:

- `docs/reference/abi/ABI_INDEX.md`
- `docs/reference/abi/ABI_BOUNDARY_MATRIX.md`
- `docs/reference/mir/metadata-facts-ssot.md`

## Contract Verification

`@rune Contract(...)` is only backend-active after verifier proof.

The current live verifier row proves a narrow subset only:

- `Contract(no_alloc)` rejects MIR instructions whose effect mask contains
  `Alloc`.
- `Contract(no_safepoint)` rejects explicit MIR `Safepoint` instructions.
- Distinct `Contract(...)` values may appear together on one declaration;
  exact duplicate contract values are rejected by the parser.
- The proven facts are not exported to backend optimization yet.

Required flow:

```text
source rune
-> parser metadata
-> MIR-owned contract fact
-> verifier proof
-> backend export
```

Backends must not trust method names, app names, or helper names as contract
proof.

## Inline Planning

Decision: M11c-preserve is live for advisory `Hint(...)` preservation into MIR
`inline_plans`, M11c-soft-leaf is live for best-effort same-module MIR leaf
inline, and M11c-required-vocab is live for preserving substrate-only
`Lowering(inline_required)` as MIR `request=required` metadata. Required
inline acceptance remains a future verifier-backed row.

Inline is required for allocator-grade fast paths, but it is not a backend
keyword and not a `.inc` responsibility.

Required future flow:

```text
@rune Hint(inline/noinline/hot/cold)
-> MIR InlinePlan / CallsiteInlinePlan
-> M11c-preserve metadata row
-> current M11c-soft-leaf best-effort same-module leaf inline where accepted
-> required-vocab metadata where Lowering(inline_required) is present
-> future verifier where required
-> MIR transform or intrinsic route
-> backend emits the result
```

Strict allocator/substrate rows may later reserve:

```hako
@rune Lowering(inline_required)
@rune Contract(no_alloc)
@rune Contract(no_safepoint)
```

`Lowering(inline_required)` is live vocabulary now and requires Rust parser /
`.hako` parser parity because it widens rune metadata. It is preserved with
`verified=false`, so backends must not infer required inline from this row or
from symbol names.

## Rune Profile / Plan Ordering

Decision: `@rune Profile(...)` is reserved as a future authoring shortcut. It is
not live syntax and is not a backend contract.

The required future flow is:

```text
@rune Profile(...)
-> primitive rune metadata and MIR Plan facts
-> EffectPlan / CapabilityPlan / InlinePlan verifier acceptance
-> MIR transform or capability route
-> backend emits the result
```

Reserved profile names:

- `allocator.fast`
- `allocator.slow`
- `substrate.leaf`
- `intrinsic.leaf`
- `raw.layout`

Task order:

1. verifier-backed required inline acceptance
2. EffectPlan / CapabilityPlan boundary
3. mimalloc raw-page proof using explicit facts
4. Profile registry docs
5. Profile expansion to primitive facts
6. allocator fast-path EXE proof

Backends must not branch on profile names. `.inc` / ll_emit must continue to
read already-expanded MIR facts and routes only.

## Allocator Reading

`mimalloc-lite` may remain a policy/state model at the Box level.

Mimalloc-grade native fast paths require:

- numeric substrate
- raw layout substrate
- memory/pointer/buffer capabilities
- verifier-backed ownership and initialization facts
- `no_alloc` / `no_safepoint` proof
- TLS and atomics
- OS VM capability
- intrinsic and backend export facts
- InlinePlan rows for hot helper expansion without backend-local inliners

Until those rows are live, allocator code must remain narrow and fail-fast
instead of silently relying on unsupported substrate behavior.

## Numeric Substrate Row

Decision: accepted for the M0 type-name/storage lock and current shift
semantics lock.

New surface:

- `i8`, `i16`, `i32`, `i64`, `isize`
- `u8`, `u16`, `u32`, `u64`, `usize`

Owner module:

- MIR: `src/mir/numeric_substrate.rs`

Accepted backends:

- VM: current dynamic `Integer(i64)` behavior only.
- MIR/VM/LLVM: current `>>` is signed i64 arithmetic right shift.
- EXE: typed-object slots may use inline i64 storage for these declared type
  names.

Unsupported backend behavior:

- Any consumer that requires exact width, unsigned range, literal suffixes,
  wrapping arithmetic, checked arithmetic, or logical right shift must fail
  fast or stay reserved. It must not silently infer those semantics from the
  type name or from current `>>`.

Safety/verifier contract:

- This row is metadata/storage only. It provides no range verifier and no
  overflow verifier.

Fixture/gate:

- `cargo test -q numeric_substrate --lib`
- `cargo test -q typed_object_plan --lib`
- `cargo test -q mir_numeric_shift_semantics --lib`
- `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_binop_numeric_tail.py`

## Raw Layout Substrate Row

Decision: accepted for the M1 MIR vocabulary lock only.

New surface:

- MIR `repr_c_v0` raw-layout plan vocabulary.
- fixed-width numeric field storage:
  - `i8`, `i16`, `i32`, `i64`
  - `u8`, `u16`, `u32`, `u64`

Owner module:

- MIR: `src/mir/raw_layout.rs`

Accepted consumers:

- MIR tests and future metadata producers may build a `repr_c_v0` plan for the
  fixed-width numeric field set.

Unsupported backend behavior:

- Source syntax such as `struct`, `@rune Repr(C)`, `@rune Align(N)`, `sizeof`,
  and `offsetof` is not live.
- Pointer-sized fields (`usize` / `isize`), pointer fields, handle fields, and
  semantic Box fields must fail fast or stay reserved.
- Backends must not compute raw layout from semantic `box` declarations or
  application-specific field names.

Safety/verifier contract:

- This row is layout vocabulary only. It provides no pointer dereference,
  lifetime, bounds, ownership, or no-safepoint proof.

Fixture/gate:

- `cargo test -q raw_layout --lib`

## Memory/Buffer/Pointer Capability Row

Decision: accepted for the M2 buffer-capacity route ownership lock.

New surface:

- `PtrCoreBox.slot_cap_i64(handle)`
- `BufCoreBox.cap_i64(handle)` delegates to `PtrCoreBox.slot_cap_i64(handle)`.

Owner modules:

- `lang/src/runtime/substrate/ptr/ptr_core_box.hako`
- `lang/src/runtime/substrate/buf/buf_core_box.hako`

Accepted consumers:

- VM adapter and current raw collection routes may observe array-backed capacity
  through `BufCoreBox.cap_i64`.
- The direct `nyash.array.slot_cap_h` backend symbol is owned by `PtrCoreBox`
  for this live row.

Unsupported behavior:

- `hako.buf` must not own direct backend ABI symbol names.
- This row does not add unrestricted raw pointer arithmetic, `shrink`,
  `set_len`, native raw allocation policy, or user-facing pointer syntax.

Safety/verifier contract:

- This row is route ownership cleanup only. It adds no new lifetime, aliasing,
  or bounds proof.

Fixture/gate:

- `bash tools/checks/phase29cc_runtime_v0_abi_slice_guard.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_runtime_v0_adapter_fixtures_vm.sh`

## RawArray Capacity Observer Row

Decision: accepted for the M3 RawArray capacity-shape lock.

New surface:

- `RawArrayCoreBox.slot_cap_i64(handle)`

Owner modules:

- `lang/src/runtime/substrate/raw_array/raw_array_core_box.hako`
- `lang/src/runtime/substrate/buf/buf_core_box.hako`

Accepted consumers:

- RawArray substrate consumers may observe current array-backed capacity through
  `RawArrayCoreBox.slot_cap_i64`.
- The route is readable ownership-gated and then delegates to
  `BufCoreBox.cap_i64`.

Unsupported behavior:

- This row does not expose user-visible `ArrayBox.capacity`.
- This row does not add `set_len`, `shrink`, `RawBuf` length/capacity state,
  `MaybeInit`, or allocator policy.

Safety/verifier contract:

- The observed handle must pass `OwnershipCoreBox.ensure_handle_readable_i64`.
- The row adds no allocation, aliasing, or initialized-range proof.

Fixture/gate:

- `bash tools/checks/phase29cc_runtime_v0_abi_slice_guard.sh`
- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_runtime_v0_adapter_fixtures_vm.sh`

## Minimum Verifier Hardening Row

Decision: accepted for the M4 RawArray remove/insert hardening slice.

New surface:

- `BoundsCoreBox.ensure_insert_index_i64(handle, idx)`
- RawArray remove route uses ownership + bounds + initialized-range before
  calling `PtrCoreBox.slot_remove_any`.
- RawArray insert route uses ownership + insert-bounds + value ownership before
  calling `PtrCoreBox.slot_insert_any`.

Owner modules:

- `lang/src/runtime/substrate/verifier/bounds/bounds_core_box.hako`
- `lang/src/runtime/substrate/verifier/initialized_range/initialized_range_core_box.hako`
- `lang/src/runtime/substrate/verifier/ownership/ownership_core_box.hako`
- `lang/src/runtime/substrate/raw_array/raw_array_core_box.hako`

Unsupported behavior:

- RawArray slice hardening is deferred because visible slice semantics currently
  clamp out-of-range endpoints.
- Double-free / use-after-free detection is not live.
- Borrowed alias expiry is not owned by this verifier row.

Fixture/gate:

- `bash tools/checks/phase29cc_runtime_v0_abi_slice_guard.sh`

## Rune Contract Verifier Row

Decision: accepted for the M5 MIR verifier core only.

New surface:

- `@rune Contract(no_alloc)` has a MIR verifier check.
- `@rune Contract(no_safepoint)` has a MIR verifier check.

Owner modules:

- `src/mir/verification/rune_contracts.rs`
- `src/mir/verification_types.rs`

Accepted consumers:

- `MirVerifier::verify_function` checks declaration-local
  `FunctionMetadata.runes`.

Unsupported behavior:

- Backend optimization/export use is not live.
- `Contract(pure)` and `Contract(readonly)` are still metadata-only.
- This row does not infer contracts from function names, app names, helper
  calls, or backend route choices.

Safety/verifier contract:

- `no_alloc` rejects instructions with `Effect::Alloc`, including `NewBox`,
  `NewClosure`, `FutureNew`, and calls whose effect mask carries `Alloc`.
- `no_safepoint` rejects explicit `MirInstruction::Safepoint`.

Fixture/gate:

- `cargo test -q rune_contracts --lib`
