#include <stdint.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
  char *p;
} Str;

static inline Str *new_str(void) {
  Str *s = (Str *)malloc(sizeof(Str));
  s->p = strdup("x");
  return s;
}

static inline void drop_str(Str *s) {
  free(s->p);
  free(s);
}

int main(void) {
  volatile int64_t n = 1000000;
  volatile int64_t total = 0;
  for (int64_t i = 0; i < n; i++) {
    Str *tmp = new_str();
    total += (int64_t)strlen(tmp->p);
    drop_str(tmp);
  }
  return (int)(total & 0xFF);
}
