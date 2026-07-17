# Bare static recovery proof

This is the HMI-independent source and production matrix for
`R0-CALLABLE-CATALOG-L0B-CUT0`.

The checker keeps the disconnected pure decision tests, builds debug and
release VM-reference binaries, executes the pass/reject matrix, and verifies
that every accepted source emits its canonical target exactly once.

```bash
bash apps/bare-static-recovery-proof/test.sh
```

Provider-first and caller-first fixtures must compile to the same target. The
ambiguous fixture intentionally places its consumer between two providers so
even a forced dev tail resolver cannot turn the already-ambiguous catalog into
a lowering-order-dependent unique match. Qualified calls retain their earlier
route, and instance rows never contaminate static recovery.
