# map-formal-borrowed-mutation-proof

Generic source-shape proof for `R0-DELTA0`.

It tests whether an ordinary static formal can receive a caller-owned
`MapBox`, mutate it in place, and let the caller observe the mutation without:

- returning the `MapBox`;
- assigning the helper result;
- replacing the owner field;
- adding an ownership operation;
- changing MapBox runtime behavior.

The current language has no active borrow/noescape annotation. This app proves
only non-escaping source-shape mutation visibility; it is not a production
borrowed-ABI proof.

The app-local `test.sh` remains dormant until `MAPFIELD-R0-DELTA0` is
registered at G0.

