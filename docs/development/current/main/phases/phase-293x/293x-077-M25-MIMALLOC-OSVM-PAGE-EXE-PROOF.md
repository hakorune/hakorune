---
Status: done
Date: 2026-05-10
Scope: M25 mimalloc OSVM page EXE proof
---

# 293x-077 M25 Mimalloc OSVM Page EXE Proof

## Decision

`M25 mimalloc OSVM page EXE proof` makes the first allocator-shaped OS virtual
memory page sequence runnable in pure-first EXE:

```hako
local base = OsVmCoreBox.reserve_bytes_i64(4096)
local rc1 = OsVmCoreBox.commit_bytes_i64(base, 4096)
local rc2 = OsVmCoreBox.decommit_bytes_i64(base, 4096)
```

The source uses the public `hako.osvm` facade. MIR owns the extern route facts
for reserve/commit/decommit, and pure-first consumes those facts. The backend
does not match this fixture by app name and does not invent OSVM semantics.

## Owned

- `apps/mimalloc-osvm-page-proof/`
- MIR extern route rows for:
  - `hako_osvm_reserve_bytes_i64/1`
  - `hako_osvm_commit_bytes_i64/2`
  - `hako_osvm_decommit_bytes_i64/2`
- Pure-first declaration/emit table rows for the same MIR-owned route ids.
- NyRT runtime exports for the same reserve/commit/decommit ABI symbols.
- A guard that checks MIR route metadata, pure-first route trace, and stable
  EXE output (`reserved=1`, `commit=0`, `decommit=0`).

## Not Owned

- `hako_osvm_page_size_i64/0` pure-first lowering.
- OS VM release/unreserve API.
- TLS, atomics, remote-free, native pointer strong attrs, or allocator
  ownership proof.
- Platform-specific virtual memory policy beyond the existing C kernel helper.

## Acceptance

```bash
bash tools/checks/k2_wide_mimalloc_osvm_page_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Result: green on 2026-05-10.

## Next Reading

After M25, the allocator ladder has static tables, inline size selection,
raw-page free-list operations, and OSVM reserve/commit/decommit EXE proof.
Remaining substrate rows should split TLS cache slot and atomic remote-free
primitives before attempting a broader allocator fast path.
