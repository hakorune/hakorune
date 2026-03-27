#include <stdint.h>

int main(void) {
  enum { SIZE = 128 };
  const int64_t ops = 2000000;
  int64_t arr[SIZE];

  for (int i = 0; i < SIZE; i++) {
    arr[i] = i;
  }

  for (int64_t i = 0; i < ops; i++) {
    int idx = (int)(i % SIZE);
    arr[idx] += 1;
  }

  return (int)((arr[0] + arr[SIZE - 1]) & 0xFF);
}
