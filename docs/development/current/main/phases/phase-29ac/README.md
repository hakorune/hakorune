# Phase 29ac: JoinIR thaw plan (scan_with_init / split_scan near-miss → PASS)

Goal: Convert freeze-fixed scan_with_init / split_scan near-miss cases into PASS while keeping contracts intact (no by-name or silent fallback).

## Status

- P1 ✅ scan_with_init reverse scan: Plan/Normalizer で受理して PASS
- P2 ✅ scan_with_init matchscan: fixture を contract-aligned に修正して PASS
- P3 ✅ split_scan near-miss: contract-aligned な OK fixture として PASS 固定（元の契約違反 fixture は残す）

## Target list (SSOT)

- scan_with_init / split_scan の OK/contract fixture 一覧: `docs/development/current/main/design/pattern6-7-contracts.md`

## Outcome

- PASS/contract fixture の一覧は SSOT に集約（上記参照）。
- Phase29ac では split-scan の fixup OK fixture を追加し、契約違反 fixture は維持。

## Pass order (recommended)

1. **scan_with_init reverse scan**
   - Plan/Normalizer: add reverse support (`i >= 0`, step `i = i - 1`).
   - Smoke: legacy fixture pin token `phase29ab_pattern6_reverse_ok_min` now OK PASS (RC=1).
2. **scan_with_init matchscan missing step**
   - Freeze reason: `[joinir/phase29ab/scan_with_init/contract] scan-with-init contract: missing step update`
   - Resolution: add explicit `i = i + 1` step in fixture (contract-aligned).
3. **split_scan near-miss**
   - Resolve then/else update mismatches without relaxing contracts.
   - Freeze reason: `[joinir/phase29ab/split_scan/contract] split scan contract: else i update must be \`i = i + 1\``
   - Resolution: keep the contract-violation fixture as FAIL-Fast, and add a contract-aligned fixup fixture for PASS.

## Commands

- `bash tools/smokes/v2/profiles/integration/joinir/scan_with_init_regression_pack_vm.sh`
- `bash tools/smokes/v2/profiles/integration/joinir/split_scan_regression_pack_vm.sh`
- `./tools/smokes/v2/run.sh --profile integration --filter "phase29ab_pattern6_*"` (`phase29ab_pattern6_*` = historical fixture pin token family; semantic current entry is `scan_with_init_regression_pack_vm.sh`)
- `./tools/smokes/v2/run.sh --profile integration --filter "phase29ab_pattern7_*"` (`phase29ab_pattern7_*` = historical fixture pin token family; semantic current entry is `split_scan_regression_pack_vm.sh`)
