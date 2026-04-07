#include <stdint.h>
#include <string.h>

int main(void) {
  const int64_t ops = 300000;
  const char text[] = "line-seed-abcdef";
  const int len = (int)(sizeof(text) - 1);
  const int split = len / 2;
  volatile int64_t acc = 0;

  for (int64_t i = 0; i < ops; i++) {
    char left[32];
    char right[32];
    memcpy(left, text, (size_t)split);
    left[split] = '\0';
    memcpy(right, text + split, (size_t)(len - split));
    right[len - split] = '\0';
    acc += (int64_t)split + (int64_t)(len - split);
  }

  return (int)(acc & 0xFF);
}
