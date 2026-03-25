---
Status: SSOT
Decision: accepted
Date: 2026-03-14
Scope: 脱Rustタスクを lane A/B/C で固定し、担当境界と導線の混線を防ぐ。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md
  - docs/development/current/main/design/de-rust-full-rust-zero-roadmap-ssot.md
  - docs/development/current/main/design/de-rust-master-task-map-ssot.md
  - docs/development/current/main/design/de-rust-scope-decision-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-94-derust-non-plugin-done-sync-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-95-plugin-lane-bootstrap-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-96-plugin-abi-loader-acceptance-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-97-plugin-gate-pack-lock-ssot.md
  - docs/development/current/main/design/de-rust-compiler-thin-rust-roadmap-ssot.md
  - docs/development/current/main/design/hako-runtime-c-abi-cutover-order-ssot.md
  - docs/development/current/main/design/joinir-port-task-pack-ssot.md
  - docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-62-derust-done-sync-ssot.md
  - docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md
  - docs/development/current/main/phases/phase-29cf/README.md
  - docs/development/current/main/phases/phase-29y/80-RUST-VM-FEATURE-AUDIT-AND-HAKO-PORT-SSOT.md
  - docs/development/current/main/phases/phase-29y/81-RUST-VM-TO-HAKO-VM-FEATURE-MATRIX.md
  - docs/development/current/main/phases/phase-29bq/29bq-116-emit-mir-entry-order-blocker.md
---

# De-Rust Lane Map (SSOT)

## Purpose

- 「脱Rust」の対象を 1つに見せず、A/B/C の lane に分離して運用する。
- どの blocker がどの lane かを 30 秒で判定できる入口を固定する。
- 全体の完了順序（lane 以外の closeout 含む）は `de-rust-master-task-map-ssot.md` を正本とする。

## Lane Definition (fixed)

| lane | name | scope | primary SSOT | current status |
| --- | --- | --- | --- | --- |
| A | Compiler Meaning | JoinIR / Planner / CorePlan の受理・意味決定 | `de-rust-compiler-thin-rust-roadmap-ssot.md` + JoinIR gate SSOT | active（proactive, blocker=`JIR-PORT-08`） |
| B | Compiler Pipeline | `.hako` parser / mirbuilder / Stage1 compiler 導線 | `selfhost-parser-mirbuilder-migration-order-ssot.md` | active（monitor-only, blocker=`none`; latest reopen/fix=`29bq-116 emit-mir-json entry order`） |
| C | Runtime Port | Rust VM 依存機能の `.hako VM` 置換（RVP） | `phase-29y/60-NEXT-TASK-PLAN.md` + `phase-29y/81-RUST-VM-TO-HAKO-VM-FEATURE-MATRIX.md` | parked（no current blocker; reopen only if a new exact vm-hako blocker appears） |

## Scope Boundary (must keep)

- lane A は「意味決定」の修正を扱う。
- lane B は「Program/MIR 生成導線」の修正を扱う。
- lane C は「実行能力（vm-hako capability）」の修正を扱う。
- lane C の文書（`phase-29y/60`）は A/B の next 順序を上書きしない。

## Runtime Operation Policy (LLVM-first)

- runtime 実行系の主経路は LLVM（`--backend llvm`）とする。
- parent execution-lane vocabulary is `execution-lanes-and-axis-separation-ssot.md`.
- in that vocabulary, the operational default is `llvm-exe`, `vm-hako` is the reference/debug/bootstrap-proof lane, and `rust-vm` is the bootstrap/recovery/compat lane.
- lane C（`vm-hako`）は既定で monitor-only とし、fixed backlog は置かない。
- lane C の active acceptance は `phase29y_vm_hako_caps_gate_vm.sh` だけに固定し、archived throughput/probe smokes は monitor evidence として読む。
- lane C で修正に入る条件は次のいずれか:
  1. `phase29y_vm_hako_caps_gate_vm.sh` が FAIL したとき。
  2. feature matrix で row が `blocked` に戻ったとき。
  3. no-compat mainline 契約が runtime 差分で崩れたとき。
- 上記に該当しない限り、runtime 実装の優先順位は `LLVM first -> vm-hako parity monitor` を維持する。

## Full Rust 0 Tracking Split (non-blocking)

- top-level future pointer:
  - `docs/development/current/main/design/de-rust-full-rust-zero-roadmap-ssot.md`
- runtime-zero:
  - `accepted pointer / inventory-ready`
  - primary docs は `de-rust-post-g1-runtime-plan-ssot.md` / `29cc-220` / `29cc-253`
  - ただし lane C daily は引き続き `LLVM-first / vm-hako monitor-only`
- backend-zero:
  - `accepted pointer / phase-29ck queued`
  - primary docs は `de-rust-backend-zero-boundary-lock-ssot.md` / `de-rust-backend-zero-provisional-inventory-ssot.md` / `phase-29ck/README.md`
  - final shape is `.hako -> thin backend C ABI/plugin boundary -> object/exe`
  - `native_driver.rs` は bootstrap seam only
  - A/B/C の current blocker にはまだ入れない
- rule:
  - この split は future visibility だけを扱い、daily triage と reopen 条件は lane A/B/C の既存 SSOT を維持する。

## Triage Rule (tag -> lane)

- `[joinir/freeze]` / `[plan/freeze:*]` / `[plan/reject]` は lane A。
- `[freeze:contract][hako_mirbuilder]` は lane B。
- `[vm-hako/unimplemented]` / `subset-check` / `--hako-run` timeout は lane C。

## Daily Entry Order

