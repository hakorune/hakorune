#!/usr/bin/env python3
"""Export a provider package as a handoff directory plus zip archive."""

from __future__ import annotations

import argparse
import json
import shutil
import zipfile
from pathlib import Path
from typing import Any


OUTPUT_CONTRACT = "hakorune-provider-package-export-bundle-v0"


def fail(message: str) -> None:
    raise SystemExit(f"[provider-package-export-bundle] {message}")


def load_manifest(path: Path) -> dict[str, Any]:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        fail(f"invalid manifest JSON: {exc}")


def require_file(path: Path, label: str) -> None:
    if not path.is_file():
        fail(f"missing {label}: {path}")


def copy_package_file(src: Path, dst: Path) -> None:
    dst.parent.mkdir(parents=True, exist_ok=True)
    shutil.copy2(src, dst)


def render_readme(manifest: dict[str, Any], package_dir: Path) -> str:
    artifact = manifest["artifact"]["path"]
    provider_name = manifest.get("provider_name", "unknown-provider")
    provider_version = manifest.get("provider_version", "unknown")
    package_id = manifest.get("package_id", "unknown-package")
    target = manifest.get("target_triple", "unknown-target")
    platform = manifest.get("platform", "unknown-platform")
    contract_hash = manifest.get("contract_hash", "unknown")
    return f"""# Hakorune Mimalloc Provider Bundle

This bundle contains a Hakorune-generated allocator provider package for
external benchmark handoff.

## Contents

```text
hakorune_provider.json
hakorune_provider.sha256
{artifact}
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

## LD_PRELOAD Example

The provider shared library is not a malloc-family LD_PRELOAD shim by itself.
Use the generated shim from Hakorune tools, or load this provider package via a
compatible allocator benchmark harness.

For Hakorune's local replacement ladder:

```sh
tools/allocator/hako_mimalloc_provider_replacement_decision_ladder.sh \\
  --out target/provider-replacement-decision/report.out \\
  --skip-build-release
```
"""


def render_ldpreload_example(manifest: dict[str, Any]) -> str:
    artifact = manifest["artifact"]["path"]
    return f"""#!/usr/bin/env bash
set -euo pipefail

HERE="$(cd "$(dirname "$0")" && pwd)"
PROVIDER="$HERE/{artifact}"
MANIFEST="$HERE/hakorune_provider.json"

cat <<EOF
Hakorune provider package:
  manifest: $MANIFEST
  provider: $PROVIDER

This shared library exposes Hakorune provider ABI symbols. It is not a direct
malloc/free LD_PRELOAD shim. Use it with a compatible provider-backed shim or
benchmark harness.
EOF
"""


def make_zip(bundle_dir: Path, zip_path: Path) -> None:
    zip_path.parent.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(zip_path, "w", compression=zipfile.ZIP_DEFLATED) as zf:
        for path in sorted(bundle_dir.rglob("*")):
            if path.is_file():
                zf.write(path, path.relative_to(bundle_dir.parent))


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--package-dir", type=Path, required=True)
    parser.add_argument("--out-dir", type=Path, required=True)
    parser.add_argument("--bundle-name", default="hakorune-mimalloc-provider")
    parser.add_argument("--force", action="store_true")
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    package_dir = args.package_dir.resolve()
    manifest_path = package_dir / "hakorune_provider.json"
    sha_path = package_dir / "hakorune_provider.sha256"
    require_file(manifest_path, "manifest")
    require_file(sha_path, "sha256")
    manifest = load_manifest(manifest_path)
    artifact_name = manifest.get("artifact", {}).get("path")
    if not isinstance(artifact_name, str) or not artifact_name:
        fail("manifest artifact.path is missing")
    artifact_path = package_dir / artifact_name
    require_file(artifact_path, "provider artifact")

    out_dir = args.out_dir.resolve()
    bundle_dir = out_dir / args.bundle_name
    zip_path = out_dir / f"{args.bundle_name}.zip"
    if bundle_dir.exists() or zip_path.exists():
        if not args.force:
            fail("bundle output already exists; pass --force")
        if bundle_dir.exists():
            shutil.rmtree(bundle_dir)
        if zip_path.exists():
            zip_path.unlink()
    bundle_dir.mkdir(parents=True, exist_ok=True)

    copy_package_file(manifest_path, bundle_dir / "hakorune_provider.json")
    copy_package_file(sha_path, bundle_dir / "hakorune_provider.sha256")
    copy_package_file(artifact_path, bundle_dir / artifact_name)
    (bundle_dir / "README.md").write_text(render_readme(manifest, package_dir), encoding="utf-8")
    script_path = bundle_dir / "run_ldpreload_example.sh"
    script_path.write_text(render_ldpreload_example(manifest), encoding="utf-8")
    script_path.chmod(0o755)
    make_zip(bundle_dir, zip_path)

    lines = [
        f"output_contract={OUTPUT_CONTRACT}",
        f"package_dir={package_dir}",
        f"bundle_dir={bundle_dir}",
        f"bundle_zip={zip_path}",
        f"bundle_name={args.bundle_name}",
        f"manifest_path={bundle_dir / 'hakorune_provider.json'}",
        f"sha256_path={bundle_dir / 'hakorune_provider.sha256'}",
        f"provider_binary_path={bundle_dir / artifact_name}",
        f"provider_name={manifest.get('provider_name', '')}",
        f"provider_version={manifest.get('provider_version', '')}",
        f"target_triple={manifest.get('target_triple', '')}",
        f"platform={manifest.get('platform', '')}",
        f"contract_hash={manifest.get('contract_hash', '')}",
        "provider_activation=0",
        "production_replacement_active=0",
        "hook_installed=0",
        "global_allocator_product_claim=0",
        "winner_claim=0",
        "summary=ok",
    ]
    report = "\n".join(lines) + "\n"
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
