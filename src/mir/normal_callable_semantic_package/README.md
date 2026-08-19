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
  -> exact owned direct-method parameter-contract subset
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
direct Box methods contribute parameter contracts. When selected, the Dynamic
candidate and its required contract must resolve to the same private batch
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
coverage. Borrowed lowering inputs and parameter contracts remain scoped to the
callback. ExactText(StringBox-as-Text) rows remain formal source contracts in
this loan; `HomeDemandV1::Handle` does not become a runtime Text handle or
wire. HomeDemand is only a derived Dynamic-ingress view. This module does not
own callable-name selection, CFG, PHI, runtime exit chronology, physical ABI,
runtime dispatch, retry, or fallback.

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

The package now carries the bounded `VerifiedDynamicExitTransactionCoSealV1`
in the selected Dynamic row. This is the final semantic co-seal for the
current bounded lane: it transitively owns the semantic program, carrier flow,
carrier-only cleanup, and the exact logical inner-Return/outer-Tail route
pair. It does not claim runtime exit chronology or Dynamic physical
completion. In particular,
`ParserScanLoopBox.skip_while/4` currently reaches the installed package and
then fails closed at the existing physical source-ledger consumption boundary.
The package owns only semantic carrier flow, cleanup evidence, and the logical
inner-Return/outer-Tail function-exit relation; actual rebind, End, Home,
physical Return/ABI, and physicalization remain later owners.

For the selected cataloged Dynamic row, the installed catalog declaration also
issues one move-only physical-header projection. The package loan transports
that projection to A-prime exactly once; A-prime consumes it for the physical
header and never re-observes the AST/root declaration.

## Callable physical signature mapping (2026-08-16)

The package now issues one non-`Clone` physical-signature cohort from the same
selected/batch identity, declaration mode, exact instance `Receiver` binding,
and complete explicit parameter-contract rows. It is a projection-only
mapping: no `ValueId`, Completion, runtime token, call-site actual, or Text
residence is owned here.

```text
source_logical_arity       = explicit source formal count (/N)
receiver_lane_count        = 1 iff InstanceBoxMethod, otherwise 0
physical_formal_lane_count = sum(explicit formal lane widths)
physical_callable_lane_count
  = receiver_lane_count + physical_formal_lane_count

physical order:
  [InstanceReceiver?]
  then each explicit formal in ordinal order
    ordinary   -> [OrdinaryScalar]
    ExactText  -> [ExactTextSlot, ExactTextGeneration]
```

The receiver is `SourceBindingSiteV1::Receiver`, not an explicit formal and
not an ExactText ordinal. Static methods have no receiver lane. Lengths of
`/N`, `FunctionSignature`, or `Vec<ValueId>` never repair or infer this map.
The installed Port lends the selected input, parameter contracts, package
S6C child, and its matching signature row through one exactly-once HRTB loan;
separate child/key loans cannot be recombined. Skeleton, call-edge, and
Canonical-session consumers remain later mechanical projections.

## Source/header transport (2026-08-15)

The package also has one caller-zero, Builder-free sparse source/header cohort.
Each selected cataloged row that has its own complete formal parameter contract
and an explicit source result annotation accepted by the current scalar
bootstrap (`i64`) is co-sealed with `verify_function_completion_v1`. Missing or
unannotated sibling rows are absent from the cohort and cannot erase an
eligible row. The cohort itself is always present and moves through install;
only row lookup is optional and is lent as an
`Option<CallablePhysicalHeaderRefV1>` on the selected lowering view. A callable
without that explicit row admission remains valid for its ordinary semantic
route and lends `None`; it is not inferred from a body, MIR, ResultCatalog, or
fixture expectation.

This row does not define a Text handle ABI or runtime wire. In particular,
`StringBox`/ExactText physical parameters, TextEq residence, Builder IDs, and
the S6C physical session remain downstream design stops. The cohort owns the
source result spelling and Completion proof only; the formal parameter issuer
continues to own ordinal, BindingRef, owner, and formal-kind evidence.

## Installed S6C child composition (2026-08-15)

The caller-zero S6C child row is issued only inside the same package issuer
that owns the selected batch map. A private Completion seed cohort performs
the one `verify_function_completion_v1` pass; an exact `AppMainStaticChild`
candidate consumes its seed into `VerifiedS6CSemanticChildV1`, while ordinary
seeds alone are offered to the generic header projection. The child retains
the complete S6C Facts/Recipe/Join/prephysical parent and lends only its
result/parity views; Completion, Loop Return, and callable Tail remain
projections of that retained parent, not a second field or clone.

After install, `NormalCallableSemanticPackagePortV1::with_s6c_child` is the
sole child loan. It has no slot/key argument, rejects the generic selected
entry points for the Main-child role, and records the child key in the same
exactly-once coverage ledger. A second child loan is typed rejection. This
row is still Builder-free and caller-zero: it does not define TextFormal wire
mapping, TextEq residence/route, V2 transport, MIR IDs, a production caller,
fallback, or retry.

