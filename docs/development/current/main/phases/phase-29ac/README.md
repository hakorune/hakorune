# Phase 29ac: JoinIR thaw plan (Pattern6/7 near-miss → PASS)

Goal: Convert freeze-fixed Pattern6/7 near-miss cases into PASS while keeping contracts intact (no by-name or silent fallback).

## Status

- P1 ✅ Pattern6 reverse scan: Plan/Normalizer で受理して PASS
- P2 ✅ Pattern6 matchscan: fixture を contract-aligned に修正して PASS
- P3 ✅ Pattern7 split-scan: near-miss を contract-aligned な OK fixture として PASS 固定（元の契約違反 fixture は残す）

## Target list (SSOT)

- Pattern6/7 の OK/contract fixtures 一覧: `docs/development/current/main/design/pattern6-7-contracts.md`

## Outcome

- PASS/contract fixture の一覧は SSOT に集約（上記参照）。
- Phase29ac では split-scan の fixup OK fixture を追加し、契約違反 fixture は維持。

## Pass order (recommended)

1. **Pattern6 reverse scan**
   - Plan/Normalizer: add reverse support (`i >= 0`, step `i = i - 1`).
   - Smoke: `phase29ab_pattern6_reverse_ok_min` now OK PASS (RC=1).
2. **Pattern6 matchscan missing step**
   - Freeze reason: `[joinir/phase29ab/scan_with_init/contract] scan-with-init contract: missing step update`
   - Resolution: add explicit `i = i + 1` step in fixture (contract-aligned).
3. **Pattern7 split-scan near-miss**
   - Resolve then/else update mismatches without relaxing contracts.
   - Freeze reason: `[joinir/phase29ab/split_scan/contract] split scan contract: else i update must be \`i = i + 1\``
   - Resolution: keep the contract-violation fixture as FAIL-Fast, and add a contract-aligned fixup fixture for PASS.

## Commands

- `./tools/smokes/v2/run.sh --profile integration --filter "phase29ab_pattern6_*"`
- `./tools/smokes/v2/run.sh --profile integration --filter "phase29ab_pattern7_*"`
- `./tools/smokes/v2/run.sh --profile integration --filter "phase29ab_pattern7_*"`
