# Runtime Substrate Capabilities Manual

Status: provisional reference

This manual is the user-facing/reference entry for low-level substrate
capabilities used by future allocator, collection, and runtime internals.

The design SSOT is:

- `docs/development/current/main/design/substrate-capability-ladder-ssot.md`
- `docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md`

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
| verifier | bounds, initialized-range, and ownership gates exist for current raw collection routes; RawArray remove/insert are verifier-gated before pointer-substrate calls |
| `RawArray` | first raw-array path exists for slot load/store/len/append/reserve/grow |
| `RawBuf` | first allocation facade exists over `MemCoreBox` |
| `hako.atomic` | helper-shaped `fence_i64` row exists |
| `hako.tls` | helper-shaped `last_error_text_h` row exists |
| `hako.gc` | helper-shaped `write_barrier_i64` row exists |
| `hako.osvm` | reserve/commit/decommit rows exist; page-size is not the public row yet |

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
- atomic CAS/fetch operations with memory order
- language-level TLS cells
- `@rune Contract(no_alloc)` backend-active use
- `@rune Contract(no_safepoint)` backend-active use
- `clz`, `ctz`, `popcnt`, `prefetch`, `assume`, `unreachable`
- `noalias`, `nonnull`, `dereferenceable`, stronger alignment export
- const-evaluated static tables

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
