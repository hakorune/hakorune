# PHI Type Publication

This Builder-private module owns one pure lowering-time destination-type
decision for completed PHIs.

S0 boundary:

- input is the logical incoming-value list plus the current transient type map;
- missing and `Unknown` are non-facts; every other `MirType` is exact;
- existing destination and explicit hint facts constrain but do not create a
  candidate;
- concrete conflicts are deterministic typed failures;
- only a prepared `Publish` decision mutates the supplied type map;
- no PHI insertion, CFG validation, rematerialization, origin propagation,
  receiver proof, field lookup, final metadata, runtime, or backend authority
  lives here.

The module has zero production consumers through TYPE-PUBLISH0-S0. The later
I0 row may connect exactly the four decision-locked Builder PHI completion
entries without moving their mutation authority into this folder.
