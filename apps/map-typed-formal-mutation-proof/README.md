# map-typed-formal-mutation-proof

Generic two-file observation fixture for `R0-TYPE0`.

The imported command declares `storage: MapBox`. The fixture checks that local
and field-held maps can be mutated in place through that typed static formal,
including validation/early-return control flow before a late field read.

This app proves only the source/runtime observation matrix. It does not classify
the compiler result, define a borrow ABI, or add ownership syntax.

The source law is:

- the caller retains the local or field owner;
- `put_proven` returns no `MapBox`;
- callers never bind the mutator result;
- storage fields are assigned only during `birth`;
- no `share`, `move`, or `clone` form is used.

The app-local `test.sh` remains dormant until `MAPFIELD-R0-TYPE0` is registered
in the existing MapFieldOwner proof manifest at `R0-TYPE0-G0`.
