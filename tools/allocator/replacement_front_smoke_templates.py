"""Focused smoke C sources for replacement-front benchmark probes."""

from __future__ import annotations


REPLACEMENT_FRONT_CROSS_THREAD_FREE_SMOKE_C = r"""
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>

static void* shared_ptr = 0;

static void* free_from_worker(void* arg) {
  (void)arg;
  free(shared_ptr);
  shared_ptr = 0;
  return 0;
}

int main(void) {
  shared_ptr = malloc(64);
  if (!shared_ptr) {
    fputs("malloc failed\n", stderr);
    return 1;
  }
  pthread_t thread;
  if (pthread_create(&thread, 0, free_from_worker, 0) != 0) {
    fputs("pthread_create failed\n", stderr);
    return 2;
  }
  if (pthread_join(thread, 0) != 0) {
    fputs("pthread_join failed\n", stderr);
    return 3;
  }
  void* drained = malloc(64);
  if (!drained) {
    fputs("drain malloc failed\n", stderr);
    return 4;
  }
  free(drained);
  return 0;
}
"""


REPLACEMENT_FRONT_ABANDONED_OWNER_SMOKE_C = r"""
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>

static void* shared_ptr = 0;

static void* allocate_from_worker(void* arg) {
  (void)arg;
  shared_ptr = malloc(64);
  if (!shared_ptr) {
    return (void*)1;
  }
  return 0;
}

int main(void) {
  pthread_t thread;
  void* thread_result = 0;
  if (pthread_create(&thread, 0, allocate_from_worker, 0) != 0) {
    fputs("pthread_create failed\n", stderr);
    return 1;
  }
  if (pthread_join(thread, &thread_result) != 0) {
    fputs("pthread_join failed\n", stderr);
    return 2;
  }
  if (thread_result != 0 || !shared_ptr) {
    fputs("worker malloc failed\n", stderr);
    return 3;
  }
  free(shared_ptr);
  shared_ptr = 0;
  return 0;
}
"""


REPLACEMENT_FRONT_CROSS_THREAD_REALLOC_SMOKE_C = r"""
#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>

static void* shared_ptr = 0;
static void* realloc_result = 0;

static void* realloc_from_worker(void* arg) {
  (void)arg;
  realloc_result = realloc(shared_ptr, 128);
  return 0;
}

int main(void) {
  shared_ptr = malloc(64);
  if (!shared_ptr) {
    fputs("malloc failed\n", stderr);
    return 1;
  }
  pthread_t thread;
  if (pthread_create(&thread, 0, realloc_from_worker, 0) != 0) {
    fputs("pthread_create failed\n", stderr);
    return 2;
  }
  if (pthread_join(thread, 0) != 0) {
    fputs("pthread_join failed\n", stderr);
    return 3;
  }
  if (realloc_result != 0) {
    fputs("cross-thread realloc unexpectedly succeeded\n", stderr);
    return 4;
  }
  free(shared_ptr);
  shared_ptr = 0;
  return 0;
}
"""


REPLACEMENT_FRONT_MALLOC_FAMILY_SMOKE_C = r"""
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
  void* p = malloc(64);
  if (!p) {
    fputs("malloc failed\n", stderr);
    return 1;
  }
  memset(p, 0x5a, 64);

  void* z = calloc(4, 16);
  if (!z) {
    fputs("calloc failed\n", stderr);
    return 2;
  }
  for (size_t i = 0; i < 64; i++) {
    if (((unsigned char*)z)[i] != 0) {
      fputs("calloc did not zero\n", stderr);
      return 3;
    }
  }

  void* r = realloc(p, 80);
  if (!r) {
    fputs("realloc failed\n", stderr);
    return 4;
  }
  p = r;
  free(0);
  free(z);
  free(p);
  return 0;
}
"""
