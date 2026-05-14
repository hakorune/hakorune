# Compiler cleanup sidecar task breakdown SSOT

Decision: accepted.

This document captures cleanup tasks discovered while the main lane is on the
mimalloc blueprint. These rows are **BoxShape-only** and must not be mixed with
MIMAP BoxCount feature rows.

Current main blocker remains:

```text
MIMAP-012 object-backed lifecycle queue LLVM route pilot
```

## Ground rules

- Do not change accepted language semantics in a cleanup row.
- Do not add new source acceptance shapes in a cleanup row.
- Keep `LoopRange` on the Stage1 route; do not source-desugar range loops.
- Keep VM object-heavy limitations separate from compiler cleanup rows.
- If a cleanup row discovers a semantic blocker, stop and split a new design row.

## Verified observations

As of the latest local inventory:

```text
ASTNode::While references: 74 refs / 47 files
#[allow(dead_code)] references: 129 refs
parse_for_range_stage3: still present
expression_to_json_v0: large single expression lowering function
statement_to_json_v0: still large enough to split after expression cleanup
```

Stage-3 `while` parser output is already normalized to canonical `ASTNode::Loop`.
The remaining `ASTNode::While` vocabulary is legacy/compat residue.

## Recommended order

| Row | Status | Scope | Acceptance |
| --- | --- | --- | --- |
| `CLEAN-WHILE-001` | ready | While deletion readiness inventory. | Finds every `ASTNode::While` constructor/ref and classifies it as parser-output impossible, legacy decode compat, test fixture, or delete target. |
| `CLEAN-WHILE-002` | ready after 001 | Delete `ASTNode::While` variant and direct refs. | Parser `while` still emits `Loop`; legacy JSON/roundtrip paths decode old `While` shape into `Loop`; no `ASTNode::While` refs remain. |
| `CLEAN-LOWER-001` | ready after While cleanup | Split `expression_to_json_v0` by expression family. | Behavior-preserving helpers for literal/var/op/call/member/record/enum/array lanes; no new Program JSON shape. |
| `CLEAN-LOWER-002` | ready after 001 | Split `statement_to_json_v0` by statement family. | Behavior-preserving helpers for local/print/return/if/loop/range/match/check lanes. |
| `CLEAN-FOR-001` | parked | Decide legacy `parse_for_range_stage3` fate. | Either quarantine as legacy syntax with documented retire condition, or remove after compatibility check. |
| `CLEAN-DEAD-001` | parked | Continue `#[allow(dead_code)]` pruning by cluster. | One cluster per commit; keep reason comments for intentional staging residue. |

## CLEAN-WHILE-001 details

Purpose:

```text
prove ASTNode::While has no current canonical producer
```

Must inspect:

- `src/ast/mod.rs`
- parser Stage-3 while route
- Program JSON v0 lowering / roundtrip / compat decode
- AST visitors and rewriters
- test-only direct constructors

Do not delete code in this row unless the inventory itself is tiny and guarded.

## CLEAN-WHILE-002 details

Purpose:

```text
remove the second loop-shaped AST vocabulary
```

Required shape:

- `while` source remains Stage-3 compatibility syntax if still accepted.
- parser output is `ASTNode::Loop`.
- legacy serialized `While` input, if still accepted, normalizes to `Loop` at boundary.
- lowering has one Loop path only.

Forbidden:

- reintroducing `While` as a canonical AST variant
- changing `LoopRange` lowering policy
- changing source-level loop semantics

## CLEAN-LOWER rows

`expression_to_json_v0` and `statement_to_json_v0` should be split only after the
While vocabulary is removed. Otherwise the split preserves duplicate loop arms and
spreads the cleanup across more files.

Suggested helper families:

```text
expression literal/variable
expression unary/binary
expression call/method/static path
expression field/member
expression record literal/update
expression match/enum
statement local/assignment/print/return
statement if/loop/range
statement check/match/try/catch
```

## Return target after docs-only cleanup tasking

After this docs-only task split, return to the VM limitation thread:

```text
VM-LIM-001 object-heavy page queue/facade route
```

Deep-dive focus:

```text
ArrayBox-held InstanceBox identity across push/get
object_key_for Arc ptr dependency
returned user/page object as method receiver
```

This VM investigation remains non-blocking for MIMAP-011+ LLVM/EXE acceptance.
