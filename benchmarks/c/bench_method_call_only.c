#include <stdint.h>
#include <string.h>

int main(void) {
  volatile int64_t n = 2000000;
  volatile int64_t total = 0;
  const char *s = "nyash";
  for (int64_t i = 0; i < n; i++) {
    total += (int64_t)strlen(s);
  }
  return (int)(total & 0xFF);
}
