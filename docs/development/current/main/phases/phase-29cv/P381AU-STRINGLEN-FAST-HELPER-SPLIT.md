---
Status: Done
Decision: accepted
Date: 2026-05-05
Scope: phase-29cv StringLen fast-path split that keeps `nyash.string.len_h` as the truthful correctness helper and introduces `nyash.string.len_fast_h` as a separate direct route
Related:
  - docs/development/current/main/phases/phase-29cv/P381AR-INC-BOUNDARY-TRUTH-AND-RUNTIME-DECL-ATTR-AUDIT.md
  - docs/development/current/main/phases/phase-29cv/P381AS-LOWERING-PLAN-TIER-VOCAB-REIFY.md
  - docs/development/current/main/phases/phase-29cv/P381AT-UNIFORM-MULTI-FUNCTION-EMITTER-GAP-PLAN.md
  - docs/development/current/main/design/runtime-decl-manifest-v0.toml
  - lang/src/runtime/meta/core_method_contract_box.hako
  - crates/nyash_kernel/src/exports/string.rs
  - crates/nyash_kernel/src/exports/string_helpers.rs
---

# P381AU: StringLen Fast Helper Split

## Problem

The current repo truth after P381AR is:

- `nyash.string.len_h` must stay the truthful general helper
- it still carries dispatch / observe / cache / trace behavior
- `HotInline` exists only as MIR vocabulary, not as a landed StringLen lowering

That left a gap:

- direct StringLen routes had no separate fast helper
- but tightening `len_h` attrs would have lied about current behavior

## Decision

Introduce a separate **fast helper route** instead of tightening `len_h`.

Implemented:

- new runtime export: `nyash.string.len_fast_h`
- direct StringLen route surfaces now point to `len_fast_h`
- existing `nyash.string.len_h` stays intact as the general/correctness helper

Current `len_fast_h` contract is intentionally modest:

- it first tries a direct `TextReadSession.str_handle(...).len()` read
- if that cannot serve the handle, it falls back to existing `len_h` behavior
- it does **not** seed the length cache itself

## Attr truth

Do **not** mark `len_fast_h` as `readonly` / `memory=read` yet.

Why:

- `TextReadSession` still records perf-observe counters on read
- fallback still reaches existing `len_h`
- so this slice is about helper separation and route cleanup, not about proving a
  pure leaf

For now both helpers stay truthful:

```toml
attrs = ["nounwind"]
memory = "readwrite"
```

## Boundary

Allowed:

- direct StringLen routes and helper-name vocab switch from `len_h` to
  `len_fast_h`
- `len_h` remains available as the existing general helper

Not allowed:

- claiming `HotInline` is landed for StringLen
- tightening attrs without removing perf-observe/fallback side effects
- moving StringLen legality or body semantics into `.inc`

## Acceptance

```bash
cargo fmt --all
python3 tools/core_method_contract_manifest_codegen.py --check
python3 tools/backend_runtime_decl_manifest_codegen.py --check
cargo test --release string_len_fast_export_reads_string_without_seeding_len_cache -- --nocapture
cargo test --release string_routes -- --nocapture
cargo test --release core_method_op -- --nocapture
cargo test --release string_corridor_names -- --nocapture
```

## Result

Done:

- split StringLen into a direct fast helper route and the original correctness
  helper
- updated MIR/helper-name/runtime-decl/selfhost call-policy surfaces to the new
  direct route
- kept attrs honest instead of pretending the fast helper is already pure

Next:

1. return to the selected-set uniform multi-function emitter work
2. revisit StringLen attr tightening only after a genuinely pure handle-read seam
   exists
3. consider StringLen `HotInline` only after proof-bearing layout/consumer
   plumbing exists
