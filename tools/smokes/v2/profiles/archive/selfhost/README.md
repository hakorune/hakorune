# selfhost archive smokes

Archive-only selfhost smoke scripts live here.

Rules:
- Not part of `tools/smokes/v2/run.sh --profile integration/selfhost` discovery.
- Keep only historical, always-skip, or manually replayed canaries here.
- If a smoke returns to active coverage, reintroduce it through a documented semantic entrypoint under `profiles/integration/selfhost/`.