1. `de-rust-master-task-map-ssot.md` を開く（全体順序確認）
2. lane map を開く（この文書）
3. `CURRENT_TASK.md` の blocker を確認
4. 該当 lane の SSOT へ移動して 1タスクだけ実行

## Done Criteria (summary)

- lane A: JoinIR fast gate と planner-required gate が緑で blocker が `none`。
- lane B: Stage1-first identity と `.hako` mirbuilder pin が緑で導線契約が固定。
- lane C: feature matrix の `blocked` が順に `ported` へ進み、lane gate が緑。
- de-rust done scope（non-plugin done / plugin separate lane）は
  `de-rust-scope-decision-ssot.md` を正本とする。
- non-plugin done 宣言の証跡は
  `phase-29cc/29cc-94-derust-non-plugin-done-sync-ssot.md` を正本とする。
- plugin separate lane の準備順序は
  `phase-29cc/29cc-95-plugin-lane-bootstrap-ssot.md` を正本とする。
- plugin separate lane の ABI/loader acceptance（PLG-01）は
  `phase-29cc/29cc-96-plugin-abi-loader-acceptance-lock-ssot.md` を正本とする。
- plugin separate lane の gate pack lock（PLG-02）は
  `phase-29cc/29cc-97-plugin-gate-pack-lock-ssot.md` を正本とする。

## De-Rust Done Handshake (X32-X35)

- lane gate quick/full の緑は日常運用の健康診断であり、de-rust transfer lane の done 宣言とは別契約。
- de-rust done 判定は `docs/development/current/main/phases/phase-29x/29x-62-derust-done-sync-ssot.md` の
  `Done Criteria × Gate Coverage Matrix`（X32/X33/X34/X35）を正本とする。
- したがって done 宣言時は次の 4 smoke replay を必須とする。
  1. `phase29x_derust_route_dualrun_vm.sh`
  2. `phase29x_derust_verifier_vm.sh`
  3. `phase29x_derust_safety_vm.sh`
  4. `phase29x_derust_strict_default_route_vm.sh`
- `PHASE29Y_DERUST_DONE_MATRIX_CHECK=1` は quick 末尾で matrix を補助実行するための診断導線であり、
  daily quick の既定セットには含めない。

## `.hako`-only Focus Criteria (switch gate)

- 目的:
  - 「普段は `.hako` 側だけを見て進める」運用へ切り替える前提条件を固定する。
- 条件（全て満たす）:
  1. lane B: `--emit-mir-json` と `--hako-emit-mir-json` の差分が対象 fixture 群で 0。
  2. lane C: current runtime blocker が feature matrix で `ported` へ昇格済み。
  3. no-compat mainline が緑（明示 fallback なし）。
- 条件未達時:
  - `.hako` 単独デバッグへ寄せず、`60-NEXT-TASK-PLAN.md` の Debug Procedure Lock で lane を確定してから修正する。

## Remaining Tasks Snapshot (2026-03-09)

- lane A:
  - done range は `JIR-PORT-00..07`（詳細は `joinir-port-task-pack-ssot.md` の Current State）。
  - current blocker は `JIR-PORT-08`（normalizer BlockExpr with prelude is not supported in value context）。
  - next は `none`（tail active）。
  - status mirror SSOT は `CURRENT_TASK.md`（この文書は mirror）。
  - JoinIR 移植の fixed order は `joinir-port-task-pack-ssot.md`（JIR-PORT-00..07）を正本とする。
- lane B:
  - 固定順序は `selfhost-parser-mirbuilder-migration-order-ssot.md`（mirbuilder先行 / parser後行）。
  - current blocker: `none`（binary-only emit route は ported、lane B は monitor-only）
  - latest fixed blockers:
    - `29bq-116`: `--emit-mir-json` now serializes `main` before helper functions
    - `29bq-117`: llvmlite harness now accepts `ArrayBox.birth()` on the fast EXE entry-prologue path
  - 実運用は `29bq-90-selfhost-checklist.md` の daily/milestone checklist を回し、PROBE->FIX->PROMOTE で継続する。
  - known parity debt（non-gating）: expression lowering（nested ternary family）は Rust/.hako route の canonical compare で監視し、Rust-only green を観測した時点で blocker を再起動する。
  - monitor probe: `phase29y_hako_emit_mir_nested_ternary_debt_probe_vm.sh`（strict check: `STRICT=1`）
  - non-gating blocker pin: `phase29y_hako_emit_mir_continue_assignment_timeout_block_vm.sh`（`--hako-emit-mir-json` internal timeout fail-fast marker）
  - non-gating monitor pin: `phase29y_hako_emit_mir_binary_only_ported_vm.sh`（repo外 `--hako-emit-mir-json` ported contract）
  - non-gating monitor pin: `phase29y_hako_emit_mir_preemit_io_monitor_vm.sh`（pre-emit I/O cold/hot 観測。`--strict` は手動 triage 専用）
  - current focus: `binary-only --hako-emit-mir-json`（SSOT: `selfhost-bootstrap-route-ssot.md` の Binary-only contract）
- lane C:
  - `RVP-C16..RVP-C28` まで `ported` 昇格済み。
  - parked blocker は `none (parked; reopen only if a new exact vm-hako blocker appears)`。
  - quick map collection smokes are no longer blocked in `.hako VM` for the current MapBox sweep.
- orchestration / aftercare:
  - `phase-29cc`: accepted monitor-only（top-level de-rust selfhost closeout done）
  - `phase-29ce`: accepted（live compat retirement closeout）
  - `phase-29cf`: accepted monitor-only（`VM fallback compat lane` / `bootstrap boundary reduction` follow-up）
