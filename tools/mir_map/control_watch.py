"""Source control-flow watchlist selected from emission sites and route consumers."""
from collections import Counter
from pathlib import Path
import json

from support import ROOT


def watchlist():
    path = Path(__file__).with_name("control_flow_sites.json")
    rows = json.loads(path.read_text())
    files = {}
    seen = set()
    for row in rows:
        if row["id"] in seen or not row["sites"] or not row["evidence"]:
            raise ValueError(f"missing or duplicate watch evidence: {row['id']}")
        seen.add(row["id"])
        file = row["file"]
        if file not in files:
            files[file] = (ROOT / file).read_text().splitlines()
        for site in row["sites"]:
            actual = files[file][site["line"] - 1].strip()
            if actual != site["code"]:
                raise ValueError(f"watch evidence moved: {file}:{site['line']}")
    return rows


def hits(membership, rows):
    lines = membership["lines"]
    return [
        row["id"] for row in rows
        if any(site["line"] in lines.get(row["file"], ()) for site in row["sites"])
    ]


def summary(rows, universe, a, b):
    result = []
    for row in rows:
        lines = universe.get(row["file"], set())
        has_all_sites = all(site["line"] in lines for site in row["sites"])

        def count(data):
            observed = data["control_watch_counts"][row["id"]]
            return observed if (observed or has_all_sites and data["full"]) else None

        result.append(row | {"A_inputs": count(a), "B_inputs": count(b),
                             "all_sites_instrumented": has_all_sites})
    return result


def markdown_counts(rows):
    counts = Counter((row["category"], row["role"]) for row in rows)
    categories = [("if", "if"), ("loop-header-phi", "loop header PHI"),
                  ("loop-break", "break"), ("loop-continue", "continue")]
    return [
        "", "## 制御フローの定義数", "",
        "| 対象 | 受け口 | 独立の定義 | 共通部品 |",
        "|---|---:|---:|---:|",
        *[f"| {name} | {counts[key, 'forwarder']} | {counts[key, 'writer']} | {counts[key, 'shared']} |"
          for key, name in categories],
        "", "発行・転送の根拠は `tools/mir_map/control_flow_sites.json`、"
        "入力ごとの到達から集計した A/B 件数は生成結果の `control_flow_watchlist`。"
    ]
