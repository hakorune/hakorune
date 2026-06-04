# vm-helper-exact-numeric-field-mutation-proof

Purpose
- Proves that receiver field mutation inside a helper method is visible to the
  caller on both VM and pure-first EXE when the mutated field is `usize` and MIR
  attaches an exact numeric dynamic range contract.
- Keeps the fixture independent of allocator policy so VM/MIR correctness does
  not get inferred from a mimalloc optimization row.

Stop line
- No allocator policy.
- No DirectArray or typed-object optimization.
- No performance claim.

Run

```bash
tools/checks/k2_wide_vm_exact_numeric_helper_field_mutation_guard.sh
```
