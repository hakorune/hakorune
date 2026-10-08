# Dynamic operator contract

This module is the sole profile-neutral semantic owner for supported Dynamic
Add/Mul/Less/Greater/LessEqual/Equal execution domains.

It issues only complete borrowed envelopes. Callers cannot construct or pair
effect, ordering, suspension, control, input access, Normal result, Fault, or
lifecycle axes independently.

The source/Recipe co-seal later binds exact items and sites to this contract.
This module does not own source traversal, Recipe keys, provider/runtime
dispatch, Home, destination flow, cleanup, CFG/MIR, retry, or fallback.

Checked Greater and LessEqual borrow the same logical NormalInteger operand
classes and publish TrivialBool with no result lifecycle. Either successful
Boolean outcome establishes the operand class; Fault grants no refinement.
These complete envelopes do not issue source guards or actual-argument proof.
The LessEqual source/physical correspondence is a separate required adapter;
its existence here does not expand the borrowed operand verifier.

NormalInteger multiplication produces a fresh NormalInteger without result
lifecycle or operand mutation. Its exact source/guard/call-child and physical
correspondence are separate required consumers; the envelope activates no ABI.
