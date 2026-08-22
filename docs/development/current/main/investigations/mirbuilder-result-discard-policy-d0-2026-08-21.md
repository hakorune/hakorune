---
Status: Gate 0 census and Gate 1 assignment fix complete; Gate 2 guard selected
Date: 2026-08-21
Priority: Medium-High policy / High assignment consumer
Decision: MIRBUILDER-FALLIBLE-RESULT-DISCARD-POLICY-D0
Parent: docs/development/current/main/investigations/mirbuilder-post-audit-follow-up-queue-2026-08-21.md
NextCard: MIR-RESULT-DISCARD-GUARD-I0
---

# MIRBUILDER-FALLIBLE-RESULT-DISCARD-POLICY-D0

## Six-line brief

Decision: adopt a three-gate policy for fallible-result discards, but scope
the first implementation to MIRBuilder. First classify the existing discards,
then fix assignment publication, then add a narrow structural/lint gate. A
repo-wide deny is not authorized until the workspace scope and intentional
cleanup cases are known.

Source authority + canonical issuer: `MirBuilder::emit_instruction` remains
the sole physical writer; the enclosing function/session discard owner owns
rollback of earlier physical and metadata mutations; assignment's
`variable_ctx.variable_map` remains the sole local publication owner. The
policy issuer is a checked-in guard plus the selected Cargo/Clippy lint, not a
new semantic receipt.

Non-authority: a raw `rg` count, `let _ =` syntax alone, a blanket
`#[allow]`, debug comments, `variable_map` state, a future `EmitReceipt`, or
an individual test does not prove that a physical effect succeeded. Cleanup,
FFI, synchronization, and test teardown need distinct dispositions.

Fail-fast boundary: a fallible physical effect must be propagated before the
corresponding semantic/local publication and before the enclosing function
session commits. For assignment this is immediately before
`variable_map.insert`; any earlier partial mutation is covered only by the
existing session discard contract or a narrowly scoped prepare/commit seam.

Smallest next slice: `MIR-RESULT-DISCARD-CENSUS-D0` reads and classifies
MIRBuilder discards, verifies the available Clippy lint, and names the first
guard scope. It does not change code, Cargo lints, fallback, or physical
emission.

Non-claims: no workspace-wide cleanup, generic `emit_instruction` rewrite,
second physical writer, `EmitReceipt` type, assignment behavior change, A/C,
Recipe/Join, backend, performance, or production switch.

## Local evidence and correction to the proposed count

The current checkout has no `[lints]` or `[workspace.lints]` section in the
root `Cargo.toml`, and workspace members do not opt into a shared lint table.
Rust 1.89/Clippy 0.1.89 recognizes
`clippy::let-underscore-must-use`, but it is `allow` by default and a root
package setting would not automatically enforce every workspace member.

The exact source census on 2026-08-21 was:

```text
exact `let _ =` in Rust, archive excluded: 603
exact `let _ =` under src/mir:            168
exact `let _ =` under src/mir/builder:    109
```

These are syntax counts, not counts of ignored `Result`s. The broader pattern
`let _name = ...` is also common for guards, reserved bytes, and owned values;
it must not be merged into the same defect class. The external “122” count is
therefore not an SSOT fact for this checkout.

## Gate 0 census checkpoint — 2026-08-22

The read-only census was run against the current checkout with archive paths
excluded. The commands and exact counts are:

```text
rg -n --glob '*.rs' --glob '!archive/**' \
  '^[[:space:]]*let[[:space:]]+_[[:space:]]*=' src/mir
  -> 168

rg -n --glob '*.rs' --glob '!archive/**' \
  '^[[:space:]]*let[[:space:]]+_[[:space:]]*=' src/mir/builder
  -> 109

let _<named> = ... under src/mir/builder
  -> 172

let _: ... = ... under src/mir/builder
  -> 4
```

The 109 exact `let _ =` rows are not 109 ignored physical `Result`s. The
confirmed physical/owner classes are:

