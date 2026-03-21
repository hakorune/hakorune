# RC / GC Alignment

This directory hosts the first semantic split out of `integration/apps/`.
It keeps the RC/GC alignment family together so daily discovery can stop
reading those pins as generic app-level noise.

Contained scripts:

- `rc_gc_alignment_g1_lifecycle_parity_vm_llvm.sh`
- `rc_gc_alignment_g2_fast_milestone_gate.sh`
- `rc_gc_alignment_g3_cycle_timing_gate.sh`
- `rc_gc_alignment_g5_mode_invariance_vm_llvm.sh`

Contract:

- keep these pins under the `integration` profile only
- keep them live and executable by `run.sh`
- update the corresponding `rc-gc-alignment-*` SSOT docs when paths move again
