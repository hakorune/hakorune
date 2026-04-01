Status: Completed (2025-12-20)
Scope: `StringUtils.index_of_string/2`（dynamic window scan）を JoinIR で受理して `--profile quick` を進める。
Related:
- Now: `docs/development/current/main/10-Now.md`
- Phase 257: `docs/development/current/main/phases/phase-257/README.md`
- Phase 259 (Next): `docs/development/current/main/phases/archive/phase-259/README.md`
- Design goal: `docs/development/current/main/design/join-explicit-cfg-construction.md`

# Phase 258: `StringUtils.index_of_string/2` (dynamic window scan)

## Current Status (SSOT)

- ✅ **P0 完了**（2025-12-21）
- Former first FAIL: `json_lint_vm / StringUtils.index_of_string/2`
- **New first FAIL**: `json_lint_vm / StringUtils.is_integer/1`（Phase 259へ移行）
- Shape summary (from quick log):
  - prelude: `if substr.length()==0 return 0`, `if substr.length()>s.length() return -1`
  - loop: `loop(i <= s.length() - substr.length()) { if s.substring(i, i+substr.length()) == substr return i; i=i+1 }`
  - post: `return -1`
  - caps: `If,Loop,Return`

## Goal

- ✅ `./tools/smokes/v2/run.sh --profile quick` が `StringUtils.index_of_string/2` を突破し、次の FAIL へ進む

## Proposed Approach (P0)

方針: ScanWithInit route（historical label `6`）を “needle length 付き scan” に最小拡張する（構造で解決）

historical label `6` route との差分だけを足す:
- loop cond: `i <= s.length() - substr.length()`（bound 付き）
- window: `substring(i, i + substr.length())`
- needle: `substr`（String）

JoinIR 側は P0 では “毎回 length() を呼んでよい”。まず correctness を固定:
- `needle_len = substr.length()`
- `bound = s.length() - needle_len`
- stop: `i > bound` で k_exit（not-found return）
- match: `s.substring(i, i + needle_len) == substr` で return i
- step: `i = i + 1`

## Tasks (Draft)

1) Fixture + integration smokes
2) ScanWithInit extractor を拡張して `index_of_string/2` の loop 形を accept（Fail-Fast）
3) ScanWithInit lowerer を拡張（dynamic window length / bound）して JoinIR を生成
4) `--verify` / integration / quick を回して SSOT 更新
