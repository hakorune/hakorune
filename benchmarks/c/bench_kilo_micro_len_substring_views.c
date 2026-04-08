#include <stdint.h>
#include <string.h>

int main(void) {
  const int64_t ops = 300000;
  const char text[] = "line-seed-abcdef";
  const int len = (int)(sizeof(text) - 1);
  const int split = len / 2;
  char left[32];
  char right[32];
  volatile int64_t acc = 0;

  memcpy(left, text, (size_t)split);
  left[split] = '\0';
  memcpy(right, text + split, (size_t)(len - split));
  right[len - split] = '\0';

  for (int64_t i = 0; i < ops; i++) {
    acc += (int64_t)strlen(left) + (int64_t)strlen(right);
  }

  return (int)((acc + len) & 0xFF);
}
