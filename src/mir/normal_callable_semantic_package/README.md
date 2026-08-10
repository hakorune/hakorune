# Normal Callable Semantic Package

This module owns the one pre-Builder semantic package for a final parser
callable source batch.

```text
VerifiedFinalCallableProgramSourceV1
  -> one FunctionSemanticResolverSessionV1 allocation
  -> complete resolved callable batch
  -> exact owned direct-method parameter-demand subset
  -> exact one Dynamic full-body source/Recipe candidate
  -> owned catalog-neutral Dynamic call relations
  -> Recipe / JoinSig / invocation lifecycle / operator lifecycle
```

Complete batch membership comes from final callable anchors, not the parameter
catalog. A mixed Program may therefore retain top-level rows while only its
direct Box methods contribute parameter demands. The Dynamic candidate and its
required demands must still resolve to the same private batch slot.

The package is non-Clone and non-splittable. Borrowed lowering inputs and
borrowed parameter-demand catalogs exist only inside the issuer. This module
does not own callable-name selection, CFG, PHI, Completion consumption,
physical ABI, runtime dispatch, retry, or fallback.

Dynamic call relations are issued from the same batch-scoped lowering input as
owned owner/site/binding/argument rows. The Recipe co-seal no longer stores a
callable catalog borrow, and the resulting semantic/lifecycle program carries
no catalog lifetime. The old target catalog remains only as a migration
adapter for the not-yet-replaced Builder caller.

The bounded cutover must next make zero-Dynamic complete batches a typed valid
unselected projection, seal the selected callable key to its exact private
batch slot, and then replace the selected production caller while deleting the
old Builder seal, Dynamic extension, and loan port together.
