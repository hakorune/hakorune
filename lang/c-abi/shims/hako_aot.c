// hako_aot.c — Small standalone AOT C‑ABI
// Notes: duplicates minimal TLS + memory + AOT compile/link from kernel shim,
// so it can be linked independently as libhako_aot.{so|dylib|dll}.

#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include <stdio.h>
#include <time.h>

#if defined(_WIN32)
#include <process.h>
#define GETPID _getpid
#else
#include <unistd.h>
#define GETPID getpid
#endif

struct hako_ctx;

// Diagnostics helpers
#include "../include/hako_diag.h"

// ---- Shared diagnostics + memory (libc)
#include "hako_diag_mem_shared_impl.inc"

#include "hako_aot_shared_impl.inc"
