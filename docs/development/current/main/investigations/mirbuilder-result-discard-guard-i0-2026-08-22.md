Status: implemented and verified; canonical strictness is the next independent slice
Task: MIR-RESULT-DISCARD-GUARD-I0
Date: 2026-08-22
Priority: Medium-High
Owner: `tools/checks/mirbuilder_physical_result_discard_guard.sh`
NextCard: MIR-EMIT-CANONICAL-STRICTNESS-D0
---

# MIRBuilder physical result-discard guard I0

## Six-line brief

Decision: enforce one small lexical guard over the MIRBuilder physical-writer
surface. An `emit_instruction` result may be propagated or intentionally
handled, but it may not be hidden in an underscore binding, `.ok()`, or
`drop(...)`.

Source authority + canonical issuer: `MirBuilder::emit_instruction` remains
the sole physical writer. This guard is only a regression fence; it does not
issue semantic facts, receipts, local bindings, or rollback authority.

Non-authority: the full `let _ =` census, cleanup/restore calls, FFI and IO,
workspace-wide Clippy configuration, comments, and a raw match count are not
classified or reowned by this slice.

Fail-fast boundary: before a change is committed, the guard scans only
`src/mir/builder/**/*.rs` and rejects the three selected discard shapes. Its
embedded multiline fixtures fail if the matcher becomes line-only.

Smallest next slice: add this reusable shell guard, register it in the check
index, and record the focused pass. Do not widen it to all fallible results or
change `emit_instruction` itself.

Non-claims: no workspace lint rollout, cleanup allowlist, generic transaction,
`EmitReceipt`, strict canonical emission API, fallback change, A/C, Recipe,
Join, backend, or performance claim.

## Accepted policy

The Gate 0 census found 109 exact `let _ =` rows under `src/mir/builder`, but
only one confirmed physical MIR discard: assignment's `ReleaseStrong`, now
fixed by Gate 1. The other rows include diagnostics, initialization,
cleanup/restore, fixtures, and non-Result values; a blanket deny would mix
different owners and make the check heavy without improving the authority
boundary.

This guard therefore protects the confirmed physical-writer shape directly:

```text
let _ = ... emit_instruction(...)
let _name = ... emit_instruction(...)
emit_instruction(...).ok()
drop(emit_instruction(...))
```

The underscore rule is intentionally conservative: an underscore-prefixed
binding around the physical writer is treated as discard-shaped and requires
an explicit design change before it can enter production. A normal named
binding that propagates or later handles the `Result` is outside this lexical
fence.

## Guard contract

The public entry point is:

```text
bash tools/checks/mirbuilder_physical_result_discard_guard.sh
```

It has two bounded responsibilities:

1. scan Rust files below `src/mir/builder` for the three forbidden shapes;
2. run small in-script positive/negative fixtures, including multiline calls.

The matcher is not a Rust parser and is not a general must-use classifier.
If a future syntax falls outside these three shapes, it needs a separate
classified row rather than silently expanding this guard into a repository
lint.

## Implementation checkpoint — 2026-08-22

The guard is implemented in a 76-line shell owner. Its real-source scan and
embedded multiline fixtures both pass:

```text
bash tools/checks/mirbuilder_physical_result_discard_guard.sh -> PASS
bash tools/checks/current_state_pointer_guard.sh              -> ok
bash -n tools/checks/mirbuilder_physical_result_discard_guard.sh -> passed
guard source lines                                           -> 76
CURRENT_STATE.toml lines                                     -> 120
```

No production MIR code, Cargo manifest, or workspace lint changed in this
slice. The guard is deliberately an explicit focused command rather than a
new default heavy gate.

## Acceptance evidence

Positive:

- the real MIRBuilder source contains zero selected discard shapes;
- multiline underscore binding, `.ok()`, and `drop(...)` fixtures are
  rejected;
- propagated `?`, a named propagated result, and unrelated `drop(...)` remain
  allowed.

Negative:

- adding any selected shape under `src/mir/builder` makes the guard fail and
  prints the matching source location;
- the guard does not scan `src/mir` siblings or workspace crates;
- no cleanup/FFI/IO exception is silently accepted by this guard.

Structural:

```text
workspace-wide Clippy deny introduced                         = 0
blanket allow over MIRBuilder                                 = 0
second physical writer                                          = 0
guard source scope                                               = src/mir/builder/**/*.rs
assignment ReleaseStrong discard after Gate 1                   = 0
guard source file                                                < 760 lines
```

## Follow-up boundary

The next design card is `MIR-EMIT-CANONICAL-STRICTNESS-D0`: decide how the
canonical verified path differs from legacy repair while retaining one final
physical writer. That design may introduce a named prepared-emission seam,
but it must not be smuggled into this lexical guard. `EmitReceipt` remains a
parked option after that design, not an implementation dependency here.
