# MIRBUILDER-ME-UPVAR-RECEIVER-SEAL-S13

**Row**: `MIRBUILDER-ME-UPVAR-RECEIVER-SEAL-S13`
**BoxShape**: one production edge — the canonical `Me` seal arm accepts
the designed `ResolvedLexicalRefV1::Upvar` receiver ref, mapping it to
its exact parent source binding. No new vocabulary, no new admitted
shape, no authority move.
**Mode**: `fast` — focused gates only.

## Evidence

`efbc22b14b` (2026-09-08, "seal exact Array source lifecycle before
typed Local") introduced the `Me` receiver-authority arm in
`seal_shadow_body_shape`. The arm accepts:

- `ResolvedLexicalRefV1::Local(binding)` → `BodyMeReceiverV1::Lexical`
- no ref + `DeclaredFunction` root with `StaticCurrentOwner` policy →
  `BodyMeReceiverV1::StaticCurrentOwner`
- everything else → `DraftInvariant("body Me shape lacks exact receiver
  authority")`

`resolver_canonicalization.rs` already maps a shadow `Ancestor` capture
to `ResolvedLexicalRefV1::Upvar(UpvarRefV1 { capturing_owner, source })`
where `source` is the parent root's exact receiver `BindingRefV1`. A
lambda body reading `me` therefore reaches the seal carrying the
designed Upvar ref — which the arm does not accept — so
`resolve_forest` fails inside the seal.

The red pin since 9/8:

- `mir::resolved_semantics::owner_forest_tests::resolver_seals_receiver_read_as_structural_upvar`
  expects `forest.upvars().len() == 1` with
  `upvars()[0].source().owner() == roots()[0]` — the designed
  structural-capture contract the forest machinery
  (`derive_and_verify_upvars`, `normalized_upvars`,
  `ordered_capture`) was built to seal.

`BodyMeReceiverV1` consumers (`instance_construction`,
`instance_constructor_non_escape`, `query_body_facts`,
`body_effect_control_coverage`, `query_body_conformance_evidence`)
match `Lexical(binding)` as "the receiver's exact source binding".
`BindingRefV1` self-identifies its owning root, so consumers can still
distinguish a captured parent binding from a same-root local.

## Fix

In `body_shape_seal.rs`, extend the `Me` arm:

```rust
Some(ResolvedLexicalRefV1::Upvar(upvar)) => BodyMeReceiverV1::Lexical(upvar.source())
```

The Upvar edge itself is already recorded in `forest.upvars()`; the
seal only needs to accept the same exact receiver authority it already
requires for `Local`. No fabricated binding (`UpvarRefV1::source` is an
exact parent `BindingRefV1`), consistent with the enum's documented
"never fabricate a `BindingRefV1`" contract.

## Pins

- `resolver_seals_receiver_read_as_structural_upvar` green.
- `mir::resolved_semantics` module green except the two known stale
  pins (`accepted_vocabulary_is_closed_and_reviewable`,
  `forest_parent_rejects_unsupported_ancestry_and_orphan_scope`) owned
  by the S14 repin batch.
- match/qmark module stays 5/5 green.

## Non-claims

- Does not reopen upvar capture semantics, receiver policy, or the
  `BodyMeReceiverV1` variant set (no new variant added).
- Does not touch script admission or the retired raw-compat boundary.
- Gate-1 suite state unchanged.

## Landed evidence

- `body_shape_seal.rs`: the `Me` arm now maps
  `Some(ResolvedLexicalRefV1::Upvar(upvar))` to
  `BodyMeReceiverV1::Lexical(upvar.source())` — the exact parent
  receiver binding already carried by the designed Upvar ref.
- `mir::resolved_semantics`: 346/348 green, including
  `resolver_seals_receiver_read_as_structural_upvar` (was red since
  efbc22b14b). The 2 residual reds are the known S14 stale pins:
  `accepted_vocabulary_is_closed_and_reviewable` (Program vocabulary
  drift, 0ac2b93e52) and
  `forest_parent_rejects_unsupported_ancestry_and_orphan_scope`
  (IfThen ancestry fixture, 440c16216d — same class as S9).
