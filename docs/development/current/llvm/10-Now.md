# Self Current Task — Now (llvm)

2025‑09‑08：現状と直近タスク
- P0 完了（BitOps/Array/Echo/Map 緑）。
- VInvoke(by‑name/by‑id vector) で戻り値マッピングの課題（ret_tag=3 でも 0 になる）。

直近タスク
1) VInvoke 戻り値の短期特例：既知メソッド（例: MapBox.get）の整数返りは i64 として保持。
2) by‑id vector（`nyash.plugin.invoke_tagged_v_i64`）も同様に統一。
3) 受け入れ：
   - `NYASH_LLVM_VINVOKE_SMOKE=1` → `VInvokeRc: 42`
   - `NYASH_LLVM_VINVOKE_RET_SMOKE=1` → `Result: 42`

旧LLVM手動smokeは退役済み。上記は2025年時点の調査記録です。現行の受け入れ・作業は `../main/CURRENT_STATE.toml` と選択app cardを参照してください。