The common-V2 caller-zero row is now issued through
`NormalCallableSemanticPackagePortV1::with_s6c_common_v2_pre_session`. This
single HRTB lends the selected callable, its physical-signature sibling, the
installed child, and `PreparedLoopV2PreSessionEnvelopeV1` together. The
envelope is created inside the child’s retained source loan, so operation,
control, and coverage products cannot be acquired or re-paired independently.
`with_s6c_child` remains only a compatibility wrapper over this canonical
seam. The port still consumes the selected Main-child key exactly once and
does not open Builder/session, lifecycle, Text residence, route, fallback, or
production publication.

The selected lowering view also lends the batch-owned
`VerifiedResolvedBlockExpressionExpectationV1` from that same callable row.
This is a scoped transport accessor only: the package does not clone, reissue,
recount, or expose a raw `usize`; a missing batch row is terminal. The receipt
can therefore reach the later common admission without opening a session,
Completion consumption, CFG/SSA/PHI, physical lowering, fallback, or retry.

## S6C physical header/effects co-seal I0 (2026-08-17)

The installed S6C loan now carries two additional sibling projections from
the same source-backed cohort. `VerifiedS6CStorageHeaderProjectionV1` is a
catalog-declaration projection for storage-facing params/result/attrs/uses;
the catalog declaration remains its sole source authority and the projection
is intentionally distinct from the Dynamic legacy header. The child also
lends `VerifiedS6CPhysicalFunctionEffectsV1`, whose `EffectMask::READ` is
issued only after the retained S6C call contracts prove the two external
operations are exactly the expected pure reads. CoreMethod/Facts remain the
semantic effect authority.

This I0 is caller-zero and Builder-free. It opens no skeleton, `ValueId`,
formal lane adoption, Loop CFG/PHI, Completion claim, DraftSeal, lifecycle,
Text residence, route selection, production caller, fallback, or retry. The
future function-entry consumer must take header, signature, result, and
effects through the same HRTB cohort rather than re-reading a declaration or
reconstructing an effect from MIR.

## Typed BlockExpr transport I0 receipt (2026-08-17)

`SelectedCallableLoweringInputRefV1::block_expr_expectation` now borrows the
resolver-issued expectation through the existing package HRTB. A selected
static method containing a BlockExpr is covered by the focused handoff test;
the input and its physical-signature sibling remain the only callback-scoped
views. No second source issuer or package-owned semantic count was added.

## Anchor/package audit reconciliation (2026-08-10)

The selected-mapping boundary is closed and must not be reopened as a second
catalog or batch authority. The accepted shape is:

```text
parser-issued anchor
  -> cloneable comparison-only identity view
  -> source-backed catalog row
  + complete resolved semantic-batch row
  -> one exact identity co-seal
  -> selected key -> private batch slot
```

The HRTB syntax loan remains the only AST-borrowing boundary. The cloneable
identity is comparison-only: it cannot be serialized, converted to a pointer
or number, used as a key, or used to issue a resolver owner. The catalog and
batch are sibling products from the same final source; neither is derived from
the other. `prepare_install`/`commit` is the only catalog installation path,
and `with_selected_lowering_input(key, callback)` is the only production
lowering surface.

Dynamic admission uses the resolved declaration mode from each complete-batch
row and lends only `StaticBoxMethod` rows to the bounded Dynamic issuer.
Instance and top-level rows remain ordinary-owned without Dynamic source or
parameter probing; production selection is still restricted to rows present
in the sealed selected-map. A valid unselected row remains in the complete
batch and cannot steal the selected route. Missing, duplicate, foreign, or
repaired identity is terminal; the AST-only compatibility catalog is never a
retry path.

This audit does not authorize a new physical target catalog or a raw
`CallSlot` getter. Exact source-bound target retention for a future V2
operation/physical-demand bridge is a separate design stop; it must either be
co-sealed in that bridge or be retained by an existing private semantic
product. No package API, public selected key, batch slot, owner number, or
standalone ingress relation may be added for that purpose.

## Instance constructor semantic owner I0 (2026-08-20)

The final parser callable source now retains one non-Clone constructor source
catalog whose opaque `ConstructorSourceIdV1` comes from the parser transaction.
Final Box ordinals and normalized constructor keys validate placement only;
Builder work ordering and `(box name, key)` never reissue occurrence identity.

Before the ordinary callable batch consumes the final source, the existing
Brand-aware resolver issues one complete owner forest for every constructor
root and all nested lambdas. The semantic package retains those forests as one
non-Clone batch and carries them through installation unchanged. Missing or
changed syntax, deferred resolution, root/profile drift, and cardinality drift
reject before Builder effects. This row does not consume the batch physically,
activate Brand unwrap, or remove the legacy `is_brand_declared` probe.

## Script instance-Box transferred boundary I0 (2026-08-20)

For non-app selected Script, an instance Box is transparent to the Script
resolver only after one pre-Builder census proves that every retained method is
owned by the callable semantic package and every constructor is owned by the
parser-issued constructor semantic batch. Exact Program ordinals select the
cohort; Box names and constructor keys only validate issued placement.

The root demand window records `InstanceBoxSemanticOwner`, so the Script
resolver owns surrounding expressions without entering the transferred Box.
Missing, foreign, or extra method/constructor coverage rejects before Builder
effects. Runtime Box lowering is unchanged, and this row neither consumes Brand
relations physically nor removes the legacy raw name probe.
