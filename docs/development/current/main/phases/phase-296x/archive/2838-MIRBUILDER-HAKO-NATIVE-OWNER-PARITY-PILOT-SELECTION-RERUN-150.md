# 2838 MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-150

Status: Completed
Date: 2026-07-05

## Decision

Select `fastmem_llvm_owner_runtime_memop_kind_classifier` as the
one-hundred-forty-seventh narrow Rust-oracle parity pilot owner.

## Evidence

```text
selected_owner:
  fastmem_llvm_owner_runtime_memop_kind_classifier
source_surface:
  src/mir/contracts/fastmem_ops.rs:86
  is_fastmem_llvm_owner_runtime_memop_kind
```

## Non-Claims

- Source Selfhost remains unclaimed.
- FastMem dialect validation remains Rust.
- Backend lowering and MIR mutation remain Rust.

## Next

`MIRBUILDER-FASTMEM-LLVM-OWNER-RUNTIME-MEMOP-KIND-CLASSIFIER-RUST-ORACLE-FIXTURE-001`
