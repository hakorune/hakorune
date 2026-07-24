# RAW PUBLICATION ADAPTER / PUBLIC INGRESS — design-stop question

Status: **Design stop — PUBLICATION0 is closed; no adapter or ingress implementation is authorized**
Date: 2026-07-24
Predecessor: `cut0-i0-raw-source0-lower-root-post0-publication0-s0-execution-task-2026-07-24.md`
Sunset: `RAW-PUBLICATION-SUNSET-001`

## Evidence

`PUBLICATION0-S0` now publishes a narrow Raw source owner through one shared
live-Builder assignment kernel. The result is typed `Script/App`, retains the
opaque published module, runtime snapshot, route/evidence aggregate, and Raw
verification evidence. The current public API still uses the legacy path:

```text
compile_with_source
  -> compile_legacy
  -> compile_with_source_internal
  -> MirBuilder::build_module
  -> MirCompileResult
```

Other direct production surfaces remain in `runtime/mirbuilder_emit.rs` and
the AST-JSON host provider. The old Raw finalizer is caller-zero but remains
migration scaffolding. No source, manifest, module inventory, or route
authority may be recreated at this boundary.

## Authority / non-authority

```text
authority:
  RawPublishedInvocationV1::{Script, App}
  RawPostprocessEvidenceV1
  Raw publication receipt and opaque published module
  existing public MirCompileResult contract (compatibility target only)

non-authority:
  module symbols for Script/App inference
  AST/source/catalog re-resolution
  old Raw { ledger, root } evidence downgrade
  MirBuilder::build_module as a Raw fallback
  caller-selected retry/fallback policy
  AST-JSON or Program(JSON v0) payload shape changes
```

## Questions to decide

### Q1 — result authority

How should the first public adapter consume `RawPublishedInvocationV1`?

```text
A. One private consuming adapter projects the opaque published module into the
   existing MirCompileResult only at the public boundary. Raw route/evidence
   is validated before projection and verifier Err remains the existing
   reportable Result field.
B. Introduce a new public Raw result and migrate every runner/executor first.
C. Keep the published owner disconnected and postpone any result adapter.
```

The adapter must be the only place allowed to open the published module; no
second module accessor or legacy evidence downgrade may be added.

### Q2 — ingress cutover

Which production entry is opened first?

```text
A. Add one explicit `compile_raw_with_source` entry for the sealed narrow
   Raw grammar; keep `compile_with_source` legacy until measured parity is
   complete.
B. Switch `compile_with_source` directly to Raw and reject every unsupported
   source without a legacy retry.
C. Wire Raw through the runtime/AST-JSON bridge first.
```

The first slice must have one compiler-owned capture policy and must not make
AST-JSON, Program(JSON v0), executor, or public fallback a second authority.

### Q3 — failure mapping

How are typed Raw rejection owners exposed at the public boundary?

```text
A. Map each rejected owner to the existing public error transport only after
   inspection data is sealed; consume/discard the owner with no retry.
B. Return the rejection owner publicly.
C. Convert rejection to a legacy String early and continue through old paths.
```

No failure may re-enter source binding, rebuild a module, downgrade
Selected/NotSelected, or publish a partial module.

### Q4 — route and runtime evidence

Which evidence is retained after public projection?

```text
A. Preserve Script/App route, runtime snapshot, callable-Main disposition,
   RawDrainWitness, parity seals, and reportable verifier evidence in one
   private publication-to-result seal; expose only the compatibility view.
B. Retain only MirCompileResult.module and verification_result.
C. Reconstruct route/runtime facts from the published module.
```

### Q5 — JSON compatibility boundary

Should the first ingress slice touch JSON?

```text
A. No. Keep AST and Program(JSON v0) compatibility lanes unchanged and add
   no JSON-to-Raw authority in this row.
B. Make AST-JSON the first Raw ingress.
C. Make Program(JSON v0) the first Raw ingress.
```

### Q6 — old-path retirement gate

What evidence is required before switching the normal public entry?

```text
A. Raw focused success/failure parity, direct-caller census, one production
   consumer, and an explicit zero-consumer sunset for the old Raw bridge.
B. A compile check only.
C. Delete the old bridge before any Raw public consumer exists.
```

## Recommended narrow next slice

Select a single explicit adapter/ingress pair only after Q1–Q6 are answered.
The first executable row should be:

```text
RAW-SOURCE0-LOWER0-ROOT0-POST0-PUBLICATION-ADAPTER0-S0
```

Its minimum scope is one consuming result adapter plus one focused fixture;
public executor wiring, JSON changes, legacy retirement, and CUT0 remain
zero. The adapter must preserve the opaque-owner and discard-only failure
laws, keep every new source/check file below 800 lines, and carry
`RAW-PUBLICATION-SUNSET-001` to the later retirement row.

## Non-claims at this design stop

```text
public Raw ingress
compile_with_source cutover
MirCompileResult production adapter
AST-JSON / Program(JSON v0) changes
executor or selfhost activation
old Raw bridge deletion
legacy fallback removal
CUT0 activation
```
