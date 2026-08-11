# Dynamic full-body Recipe boundary

This directory owns the bounded source-to-Recipe path for the selected
`ParserScanLoopBox.skip_while/4` cohort.  The production path is deliberately
one-way:

```text
resolver-backed source inventory
  -> A2 callable parameter contract
  -> one mixed typed Recipe candidate
  -> atomic source / Recipe / CallSlot / evidence co-seal
  -> semantic program / JoinSig / After
  -> invocation-result lifecycle
  -> invocation-only cleanup projection
  -> exit-transaction co-seal
```

The producer is bounded and exact.  It is not a generic Dynamic-value
classifier and it does not open physical lowering, Home, ABI, CFG, or
runtime representation.

## Owners

- `mapping.rs` is the sole bounded Recipe producer.  It emits the mixed typed
  shape: `pos`, `end`, induction binding, carrier, `CompareI64`, and the two
  `BinaryI64(Add)` operations are `I64`; string/call values remain `Dynamic`.
- `dynamic_full_body_recipe/mod.rs` consumes the exact A2 parameter contract
  together with the source inventory.  Its private four-row relation keeps
  ordinal, `BindingRef`, source role, and Recipe class together until the
  envelope co-seal.  It never reconstructs the contract from names or AST.
- `claims.rs` owns the private complete role-to-Recipe claim table.
- `coseal/coverage.rs` validates all six binding roles and all twenty-eight
  source roles against the mixed Recipe classes.
- `coseal/calls.rs` validates the exact call classes:
  `I6: Dynamic receiver + I64/I64 arguments -> Dynamic`, and
  `I7: Dynamic receiver + Dynamic argument -> I64`.
- `coseal/physical_evidence.rs` owns the private 17-placement / 15-operation
  source-effect ledger.  It relates already verified facts; it is not a new
  AST observer or execution authority.
- `coseal/semantic_program/` consumes the whole envelope and derives one
  non-splittable JoinSig/After closure and the exact two-row Fault catalog:
  `I6` and `I7`.  `I1`, `I5`, `I9`, and `I15` are typed non-faulting operations
  in this cohort.
- `invocation_carrier_lifecycle.rs` remains the sole lifecycle owner for the
  one Dynamic invocation result: `I6/V10` is the Loop-body temporary.  The
  `I7/V11` exact-I64 result has no Dynamic lease or End obligation.  It does
  not own the I64 induction carrier.
- `invocation_cleanup.rs` is the only cleanup projection for this cohort.  It
  retains the invocation lifecycle and the exact four logical cut points:
  `I6` Fault, `I7` Fault, inner Return, and
  Backedge.  The induction disposition is the private
  `ExactI64TrivialNoEnd` marker.
- `exit_transaction.rs` consumes that projection and seals the inner Recipe
  Return and outer callable Tail to the existing function Completion target.

All complete products are non-`Clone` and non-splittable.  Production callers
cannot supply a second Recipe, source catalog, JoinSig, After, lifecycle, or
Completion.  Missing, foreign, duplicate, or ambiguous facts reject as a
whole; there is no fallback or retry.

## Mixed typed Recipe contract

The bounded Recipe has one I64 induction carrier and keeps only the genuinely
opaque values Dynamic:

```text
V1 pos / V2 end / V4,V6,V7,V8,V9,V11,V12,V14,V15,V16,V17 = I64
V5,V13 = Bool
V0 src / V3 pred_chars / V10 = Dynamic

I1  CompareI64(Less)
I5  BinaryI64(Add)
I15 BinaryI64(Add)
I6  CallSlot(substring)
I7  CallSlot(indexOf)
I9  CompareI64(Less)
```

The `i64` source contract is transported by the existing callable parameter
contract owner.  The Recipe producer validates that contract against the
source binding rows before it consumes the inventory.  It does not infer I64
from a loop variable name, ValueId, arithmetic spelling, or selector.

## Physical-input boundary

The only physical-input entry is the package-held
`VerifiedDynamicExitTransactionCoSealV1`.  Its HRTB view contains the exact
placement, operation/source-effect, CallSlot, Fault, and JoinSig/control rows.
It contains no `ValueId`, `BasicBlockId`, ABI, Home, Completion writer, CFG,
PHI, DraftSeal, Collector, or runtime representation.

The physical demand remains a consumer of this complete view.  It must not
rescan the Recipe or source ledger, and it must not expose single-item
selection or raw Recipe/JoinSig access.

## Non-authority

This directory does not own:

- parser syntax, resolver allocation, or callable parameter policy;
- Dynamic Home classification or local destination ownership;
- general all-V2 physical representation or checked ABI;
- physical Callable Tail/Completion consumption or return ABI;
- Builder, MIR, CFG, PHI, provider selection, runtime invocation;
- fallback, retry, or compatibility repair.

The selected A-prime cohort is intentionally narrower than a general Dynamic
carrier corridor.  If a future source shape needs an opaque Dynamic induction
carrier or a runtime-polymorphic result representation, it must open a new
design stop rather than widening this producer by inference.

## Acceptance and cleanup

Acceptance requires the exact parameter contract, mixed Recipe classes, the
two-row Fault catalog, one invocation lifecycle row, four cleanup cut
points, and the existing two Completion sites to remain co-sealed.  Focused
tests must reject foreign source, wrong class, missing/duplicate CallSlot,
wrong Fault coverage, and old/new lifecycle pairing.

The old operator/ingress/rebind/flow/induction lifecycle modules are retired;
their replacement is the narrow invocation lifecycle plus invocation cleanup
projection above.  Future common Loop physicalizer cleanup is tracked
separately and must consume this complete evidence rather than reconstructing
transfers from the Recipe.
