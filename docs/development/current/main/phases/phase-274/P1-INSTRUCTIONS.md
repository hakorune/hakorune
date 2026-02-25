# Phase 274 P1 — Implement `TypeOp` on Rust VM (Instruction Guide)

Goal: make `x.is("T")` / `x.as("T")` runnable on the primary backend (Rust VM), aligning language docs and runtime behavior.

Scope: small, behavior-preserving where possible; fail-fast on unsupported type operations.

SSOT:
- Language semantics: `docs/reference/language/types.md`
- MIR instruction: `MirInstruction::TypeOp` (`src/mir/instruction.rs`)
- Rust VM executor: `src/backend/mir_interpreter/*`

---

## 1) What already exists

- Frontend lowering emits `MirInstruction::TypeOp` for `.is("Type")` / `.as("Type")`:
  - `src/mir/builder/exprs.rs`
  - type-name mapping: `src/mir/builder/calls/special_handlers.rs` (`parse_type_name_to_mir`)
- A fixture exists for P1 acceptance:
  - `apps/tests/phase274_p1_typeop_is_as_min.hako`
  - smoke: `tools/smokes/v2/profiles/integration/apps/phase274_p1_typeop_is_as_vm.sh`

---

## 2) Implementation plan (Rust VM)

### 2.1 Add instruction execution

Implement execution of:
- `MirInstruction::TypeOp { dst, op: Check, value, ty }` → `dst = Bool(matches(value, ty))`
- `MirInstruction::TypeOp { dst, op: Cast, value, ty }` → `dst = value` if matches, else `TypeError`

Files:
- `src/backend/mir_interpreter/handlers/type_ops.rs` (new module is OK)
- `src/backend/mir_interpreter/handlers/mod.rs` (dispatch arm)
- `src/backend/mir_interpreter/mod.rs` (import `MirType`, `TypeOpKind` into interpreter module)

### 2.2 Matching rules (minimal, fail-fast)

Match by `MirType`:
- `Integer/Float/Bool/String/Void`: accept both primitive VM values and their core Box equivalents when present.
- `Box("Foo")`: accept:
  - user-defined `InstanceBox` where `class_name == "Foo"`
  - builtin/plugin boxes where `type_name() == "Foo"`
  - (best-effort) builtin `InstanceBox(from_any_box)` inner `type_name() == "Foo"`
- Others:
  - `Unknown` matches anything (diagnostic-friendly).

Do not add new environment variables. Keep behavior deterministic and fail-fast.

---

## 3) Testing / Verification

### 3.1 Build

```bash
cargo build --release
```

### 3.2 Smoke (required)

```bash
HAKORUNE_BIN=./target/release/hakorune bash \
  tools/smokes/v2/profiles/integration/apps/phase274_p1_typeop_is_as_vm.sh
```

Expected:
- PASS (exit=3)

### 3.3 Optional MIR inspection

```bash
NYASH_VM_DUMP_MIR=1 ./target/release/hakorune --backend vm \
  apps/tests/phase274_p1_typeop_is_as_min.hako
```

Confirm:
- MIR contains `TypeOp(check, ...)` and `TypeOp(cast, ...)`.

---

## 4) Acceptance criteria (P1)

- Rust VM executes `TypeOp` (no “unimplemented instruction”).
- `phase274_p1_typeop_is_as_vm.sh` passes.
- No new env vars are introduced.
- Docs remain consistent with runtime:
  - `docs/reference/language/types.md` describes runtime `TypeOp` behavior.

