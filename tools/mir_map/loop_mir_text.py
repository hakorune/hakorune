"""Conservative alpha-renaming for the existing human-readable MIR dump."""
from collections import deque
import re

BLOCK = re.compile(r"bb(\d+):")
INSTRUCTION = re.compile(r"^\s*\d+:\s*(.+)$")
TOKEN = re.compile(r'"(?:\\.|[^"\\])*"|(?<![\w@.])(?:(%)(\d+)|\b(bb)(\d+)\b|\b(ValueId|BasicBlockId)\((\d+)\))')
PHI = re.compile(r"^(%\d+(?::.*?)?\s*=\s*phi)\s+(.+)$")
PAIR = re.compile(r"\[%([0-9]+), bb([0-9]+)\]")


def extract(text):
    start = text.find("; MIR Module:")
    if start < 0:
        return None
    lines = text[start:].splitlines()
    ends = [index for index, line in enumerate(lines) if line == "}"]
    return "\n".join(lines[:ends[-1] + 1]) if ends else None


def parse(text):
    functions = {}
    current = None
    for line in text.splitlines():
        if line.startswith(("; MIR Module:", "; Source:")) or not line:
            continue
        if line.startswith("define ") and line.endswith(" {") and current is None:
            name = re.search(r" @(.+?)\(", line)
            if name is None or name[1] in functions:
                raise ValueError("unexpected function header")
            current = name[1]
            functions[current] = (line, {})
            block = None
        elif line == "}" and current is not None:
            current = None
        elif current is not None and (match := BLOCK.fullmatch(line)):
            blocks = functions[current][1]
            block = int(match[1])
            if block in blocks:
                raise ValueError("duplicate block")
            blocks[block] = []
        elif current is not None and block is not None and (match := INSTRUCTION.fullmatch(line)):
            functions[current][1][block].append(match[1])
        else:
            raise ValueError("unsupported text in MIR dump")
    if current is not None or not functions:
        raise ValueError("incomplete MIR dump")
    return functions


def successors(instruction):
    jump = re.fullmatch(r"br label bb(\d+)", instruction)
    branch = re.fullmatch(r"br %\d+, label bb(\d+), label bb(\d+)", instruction)
    invoke = re.fullmatch(r"invoke .* normal=BasicBlockId\((\d+)\) fault=BasicBlockId\((\d+)\)", instruction)
    if match := jump or branch or invoke:
        return tuple(map(int, match.groups()))
    if instruction.startswith(("ret ", "return.fault ", "unreachable", "throw ")):
        return ()
    raise ValueError(f"unsupported terminator: {instruction}")


def block_order(blocks):
    edges = {block: successors(items[-1]) for block, items in blocks.items() if items}
    if set(edges) != set(blocks) or any(target not in blocks for pair in edges.values() for target in pair):
        raise ValueError("incomplete CFG")
    roots = set(blocks).difference(target for pair in edges.values() for target in pair)
    if len(roots) != 1:
        raise ValueError("entry is ambiguous in printed MIR")
    pending = deque(roots)
    seen = {}
    while pending:
        block = pending.popleft()
        if block not in seen:
            seen[block] = len(seen)
            pending.extend(edges[block])
    if set(seen) != set(blocks):
        raise ValueError("unreachable blocks in printed MIR")
    return seen


def replace_ids(text, blocks, values, define=False):
    def token(match):
        percent, value, bb, block, wrapper, wrapped = match.groups()
        if not (percent or bb or wrapper):
            return match[0]
        if bb or wrapper == "BasicBlockId":
            number = blocks[int(block or wrapped)]
            return f"bb{number}" if bb else f"BasicBlockId({number})"
        number = int(value or wrapped)
        if define:
            if number in values:
                raise ValueError("duplicate SSA definition")
            values[number] = len(values)
        return f"%{values[number]}" if percent else f"ValueId({values[number]})"
    return TOKEN.sub(token, text)


