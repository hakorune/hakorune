---
Status: SSOT
Date: 2026-04-16
Scope: current lane / blocker / next pointer だけを置く薄い mirror。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
---

# Self Current Task — Now (main)

## Current

- current implementation lane: `phase29bq selfhost mirbuilder failure-driven`
- sibling guardrail lane: `phase137x string corridor / exact-keeper guardrail`
- immediate next: `compiler expressivity first`
- immediate follow-on: `phase29bq loop owner seam cleanup`
- top queued cut: `loop_scan_phi_vars_v0::nested_loop_handoff` cleanup / `loop_cond_shared` helper split
- Compiler lane: `phase-29bq`（JIR-PORT-00..08 done / active blocker=`none` / next=`none`）
- JoinIR port mode（lane A）: monitor-only（failure-driven）
- loop owner direction:
  - `facts -> route -> recipe -> cfg skeleton -> join sig -> phi materializer -> verifier -> cleanup`
  - first migrated family seam: `LoopCondReturnInBody` join-sig extraction

## Landing Snapshot

- latest landed:
  - `phase277x`: optimization lane closeout judgment froze the landed optimization roadmap and handed the mainline back to compiler expressivity / selfhost entry
- active:
  - `phase29bq`: failure-driven selfhost mirbuilder lane under compiler-expressivity-first policy
- detail owner:
  - landed history stays in phase docs and roadmap SSOT

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
3. `docs/development/current/main/15-Workstream-Map.md`
4. `docs/development/current/main/design/compiler-expressivity-first-policy.md`
5. `docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md`
6. `docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md`

## Proof Bundle

```bash
git status -sb
cargo test --lib --no-run
cargo check --bin hakorune
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq
```
