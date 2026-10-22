# Dynamic operator contract

This module is the sole profile-neutral semantic owner for supported operator
execution domains: opaque Dynamic, checked Integer, checked String and exact Bool equality.

It issues only complete borrowed envelopes. Callers cannot construct or pair
effect, ordering, suspension, control, input access, Normal result, Fault, or
lifecycle axes independently.

The source/Recipe co-seal later binds exact items and sites to this contract.
This module does not own source traversal, Recipe keys, provider/runtime
dispatch, Home, destination flow, cleanup, CFG/MIR, retry, or fallback.

Checked Equal, NotEqual, Less, Greater, GreaterEqual and LessEqual borrow the same logical NormalInteger operand
classes and publish TrivialBool with no result lifecycle. Either successful
Boolean outcome establishes the operand class; Fault grants no refinement.
These complete envelopes do not issue source guards or actual-argument proof.
Each exact source operator (including Eq versus Ne) and its physical correspondence is a separate required adapter;
its existence here does not expand the borrowed operand verifier.

NormalInteger multiplication produces a fresh NormalInteger without result
lifecycle or operand mutation. Its exact source/guard/call-child and physical
correspondence are separate required consumers; the envelope activates no ABI.

Checked String Add has exact String/String, String/NormalInteger and
String/Dynamic domains. Dynamic requires a retained original tagged source;
`Other` alone does not issue it. Each Normal publishes one independently owned
immutable String value with the same End-exactly-once vocabulary; immutable
payload sharing is allowed, operand-token reuse is not. String/Dynamic follows
the shared per-tag display owner and preserves Null/Void TypeError.
Print borrows the resulting value. The expression scope owns temporary End on
Normal and Fault; the envelope alone authorizes no String ABI or Home emission.
Source representation proof, complete operand/call evaluation, source/physical
correspondence and independent final cleanup validation remain required.

Checked String/String comparisons use the same six comparison families and
complete borrowed Normal/Fault envelope. They preserve exact Unicode scalar
sequence equality and lexicographic order (proper prefix first), without
normalization or locale collation. They produce TrivialBool, own no result,
and grant no NormalInteger refinement. Mixed classes remain outside this view.
Source Text read/residence, short-circuit reach, checked physical comparison,
lifetime and final consumer correspondence are separate required connections;
these envelopes alone do not select a Bool or Loop execution route.

Exact source-qualified Bool/Bool Equal and NotEqual use their own complete
TrivialBool envelope, with no result ownership or Integer refinement. Mixed
Bool/Integer, Bool arithmetic and Bool ordering remain unsupported. The existing
condition Call/Compare writer and independent source/operand correspondence are
still required; this semantic envelope grants no Call or transport permission.
