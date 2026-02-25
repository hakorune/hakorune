#!/usr/bin/env python3

def main():
    n = 1_000_000
    total = 0
    for _ in range(n):
        tmp = "".join(["x"])
        total += len(tmp)
    print(total)


if __name__ == "__main__":
    main()