def normalize_function(header, blocks):
    order = block_order(blocks)
    values = {}
    normalized_header = replace_ids(header, order, values, define=True)
    for block in order:
        for instruction in blocks[block]:
            if match := re.match(r"^(%\d+)(?::.*?)?\s*=", instruction):
                replace_ids(match[1], order, values, define=True)

    def instruction_text(instruction):
        if not (phi := PHI.fullmatch(instruction)):
            return replace_ids(instruction, order, values)
        operands = list(PAIR.finditer(phi[2]))
        if not operands or ", ".join(pair[0] for pair in operands) != phi[2]:
            raise ValueError("unsupported PHI operands")
        incoming = sorted((order[int(pair[2])], values[int(pair[1])]) for pair in operands)
        lhs = replace_ids(phi[1], order, values)
        return lhs + " " + ", ".join(f"[%{value}, bb{block}]" for block, value in incoming)

    return normalized_header, {
        order[block]: [instruction_text(instruction) for instruction in blocks[block]]
        for block in order
    }


def normalize(text):
    try:
        return {name: normalize_function(*function) for name, function in parse(text).items()}
    except (ValueError, KeyError):
        return None


def locations(before, after):
    """Report a location for each differing instruction after available normalization."""
    try:
        left, right = before or {}, after or {}
        changes = []
        for name in sorted(left.keys() | right.keys()):
            if name not in left or name not in right:
                changes.append(dict(function=name, change="function added or removed"))
                continue
            lhead, lblocks = left[name]
            rhead, rblocks = right[name]
            if lhead != rhead:
                changes.append(dict(function=name, change="signature", before=lhead, after=rhead))
            for block in sorted(lblocks.keys() | rblocks.keys()):
                a, b = lblocks.get(block, []), rblocks.get(block, [])
                for i in range(max(len(a), len(b))):
                    old = a[i] if i < len(a) else None
                    new = b[i] if i < len(b) else None
                    if old != new:
                        changes.append(dict(function=name, block=block, instruction=i,
                                            before=old, after=new))
        return changes
    except (TypeError, ValueError):
        return []


def self_test():
    original = """; MIR Module: sample
define i64 @main(i1 %0) {
bb0:
    1: %1 = const 3
    1: br %0, label bb1, label bb2
bb1:
    2: %2 = const 7
    2: br label bb3
bb2:
    3: %3 = const 9
    3: br label bb3
bb3:
    4: %4 = phi [%2, bb1], [%3, bb2]
    4: ret %4
}"""
    equivalent = """; MIR Module: sample
define i64 @main(i1 %11) {
bb4:
    1: %12 = const 3
    1: br %11, label bb9, label bb8
bb7:
    4: %16 = phi [%13, bb8], [%15, bb9]
    4: ret %16
bb8:
    3: %13 = const 9
    3: br label bb7
bb9:
    2: %15 = const 7
    2: br label bb7
}"""
    loop = """; MIR Module: sample
define i64 @main(i1 %0, i1 %1) {
bb0:
    1: %2 = const 3
    1: br label bb1
bb1:
    2: %3 = phi [%2, bb0], [%4, bb4]
    2: br %0, label bb2, label bb3
bb2:
    3: %5 = const 7
    3: br label bb5
bb3:
    4: br %1, label bb4, label bb5
bb4:
    5: %4 = add %3, %2
    5: br label bb1
bb5:
    6: %6 = phi [%5, bb2], [%3, bb3]
    6: ret %6
}"""
    exit_phi = "[%2, bb1], [%3, bb2]"
    changes = {
        "phi-values-swapped": (original, original.replace(exit_phi, "[%3, bb1], [%2, bb2]")),
        "true-false-successors-swapped": (
            original, original.replace("label bb1, label bb2", "label bb2, label bb1")),
        "break-edge-value-changed": (loop, loop.replace("[%5, bb2]", "[%2, bb2]")),
    }
    held = normalize(original)
    assert held is not None and held == normalize(equivalent)
    for name, (before, after) in changes.items():
        base, mutated = normalize(before), normalize(after)
        assert base is not None and mutated is not None and mutated != base, name
        assert locations(base, mutated), name
    assert replace_ids('const "bb3 %1 ValueId(1)"', {3: 0}, {1: 0}) == (
        'const "bb3 %1 ValueId(1)"'
    )
    return {"positive_renumbering": 1, "mutations_detected": list(changes)}
