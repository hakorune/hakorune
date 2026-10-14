# Dynamic operator contract

This module is the sole profile-neutral semantic owner for supported operator
execution domains: opaque Dynamic, checked Integer and checked String.

It issues only complete borrowed envelopes. Callers cannot construct or pair
effect, ordering, suspension, control, input access, Normal result, Fault, or
lifecycle axes independently.

The source/Recipe co-seal later binds exact items and sites to this contract.
This module does not own source traversal, Recipe keys, provider/runtime
dispatch, Home, destination flow, cleanup, CFG/MIR, retry, or fallback.

Checked Greater, GreaterEqual and LessEqual borrow the same logical NormalInteger operand
classes and publish TrivialBool with no result lifecycle. Either successful
Boolean outcome establishes the operand class; Fault grants no refinement.
These complete envelopes do not issue source guards or actual-argument proof.
The GreaterEqual and LessEqual source/physical correspondences are separate required adapters;
its existence here does not expand the borrowed operand verifier.

NormalInteger multiplication produces a fresh NormalInteger without result
lifecycle or operand mutation. Its exact source/guard/call-child and physical
correspondence are separate required consumers; the envelope activates no ABI.

Checked String Add has exact String/String and String/NormalInteger domains.
Normal publishes one independently owned immutable String value with the same
End-exactly-once vocabulary; immutable payload sharing is allowed, operand-token
reuse is not. This is distinct from the opaque Dynamic carrier result.
Print borrows the resulting value. The expression scope owns temporary End on
Normal and Fault; the envelope alone authorizes no String ABI or Home emission.
Source representation proof, complete operand/call evaluation, source/physical
correspondence and independent final cleanup validation remain required.
