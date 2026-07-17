# PHI Type Publication

This Builder-private module owns one pure lowering-time destination-type
decision for completed PHIs.

Decision boundary:

- input is the logical incoming-value list plus the current transient type map;
- missing and `Unknown` are non-facts; every other `MirType` is exact;
- existing destination and explicit hint facts constrain but do not create a
  candidate;
- concrete conflicts are deterministic typed failures;
- only a prepared `Publish` decision mutates the supplied type map;
- no PHI insertion, CFG validation, rematerialization, origin propagation,
  receiver proof, field lookup, final metadata, runtime, or backend authority
  lives here.

TYPE-PUBLISH0-I0 connects exactly four Builder completion entries: raw emit,
complete final insertion, provisional patch, and atomic batch insertion. Each
entry prepares from logical inputs before mutation and commits only after its
own PHI mutation succeeds. Provisional define and function-level PHI APIs stay
non-consumers. Raw origin publication remains a separate success-committed
owner; lifecycle entries never publish origin facts.
