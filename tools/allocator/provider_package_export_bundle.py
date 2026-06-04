#!/usr/bin/env python3
"""Export a provider package as a handoff directory plus zip archive."""

from __future__ import annotations

import argparse
import hashlib
import json
import shutil
import subprocess
import sys
import zipfile
from pathlib import Path
from typing import Any

from provider_package_export_bundle_render import (
    render_ldpreload_example,
    render_readme,
)


OUTPUT_CONTRACT = "hakorune-provider-package-export-bundle-v0"
DEFAULT_SHIM_NAME = "libhakorune_provider_ldpreload.so"
REPO_ROOT = Path(__file__).resolve().parents[2]
LDPRELOAD_SMOKE_TOOL = REPO_ROOT / "tools/allocator/provider_package_ldpreload_replacement_smoke.py"


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


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def parse_kv_report(path: Path) -> dict[str, str]:
    fields: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8").splitlines():
        if "=" not in line:
            continue
        key, value = line.split("=", 1)
        fields[key] = value
    return fields


def build_ldpreload_shim(manifest_path: Path, build_dir: Path) -> tuple[Path, Path, dict[str, str]]:
    require_file(LDPRELOAD_SMOKE_TOOL, "LD_PRELOAD smoke tool")
    build_dir.mkdir(parents=True, exist_ok=True)
    report_path = build_dir / "provider_ldpreload_smoke.out"
    subprocess.run(
        [
            sys.executable,
            str(LDPRELOAD_SMOKE_TOOL),
            "--manifest",
            str(manifest_path),
            "--out-dir",
            str(build_dir),
            "--out",
            str(report_path),
        ],
        check=True,
    )
    fields = parse_kv_report(report_path)
    if fields.get("summary") != "ok":
        fail(f"LD_PRELOAD shim smoke failed: {report_path}")
    shim_path = Path(fields.get("shim_artifact_path", ""))
    require_file(shim_path, "LD_PRELOAD shim artifact")
    return shim_path, report_path, fields


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
    parser.add_argument("--shim-name", default=DEFAULT_SHIM_NAME)
    parser.add_argument("--no-ldpreload-shim", action="store_true")
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
    include_shim = not args.no_ldpreload_shim
    if include_shim and "/" in args.shim_name:
        fail("--shim-name must be a basename")

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

    shim_bundle_path: Path | None = None
    shim_smoke_report: Path | None = None
    shim_smoke_fields: dict[str, str] = {}
    if include_shim:
        shim_src, shim_smoke_report, shim_smoke_fields = build_ldpreload_shim(
            manifest_path,
            out_dir / ".build" / args.bundle_name / "ldpreload-shim",
        )
        shim_bundle_path = bundle_dir / args.shim_name
        copy_package_file(shim_src, shim_bundle_path)

    copy_package_file(manifest_path, bundle_dir / "hakorune_provider.json")
    copy_package_file(sha_path, bundle_dir / "hakorune_provider.sha256")
    copy_package_file(artifact_path, bundle_dir / artifact_name)
    shim_sha = sha256_file(shim_bundle_path) if shim_bundle_path else None
    (bundle_dir / "README.md").write_text(
        render_readme(
            manifest,
            shim_name=args.shim_name if include_shim else None,
            shim_sha256=shim_sha,
        ),
        encoding="utf-8",
    )
    script_path = bundle_dir / "run_ldpreload_example.sh"
    script_path.write_text(
        render_ldpreload_example(manifest, shim_name=args.shim_name if include_shim else None),
        encoding="utf-8",
    )
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
        f"ldpreload_shim_included={1 if include_shim else 0}",
        f"ldpreload_shim_path={shim_bundle_path or ''}",
        f"ldpreload_shim_sha256={shim_sha}",
        f"ldpreload_shim_smoke_report={shim_smoke_report or ''}",
        f"ldpreload_shim_smoke_summary={shim_smoke_fields.get('summary', '')}",
        f"provider_name={manifest.get('provider_name', '')}",
        f"provider_version={manifest.get('provider_version', '')}",
        f"target_triple={manifest.get('target_triple', '')}",
        f"platform={manifest.get('platform', '')}",
        f"contract_hash={manifest.get('contract_hash', '')}",
        "provider_activation=0",
        "production_replacement_active=0",
        "ldpreload_shim_exported=1" if include_shim else "ldpreload_shim_exported=0",
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
