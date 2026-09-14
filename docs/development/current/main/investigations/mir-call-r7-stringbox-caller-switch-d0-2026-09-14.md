Status: selected__design_stop__2026-09-14
Task: MIR-CALL-R7-STRINGBOX-CALLER-SWITCH-D0
Date: 2026-09-14
Priority: identify the live source caller that still drops the co-sealed artifact and freeze one caller-local successor
Parent: mir-call-r7-stringbox-source-artifact-i0-2026-09-14.md
NextCard: MIR-CALL-R7-STRINGBOX-CALLER-SWITCH-I0
Implementation permission: false until this D0 accepts one real caller, its transport, and its exact deletion tuple
---

# StringBox caller switch D0

## Six-line brief

```text
Decision: select a caller-switch design stop that distinguishes the live source-owning edge from retained body-only Program(JSON) callers; do not delete StringBox emitters yet.
Source authority + canonical issuer: Rust Stage1 AST parse/lower transaction -> Stage1ProgramJsonSourceArtifactV1 -> SourceProgramJsonAuthority; the source helper already consumes this artifact.
Non-authority: body-only JSON, serialized source_anchor without its product, registry first-string scans, method spelling, JSON/MIR indices, legacy boxcall shape, fallback, retry, and reusing an already-switched source helper as new cutover evidence.
Fail-fast boundary: selected source relations must reach one enriched handoff and typed admission before finalized MIR; admission failure is terminal and may not re-enter body-only registry/fallback.
Smallest next slice: prove one real source ingress that still loses the artifact, choose the transport boundary that preserves the co-sealed product, and name the exact outgoing old edge plus its retained compatibility scope.
Non-claims: no Hako caller switch, no global _emit_* deletion, no indexOf migration, no body-only parse removal, no shared LegacyCallV0 retirement, no backend parity, and no whole-R7 caller-zero.
```

## Bounded census

The design boundary is one source ingress through the selected caller, the
source-artifact handoff, the existing bridge/admission terminal, and the
retained compatibility exits. It includes the direct/New StringBox
`length|size` cohort and both places that can currently receive body-only
Program(JSON). It excludes unrelated registry patterns, `indexOf` migration,
direct MIR schema readers, backend work, and shared schema retirement.

| Source/input state | Artifact disposition | Caller-switch disposition |
| --- | --- | --- |
| direct `Str`/`String` literal receiver, `length` or `size`, zero method args | one source relation and one lowering-issued anchor | selected source ingress must pass the enriched handoff and publish once |
| `New(StringBox)` with one direct string constructor arg, no field initializers/type args, `length` or `size`, zero method args | one source relation and one anchor | same selected handoff and terminal |
| empty direct string literal | selected and valid | same as the direct literal row |
| `indexOf`, extra constructor/method args, embedded/opaque/non-literal receivers, field/type arguments | no selected artifact; existing compatibility or terminal contract | retain compatibility; do not reinterpret absence as a migration candidate |
| helper-method occurrence | source collector disabled for helper defs | out of this source product boundary |
| normal source with no selected relation | product/crosswalk/receipt cardinality zero is valid | no candidate is not an error and must not trigger fallback or retry |
| malformed, missing, or unrecognized source relation | issuer or admission returns a named error | terminal before publication; no body-only re-entry |
| body-only Program(JSON) input | `Stage1ProgramJsonModuleHandoff::parse(&str)` remains body-only | retain explicit Hako compatibility callers |

## Current caller inventory

The Rust source path is already artifact-aware: `source_to_mir_json` calls
`SourceProgramJsonAuthority::for_source`, and the Hako-named
`MirBuilderBox.emit_from_source_v0` maps to the existing
`nyash.stage1.emit_mir_from_source_v0_h` extern route. Reusing that route is
not a new caller switch.

The actual unswitched candidate is the Hako source-compat seam in
`lang/src/mir/builder/MirBuilderBox.hako`: it calls
`BuildBox.emit_program_json_v0` and then passes body-only text to
`MirBuilderBox.emit_from_program_json_v0`. The D0 must first prove that this
edge is live for the selected runtime entry. The direct Program(JSON) entry
(`lang/src/mir/builder/compat/program_json_v0_entry.hako`) and the explicit
`emit-mir-program` compatibility paths also receive body-only text and remain
supported unless a later card names a separate product transport.

## Design tasks

1. **Prove the live caller.** Trace one selected source invocation from its
   public entry through module/extern dispatch to either the already-artifact
   aware Rust helper or the Hako `BuildBox -> emit_from_program_json_v0`
   edge. Record the actual terminal and distinguish a live edge from a shape
   fixture or a route that is already switched.
2. **Freeze the transport boundary.** If the live caller is already the Rust
   source helper, record the row as already cut over and select the next real
   body-only ingress. If Hako still owns the live source call, pass ownership
   to the Rust source transaction before Program(JSON) text is materialized.
   Do not reconstruct the product from JSON, put it in an environment/global
   side channel, or serialize a semantic receipt without its source product.
3. **Settle selected membership and compatibility.** Preserve the exact two
   selected `length|size` rows, valid empty literals, and zero-relation
   success. Keep `indexOf`, extra/opaque shapes, helper defs, and body-only
   Program(JSON) on their existing compatibility or terminal contracts.
4. **Name the caller-local delete tuple.** A successor may remove only the
   source-to-body-only edge it replaces. The later I0 may delete the selected
   direct/New `length|size` dispatch edges and helpers
   `_emit_length_mir`, `_emit_size_mir`, and
   `_emit_new_stringbox_boxcall0` only when no retained body-only caller still
   needs them. Keep `_emit_new_stringbox_boxcall1_string`, `indexOf`, the
   body-only `parse`, and unrelated lowerers until their own owner closes.
5. **Define the executable I0.** Choose one source caller, one successor
   handoff, one successful finalized terminal, one negative terminal, and the
   focused local guards. CI is follow-up evidence, not a prerequisite for
   accepting this design.

## D0 acceptance

```text
one live source caller and its exact source-to-terminal route are named
the artifact transport keeps Program-v0, source product, and crosswalk co-sealed
selected direct/New length|size membership and zero-relation behavior are fixed
indexOf, unsupported shapes, helper defs, and body-only Program(JSON) retention are explicit
the exact old-edge delete tuple is caller-local and no global emitter deletion is claimed
the successor I0 has one positive, one negative, and one structural re-entry guard
README/reference/pointer updates can be made in the same bounded series
```

## Non-claims and evidence boundary

The source-anchor I0 at commit `6adbfa8abb` proves the Rust artifact, bridge
receipt, and exact-once admission for its focused positive and missing-anchor
cases. It does not prove that the Hako source-compat seam is a live caller or
that either Hako registry/fallback owner can receive the artifact. Local source
helper green is therefore not caller-cutover evidence.

The read-only worker audit on 2026-09-14 confirmed the existing source helper
route, the Hako body-only edge, and the need for a caller-local deletion tuple.
No code, fixture, Cargo run, fallback, or production switch is authorized by
this card. Until D0 acceptance is recorded, `work_mode` remains `design_stop`
and `next_execution_card` remains none.
