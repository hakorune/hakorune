# selfhost archive smokes

Archive-only selfhost smoke scripts live here.

Rules:
- Not part of `tools/smokes/v2/run.sh --profile integration` discovery.
- Keep only historical, always-skip, or manually replayed selfhost canaries here.
- Stage-B selfhost diagnostics live here once they are no longer part of the active integration profile.
- If a smoke returns to active coverage, reintroduce it through a documented semantic entrypoint under `profiles/integration/selfhost/`.
