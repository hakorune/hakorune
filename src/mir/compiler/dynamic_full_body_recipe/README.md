# Dynamic full-body Recipe producer

This directory owns one caller-zero, complete V2 Recipe producer for the
resolver-backed Dynamic Loop source inventory.

Authority flow:

```text
VerifiedDynamicLoopFullBodySourceInventoryV1
  -> consume resolver Loop token exactly once
  -> VerifiedLoopRecipeArtifactV2 source claim
  + retained frame/scope, 6 binding rows, 28 source rows, Completion
  + private role-to-key claims
  -> later atomic source/envelope co-seal
```

The producer does not select a route, inspect a Dynamic envelope, lower MIR,
or infer Fault/Home/Tail semantics. `ch` remains a source local relation to
V10, not a Recipe binding. The outer `return i` remains Callable Tail; only
the inner return is a Recipe Exit.

The resolver Loop capability is transferred, never cloned. Therefore the
candidate retains the remaining source facts as
`DynamicFullLoopRetainedSourceV1`; the artifact owns only the structurally
verified Loop-source path claim. Exact correspondence between source roles and
Recipe keys remains private candidate truth until the next atomic co-seal.
