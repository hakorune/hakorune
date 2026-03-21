# MIR Shape Guard

This directory hosts the `mir_shape_guard` smoke pin split out of `integration/apps/`.
It keeps the MIR shape probe isolated from generic app-level noise.

Contained scripts:

- `mir_shape_guard_vm.sh`

Contract:

- keep this pin under the `integration` profile only
- keep it live and executable by `run.sh`
- keep the family small and evidence-oriented
