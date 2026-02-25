#!/usr/bin/env python3
# APP-PERF-01: CHIP-8 kernel benchmark (small) - Python reference
# Deterministic emulator-style workload for cross-language comparison
# 400,000 iterations with branch-heavy logic

def main():
    n = 400000
    i = 0
    pc = 0
    op = 7
    acc = 1
    sum_val = 0
    MOD = 1000000007

    while i < n:
        op = (op * 73 + 19) % 65536
        branch = op % 16

        if branch < 4:
            acc = (acc + (op % 251) + pc) % MOD
        elif branch < 8:
            acc = (acc + (op % 127) + 3) % MOD
        elif branch < 12:
            acc = (acc + (branch * branch) + (pc % 17)) % MOD
        else:
            acc = (acc + (op % 97) + (pc % 13) + 11) % MOD

        pc = (pc + 2 + (op % 3)) % 4096
        sum_val = (sum_val + acc + pc + branch) % MOD
        i = i + 1

    result = (sum_val + acc + pc) % MOD
    print(result)


if __name__ == "__main__":
    main()
