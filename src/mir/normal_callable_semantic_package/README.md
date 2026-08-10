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
         commit-before-end rebind relation (I13/I15/I16/Fault/Backedge)
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
no catalog lifetime.

The normal/default source-backed lifecycle issues this package before Builder
effects, consumes its catalog through one typed install transition, and lowers
selected callables through the package port. The Builder-side adapter owns only
raw source lineage and temporary lowering-state installation. The old semantic
seal, Dynamic target extension, Complete-source pairing, and semantic-loan port
are absent from that production edge. Source-backed package failure is
terminal; the AST-only compatibility catalog is not a retry route.

The package now carries the bounded logical two-site Completion projection in
the selected Dynamic row. This still does not claim Dynamic physical
completion. In particular,
`ParserScanLoopBox.skip_while/4` currently reaches the installed package and
then fails closed at the existing physical source-ledger consumption boundary.
The package owns only semantic carrier flow, cleanup evidence, and the logical
inner-Return/outer-Tail function-exit relation; actual rebind, End, Home,
physical Return/ABI, and physicalization remain later owners.
