# Normal Callable Semantic Package

This module owns the one pre-Builder semantic package for a final parser
callable source batch.

```text
VerifiedFinalCallableProgramSourceV1
  -> one FunctionSemanticResolverSessionV1 allocation
  -> complete resolved callable batch
  -> complete owned parameter demands
  -> exact one Dynamic full-body source/Recipe candidate
```

The package is non-Clone and non-splittable. Borrowed lowering inputs and
borrowed parameter-demand catalogs exist only inside the issuer. This module
does not own callable-name selection, CFG, PHI, Completion consumption,
physical ABI, runtime dispatch, retry, or fallback.

The current bounded cutover will next add catalog-neutral Dynamic invocation
relations and transform this package whole into the existing Recipe/JoinSig
semantic program before replacing the selected production caller.
