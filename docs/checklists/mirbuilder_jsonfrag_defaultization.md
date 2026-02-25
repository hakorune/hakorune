# MirBuilder JsonFrag Defaultization Checklist

Purpose: define clear, testable conditions to move the JsonFrag-based MirBuilder path from opt-in to default without changing observable behavior.

Scope
- Loop lowers (simple / sum_bc / count_param) via `loop_opts_adapter.build2`.
- Normalizer seam: `hako.mir.builder.internal.jsonfrag_normalizer`.

Toggles (dev-only, default OFF)
- `HAKO_MIR_BUILDER_LOOP_JSONFRAG=1` — enable JsonFrag minimal MIR assembly.
- `HAKO_MIR_BUILDER_JSONFRAG_NORMALIZE=1` — apply normalization pass (phi/ret/const; incremental).
- `HAKO_MIR_BUILDER_SKIP_LOOPS=1` — canary guard to bypass loop lowers (safety valve; must remain honored).
- `HAKO_MIR_BUILDER_NORMALIZE_TAG=1` — emit normalization tag lines; default is quiet (no tag).

Acceptance Criteria
- Green on quick representative smokes and phase2231 canary.
- Tag observability present only when opt-in flags are set:
  - `[mirbuilder/internal/loop:jsonfrag]` when JsonFrag path is taken.
  - `[mirbuilder/normalize:jsonfrag:pass]` when normalization is applied.
- Parity: JsonFrag+Normalizer output is semantically identical to the default path (no diff in verification runners, exit code parity).
- Rollback: removing toggles restores legacy path immediately with zero residual side effects.

Verification Steps
1) Enable JsonFrag path for loop lowers and run quick smokes:
   - `HAKO_MIR_BUILDER_LOOP_JSONFRAG=1 tools/smokes/v2/run.sh --profile quick`
2) Enable Normalizer additionally and re-run:
   - `HAKO_MIR_BUILDER_LOOP_JSONFRAG=1 HAKO_MIR_BUILDER_JSONFRAG_NORMALIZE=1 tools/smokes/v2/run.sh --profile quick`
3) Observe tags in logs (only when toggles ON), confirm absence when OFF.
4) Use `tools/smokes/v2/lib/mir_canary.sh` helpers to extract MIR and assert key tokens as needed.
5) Heavy EXE/AOT reps present: consider `--timeout 120` for `--profile quick` when NORMALIZE=1.

Rollback Plan
- Disable toggles to revert (`export` unset or set to `0`). No code removal required.
- If unexpected diffs appear, capture `[mirbuilder/*]` tags from logs and attach to CURRENT_TASK.md for follow-up.

Notes
- Normalizer is introduced as a pass-through seam first; refine in small, guarded steps (phi alignment, ret normalization, const folding) while keeping default OFF.
- Do not change default behavior or widen scope during this phase; prioritize stability and diagnostics.
- f64 canonicalization is shared via `selfhost.shared.json.utils.json_number_canonical`; prefer reusing this utility instead of local string hacking.
- Dev helpers: `enable_mirbuilder_dev_env` can inject NORMALIZE via `SMOKES_DEV_NORMALIZE=1` (profile-based injection example provided in comments).
