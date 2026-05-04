---
Status: Accepted
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381F, Stage1 env/debug flag source cleanup
Related:
  - docs/development/current/main/phases/phase-29cv/P381E-PROGRAM-JSON-TEXT-GUARD-SCALAR-SPLIT.md
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - lang/src/runner/stage1_cli_env.hako
  - lang/src/runner/stage1_cli_env/input_contract.hako
  - lang/src/runner/stage1_cli_env/emit_pipeline.hako
---

# P381F: Stage1 Short-Circuit Nested Guard

## Follow-up

P381F is a temporary Stage1 source-surface cleanup. It does not revoke language
support for `&&` / `||`.

Durable compiler-owned handling continues in
`docs/development/current/main/phases/phase-29cv/P381J-BOOLEAN-SHORT-CIRCUIT-PHI-LOWERING.md`.

## Problem

P381E removed the `String|Void` shape blockers. The phase29cg replay then reached
LLVM IR emission and failed first in `Stage1InputContractBox._env_flag_enabled/1`:

```text
opt: /tmp/p381e_bad.ll:257:19: error: '%r8' defined with type 'i1' but expected 'i64'
  %r9 = phi i64 [ %r8, %bb1 ], [ 0, %bb2 ]
                  ^
```

The first source shape was a short-circuit boolean expression:

```text
flag != null && me._coerce_text_compat(flag) == "1"
```

After rewriting that check, the same type pattern appeared in
`Stage1InputContractBox.clean_env_value/1` from chained `||` comparisons. Current
Stage0 lowering emits these short-circuit PHIs as `i64`, while compare producers
are `i1`. Teaching Stage0 another special boolean repair here would increase
backend cleanup debt.

## Decision

Rewrite Stage1 env cluster short-circuit checks as nested guards:

```text
if flag != null {
  if me._coerce_text_compat(flag) == "1" { return 1 }
}
return 0
```

This keeps the same behavior, removes the boolean short-circuit PHI from the
Stage1 source surface, and avoids C shim/type-fix widening.

## Non-Goals

- no C shim boolean PHI repair
- no new `GlobalCallTargetShape`
- no behavior change to `STAGE1_CLI_DEBUG` / mode aliases / input guards
- no VM fallback

## Acceptance

```bash
KEEP_OUT_DIR=1 OUT_DIR=/tmp/p381f_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected: the replay moves beyond the short-circuit bool PHI type errors.

## Result

Accepted.

`rg -n "\|\||&&"` against the Stage1 env cluster files returns no remaining
short-circuit expressions. The phase29cg replay moves beyond the short-circuit
bool PHI type errors and reaches the next backend hygiene blocker:

```text
opt: /tmp/p381f_bad.ll:1015:3: error: multiple definition of local value named 'print_call_2'
  %print_call_2 = call i64 @"nyash.console.log_handle"(i64 %r2)
```

Next card: fix the repeated print-call SSA temporary name without adding Stage0
body semantics or a boolean PHI repair.
