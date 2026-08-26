---
Status: Superseded evidence — not a current queue or implementation authority
Date: 2026-08-25
Decision: MIR-CALL-FINAL-SHAPE-AND-INGRESS-BOUNDARY-D0
Observed commit: 183c1418d7
Owner: MIR-CALL-RETIREMENT-v1
---

# MIR Call final shape and ingress boundary historical evidence

This card preserves the 2026-08-25 review evidence only. Current decisions and
task order are owned by `CURRENT_STATE.toml`, the rolling MirBuilder workstream,
and `design/mir-canonical-callsite-lane-ssot.md`; this file authorizes no row.

## Decision

Keep the final core and compatibility boundary separate:

```text
typed producer / owner-private ingress
  -> exact Callee
  -> MirInstruction::call
  -> canonical consumers

Call {
  dst: Option<ValueId>,
  callee: Callee,
  args: Vec<ValueId>,
  effects: EffectMask,
}
```

The three surviving design rows are ordered, not competing alternatives.

### Row A — `MIR-CALL-FINAL-SHAPE-CALLFLAGS-CENSUS-D0`

The local HEAD does **not** support a blanket `MirCall` retirement claim:
`unified_emitter/physical_terminal.rs` still consumes `MirCall` and converts it
to the canonical `MirInstruction::Call`. Therefore the row is split. Census
`CallFlags` semantic readers, public exports, and construction sites first. A
zero-reader result may retire only the `CallFlags` field/constructor edges in a
later bounded child; `MirCall` remains live transport until the physical
terminal has its own replacement row. If any reader owns meaning, stop and
name that owner instead of deleting or defaulting the flags.

Evidence/task: `mir-call-final-shape-callflags-census-d0-2026-08-26.toml`.

### Row B — `MIR-CALL-METHOD-NONE-PRODUCER-FIRST-D0`

Close producers before changing the enum:

```text
static call with exact qualified target -> Callee::Global(owner.method/arity)
instance call with receiver            -> Callee::Method { receiver, ... }
receiver absent                        -> typed reject before publication
```

Only after the finite producer census reaches zero for `Method { receiver:
None, ... }` may `receiver: Option<ValueId>` become a required `ValueId`.
`Method(None)` is not a static-call authority and must not be repaired by
optimizer, backend, name lookup, or a sentinel receiver.

### Row C — `MIR-INGRESS-CONTRACT-GUARD-P0`

Introduce a boundary guard before moving vocabulary to a neutral
`src/mir/ingress_contract/` module. Phase 0 freezes the current builder and
compiler reference counts and rejects any increase. Only types consumed by
both sides move; source/semantic authority stays in its existing owner. The
guard must prove that the move removes the builder-to-compiler reverse edge,
does not create a test-only cycle, and does not export a new production
authority.

## Ingress hardening queue

The review also found fail-fast defects that require a separate census before
any implementation:

1. **JSON-v1 dummy target audit** — `func = 0` is not equivalent to
   `ValueId::INVALID`; certainty defaults, discarded extern effects, and
   warn-to-log aliasing must be classified as observed behavior versus an
   acceptance condition. Explicit malformed targets must reject before block
   publication.
2. **JSON-v0 effect-state audit** — unknown effect names must not be silently
   continued and omitted effects must not receive an unowned default. Define a
   finite typed state matrix and a single owner before changing parsing.
3. **Macro ingress audit** — partial JSON matching, the 128 KiB argv ceiling,
   per-pass AST duplication, and duplicated body-shape inference need a
   read-only owner/terminal census. `.hako` macro production remains a
   compatibility concern and is not a license to widen the selected backend.

These three audits are `NoSafeSlice` until their source authority, canonical
issuer, and fail-fast boundary are named. They must not be folded into Row A,
Row B, or the neutral-contract move.

## Authority and non-authority

| Boundary | Authority | Non-authority |
|---|---|---|
| Call schema | one atomic core decision after writer/consumer closure | `func = 0`, `INVALID`, default `Callee`, or `Option` recovery |
| Method target | producer-owned exact source relation | `Method(None)`, optimizer inference, backend lookup |
| ingress contract | owner-local typed input and one-shot resolver | shared global resolver, string retry, log-only demotion |
| effects | explicit ingress/plan relation | omitted-field default, unknown-name `continue`, backend guess |
| macro boundary | one declared input/shape owner | partial JSON scan, repeated AST reconstruction, argv truncation |

## Finite acceptance matrix

| State | Outcome |
|---|---|
| exact typed target and exact effects | issue one canonical `Call` |
| malformed explicit target/effect | typed reject before publication |
| legacy target with one exact owner relation | resolve once, then issue `Call` |
| missing/ambiguous/foreign target | typed reject; no retry |
| `Method(None)` producer | producer census blocker; no enum cutover |
| neutral-contract reference count increase | guard failure; move not accepted |
| no `CallFlags` semantic reader after public/API and generated-projection disposition | `CallFlags`-only retirement candidate; `MirCall` remains live until its physical-terminal replacement row |

## Non-claims

This queue does not retire `Callee::Closure`, decide Constructor/NewBox
ownership, remove all JSON-v0 compatibility, change backend selection, promote
PyVM/reference/Python (they remain `ParkedSealed`), or clean every warning.
It also does not claim that static review or a local green test proves the
production graph; each row still needs producer inventory, positive/negative
proof, guard, and closeout receipt.
