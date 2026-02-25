#!/usr/bin/env python3
# bench_numeric_mixed_medium.py — 整数演算+分岐+mod ベンチマーク (Python reference)
# Phase 21.5: String偏重ではない数値計算ベンチ

def main():
    i = 0
    acc = 1
    sum = 0
    n = 800000

    while i < n:
        m = i % 31
        t = (i * 3) + (m * 7) + acc

        if m < 10:
            acc = acc + (t % 97) + 1
        elif m < 20:
            acc = acc + (t % 89) + 3
        else:
            acc = acc + (m * m) + (t % 53)

        sum = sum + (acc % 17)
        i = i + 1

    print(sum + acc)

if __name__ == "__main__":
    main()
