# AI Plan Review Checklist (SSOT)

Purpose: reduce “LLM plan drift” by making wrong plans mechanically rejectable, and by forcing Facts→Recipe→Parts (recipe-first) work to stay on a single, verifiable rail.

Scope:
- JoinIR / planner-required work
- `src/mir/builder/control_flow/plan/**` (Facts / Recipe / Parts / features)
- selfhost canary → blocker capture → BoxCount loops

Non-goals:
- This is not a design doc for new language features.
- This is not a substitute for fixture+gate pinning.

---

## 0) Required Attachments (plans without these are rejected)

Every plan must include:
- Baseline: `git status -sb` output + current branch + HEAD commit hash.
- Repro command: the *exact* command that fails (copy/paste-ready).
- The first failure line (SSOT):
  - “first freeze/reject” 1 line (e.g. `[freeze:contract]...` / `[plan/reject:*]...`).
  - StepTree root 1 line (function name + root summary).
  - The log path fixed to `/tmp/...` if applicable.
- When working under `planner_required`:
  - the environment switch used (exact env var names + values).
  - whether the failure reproduces in `--release` or only in `dev`.

Recommended capture workflow:
- Use the existing capture script when available:
  - `./tools/smokes/v2/profiles/integration/joinir/phase29bq_collect_planner_required_blocker_vm.sh <fixture> <label>`

---

## 1) The Plan Must Declare BoxCount vs BoxShape (explicitly)

### BoxCount (acceptance vocabulary increases)
Definition:
- Adds exactly one new accepted shape (one “lego piece”).

Hard rules (reject plan if violated):
- 1 blocker = 1 accepted shape = 1 fixture = 1 gate line = 1 commit.
- No “and also” work in the same commit (cleanup, parity, refactors, extra patterns).
- No silent fallback: non-matching shapes must remain `Ok(None)` or fail-fast with a contract tag.
- Facts that return `Some(...)` must be lowerable by the corresponding lowerer (Facts→Recipe→Parts contract).

### BoxShape (structure, no acceptance expansion)
Definition:
- Improves structure/SSOT/diagnostics *without* accepting new shapes.

Hard rules (reject plan if violated):
- Must not add new accepted shapes (no new planner rules/pattern routes).
- Must not “fix” by adding ad-hoc if/else special cases in random places.
- Changes must be isolated and reversible; if uncertain, stop and write SSOT first.

---

## 2) Recipe-first Invariants (plans must prove they hold)

### Invariant A: Facts→Recipe→Parts is a single rail
Plans must identify:
- Facts observation point (where shape is recognized).
- Recipe payload (what is carried; avoid raw AST “bags” when possible).
- Parts entry used (which `parts::*` entry lowers it).

Reject plan if:
- It introduces or relies on AST-direct lowering “rescue” paths in `features/*` or pipelines.
- It adds a pattern variant but still re-scans/lower AST directly in features.

### Invariant B: Verification gate is single-source
Plans must not re-introduce “acceptance re-check” outside the verifier gate.
If a plan adds new lowering entrypoints, it must point to the verifier/entry SSOT and show how it remains the only acceptance gate.

---

## 3) Mechanical Reject Checks (run these before accepting a plan)

### 3.1 Existence checks (prevent “imaginary types”)
Reject if a plan references non-existent constructs. Require at least one of:
- `rg -n "<symbol>" <paths>`
- `rg -n "enum <Name>|struct <Name>" <paths>`

Example (do not copy blindly; adapt to the plan):
- `rg -n "ASTNode::Block\\b" src` (must be 0 if the plan claims it exists)

### 3.2 Recipe-first drift checks (core)
These are used as “return-to-zero” checks. If a plan changes their meaning, it must update SSOT docs first.

- No old block-lowering in features:
  - `rg -n "block_lowering::lower_loop_cond_(block|recipe_block)\\b" src/mir/builder/control_flow/plan/features --glob '!*.md'` → 0
- No manual control-flow detectors in features:
  - `rg -n "count_control_flow|ControlFlowDetector" src/mir/builder/control_flow/plan/features --glob '!*.md'` → 0
- No direct dispatch access from features:
  - `rg -n "parts::dispatch::|dispatch::" src/mir/builder/control_flow/plan/features --glob '!*.md'` → 0

### 3.3 New-module “no AST bag” check (local)
For any *new* BoxCount module directory (or when refactoring a module to be recipe-first):
- `rg -n "Vec<ASTNode>" src/mir/builder/control_flow/plan/<module> --glob '!*.md'` → 0

Allowed exception:
- `RecipeBody::new(stmts.to_vec())` inside Facts-level recipe builders (arena registration).

---

## 4) Fail-fast Tag Policy (plans must follow)

Required:
- Contract failures must be short-distance and tagged.
- Use existing stable tags when possible (avoid tag proliferation).

Forbidden:
- Silent fallback on contract violations.
- Unconditional `eprintln!` (must be behind the existing debug toggle).

---

## 5) Plan Template (copy/paste)

### Summary
- Problem:
- Goal:
- BoxCount or BoxShape:

### Required Attachments
- `git status -sb`:
- Repro command:
- first_freeze_or_reject (1 line):
- StepTree root (1 line):
- Log path (`/tmp/...`):
- Env (planner_required / strict / dev):

### Files to Touch (exact list)
- (one file per commit, when applicable)

### Mechanical Checks (must pass)
- `rg ...` (0件/想定件数):
- Gate command(s):

### Commit Plan
- Commit 1:
- Commit 2:

