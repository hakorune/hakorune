# Hako Alloc Mimalloc Comparison Realloc/Aligned EXE Proof

Row: `295x-17`

This proof app is the exact-EXE-friendly realloc/aligned evidence entry for
phase-295x. It reuses the same hako_alloc owners as the VM/MIR
realloc/aligned slice, but keeps the executable path narrow: no `ProofCheck`
object, no branch-heavy assertion body, and no winner claim.

Run:

```bash
bash tools/checks/k2_wide_phase295x_realloc_aligned_hako_exe_acceptance_guard.sh
```
