# Local initializer call route owner (stage 0 rules and overlap measurement)

Date: 2026-11-06. Branch `mir-chain`. Follows S2c stage 3 and S3a stage 4.

## Rules (written before the implementation)

R1. The call kind of a Local initializer site is decided by name resolution
only, in this order of questions, each answered from one inventory:
- the site is in `direct_call_observations` -> **Direct**;
- the site is in `method_calls` and its receiver is `QualifiedUnbound` or
  `CurrentOwner` -> **Static**;
- the receiver is `Lexical(Local(b))` and `b` is a `Local` binding -> **Lexical**;
- the receiver is `Lexical(Local(b))` and `b` is the `Receiver` binding (`me`)
  -> **Receiver**.
The inventories are disjoint (a site is one source node), so one kind is
answered, never several.

R2. The result class of a site is decided by the callee owner, once, by the
kind's own class source (the result cohort of S3a for the callee):
- Direct: callee signature `i64` -> I64; otherwise callee is Map-owned -> Map;
  otherwise callee returns a construction -> Handle.
- Static: the sealed `ExactI64` static-call claim -> I64 (carries the claim).
- Lexical: callee has a `NullableObject` claim -> Nullable; otherwise callee
  returns a construction -> Handle; otherwise callee is a normal exact-i64
  result with no claim -> I64.
- Receiver: the `me` receiver observation's class: `NullableObject` -> Nullable,
  `Object` -> Handle.
A kind never asks another kind's class source.

R3. Install is decided by the result class (one table):
| route | local call result | Home | install |
|---|---|---|---|
| Direct/Static/Lexical I64 | I64 | none | `install_i64_call_result` |
| Direct Map | Map | `Stable(binding)` | `install_map` |
| Direct/Lexical/Receiver Handle | Handle | `Stable(binding)` | `install_received_handle` |
| Lexical/Receiver Nullable | Nullable | `Stable(binding)` | `install_received_nullable` |

R4. The `issue_*` functions no longer ask membership. They check the call
structure (direct observation, literal or sealed arguments) and seal the
arguments. Borrowed/`BorrowedNullableLocalArguments`, `ReceivedHandle`,
`SourceResultArguments` ports stay inside the route's own issuer: they are
requests for argument evidence, not membership.

R5. No route -> a named rejection (`PrefixNotCovered`), as before. No arm is
tried after another arm failed.

## Old arm -> rule row

| old arm (order) | rule |
|---|---|
| 4 `terminal_call` + `issue_local_call(I64)` | R1 Direct, R2 Direct I64 |
| 6 `local_static_call` | R1 Static, R2 Static |
| 8 `local_lexical_nullable_call` | R1 Lexical, R2 Lexical Nullable |
| 9 `local_lexical_i64_call` | R1 Lexical, R2 Lexical I64 |
| 10 `local_map_call` | R1 Direct, R2 Direct Map |
| 11 `local_handle_call` (+ lexical Handle) | R1 Direct/Lexical/Receiver, R2 Handle |
| 12 `local_nullable_call` | R1 Receiver, R2 Receiver Nullable |

Port arms (3 borrowed nullable, 5 static Bool, 7 received producer) answer
argument-evidence requests of their own and stay before the route question.

## Overlap measurement (stage 0)

Method: a temporary probe (not committed) at the start of the call chain asked
all seven membership closures for every Local call initializer and printed the
set that answered yes. Inputs: 330 compiled inputs (229 comparison inputs,
100 `apps/**`, with duplicates between the lists) with the base-equivalent
binary. Cohort: the same probe through the cohort test command (`--nocapture`).

| set that answered | sites | note |
|---|---|---|
| `static` | 72 | one claim |
| `terminal` | 16 | one |
| `handle` | 8 | one |
| `terminal` + `lex_i64` | 4 | below |

Cohort sites (probe printed at every Local call initializer):

| set that answered | sites |
|---|---|
| `static` | 503 |
| `handle` | 168 |
| `terminal` | 97 |
| `lex_i64` | 56 |
| `terminal` + `lex_i64` | 36 + 1 |
| `lex_null` | 21 |
| `map` | 12 |
| `terminal` + `handle` | 9 |
| `terminal` + `lex_null` | 4 |

Every overlap, in the comparison inputs and in the cohort, contains `terminal`
together with exactly one lexical-kind closure (`lex_i64`, `handle` or
`lex_null`). No two of static, lexical, map, handle, nullable and me answered
for the same site except through `terminal`. The overlap sets: a zero-argument lexical call such as `local k = p.get()`
(inputs `x/m7.hako`, `fix/p2.hako`, `place-survey/mx/local__call_result.hako`,
`apps/ownership-typed-null-min`). `terminal_call` answers yes because its
package closure also carries an app-main "zero-argument lexical call" clause
for the Return/print consumers. `issue_local_call` then requires a
`direct_call_observations` row, which a method call does not have, so the arm
returns `None` and the chain falls to `lex_i64`. The accepted result is I64
either way, with the same `install_i64_call_result`. Under R1 the site is
Lexical and R2 gives I64 from the callee's cohort: the same outcome, decided
once. The `terminal_call` closure itself stays for the Return/print consumers
that own that clause.
