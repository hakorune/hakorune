"""Render bundle-facing text for provider package exports."""

from __future__ import annotations

from typing import Any


def render_readme(
    manifest: dict[str, Any],
    *,
    shim_name: str | None,
    shim_sha256: str | None,
) -> str:
    artifact = manifest["artifact"]["path"]
    provider_name = manifest.get("provider_name", "unknown-provider")
    provider_version = manifest.get("provider_version", "unknown")
    package_id = manifest.get("package_id", "unknown-package")
    target = manifest.get("target_triple", "unknown-target")
    platform = manifest.get("platform", "unknown-platform")
    contract_hash = manifest.get("contract_hash", "unknown")
    shim_contents = f"{shim_name}\n" if shim_name else ""
    shim_provider_fields = (
        f"ldpreload_shim={shim_name}\nldpreload_shim_sha256={shim_sha256}\n"
        if shim_name
        else "ldpreload_shim=not-included\n"
    )
    if shim_name:
        ldpreload_section = f"""## LD_PRELOAD Example

The provider shared library exposes Hakorune provider ABI symbols. The bundled
`{shim_name}` is the generated malloc-family LD_PRELOAD shim that loads the
provider through `HAKORUNE_PROVIDER_LIBRARY`.

Run a benchmark command through the helper script:

```sh
./run_ldpreload_example.sh ./bench_random_mixed_system 1000 128 42
```

The helper sets:

```text
HAKORUNE_PROVIDER_LIBRARY=$HERE/{artifact}
HAKORUNE_PROVIDER_LDPRELOAD_REPORT=$HERE/ldpreload_counts.out
LD_PRELOAD=$HERE/{shim_name}
```
"""
    else:
        ldpreload_section = """## LD_PRELOAD Example

This bundle does not include a malloc-family LD_PRELOAD shim. Use the provider
shared library with a compatible provider-backed shim or regenerate this bundle
without `--no-ldpreload-shim`.
"""
    return f"""# Hakorune Mimalloc Provider Bundle

This bundle contains a Hakorune-generated allocator provider package for
external benchmark handoff.

## Contents

```text
hakorune_provider.json
hakorune_provider.sha256
{artifact}
{shim_contents.rstrip()}
run_ldpreload_example.sh
```

## Provider

```text
package_id={package_id}
provider_name={provider_name}
provider_version={provider_version}
target_triple={target}
platform={platform}
contract_hash={contract_hash}
{shim_provider_fields.rstrip()}
```

## Stop Lines

This bundle is for explicit external benchmarking. It is not a Hakorune product
default and does not claim allocator replacement by itself.

```text
provider_activation=0
production_replacement_active=0
hook_installed=0
global_allocator_product_claim=0
winner_claim=0
```

{ldpreload_section}
"""


def render_ldpreload_example(manifest: dict[str, Any], *, shim_name: str | None) -> str:
    artifact = manifest["artifact"]["path"]
    if shim_name is None:
        return f"""#!/usr/bin/env bash
set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
PROVIDER="$HERE/{artifact}"

cat <<EOF
Hakorune provider package:
  provider: $PROVIDER

This bundle was exported without a malloc-family LD_PRELOAD shim.
Regenerate it without --no-ldpreload-shim for direct LD_PRELOAD handoff.
EOF
exit 2
"""
    return f"""#!/usr/bin/env bash
set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
PROVIDER="$HERE/{artifact}"
MANIFEST="$HERE/hakorune_provider.json"
SHIM="$HERE/{shim_name}"
REPORT="${{HAKORUNE_PROVIDER_LDPRELOAD_REPORT:-$HERE/ldpreload_counts.out}}"

if [ "$#" -eq 0 ]; then
  cat <<EOF
usage: $0 <benchmark-or-command> [args...]

Hakorune provider package:
  manifest:      $MANIFEST
  provider:      $PROVIDER
  ldpreload shim: $SHIM
  report:        $REPORT
EOF
  exit 2
fi

export HAKORUNE_PROVIDER_LIBRARY="$PROVIDER"
export HAKORUNE_PROVIDER_LDPRELOAD_REPORT="$REPORT"
export LD_PRELOAD="$SHIM${{LD_PRELOAD:+:$LD_PRELOAD}}"

exec "$@"
"""
