#!/usr/bin/env python3
"""Move rooted-unreachable document clusters from partially live phases."""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path

from archive_unreachable_phase_clusters import (
    archive_target_for_source,
    CURRENT_PHASE_ROOT,
    INVENTORY,
    MANIFEST,
    ROOT,
    ensure_clean_worktree,
    readable_text,
    repository_files,
    rewrite_markdown_links,
    validate_move_map,
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--apply", action="store_true")
    parser.add_argument("--max-files", type=int, default=200)
    return parser.parse_args()


def select_batch(max_files: int) -> list[str]:
    if max_files < 1:
        raise ValueError("max-files must be positive")
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    partial = manifest["document_reachability"]["partial_phase_unreachable"]
    if partial["reachable_incoming_edge_count"] != 0:
        raise RuntimeError("partial candidates have reachable incoming edges")
    if partial["archive_target_collision_count"] != 0:
        raise RuntimeError("partial candidates have archive target collisions")
    clusters = partial["clusters"]
    if not clusters:
        return []
    if clusters[0]["file_count"] > max_files:
        return list(clusters[0]["documents"])
    selected: list[str] = []
    for cluster in clusters:
        if len(selected) + cluster["file_count"] > max_files:
            continue
        selected.extend(cluster["documents"])
    return sorted(selected)


def prune_empty_parents(path: Path) -> None:
    stop = ROOT / CURRENT_PHASE_ROOT
    current = path
    while current != stop and current.is_dir():
        try:
            current.rmdir()
        except OSError:
            return
        current = current.parent


def apply_batch(selected: list[str]) -> None:
    ensure_clean_worktree()
    subprocess.run(
        [sys.executable, str(INVENTORY), "--check", "--strict"],
        cwd=ROOT,
        check=True,
    )
    move_map = {source: archive_target_for_source(source) for source in selected}
    repository_paths = set(repository_files())
    validate_move_map(move_map, repository_paths)

    rewritten: dict[str, str] = {}
    preserved_links: list[tuple[str, str]] = []
    markdown_rewrites = 0
    plain_rewrites = 0
    manifest_relative = MANIFEST.relative_to(ROOT).as_posix()
    replacements = sorted(move_map.items(), key=lambda item: (-len(item[0]), item[0]))
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
        for old_path, new_path in replacements:
            occurrences = updated.count(old_path)
            if occurrences:
                updated = updated.replace(old_path, new_path)
                plain_rewrites += occurrences
        if updated != text:
            rewritten[new_source] = updated

    touched_parents: set[Path] = set()
    for source, target in sorted(move_map.items()):
        target_path = ROOT / target
        target_path.parent.mkdir(parents=True, exist_ok=True)
        subprocess.run(
            ["git", "mv", str(ROOT / source), str(target_path)],
            cwd=ROOT,
            check=True,
        )
        touched_parents.add((ROOT / source).parent)
    for relative, text in rewritten.items():
        (ROOT / relative).write_text(text, encoding="utf-8")
    for parent in sorted(touched_parents, key=lambda path: len(path.parts), reverse=True):
        prune_empty_parents(parent)

    subprocess.run(
        [sys.executable, str(INVENTORY), "--write"], cwd=ROOT, check=True
    )
    for source, target in preserved_links:
        if not (ROOT / source).is_file() or not (ROOT / target).exists():
            raise RuntimeError(f"relocated link lost: {source} -> {target}")
    for relative in repository_files():
        if relative == manifest_relative:
            continue
        text = readable_text(relative)
        if text is None:
            continue
        if old_path := next((path for path in move_map if path in text), None):
            raise RuntimeError(f"old partial phase path remains: {old_path} in {relative}")
    print(
        "[archive-unreachable-partial-phases] applied "
        f"files={len(selected)} markdown_rewrites={markdown_rewrites} "
        f"plain_rewrites={plain_rewrites}"
    )


def main() -> int:
    args = parse_args()
    selected = select_batch(args.max_files)
    move_map = {source: archive_target_for_source(source) for source in selected}
    validate_move_map(move_map, set(repository_files()))
    print(
        "[archive-unreachable-partial-phases] plan "
        f"files={len(selected)} max_files={args.max_files}"
    )
    if args.apply and selected:
        apply_batch(selected)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
