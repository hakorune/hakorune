# Bare static recovery proof

This is the HMI-independent source matrix for
`R0-BARE-STATIC-RECOVERY0-P0`.

P0 parses these sources into the disconnected complete declaration catalog and
applies the pure zero/unique/ambiguous decision. It does not connect that
decision to production call resolution.

```bash
bash apps/bare-static-recovery-proof/test.sh
```

The provider-first fixtures intentionally place `m_seed` before `z_use` in
the current sorted lowering order. Caller-first fixtures place `a_use` before
the same `m_seed` target. Both must normalize to the same canonical target in
the P0 decision.
