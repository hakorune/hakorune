#!/usr/bin/env python3
"""Run NyRT exact-EXE plugin load-set RSS diagnostics.

The tool is diagnostic-only. It runs an existing exact-EXE with
HAKO_NYRT_RSS_CHECKPOINTS=1 under several generated `nyash.toml` load sets and
reports the plugin-host RSS deltas as JSON.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path

try:
    import tomllib
except ModuleNotFoundError:  # pragma: no cover - Python < 3.11 fallback is not expected in CI.
    tomllib = None


CHECKPOINT_RE = re.compile(
    r"\[(?:nyrt/rss|runtime/rss)\] checkpoint=([a-z0-9_]+) rss_bytes=(\d+)"
)

REQUIRED = [
    "after_runtime_hooks",
    "plugin_host_load_libraries_start",
    "plugin_host_after_host_config_parse",
    "plugin_loader_load_all_start",
    "plugin_loader_after_library_loop",
    "plugin_host_after_load_all_plugins",
    "after_plugin_host",
]

CORE_SIX_LIBS = {
    "libnyash_string_plugin.so",
    "libnyash_array_plugin.so",
    "libnyash_map_plugin.so",
    "libnyash_console_plugin.so",
    "libnyash_filebox_plugin.so",
    "libnyash_path_plugin.so",
}


def case_name_for_library(name: str) -> str:
    safe = re.sub(r"[^a-zA-Z0-9]+", "_", name).strip("_").lower()
    return f"single_{safe}"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--repo-root", required=True)
    parser.add_argument("--exe", required=True)
    return parser.parse_args()


def load_libraries(repo_root: Path) -> dict[str, dict[str, object]]:
    if tomllib is None:
        raise SystemExit("tomllib is required")
    raw = tomllib.loads((repo_root / "hako.toml").read_text(encoding="utf-8"))
    libraries = raw.get("libraries", {})
    if not isinstance(libraries, dict):
        return {}
    return libraries


def existing_libraries(repo_root: Path, names: set[str] | None = None) -> dict[str, dict[str, object]]:
    result: dict[str, dict[str, object]] = {}
    for name, value in sorted(load_libraries(repo_root).items()):
        if names is not None and name not in names:
            continue
        if not isinstance(value, dict):
            continue
        path_text = str(value.get("path", name))
        path = Path(path_text)
        abs_path = path if path.is_absolute() else repo_root / path
        if not abs_path.exists():
            continue
        boxes = value.get("boxes", [])
        if not isinstance(boxes, list):
            boxes = []
        result[name] = {
            "boxes": [str(item) for item in boxes],
            "path": str(abs_path),
        }
    return result


def write_config(path: Path, libraries: dict[str, dict[str, object]]) -> None:
    lines = ["[libraries]"]
    for name, value in sorted(libraries.items()):
        boxes = value.get("boxes", [])
        path_text = str(value.get("path", name))
        lines.append("")
        lines.append(f"[libraries.{json.dumps(name)}]")
        lines.append("boxes = [" + ", ".join(json.dumps(str(item)) for item in boxes) + "]")
        lines.append(f"path = {json.dumps(path_text)}")
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def run_case(exe: Path, cwd: Path) -> dict[str, int | str]:
    env = os.environ.copy()
    env["HAKO_NYRT_RSS_CHECKPOINTS"] = "1"
    env["NYASH_NYRT_SILENT_RESULT"] = "1"
    proc = subprocess.run(
        [str(exe)],
        cwd=str(cwd),
        env=env,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.PIPE,
        text=True,
        encoding="utf-8",
        errors="replace",
        check=False,
    )
    if proc.returncode != 0:
        raise SystemExit(f"{cwd.name}: exact EXE returned {proc.returncode}\n{proc.stderr}")
    values: dict[str, int] = {}
    for line in proc.stderr.splitlines():
        match = CHECKPOINT_RE.search(line)
        if match:
            values[match.group(1)] = int(match.group(2))
    missing = [label for label in REQUIRED if label not in values]
    if missing:
        raise SystemExit(f"{cwd.name}: missing checkpoints {missing}\n{proc.stderr}")
    return {
        "case": cwd.name,
        "after_runtime_hooks": values["after_runtime_hooks"],
        "after_host_config_parse": values["plugin_host_after_host_config_parse"],
        "after_library_loop": values["plugin_loader_after_library_loop"],
        "after_plugin_host": values["after_plugin_host"],
        "config_delta_bytes": values["plugin_host_after_host_config_parse"]
        - values["plugin_host_load_libraries_start"],
        "library_loop_delta_bytes": values["plugin_loader_after_library_loop"]
        - values["plugin_loader_load_all_start"],
        "total_plugin_host_delta_bytes": values["plugin_host_after_load_all_plugins"]
        - values["after_runtime_hooks"],
    }


def run_generated_case(tmp: Path, exe: Path, case: str, libraries: dict[str, dict[str, object]]) -> dict[str, int | str]:
    case_dir = tmp / case
    case_dir.mkdir()
    write_config(case_dir / "nyash.toml", libraries)
    return run_case(exe, case_dir)


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    exe = Path(args.exe).resolve()

    all_existing = existing_libraries(repo_root)
    core_six = existing_libraries(repo_root, CORE_SIX_LIBS)
    console_only = existing_libraries(repo_root, {"libnyash_console_plugin.so"})
    regex_only = existing_libraries(repo_root, {"libnyash_regex_plugin.so"})

    with tempfile.TemporaryDirectory(prefix="hakorune_nyrt_loadset.") as tmp_text:
        tmp = Path(tmp_text)
        rows = [
            run_generated_case(tmp, exe, "empty_config", {}),
            run_generated_case(tmp, exe, "console_only", console_only),
            run_generated_case(tmp, exe, "core_six_existing", core_six),
            run_generated_case(tmp, exe, "regex_only", regex_only),
            run_generated_case(tmp, exe, "all_existing", all_existing),
            run_case(exe, repo_root) | {"case": "root_current"},
        ]
        for name, value in sorted(all_existing.items()):
            rows.append(run_generated_case(tmp, exe, case_name_for_library(name), {name: value}))

    print(json.dumps({"output_contract": "nyrt-plugin-loadset-footprint-v0", "rows": rows}, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
