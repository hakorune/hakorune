#!/usr/bin/env python3
"""Move rooted-unreachable phase clusters while preserving local links."""

from __future__ import annotations

import argparse
import json
import os
import posixpath
import re
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
MANIFEST = ROOT / "tools/checks/manifests/repository_artifact_lifecycle_v0.json"
INVENTORY = ROOT / "tools/docs/repository_artifact_lifecycle_inventory.py"
CURRENT_PHASE_ROOT = "docs/development/current/main/phases"
ARCHIVE_PHASE_ROOT = "docs/development/archive/phases"
MARKDOWN_LINK = re.compile(r"(\[[^\]]*\]\()([^)]+)(\))")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--apply", action="store_true")
    return parser.parse_args()


def repository_files() -> list[str]:
    raw = subprocess.check_output(
        [
            "git",
            "-C",
            str(ROOT),
            "ls-files",
            "-z",
            "--cached",
            "--others",
            "--exclude-standard",
        ]
    )
    return sorted(
        item.decode("utf-8", errors="surrogateescape")
        for item in raw.split(b"\0")
        if item
    )


def phase293x_card_bucket(filename: str) -> str | None:
    match = re.match(r"^293x-([0-9]+)-", filename)
    if not match:
        return None
    lower = (int(match.group(1), 10) // 100) * 100
    return f"293x-{lower:03d}-{lower + 99:03d}"


def phase_card_locations(phase: str, filename: str) -> tuple[str, ...]:
    phase_name = phase if phase.startswith("phase-") else f"phase-{phase}"
    live_root = f"{CURRENT_PHASE_ROOT}/{phase_name}"
    global_root = f"{ARCHIVE_PHASE_ROOT}/{phase_name}"
    locations = [
        f"{live_root}/{filename}",
        f"{global_root}/{filename}",
        f"{global_root}/cards/{filename}",
    ]
    if phase_name == "phase-293x":
        if bucket := phase293x_card_bucket(filename):
            locations.append(f"{global_root}/cards/{bucket}/{filename}")
    locations.append(f"{CURRENT_PHASE_ROOT}/archive/{phase_name}/{filename}")
    if phase_name == "phase-293x":
        if bucket := phase293x_card_bucket(filename):
            locations.append(f"{live_root}/archive/cards/{bucket}/{filename}")
    locations.append(f"{live_root}/archive/{filename}")
    return tuple(locations)


def archive_target_for_source(source: str) -> str:
    shared_prefix = f"{CURRENT_PHASE_ROOT}/archive/"
    if source.startswith(shared_prefix):
        remainder = source[len(shared_prefix) :]
        phase_name, separator, relative = remainder.partition("/")
        if not separator or not phase_name.startswith("phase-"):
            raise ValueError(f"invalid transitional phase source: {source}")
        return f"{ARCHIVE_PHASE_ROOT}/{phase_name}/{relative}"

    live_prefix = f"{CURRENT_PHASE_ROOT}/"
    if not source.startswith(live_prefix):
        raise ValueError(f"phase source is outside current roots: {source}")
    remainder = source[len(live_prefix) :]
    phase_name, separator, relative = remainder.partition("/")
    if not separator or not phase_name.startswith("phase-"):
        raise ValueError(f"invalid phase source: {source}")
    if relative.startswith("archive/"):
        relative = relative.removeprefix("archive/")
        if phase_name == "phase-296x" and not relative.startswith("cards/"):
            relative = f"cards/{relative}"
    return f"{ARCHIVE_PHASE_ROOT}/{phase_name}/{relative}"


def validate_move_map(
    move_map: dict[str, str], repository_paths: set[str]
) -> None:
    targets = list(move_map.values())
    if len(targets) != len(set(targets)):
        raise RuntimeError("multiple phase sources select the same archive target")
    for source, target in move_map.items():
        if source not in repository_paths:
            raise RuntimeError(f"phase move source is missing: {source}")
        if target in repository_paths or (ROOT / target).exists():
            raise RuntimeError(f"archive target already exists: {target}")


def bounded_cluster_batch(
    clusters: list[dict[str, object]], maximum: int = 200
) -> dict[str, object]:
    selected: list[str] = []
    count = 0
    for cluster in clusters:
        documents = cluster["documents"]
        if cluster.get("inbound_edge_count", 0) != 0:
            continue
        if len(documents) > maximum or len(selected) + len(documents) > maximum:
            continue
        selected.extend(documents)
        count += 1
    return {"cluster_count": count, "file_count": len(selected), "inbound_edge_count": 0, "documents": selected}


def readable_text(relative: str) -> str | None:
    path = ROOT / relative
    try:
        if not path.is_file() or path.stat().st_size > 2_000_000:
            return None
        data = path.read_bytes()
    except OSError:
        return None
    if b"\0" in data:
        return None
    return data.decode("utf-8", errors="ignore")


def split_target(raw: str) -> tuple[str, str, bool]:
    stripped = raw.strip()
    if stripped.startswith("<") and ">" in stripped:
        end = stripped.index(">")
        return stripped[1:end], stripped[end + 1 :], True
    parts = stripped.split(maxsplit=1)
    return parts[0], f" {parts[1]}" if len(parts) == 2 else "", False


def split_suffix(target: str) -> tuple[str, str]:
    positions = [position for marker in ("#", "?") if (position := target.find(marker)) >= 0]
    if not positions:
        return target, ""
    first = min(positions)
    return target[:first], target[first:]


def resolve_target(source: str, target: str) -> str | None:
    if not target or target.startswith(("#", "http://", "https://", "mailto:")):
        return None
    path, _ = split_suffix(target)
    if path.startswith("/"):
        return posixpath.normpath(path[1:])
    if path.startswith("docs/"):
        return posixpath.normpath(path)
    return posixpath.normpath(posixpath.join(posixpath.dirname(source), path))


def render_target(
    original: str, new_source: str, new_target: str, suffix: str
) -> str:
    if original.startswith("/"):
        return f"/{new_target}{suffix}"
    if original.startswith("docs/"):
        return f"{new_target}{suffix}"
    relative = posixpath.relpath(new_target, posixpath.dirname(new_source))
    if original.startswith("./") and not relative.startswith("."):
        relative = f"./{relative}"
    return f"{relative}{suffix}"


def rewrite_markdown_links(
    source: str,
    new_source: str,
    text: str,
    move_map: dict[str, str],
    repository_paths: set[str],
) -> tuple[str, list[tuple[str, str]], int]:
    preserved: list[tuple[str, str]] = []
    rewrite_count = 0

    def replace(match: re.Match[str]) -> str:
        nonlocal rewrite_count
        raw, trailing, angled = split_target(match.group(2))
        old_target = resolve_target(source, raw)
        if old_target is None or old_target not in repository_paths:
            return match.group(0)
        new_target = move_map.get(old_target, old_target)
        _, suffix = split_suffix(raw)
        rendered = render_target(raw, new_source, new_target, suffix)
        if angled:
            rendered = f"<{rendered}>"
        rendered = f"{rendered}{trailing}"
        if rendered != match.group(2).strip():
            rewrite_count += 1
        preserved.append((new_source, new_target))
        return f"{match.group(1)}{rendered}{match.group(3)}"

    return MARKDOWN_LINK.sub(replace, text), preserved, rewrite_count


def load_plan() -> tuple[list[str], dict[str, str]]:
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    phases = manifest["document_reachability"]["whole_phase_unreachable"]
    move_map: dict[str, str] = {}
    for relative in repository_files():
        for phase in phases:
            prefix = f"{CURRENT_PHASE_ROOT}/{phase}/"
            if relative.startswith(prefix):
                move_map[relative] = archive_target_for_source(relative)
                break
    validate_move_map(move_map, set(repository_files()))
    return phases, move_map


def ensure_clean_worktree() -> None:
    status = subprocess.check_output(
        ["git", "-C", str(ROOT), "status", "--porcelain"], text=True
    )
    if status.strip():
        raise RuntimeError("worktree must be clean before archive relocation")


def apply_plan(phases: list[str], move_map: dict[str, str]) -> None:
    ensure_clean_worktree()
    subprocess.run(
        [sys.executable, str(INVENTORY), "--check", "--strict"],
        cwd=ROOT,
        check=True,
    )
    repository_paths = set(repository_files())
    validate_move_map(move_map, repository_paths)

    rewritten: dict[str, str] = {}
    preserved_links: list[tuple[str, str]] = []
    markdown_rewrites = 0
    plain_rewrites = 0
    manifest_relative = MANIFEST.relative_to(ROOT).as_posix()
    phase_prefixes = {
        f"{CURRENT_PHASE_ROOT}/{phase}/": f"{ARCHIVE_PHASE_ROOT}/{phase}/"
        for phase in phases
    }
    for source in sorted(repository_paths):
        if source == manifest_relative:
            continue
        text = readable_text(source)
        if text is None:
            continue
        new_source = move_map.get(source, source)
        updated, links, count = rewrite_markdown_links(
            source, new_source, text, move_map, repository_paths
        )
        preserved_links.extend(links)
        markdown_rewrites += count
        for old_prefix, new_prefix in phase_prefixes.items():
            occurrences = updated.count(old_prefix)
            if occurrences:
                updated = updated.replace(old_prefix, new_prefix)
                plain_rewrites += occurrences
        if updated != text:
            rewritten[new_source] = updated

    archive_root = ROOT / ARCHIVE_PHASE_ROOT
    archive_root.mkdir(parents=True, exist_ok=True)
    for phase in phases:
        source = ROOT / CURRENT_PHASE_ROOT / phase
        target = archive_root / phase
        if not source.is_dir() or target.exists():
            raise RuntimeError(f"invalid phase move: {source} -> {target}")
        subprocess.run(["git", "mv", str(source), str(target)], cwd=ROOT, check=True)
    for relative, text in rewritten.items():
        path = ROOT / relative
        path.write_text(text, encoding="utf-8")

    subprocess.run(
        [sys.executable, str(INVENTORY), "--write"], cwd=ROOT, check=True
    )
    for source, target in preserved_links:
        if not (ROOT / source).is_file() or not (ROOT / target).exists():
            raise RuntimeError(f"relocated link lost: {source} -> {target}")
    print(
        "[archive-unreachable-phases] applied "
        f"phases={len(phases)} files={len(move_map)} "
        f"markdown_rewrites={markdown_rewrites} plain_rewrites={plain_rewrites}"
    )


def main() -> int:
    args = parse_args()
    phases, move_map = load_plan()
    print(
        "[archive-unreachable-phases] plan "
        f"phases={len(phases)} files={len(move_map)}"
    )
    if args.apply:
        apply_plan(phases, move_map)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