| Pattern / owner | Count | Classification | Gate |
| --- | ---: | --- | --- |
| `assignment_lowering.rs` `ReleaseStrong` | 1 | physical MIR effect whose error is currently discarded before `variable_map` publication | Gate 1 assignment fix |
| `ring0.io.stderr_write` | 13 | diagnostic/observability IO; not MIR instruction emission | later explicit cleanup/diagnostic policy |
| `ensure_global_ring0_initialized` | 13 | initialization/synchronization owner | owner-specific audit; no blanket allow |
| `std::io::Write::write_all` | 2 | debug/guard artifact output | explicit best-effort disposition |
| cleanup/restore family (`cleanup`, `close_lexical_scope`, `pop_fastmem_region`, restoration receipt, terminal abort) | 7 | session or teardown effect | owner-specific contract |
| remaining exact `let _ =` rows | 73 | tests, fixtures, marker/value drops, propagated `?` expressions, semantic probes, or other non-MIR effects | classify before any lint |

Additional fallible-shape checks found zero named underscore bindings around
`emit_instruction`, zero `drop(emit_instruction(...))`, and one production
standalone `.ok();` statement (`control_flow/plan/features/generic_loop_body/
direct_associated.rs:274`). Four other `.ok();` matches bind an `Option` for
configuration, lookup, or test planning and are not the confirmed physical
discard. The single relevant emission match is:

```text
src/mir/builder/assignment_lowering.rs:97-98
  let _ = self.emit_instruction(MirInstruction::ReleaseStrong { ... });
```

`rustc` is 1.89.0 and `clippy-driver` reports
`clippy::let-underscore-must-use` as available and `allow` by default. No
`[lints]`, `[workspace.lints]`, or matching Clippy setting exists in any of
the 35 workspace manifests; the root `nyash-rust` package is the sole default
member. A workspace-wide deny would therefore both classify unrelated rows
and fail to cover the package topology without an explicit rollout policy.

The enclosing `CanonicalFunctionLoweringSessionV1` has a typed error/cleanup
and context-restore boundary around the lowering operation. This census does
not claim that the boundary rolls back every already-appended MIR instruction,
metadata entry, or type-contract mutation. That proof is the first acceptance
condition of `MIR-ASSIGNMENT-RELEASE-FAILFAST-I0`; until then the one
`ReleaseStrong` discard remains an explicitly open correctness defect.

Gate 0 is therefore complete with no code, Cargo lint, fallback, or
`EmitReceipt` change. The narrow next slice is the assignment consumer, and
only after its failure boundary is proven may the MIRBuilder structural guard
be selected.

## Three gates, in the correct order

### Gate 0 — census and policy boundary

`MIR-RESULT-DISCARD-CENSUS-D0` produces a read-only inventory for
`src/mir/builder` with these categories:

| Class | Example intent | Default policy |
| --- | --- | --- |
| physical effect | `emit_instruction`, file/socket write, flush, release | propagate or named owner handles failure |
| semantic/local publication | map/contract/metadata update after an effect | cannot run before effect success |
| cleanup | temporary file removal, best-effort teardown | explicit intentional-discard marker and reason |
| synchronization/FFI | join, provider setup, foreign call | owner-specific contract; no blanket allow |
| test/fixture | teardown or probe-only effect | test-local reason; never production evidence |
| non-Result underscore binding | guard lifetime, reserved value, owned drop | outside the Result policy |

The census must distinguish exact `let _ = <must-use expression>` from
`let _guard = ...`, `.ok()`, `drop(...)`, ignored `match` arms, and explicit
`if let Err(_)`. It records path, owner, effect class, and whether failure is
observable. It must not rewrite all 109 sites.

### Gate 1 — assignment failure atomicity

The existing High card remains the first concrete consumer:
[`mirbuilder-assignment-release-failure-atomicity-i0-2026-08-21.md`](./mirbuilder-assignment-release-failure-atomicity-i0-2026-08-21.md).

The minimum safe implementation is:

```text
validate declaration and local contract
  -> emit ArrayState/LocalContract effects
  -> emit ReleaseStrong and propagate its Result
  -> publish variable_map only after all required effects succeed
```

The implementation must first verify that the enclosing function/session
discards the preceding instruction, metadata, and type-contract mutations on
failure. If that contract is true, `?` is the smallest fix. If it is false,
introduce a private assignment-only prepare/commit state; do not introduce a
generic transaction API from this card. The proposed `with_assignment` shape
is a design option, not an accepted implementation requirement.

### Gate 2 — narrow machine enforcement

