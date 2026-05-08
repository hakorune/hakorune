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
| `hako.mem` | allocation facade rows exist under `MemCoreBox`; exact public surface is still substrate-internal |
| `hako.buf` | `len/cap/reserve/grow` facade rows exist under `BufCoreBox` |
| `hako.ptr` | typed pointer/span facade is staged for current raw collection routes |
| verifier | bounds, initialized-range, and ownership gates exist for current raw collection routes |
| `RawArray` | first raw-array path exists for slot load/store/len/append/reserve/grow |
| `RawBuf` | first allocation facade exists over `MemCoreBox` |
| `hako.atomic` | helper-shaped `fence_i64` row exists |
| `hako.tls` | helper-shaped `last_error_text_h` row exists |
| `hako.gc` | helper-shaped `write_barrier_i64` row exists |
| `hako.osvm` | reserve/commit/decommit rows exist; page-size is not the public row yet |

## Reserved Surface

These names are reserved but not fully live as user-facing allocator substrate:

- `usize`, `isize`, fixed-width signed and unsigned integer syntax
- wrapping and checked arithmetic syntax
- raw layout / repr-like structs
- `sizeof`, `offsetof`, explicit alignment
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
