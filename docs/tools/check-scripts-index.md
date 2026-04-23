# Check Scripts Index (SSOT)

Status: Active  
Scope: `tools/checks/*.sh` の入口を一本化して、用途別に迷わず実行できるようにする。

## Quick Entry

```bash
cd /home/tomoaki/git/hakorune-selfhost
tools/checks/dev_gate.sh quick
```

## Core Gates

| Script | Purpose |
| --- | --- |
| `tools/checks/dev_gate.sh` | 日常ゲートの統合実行（quick/hotpath/portability/milestone）。 |
| `tools/checks/current_state_pointer_guard.sh` | `CURRENT_STATE.toml` をSSOTとして current pointer の mirror drift / stale phase 名を fail-fast で検出する。 |
| `tools/checks/inc_codegen_thin_shim_guard.sh` | `.inc` codegen の raw MIR analysis debt no-growth baseline。削減は許可し、新規/増加を fail-fast で止める。明示された view-owner 領域だけは `tools/checks/inc_codegen_thin_shim_view_allowlist.tsv` で別枠固定する。 |
| `tools/checks/module_registry_hygiene_guard.sh` | `hako.toml` / `nyash.toml` の module registry 境界検証。 |
| `tools/checks/phase29cl_by_name_mainline_guard.sh` | `nyash.plugin.invoke_by_name_i64` の owner 集合を allowlist で固定し、新しい mainline caller を fail-fast で防ぐ。 |
| `tools/checks/ring1_core_scope_guard.sh` | ring1 provider の受理ドメイン境界検証。 |

## Env Hygiene

| Script | Purpose |
| --- | --- |
| `tools/checks/env_dead_accessors_report.sh` | `src/config/env/*.rs` の dead accessor 候補と doc-only 候補をCSVで棚卸し。 |
| `tools/checks/route_env_probe.sh` | emit route 直前の Env / route 表示を確認する。 |
| `tools/checks/route_no_fallback_guard.sh` | 日常 route で fallback/helper トグルが混入していないことを fail-fast で検証する。 |

使い方:

```bash
tools/checks/env_dead_accessors_report.sh
```

出力列:
- `status`: `dead` / `doc-only`
- `module`, `function`: 対象 accessor
- `keys`: 関連ENVキー
- `src_hits`, `tools_hits`, `docs_hits`: 参照件数

## Inventory / Maintenance

| Script | Purpose |
| --- | --- |
| `tools/checks/smoke_inventory_report.sh` | 任意の smoke subtree の過密状態を可視化し、suite-aware coverage summary も出す。既定では `integration/apps` を見て、`archive/lib/tmp/fixtures` は live inventory から除外する。 |
| `tools/checks/windows_wsl_cmd_smoke.sh` | Windows(WSL→CMD) 経路の保守監査。 |
| `tools/checks/macos_portability_guard.sh` | macOS portability の継続監査。 |

## Update Policy

- 新しい `tools/checks/*.sh` を追加したら、この文書へ同コミットで追記する。
- script の役割変更時は `Purpose` を先に更新し、実装差分はその後に載せる。
- 日常導線は `dev_gate.sh` を最優先にし、単発スクリプトは理由があるときだけ直接実行する。