After the census and assignment fix, add a reusable guard for the selected
MIRBuilder scope. The first guard should reject at least:

```text
let _ = self.emit_instruction(...)
let _name = self.emit_instruction(...)        # if the value is only discarded
self.emit_instruction(...).ok()
drop(self.emit_instruction(...))
```

The exact syntax matcher must be implementation-tested; a line-only grep is
not sufficient for multiline calls. A Clippy pilot may enable
`clippy::let-underscore-must-use = "deny"` for the root package, but the
workspace rollout requires an explicit package/CI scope and a classified
exception mechanism. The guard is the canonical MIRBuilder protection because
it can enforce the physical-writer call shape even where type inference or
package lint scope is incomplete.

Intentional cleanup must use a stable, adjacent reason marker or a narrowly
scoped `#[allow(clippy::let-underscore-must-use)]` with a reason that the guard
can verify. A blanket module/file allow is forbidden. No allow may cover
`emit_instruction`, local publication, contract writes, release, or other
physical effects.

## Why an `EmitReceipt` is parked

An `EmitReceipt` with a `Drop` panic is not the first fix. It would change the
hot writer API, complicate error unwinding, and still would not by itself
guarantee that `variable_map` publication is delayed. `Result` propagation
plus the assignment owner/session boundary catches the confirmed bug with a
smaller proof surface. A receipt may be reopened as
`MIR-EMIT-RECEIPT-D2` only after the canonical strict-emission design is
closed; it is not part of the current policy or assignment card.

## Task sequence

1. `MIR-RESULT-DISCARD-CENSUS-D0` — read-only MIRBuilder census, lint pilot,
   owner/failure classification, and proposed guard scope. No code changes.
2. `MIR-ASSIGNMENT-RELEASE-FAILFAST-I0` — propagate `ReleaseStrong`, or add
   the smallest assignment-local prepare/commit seam if session discard is
   insufficient. Add focused success/failure evidence.
3. `MIR-RESULT-DISCARD-GUARD-I0` — implement the narrow guard and the accepted
   Clippy scope/exception convention. Prove no new physical-result discard in
   the selected MIRBuilder production surface.
4. `MIR-EMIT-CANONICAL-STRICTNESS-D0` — separately design strict canonical
   emission versus legacy repair in the one physical writer.
5. `MIR-EMIT-RECEIPT-D2` — parked optional design only; requires evidence that
   lint plus guard plus owner/session contracts still leave a real gap.

## Acceptance for the census design task

Positive:

- the exact current counts and command scope are recorded;
- `let _ =` is separated from underscore-prefixed bindings and intentional
  cleanup;
- `clippy::let-underscore-must-use` availability and package scope are
  confirmed locally;
- `assignment_lowering.rs` is named as a consumer, not hidden in a blanket
  lint allow.

Negative:

- no repo-wide deny is added from an unclassified count;
- no cleanup/FFI/test discard is silently reclassified as a physical failure;
- no `EmitReceipt`, generic transaction, second writer, or fallback is added;
- no local green result is presented as proof of failure atomicity.

Structural:

```text
    assignment ReleaseStrong discard remains explicitly tracked = 1 at Gate 0; Gate 1 closes it
MIRBuilder physical-result discard guard scope                = named
blanket clippy allow over builder                               = 0
second physical writer                                         = 0
workspace-wide lint rollout without package scope             = 0
```

## Gate 1 checkpoint — 2026-08-22

`MIR-ASSIGNMENT-RELEASE-FAILFAST-I0` is implemented and pushed as
`dcd8359f7a`. The existing function-owned session restore contract was
verified with its fallible-checkpoint and primary-plus-cleanup-error tests;
the assignment now propagates `ReleaseStrong` before publishing
`variable_map`. Its focused suite is 6/6 green, and the old ignored physical
result is zero in the selected MIRBuilder surface.

The next bounded row is `MIR-RESULT-DISCARD-GUARD-I0`. It protects only the
three confirmed physical-writer discard shapes and deliberately does not
enable a workspace Clippy deny or classify the remaining census rows.

## Relationship to the current lane

This policy task is queued independently and does not change the current
Script source-window cell. The source-window authority remains the current
implementation frontier. The assignment card may be selected before or after
it only by an explicit pointer change; neither task may bypass the other's
authority boundary.
