
# Nyash Syntax Torture (10 minimal repros)
Date: 2025-09-16

Purpose: stress parser → AST → MIR(Core-13/PURE) → Interpreter/VM/LLVM(AOT) consistency.
Each test is **one phenomenon per file**, tiny and deterministic.

## How to run (suggested)
```
# 1) Run all modes and compare outputs
bash run_spec_smoke.sh

# 2) PURE mode (surface MIR violations):
NYASH_MIR_CORE13_PURE=1 bash run_spec_smoke.sh

# 3) Extra logging when a case fails:
NYASH_VM_STATS=1 NYASH_VM_STATS_JSON=1 NYASH_VM_DEBUG_BOXCALL=1 bash run_spec_smoke.sh
# For LLVM diagnostics (when applicable):
# NYASH_LLVM_VINVOKE_TRACE=1 NYASH_LLVM_VINVOKE_PREFER_I64=1 bash run_spec_smoke.sh
```

## Expected outputs (goldens)
We deliberately **print a single line** per test to make diffing trivial.
See inline comments in each `*.hako`.

## File list
1. 01_ops_assoc.hako – operator associativity & coercion order  
2. 02_deep_parens.hako – deep parentheses & arithmetic nesting  
3. 03_array_map_nested.hako – nested array/map literal & access  
4. 04_map_array_mix.hako – object/array cross indexing & updates  
5. 05_string_concat_unicode.hako – string/number/Unicode concatenation  
6. 06_control_flow_loopform.hako – break/continue/dispatch shape  
7. 07_await_nowait_mix.hako – nowait/await interleave determinism  
8. 08_visibility_access.hako – private/public & override routing  
9. 09_lambda_closure_scope.hako – closure capture & shadowing  
10. 10_match_result_early_return.hako – early return vs. branch merge

## CI hint
- Add this suite **before** your self-host smokes:
  - `make spec-smoke` -> `make smoke-selfhost`
- Fail fast on any diff across Interpreter/VM/LLVM.
