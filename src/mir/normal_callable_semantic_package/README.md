# Normal Callable Semantic Package

This module owns the one pre-Builder semantic package for a final parser
callable source batch.

```text
VerifiedFinalCallableProgramSourceV1
  -> one FunctionSemanticResolverSessionV1 allocation
  -> complete resolved callable batch
  -> exact owned direct-method parameter-demand subset
  -> exact one Dynamic full-body source/Recipe candidate
```

Complete batch membership comes from final callable anchors, not the parameter
catalog. A mixed Program may therefore retain top-level rows while only its
direct Box methods contribute parameter demands. The Dynamic candidate and its
required demands must still resolve to the same private batch slot.

The package is non-Clone and non-splittable. Borrowed lowering inputs and
borrowed parameter-demand catalogs exist only inside the issuer. This module
does not own callable-name selection, CFG, PHI, Completion consumption,
physical ABI, runtime dispatch, retry, or fallback.

The current bounded cutover will next add catalog-neutral Dynamic invocation
relations and transform this package whole into the existing Recipe/JoinSig
semantic program before replacing the selected production caller.
