// bench_numeric_mixed_medium.c — 整数演算+分岐+mod ベンチマーク (C reference)
// Phase 21.5: String偏重ではない数値計算ベンチ

#include <stdint.h>

int main(void) {
  volatile int64_t i = 0;
  volatile int64_t acc = 1;
  volatile int64_t sum = 0;
  volatile int64_t n = 800000;

  for (; i < n; i++) {
    int64_t m = i % 31;
    int64_t t = (i * 3) + (m * 7) + acc;

    if (m < 10) {
      acc = acc + (t % 97) + 1;
    } else if (m < 20) {
      acc = acc + (t % 89) + 3;
    } else {
      acc = acc + (m * m) + (t % 53);
    }

    sum = sum + (acc % 17);
  }

  return (int)((sum + acc) & 0xFF);
}
