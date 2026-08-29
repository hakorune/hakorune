# Bare static catalog-disposition fixtures

This is the HMI-independent source and catalog-decision matrix for
`R0-CALLABLE-CATALOG-L0B-G0`.

The checker runs the pure catalog-decision tests and freezes the current
topology counters: one catalog definition/compatibility producer/install, one
disposition consumer, one static-only candidate index, and zero old partial
authorities, result-representation consumers, or GenericLoop users. It does
not build or execute the retired broad VM-reference compatibility route, and
these fixtures are not production target authority.

```bash
bash apps/bare-static-recovery-proof/test.sh
```

Provider-first and caller-first fixtures must produce the same catalog
decision. The ambiguous fixture intentionally places its consumer between two
providers so declaration order cannot turn an ambiguous catalog into a unique
candidate. Instance rows never contaminate the static catalog decision.
