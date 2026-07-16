# current-receiver-declared-field-proof

HMI-independent source observation fixture for `R0-DECLFIELD0-S0`.

The fixture owns one user box with explicit `ArrayBox` and `MapBox` fields.
Each case is selected independently through the first script argument so a
runtime failure in one late-field shape does not hide the remaining source
inventory.

```bash
target/debug/hakorune --backend vm \
  apps/current-receiver-declared-field-proof/main.hako -- --case A1
```

The source matrix covers:

- direct and validation-heavy `ArrayBox` field mutation;
- an explicit local alias of the current receiver;
- rejected validation, repeated mutation, and instance isolation;
- a `MapBox` regression case;
- a typed `ArrayBox` helper comparison control;
- untyped-field and ordinary explicit-parameter controls.

This app only prints observations and
`selection=UNCLASSIFIED-S0`. It does not classify receiver provenance, repair
field types, activate HMI code, or define a borrow/ownership ABI.

The normalized checker belongs to `R0-DECLFIELD0-M0`. The app-local `test.sh`
and `MAPFIELD-R0-DECLFIELD0` manifest row are intentionally deferred until
`R0-DECLFIELD0-G0`.
