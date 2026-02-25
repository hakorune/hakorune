#include <stdint.h>

int main(void) {
  const int64_t size = 128;
  const int64_t ops = 2000000;
  int64_t arr[128];

  for (int64_t i = 0; i < size; i++) {
    arr[i] = i;
  }

  volatile int64_t sum = 0;
  for (int64_t i = 0; i < ops; i++) {
    int64_t idx = i % size;
    int64_t v = arr[idx];
    arr[idx] = v + 1;
    sum += arr[idx];
  }

  return (int)(sum & 0xFF);
}
