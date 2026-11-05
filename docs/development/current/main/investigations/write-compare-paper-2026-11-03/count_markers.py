#!/usr/bin/env python3
"""Count ownership markers and code lines in the write-comparison files.

Counts are regex based over source with comments and string contents removed.
Sections are delimited by a line comment of the form `// === NAME ===`.
Text before the first delimiter belongs to section COMMON.

Hakorune columns: move, take (decl / expr), share, from, mut, `?` (type suffix),
plus copy and the placeholders for constructs with no decided spelling, which
are written as block comments such as /*mut me*/.
Rust columns are a rough token count for comparison only.
"""
import re
import sys
from collections import OrderedDict
from pathlib import Path

HERE = Path(__file__).resolve().parent
SECTION = re.compile(r"^\s*//\s*===\s*(.+?)\s*===\s*$")
PLACEHOLDER = re.compile(r"/\*\s*(mut me|take me|capture[^*]*)\*/")


def strip_code(text):
    """Remove comments and blank string contents, keeping line structure."""
    out = []
    i = 0
    n = len(text)
    while i < n:
        c = text[i]
        if c == '"':
            out.append('""')
            i += 1
            while i < n and text[i] != '"':
                if text[i] == "\\":
                    i += 1
                i += 1
            i += 1
        elif text.startswith("//", i):
            while i < n and text[i] != "\n":
                i += 1
        elif text.startswith("/*", i):
            j = text.find("*/", i + 2)
            j = n if j < 0 else j + 2
            out.append("\n" * text.count("\n", i, j))
            i = j
        else:
            out.append(c)
            i += 1
    return "".join(out)


def split_sections(text):
    sections = OrderedDict()
    name = "COMMON"
    sections[name] = []
    for line in text.splitlines():
        m = SECTION.match(line)
        if m:
            name = m.group(1)
            sections.setdefault(name, [])
        else:
            sections[name].append(line)
    return OrderedDict((k, "\n".join(v)) for k, v in sections.items())


def code_lines(stripped):
    return sum(1 for l in stripped.splitlines() if l.strip())


def hako_counts(raw):
    code = strip_code(raw)
    take_decl = len(re.findall(r"\btake\s+[A-Za-z_]\w*\s*:", code))
    take_all = len(re.findall(r"\btake\b", code))
    no_line_comments = re.sub(r"//[^\n]*", "", raw)
    placeholders = PLACEHOLDER.findall(no_line_comments)
    return OrderedDict(
        loc=code_lines(code),
        move=len(re.findall(r"\bmove\b", code)),
        take_decl=take_decl,
        take_expr=take_all - take_decl,
        share=len(re.findall(r"\bshare\b", code)),
        from_=len(re.findall(r"\bfrom\b", code)),
        mut=len(re.findall(r"\bmut\b", code)),
        nullable=len(re.findall(r"[\w>]\?", code)),
        copy=len(re.findall(r"\bcopy\b", code)),
        unspelled=len(placeholders),
    )


def rust_counts(raw):
    code = strip_code(raw)
    code_no_attr = re.sub(r"#!?\[[^\]]*\]", "", code)
    return OrderedDict(
        loc=code_lines(code),
        ref=len(re.findall(r"(?<!&)&(?!&)(?:'\w+\s+)?(?!mut\b)", code_no_attr)),
        ref_mut=len(re.findall(r"&(?:'\w+\s+)?mut\b", code_no_attr)),
        box_=len(re.findall(r"\bBox\b", code_no_attr)),
        deref=len(re.findall(r"(?<![\w)\]])\*(?=[\w(&])", code_no_attr)),
        option=len(re.findall(r"\bOption<", code_no_attr)),
        take_=len(re.findall(r"\.take\(\)", code_no_attr)),
        clone=len(re.findall(r"\.clone\(\)|Rc::clone", code_no_attr)),
        replace=len(re.findall(r"mem::replace", code_no_attr)),
        lifetime=len(re.findall(r"'[a-z]\b", code_no_attr)),
    )


def table(rows, counts_fn):
    keys = None
    lines = []
    for label, raw in rows:
        c = counts_fn(raw)
        if keys is None:
            keys = list(c.keys())
            lines.append("| section | " + " | ".join(keys) + " |")
            lines.append("|" + " --- |" * (len(keys) + 1))
        lines.append("| " + label + " | " + " | ".join(str(c[k]) for k in keys) + " |")
    return lines


def main():
    tasks = sorted(p.stem for p in HERE.glob("t*.hako"))
    print("## Hakorune\n")
    for stem in tasks:
        text = (HERE / f"{stem}.hako").read_text(encoding="utf-8")
        rows = [("TOTAL", text)]
        secs = split_sections(text)
        if len(secs) > 1:
            rows += list(secs.items())
        print(f"### {stem}\n")
        print("\n".join(table(rows, hako_counts)))
        print()
    print("## Rust (rough token counts)\n")
    for stem in tasks:
        path = HERE / f"{stem}.rs"
        if not path.exists():
            print(f"### {stem}: missing .rs\n", file=sys.stderr)
            continue
        text = path.read_text(encoding="utf-8")
        rows = [("TOTAL", text)]
        secs = split_sections(text)
        if len(secs) > 1:
            rows += list(secs.items())
        print(f"### {stem}\n")
        print("\n".join(table(rows, rust_counts)))
        print()


if __name__ == "__main__":
    main()
