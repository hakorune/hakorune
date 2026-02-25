#!/usr/bin/env python3
# Mirrors benchmarks/bench_method_call_only_small.hako
# Loop 5e3 times: take length of preallocated string and accumulate.

def main():
    N = 5_000
    s = "nyash"
    total = 0
    for _ in range(N):
        total += len(s)
    print(total)

if __name__ == "__main__":
    main()

