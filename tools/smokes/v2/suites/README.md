# Smoke Suite Manifests

This directory holds manifest files for smoke suites.

Format:
- One relative path per line.
- `#` starts a comment.
- Paths are relative to `tools/smokes/v2/profiles/<profile>/`.
- Archive carriers may live under `archive/` inside the profile tree when a suite is meant to replay retired evidence explicitly.
- Keep manifests small, stable, and active-only unless they are the dedicated archive carrier for a retired lane.

Layout:
- `tools/smokes/v2/suites/<profile>/<suite>.txt`

This slice seeds integration-only suites. Other profiles can be added later with the same format.

Owner-pack discovery keeps runtime policy separate from suite ownership. An
aggregate may execute with `--profile quick` while selecting an exact suite
from another owner profile, for example:

```bash
tools/smokes/v2/run.sh --profile quick --owner-profile integration \
  --suite phase2050-owner-pack
```

Explicit `--owner-profile` requires an exact suite and rejects `--filter`.
Every manifest entry must be discovered and selected before the first test
effect; zero, partial, foreign, duplicate, or stale entries fail closed.

Integration aggregate wrappers are separate from leaf suites. The explicit
`suites/integration/aggregate-nodes.txt` manifest may list only reviewed
aggregate nodes; `ExplicitOnlyAggregate` entries are excluded from normal
integration leaf discovery and must name their exact child suite. The current
pilot contains only `core/phase2050/run_all.sh`; other wrappers remain
unchanged until their own fate is designed.
