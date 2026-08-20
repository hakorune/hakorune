---
Status: Active design stop
Date: 2026-08-20
Decision: MIR-CALL-LEGACY-TARGET-CENSUS-D0
Parent: docs/development/current/main/design/mir-canonical-callsite-lane-ssot.md
ProductionCaller: none (observation only)
ReplacementCell: none until the census closes
---

# MIR-CALL-LEGACY-TARGET-CENSUS-D0

## Six-line brief

Decision: Freeze and classify every remaining `MirInstruction::Call {
callee: None }` producer/consumer before any canonical-call guard or field
retirement. This is a read-only design census, not an implementation row.

Source authority + canonical issuer: The `MirInstruction::Call` definition
and each exact producer/consumer at the pinned commit are the source facts;
one deterministic census manifest is the sole observation issuer and adds no
semantic receipt.

Non-authority: text hit counts, `ValueId::INVALID`, `func` spelling, test names,
compatibility labels, diagnostics, inferred backend behavior, or a selected
green fixture cannot classify a row or authorize a migration.

Fail-fast boundary: before a GUARD-I0 is opened, every row must have one exact
file/line, role (`canonicalizer_input`, `explicit_compatibility`, `test`,
`diagnostic`, or `unreachable`), reachability disposition, owner, and named
replacement/retirement condition. Missing, duplicate, or commit-drifted rows
stop the lane with `NoSafeSlice`.

Smallest next slice: produce the pinned 25-mention census, split selected
native/canonical candidates from JSON-v0, interpreter, compatibility, tests,
and diagnostics, and write the GUARD-I0 acceptance without changing MIR,
loader, optimizer, backend, or printer code.

Non-claims: no `Option<Callee>`/`func` deletion, no `LegacyCall` variant, no
default/sentinel callee, no backend fallback change, no JSON-v0 retirement, no
Script transport, no production switch, and no performance claim.

## Scope and fixed order

This row is the first successor named by the canonical-call SSOT:

```text
CENSUS-D0 -> CANONICAL-CORRIDOR-GUARD-I0 -> LEGACY-TARGET-RETIREMENT-R0
```

It is explicitly selected only after the Script cleanup row closed. It does
not bypass the parked canonical Script transport or reopen the selected-normal
physical bridge. The later GUARD-I0 must first prove, for one named native /
canonical corridor, `callee.is_some() == 100%`, zero legacy `func` authority,
and zero string fallback. Only after every non-corridor family has an explicit
compatibility or retirement owner may R0 choose an end state.

## Initial source census (read-only)

At the current branch HEAD, the literal `callee: None` occurrences are spread
across 20 Rust files and 25 source mentions. The worker-audited inventory is:

| Family | Current locations | Initial role |
| --- | --- | --- |
| JSON-v0 compatibility producer | `src/runner/json_v0_bridge/lowering/expr/call_ops.rs:81,368,412` | 3 explicit compatibility producers |
| Canonicalizer input | `src/mir/passes/callsite_canonicalize/pass.rs:112` | 1 canonicalizer input |
| Native boundary reject/analysis | `src/mir/inline_leaf.rs:272`, `src/mir/contracts/backend_core_ops/allowlists.rs:14` | 2 production reject/diagnostic consumers |
| Contract/comments only | `src/mir/instruction.rs:430`, `src/mir/join_ir_to_mir/call_generator.rs:16`, `src/mir/contracts/backend_core_ops/allowlists.rs:6` | 3 non-executable references |
| Tests/fixtures/assertions | ownership, verification, instruction/backend, callsite, JSON program, LLVM guard, and VM tests | 16 test-only mentions |

The audit found zero remaining selected-native producers outside the
canonicalizer input and zero `unreachable` production rows. The selected
native/canonical corridor is the final canonicalized MIR after
`MirOptimizerLateCallAndInline`, at the boundary immediately before LLVM or
other native backend consumption; JSON-v0 and VM compatibility are excluded.
This is a design census, not permission to change that boundary.

## Required GUARD-I0 design output

The census must define one machine-readable manifest (or an equivalent stable
guard fixture) with:

- pinned commit and source digest;
- unique row ID and exact file/line/symbol;
- producer vs consumer direction;
- family role and production/test/compat/diagnostic reachability;
- whether `func` is an authority or merely a legacy carrier;
- selected-corridor eligibility;
- replacement owner and retirement condition;
- explicit `NoSafeSlice` for rows whose caller/owner is not proven.

The guard design must reject row count drift, an unclassified new occurrence,
duplicate row IDs, a selected corridor with `callee=None`, and any claim that
`ValueId::INVALID` alone proves canonicalization. It must not mutate code or
promote the manifest into semantic compiler input.

The candidate GUARD-I0 acceptance is:

```text
final canonicalized native MIR: every Call has callee.is_some() == true
selected corridor callee=None count = 0
call-missing-callee reject code = stable
legacy func re-resolution/fallback = 0
callee=Some does not require func == ValueId::INVALID
```

## Stop conditions

Return to `NoSafeSlice` and do not open GUARD-I0 implementation if:

1. a producer/consumer cannot be assigned one source-backed owner;
2. JSON-v0, interpreter, or compatibility behavior is mixed with native
   canonical behavior in one row;
3. the selected corridor still depends on a string/name fallback;
4. `func` and `callee` both remain authorities in the same route;
5. a proposed guard requires changing MIR, loader, optimizer, backend, or
   printer behavior to make the census green;
6. the source commit changes while the manifest is being issued.

The smallest successful outcome is a complete, pinned census and a named
GUARD-I0 design. It is not a production cutover.
