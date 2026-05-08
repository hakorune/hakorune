---
Status: done
Date: 2026-05-09
Scope: M12b Profile registry docs
---

# 293x-063 M12b Profile Registry Docs

## Decision

M12b is live-docs only.

At M12b landing, `@rune Profile(...)` remained disabled parser surface. M12c
later enabled reserved-name parser acceptance and expansion. The M12b decision
is still the registry SSOT itself: profile names and primitive expansion targets
live in one file:

```text
docs/reference/mir/rune-profile-registry.md
```

## Reserved Profiles

- `allocator.fast`
- `allocator.slow`
- `substrate.leaf`
- `intrinsic.leaf`
- `raw.layout`

## Not Owned

- Parser acceptance of `Profile(...)`.
- Parser acceptance of `Capability(...)`.
- Profile expansion to primitive rune metadata.
- Capability verifier acceptance.
- Backend or `.inc` profile-name consumption.
- Allocator fast-path EXE proof.

## Acceptance

```bash
bash tools/checks/k2_wide_profile_registry_docs_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
