#!/usr/bin/env python3
# APP-PERF-02: kilo kernel benchmark (small) - Python reference
# Deterministic text-buffer workload inspired by enhanced_kilo_editor
# 60,000 iterations with string edit + replace scan


def main():
    rows = 64
    ops = 60000
    lines = [f"line-{i}" for i in range(rows)]
    undo = 0
    i = 0

    while i < ops:
        row = i % rows
        line = lines[row]
        split = len(line) // 2
        lines[row] = line[:split] + "xx" + line[split:]
        undo = undo + 1

        if (i % 8) == 0:
            j = 0
            while j < rows:
                current = lines[j]
                if "line" in current:
                    lines[j] = current + "ln"
                j = j + 1

        i = i + 1

    total = 0
    j = 0
    while j < rows:
        total = total + len(lines[j])
        j = j + 1

    print(total + undo)


if __name__ == "__main__":
    main()
