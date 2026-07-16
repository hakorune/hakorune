# map-field-owner-boxshape-proof

Generic compiler BoxShape fixture for `R0-STOP0`.

It isolates one user-box-owned `MapBox` across:

- local dynamic-key control;
- literal field access;
- callee-side formal-to-String concatenation;
- caller-built String key parameters;
- same-method and same-receiver helper observation;
- one and two fallthrough control merges;
- receiver alias observation;
- independent owner instances.

This app is deliberately domain-neutral. It does not import application
modules, return the raw storage map, create a result MapBox/ArrayBox, change
MapBox runtime semantics, or provide a storage workaround.

All cases have the language-level expectation `1`. During `R0-STOP0` the
observed `0|1` rows are diagnostic evidence; the later evidence checker owns
the exclusive `KEY0` / `RECV0` / stop classification.

The app-local `test.sh` is dormant until `MAPFIELD-R0-STOP0` is registered in
the proof manifest at `R0-STOP0-G0`.
