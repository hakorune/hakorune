# ABI Index (Current SSOT)

Updated: 2026-02-13

This is the entry point for ABI documents used in current development.

## 1. Decision (fixed)

Canonical ABI surfaces are only:

1. Core C ABI (NyRT runtime boundary)
2. TypeBox ABI v2 (plugin Box method boundary)

`hako_abi_v1` is a design-first draft, not a canonical production ABI.

## 2. Canonical Documents

### Core C ABI (NyRT)

- `docs/reference/abi/nyrt_c_abi_v0.md`
- `include/nyrt.h`
- `include/nyrt_host_api.h`
- `docs/development/current/main/phases/phase-29y/10-ABI-SSOT.md`

### TypeBox ABI v2

- `docs/reference/plugin-abi/nyash_abi_v2.md`
- `include/nyash_abi.h`

### Boundary ownership map

- `docs/reference/abi/ABI_BOUNDARY_MATRIX.md`

## 3. Non-canonical / Historical

- `dist/0.1.0-linux-x86_64/include/hako_abi_v1.h`
  - Status: design-only draft from Phase 20.6 line.
  - Policy: no new production symbol additions.
- `docs/archive/phases/phase-12/`
  - Long-term blueprint and historical design discussions.

## 4. Maintainer Rules

1. Runtime lifecycle/hostcall changes must be defined in Core C ABI docs first.
2. Plugin method call changes must be defined in TypeBox ABI v2 docs first.
3. Do not introduce a third semantic ABI lane.
4. If migration facade is needed, keep it thin and generated from the two canonical ABIs.

## 5. Reference Notes

- `docs/reference/abi/PLUGIN_ABI.md` is a snapshot for LLVM-era shims and legacy by-id details.
- `docs/reference/abi/NYASH_ABI_MIN_CORE.md` is a long-term evolution sketch, not the immediate runtime contract.
