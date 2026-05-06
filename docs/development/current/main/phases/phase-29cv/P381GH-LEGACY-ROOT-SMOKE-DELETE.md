# P381GH Legacy Root Smoke Delete

Date: 2026-05-06
Scope: delete only the four P381GG legacy root-smoke candidates.

## Decision

Deleted exactly the four legacy root-smoke scripts classified by P381GG:

- `tools/smokes/archive/smoke_async_spawn.sh`
- `tools/smokes/curated_phi_invariants.sh`
- `tools/smokes/parity_quick.sh`
- `tools/smokes/unified_members.sh`

No v2 profile smoke and no `tools/archive/manual-smokes` script was touched in
this commit.

## Count Delta

| Surface | Before | After | Delta |
| --- | ---: | ---: | ---: |
| legacy `tools/smokes` outside `v2` | 14 | 10 | -4 |
| `tools/smokes/v2/profiles` | 1419 | 1419 | 0 |
| `tools/smokes/v2/profiles/integration/apps/archive` | 184 | 184 | 0 |
| `tools/archive/manual-smokes` | 35 | 35 | 0 |

## Remaining Legacy Root Smokes

```text
tools/smokes/archive/aot_smoke_cranelift.sh
tools/smokes/archive/jit_smoke.sh
tools/smokes/archive/mir15_smoke.sh
tools/smokes/archive/smoke_phase_10_10.sh
tools/smokes/archive/smoke_vm_jit.sh
tools/smokes/curated_llvm.sh
tools/smokes/curated_llvm_stage3.sh
tools/smokes/fast_local.sh
tools/smokes/phi_trace_local.sh
tools/smokes/selfhost_local.sh
```

These remain held because they still have owner refs or need a separate
lifecycle decision.

## Validation

```bash
for f in \
  tools/smokes/archive/smoke_async_spawn.sh \
  tools/smokes/curated_phi_invariants.sh \
  tools/smokes/parity_quick.sh \
  tools/smokes/unified_members.sh
do
  test ! -e "$f" || exit 1
done
```

```bash
find tools/smokes -path tools/smokes/v2 -prune -o -type f -name '*.sh' -print | sort
find tools/smokes/v2/profiles -type f -name '*.sh' | wc -l
find tools/archive/manual-smokes -type f -name '*.sh' | wc -l
```

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

T6 broad deletion remains blocked. The remaining archive/manual smoke surface
has references or owner-policy holds. The next cleanup should close out those
holds explicitly rather than deleting by directory.
