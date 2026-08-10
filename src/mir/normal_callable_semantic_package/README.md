# Normal Callable Semantic Package

This module owns the one pre-Builder semantic package for a final parser
callable source batch.

```text
VerifiedFinalCallableProgramSourceV1
  ├─ source-backed installable callable catalog
  └─ one FunctionSemanticResolverSessionV1 allocation
       -> complete resolved callable batch
            ↓ exact opaque declaration identity co-seal
       selected key -> private batch slot
  -> exact owned direct-method parameter-demand subset
  -> private Dynamic projection
       ValidUnselected
       Selected {
         owned catalog-neutral call relations
         Recipe / JoinSig / invocation lifecycle / operator lifecycle
         parameter #1 -> V1/C0/B0/Enter borrowed ingress
       }
```

Complete batch membership comes from final callable anchors, not the parameter
catalog. A mixed Program may therefore retain top-level rows while only its
direct Box methods contribute parameter demands. When selected, the Dynamic
candidate and its required demands must resolve to the same private batch
slot. A complete batch with no Dynamic candidate is a typed valid-unselected
projection, not an issuer failure or fallback.

The package is non-Clone and non-splittable. Catalog installation is one
consuming prepare/commit transition. After installation, the only lowering
surface is an exactly-once package port:

```text
begin_lowering(installed context)
  -> with_selected_lowering_input(key, callback)
  -> complete()
```

The port never exposes a batch slot, rejects foreign catalog contexts and
duplicate selected-key consumption, and closes only after complete selected
coverage. Borrowed lowering inputs and parameter demands remain scoped to the
callback. This module does not own callable-name selection, CFG, PHI,
Completion consumption, physical ABI, runtime dispatch, retry, or fallback.

Dynamic call relations are issued from the same batch-scoped lowering input as
owned owner/site/binding/argument rows. The Recipe co-seal no longer stores a
callable catalog borrow, and the resulting semantic/lifecycle program carries
no catalog lifetime. The old target catalog remains only as a migration
adapter for the not-yet-replaced Builder caller.

The bounded cutover must next replace the production caller while deleting the
old Builder seal, Dynamic extension, Complete-source pairing, and loan port in
the same replacement cell. Source-backed package failure is terminal; the
AST-only compatibility catalog is not a retry route.
