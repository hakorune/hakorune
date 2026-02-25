#!/usr/bin/env python3

def main():
    n = 2_000_000
    s = "nyash"
    total = 0
    for _ in range(n):
        total += len(s)
    print(total)


if __name__ == "__main__":
    main()
