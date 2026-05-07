# 293x-005 Pure-First General-Newbox Owner Decision

- Status: Landed
- Lane: `phase-293x real-app bringup`
- Scope: decide the owner boundary for direct EXE pure-first failures that
  stop at general user-box `newbox`.

## Finding

The real-app EXE boundary probe reaches `ny-llvmc` pure-first and fails before
allocator porting work starts:

```text
[llvm-pure/unsupported-shape] ... first_op=newbox owner_hint=mir_normalizer ...
```

The C shim already supports narrow built-in collection births (`ArrayBox`,
`MapBox`, and selected `StringBox` paths) and exact seed routes. It does not
own broad user-box allocation policy.

## Decision

- Do not add a broad general user-box `newbox` policy to the C shim.
- Keep C shim support limited to diagnostics, exact accepted seeds, and
  narrow built-in lowering surfaces.
- Treat general user-box allocation acceptance as MIR / `BackendRecipeBox`
  ownership: typed object-plan evidence must decide which constructor shapes
  pure-first may lower.
- Keep `HAKO_BACKEND_COMPAT_REPLAY=none` as the mainline proof mode.
- Proceed with the real allocator port only as a VM-only policy/state prototype
  until a typed object EXE plan exists.

## Consequence

The old blocker is resolved as an ownership decision, not as EXE parity.
The new active blocker is:

```text
phase-293x allocator port mode: VM-only policy/state prototype until typed object EXE plan
```

EXE boundary smokes remain blocker probes and must continue to fail-fast at the
known pure-first unsupported shape until the typed object plan lands.

## Verification

```bash
tools/smokes/v2/run.sh --profile integration --suite real-apps --skip-preflight
tools/smokes/v2/run.sh --profile integration --suite real-apps-exe-boundary --skip-preflight
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
