---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: `.hako` 側 HostFacade の単一入口契約（I/F, ownership, error, gate）を固定する。
Related:
  - docs/development/current/main/design/hako-fullstack-host-abi-completion-ssot.md
  - docs/reference/abi/nyrt_host_surface_v0.md
  - docs/reference/abi/nyrt_c_abi_v0.md
  - docs/reference/abi/ABI_BOUNDARY_MATRIX.md
---

# Hako HostFacade Contract (SSOT)

## Goal

- `.hako` から host API を呼ぶ経路を 1 箇所に固定する。
- direct host call を禁止し、境界責務を HostFacade に集約する。

## Single Entry (fixed)

Host call は次の Facade 入口のみを許可する。

1. `lang/src/runtime/host/host_facade_box.hako`（新規/統一入口）

禁止:

1. `.hako` runtime/plugin から host ABI を直接呼ぶこと
2. 複数 facade の併存（同責務重複）

## Facade Interface (v0)

`HostFacade.call(kind, selector, payload)` を canonical 形とする。

- `kind`: category discriminator（`loader|process|fs|net|time|runtime`）
- `selector`: method/symbol key（文字列または固定ID）
- `payload`: TLV/JSON bridge payload（borrowed）
- `return`: owned handle/value（失敗は fail-fast）

Ownership:

1. input payload = borrowed
2. returned value/handle = owned

Error contract:

1. strict/dev は fail-fast（silent fallback 禁止）
2. compat route は default-off（明示時のみ）

## Category Mapping Rule

Facade `kind` は `nyrt_host_surface_v0.md` の category へ 1:1 対応させる。

| Facade kind | Host surface category |
| --- | --- |
| `runtime` | Runtime lifecycle/bootstrap + execution/verification |
| `loader` | Host reverse-call bridge (plugin -> host); explicit cold dynamic lane for provider/codegen/box bridge calls |
| `process` | Planned extension（host surface extension policy） |
| `fs` | Planned extension（host surface extension policy） |
| `net` | Planned extension（host surface extension policy） |
| `time` | Planned extension（host surface extension policy） |

## Promotion Gate (Step2 -> Step3)

次へ進む条件を固定する。

1. `HostFacade` 経由で runtime/plugin 側の direct host call が 0 件
2. `tools/checks/dev_gate.sh runtime-exec-zero` green
3. `bash tools/checks/phase29cc_runtime_execution_path_zero_guard.sh` green
4. `bash tools/checks/phase29cc_runtime_v0_abi_slice_guard.sh` green
5. `tools/checks/dev_gate.sh portability` green
6. `bash tools/selfhost_identity_check.sh --mode full --skip-build --bin-stage1 target/selfhost/hakorune.stage1_cli --bin-stage2 target/selfhost/hakorune.stage1_cli.stage2` green

## Change Policy

HostFacade I/F を変更する場合は docs-first で次を同時更新する。

1. `hako-host-facade-contract-ssot.md`（本書）
2. `nyrt_host_surface_v0.md`（category/symbol表）
3. `ABI_BOUNDARY_MATRIX.md`（境界表）
