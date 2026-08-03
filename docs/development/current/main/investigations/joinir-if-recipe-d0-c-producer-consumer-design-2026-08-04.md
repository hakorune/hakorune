# JOINIR-IF-RECIPE-D0-C-PRODUCER-CONSUMER

Status: D0-C1/D0-C2 implementation landed and verified. D0-D physical
adoption and selected-edge cutover remain separate design-gated rows.
Date: 2026-08-04

This card fixes the first production seam for the portable If recipe. It does
not claim repository-wide PHI/SSA unification and it does not remove the old
If physicalizer yet.

## Decision

Use the existing resolved-trivial canonical lifecycle:

```text
one admitted preflight profile
  -> one same-pass VerifiedTrivialIfRecipeFactsV1
  -> one IfRecipeArtifactV1
  -> one VerifiedIfPhysicalInputV1 (JoinSig included)
  -> one CanonicalSsaFunctionSessionV2
  -> existing CanonicalTrivialSsaLowererV1
  -> existing unpublished compile candidate
```

The physical owner remains:

```text
CanonicalSsaFunctionSessionV2
  = BindingSsaBuilderV1 + CanonicalCfgSessionV1 + one PhiTxn
```

No second SSA/PHI transaction, `IfCfgSessionV1` replacement, route registry,
or new PHI writer is introduced in D0-C.

## Selected shape

Only the already admitted exact profile is eligible:

```text
one resolved-trivial If
explicit else
then/else fall through
one outer BindingRef assignment per branch
same admitted i64/Bool value class
post-merge read of that binding
no nested control, return/throw, short-circuit, Call, Record, Match, or retry
```

`recipe_facts()==None` means `NotThisShape` at the pre-effect admission
boundary. It is not a downstream `Option`, physicalizer decline, or permission
to try another route.

## Named producer

Add one neutral, small producer adapter (separate from the near-limit
`capability.rs` and lowerer files):

```text
produce_trivial_if_physical_input_v1
  -> map_trivial_if_recipe_v1(profile, input.function())
  -> VerifiedIfRecipeArtifactV1
  -> VerifiedIfPhysicalInputV1::from_artifact
```

The sole production call is in the central
`lower_resolved_trivial_function_draft_retaining_failure_v1` ingress, after
the plan is unpacked and before the inner function draft-seal session is
opened. This central seam covers the normal/main/helper/direct-call wrappers
without transporting a second non-Clone field through every wrapper.

The producer consumes the already sealed profile/facts. It must not rescan AST,
recompute route policy, construct `BindingRefV1` from a read, or call a legacy
route. Mapper/JoinSig/physical-input failure is a typed pre-effect `Freeze`.

## Named consumer

Pass the single-use physical input to `CanonicalTrivialSsaLowererV1` through a
small admission adapter. The consumer returns a typed `Result`, never an
`Option`:

```text
pre-effect NotThisShape -> do not enter this D0-C consumer
selected shape          -> Result<CanonicalIfRecipeAdmissionV1, Freeze>
```

The admission receipt is consumed exactly once by the first/only selected If.
It checks source-claim/root correspondence and the logical JoinSig against the
already admitted shape, then delegates leaf emission and physical CFG/SSA/PHI
work to the existing canonical lowerer/session. It may borrow an immutable
source view for admitted leaf emission, but source is never re-read to choose
or repair a route.

`CanonicalTrivialSsaLowererV1::lower_if` remains the parity/physical oracle in
D0-C. Removing its selected old edge is D0-D/E work, not an implicit part of
the adapter.

## Candidate and failure boundary

The producer and consumer run inside the existing unpublished resolved-module
candidate. Any mapper, JoinSig, admission, lowering, PHI, or draft-seal error
discards the candidate/session; it must leave the live `MirCompiler.builder`
unchanged and allow a fresh compile to succeed. `PhiTxn` is only a local
pending-PHI transaction; it is not the whole-module rollback boundary.

## Ordered implementation tasks

### D0-C1 — preflight producer

- Add the neutral producer adapter in a file below 800 lines.
- Promote `map_trivial_if_recipe_v1` to exactly one production caller.
- Keep all other mapper/test callers classified and preserve the caller-zero
  guard for independent artifact/JoinSig construction.
- Add explicit-shape, not-this-shape, mapper-rejection, and one-caller tests.
- Do not change physical CFG/SSA/PHI behavior.

### D0-C2 — canonical admission consumer

- Thread the non-Clone physical input through the central lowerer seam only.
- Consume it once in a typed admission receipt before the selected If lowers.
- Reuse `CanonicalSsaFunctionSessionV2`; do not add a PHI/SSA owner.
- Add semantic/MIR/CFG/PHI/interpreter parity and late-failure candidate-isolation
  tests. The old selected writer remains for parity in this row.

### D0-D — physical adoption (later)

- Make the verified JoinSig/physical input drive the selected canonical
  physicalization rather than merely admission.
- Prove the selected old writer edge is caller-zero.
- Keep raw IfForm, A+ `IfCfgSessionV1`, CorePlan/JoinIR, and JSON-v0 writers
  outside this cutover.

### D0-E — selected-edge cutover (later)

- Remove only the selected explicit-else old edge.
- Prove no post-effect `Option`, retry, reselection, or fallback remains for
  that shape.
- Repeat other If families only through new design rows.

## D0-C1/D0-C2 closeout

The named adapter is now wired at the central resolved-trivial draft ingress.
The preflight producer classifies `NotThisShape` before route execution and
the one-shot admission consumes the selected physical input at the exact
sealed If site. D0-C is intentionally admission-only: the current admission
receipt consumes and drops the payload while the existing canonical lowerer
emits the physical branch/merge/PHI shape. D0-D must replace that drop with a
typed demand handoff before any old edge is retired.

Evidence:

```text
RUSTFLAGS='-Awarnings' cargo check -q --lib
RUSTFLAGS='-Awarnings' cargo test -q --lib resolved_lowering -- --test-threads=1
RUSTFLAGS='-Awarnings' cargo test --features vm-reference -q --lib resolved_lowering::if_tests -- --test-threads=1
bash tools/checks/mirbuilder_inplace_replacement_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

All focused tests and guards are green. The adapter and touched Rust owners
remain below 800 lines. The next stop is D0-D design, not selected-edge
deletion.

## Acceptance gates

```text
producer caller count = 1
physical-input/JoinSig independent constructors = 0
consumer/admission caller count = 1
new consumer Option/Retry/reselection = 0
all touched Rust/test files < 800 lines
explicit-else fixture maps and consumes one receipt
implicit/no-If fixture is pre-effect NotThisShape
semantic + MIR + CFG + PHI + interpreter parity green
injected late failure leaves live Builder unchanged
fresh compile after failure succeeds
```

## Explicit non-claims

This row does not unify all repository PHI/SSA writers, does not retire
`IfCfgSessionV1`, raw `IfForm`, CorePlan/JoinIR, JoinIR converter, JSON-v0, or
Generic/implicit/nested/Call/Match If shapes, and does not broaden the language
grammar. Those require separate caller-zero evidence and design rows.
