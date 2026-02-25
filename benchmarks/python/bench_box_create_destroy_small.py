#!/usr/bin/env python3
# Mirrors benchmarks/bench_box_create_destroy_small.hako
# Loop 1e4 times: create a tiny string, take length, accumulate.

def main():
    N = 10_000
    total = 0
    for _ in range(N):
        # Force a fresh allocation; avoid literal interning
        tmp = ''.join(['x'])
        total += len(tmp)
    # Print to avoid being optimized away
    print(total)

if __name__ == "__main__":
    main()

